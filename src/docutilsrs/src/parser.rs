//! Minimal rST parser slice.
//!
//! Supports:
//! - blank-line-separated paragraphs
//! - inline emphasis `*x*`, strong `**x**`, inline literal `` ``x`` ``
//!
//! Out of scope for the slice (tracked in `docs/compat.md`):
//! lists, tables, directives, roles, references, substitutions, line blocks,
//! block quotes, transitions, sections, multi-line inline spans across
//! paragraphs, escape sequences, character-level start/end-string rules.
//!
//! The inline rules follow the spirit of the docutils inline markup
//! recognition rules: a start marker must be preceded by whitespace, line
//! start, or open-punctuation, and must not be immediately followed by
//! whitespace; the matching end marker must not be immediately preceded by
//! whitespace and must be followed by whitespace, line end, or
//! close-punctuation.

use crate::doctree::{Doctree, NodeId, NodeKind};

pub fn parse_rst(source: &str) -> Doctree {
    let mut tree = Doctree::new_document("<string>");
    let document = tree.root();

    for block in split_paragraphs(source) {
        let paragraph = tree.append(document, NodeKind::Paragraph);
        parse_inline(&mut tree, paragraph, &block);
    }

    tree
}

fn split_paragraphs(source: &str) -> Vec<String> {
    let mut paragraphs = Vec::new();
    let mut current: Vec<&str> = Vec::new();

    for line in source.lines() {
        if line.trim().is_empty() {
            if !current.is_empty() {
                paragraphs.push(current.join("\n"));
                current.clear();
            }
        } else {
            current.push(line);
        }
    }
    if !current.is_empty() {
        paragraphs.push(current.join("\n"));
    }

    paragraphs
}

/// Inline markup kind, in priority order. `**` must be tried before `*`.
#[derive(Clone, Copy)]
enum InlineKind {
    Strong,   // **...**
    Literal,  // ``...``
    Emphasis, // *...*
}

impl InlineKind {
    fn marker(self) -> &'static str {
        match self {
            InlineKind::Strong => "**",
            InlineKind::Literal => "``",
            InlineKind::Emphasis => "*",
        }
    }

    fn node(self) -> NodeKind {
        match self {
            InlineKind::Strong => NodeKind::Strong,
            InlineKind::Literal => NodeKind::Literal,
            InlineKind::Emphasis => NodeKind::Emphasis,
        }
    }

    /// rST inline markup does not nest within itself in the phase 1 slice.
    /// All inline spans contain a single text node.
    fn allows_nesting(self) -> bool {
        false
    }
}

const ORDER: [InlineKind; 3] = [
    InlineKind::Strong,
    InlineKind::Literal,
    InlineKind::Emphasis,
];

fn parse_inline(tree: &mut Doctree, parent: NodeId, text: &str) {
    let bytes = text.as_bytes();
    let mut cursor = 0;
    let mut text_start = 0;

    while cursor < bytes.len() {
        if let Some((kind, end_after_close)) = try_match_inline(text, cursor) {
            // Flush pending plain text.
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }

            let marker_len = kind.marker().len();
            let inner_start = cursor + marker_len;
            let inner_end = end_after_close - marker_len;
            let inner = &text[inner_start..inner_end];

            let node = tree.append(parent, kind.node());
            if kind.allows_nesting() {
                parse_inline(tree, node, inner);
            } else {
                push_text(tree, node, inner);
            }

            cursor = end_after_close;
            text_start = cursor;
        } else {
            cursor += utf8_char_len(bytes[cursor]);
        }
    }

    if text_start < bytes.len() {
        push_text(tree, parent, &text[text_start..]);
    }
}

fn push_text(tree: &mut Doctree, parent: NodeId, s: &str) {
    if !s.is_empty() {
        tree.append(parent, NodeKind::Text(s.to_string()));
    }
}

/// Try to start an inline span at `start`. Returns `(kind, end)` where `end`
/// is the byte index just past the closing marker.
fn try_match_inline(text: &str, start: usize) -> Option<(InlineKind, usize)> {
    for &kind in &ORDER {
        let marker = kind.marker();
        if !text[start..].starts_with(marker) {
            continue;
        }
        if !valid_start_context(text, start) {
            continue;
        }
        let content_start = start + marker.len();
        if content_start >= text.len() {
            continue;
        }
        // No whitespace immediately after the opening marker.
        let first = text[content_start..].chars().next()?;
        if first.is_whitespace() {
            continue;
        }
        if let Some(end_of_marker) = find_close(text, content_start, marker) {
            return Some((kind, end_of_marker));
        }
    }
    None
}

/// Search for a valid closing marker starting from `from`. Returns the byte
/// index just past the closing marker.
fn find_close(text: &str, from: usize, marker: &str) -> Option<usize> {
    let mut search = from;
    while let Some(rel) = text[search..].find(marker) {
        let abs = search + rel;
        // Must not be preceded by whitespace.
        let prev_char = text[..abs].chars().next_back()?;
        if prev_char.is_whitespace() {
            search = abs + marker.len();
            continue;
        }
        // For "*", reject if it's actually part of "**".
        if marker == "*" && text[abs..].starts_with("**") {
            search = abs + 1;
            continue;
        }
        let end = abs + marker.len();
        if !valid_end_context(text, end) {
            search = end;
            continue;
        }
        return Some(end);
    }
    None
}

fn valid_start_context(text: &str, start: usize) -> bool {
    if start == 0 {
        return true;
    }
    let prev = text[..start].chars().next_back();
    match prev {
        None => true,
        Some(c) => c.is_whitespace() || is_open_punct(c),
    }
}

fn valid_end_context(text: &str, end: usize) -> bool {
    if end >= text.len() {
        return true;
    }
    let next = text[end..].chars().next();
    match next {
        None => true,
        Some(c) => c.is_whitespace() || is_close_punct(c),
    }
}

fn is_open_punct(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<' | '\'' | '"')
}

fn is_close_punct(c: char) -> bool {
    matches!(
        c,
        ')' | ']' | '}' | '>' | '\'' | '"' | '.' | ',' | ':' | ';' | '!' | '?' | '-' | '/' | '\\'
    )
}

fn utf8_char_len(first_byte: u8) -> usize {
    match first_byte {
        b if b < 0x80 => 1,
        b if b < 0xC0 => 1, // continuation byte; shouldn't be a cursor position
        b if b < 0xE0 => 2,
        b if b < 0xF0 => 3,
        _ => 4,
    }
}
