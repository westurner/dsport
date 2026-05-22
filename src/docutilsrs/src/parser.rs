//! Minimal rST parser slice.
//!
//! Supports:
//! - blank-line-separated paragraphs
//! - bullet lists with markers `-`, `*`, `+` (one item per line; no
//!   continuation lines, no nested lists)
//! - inline emphasis `*x*`, strong `**x**`, inline literal `` ``x`` ``
//! - backslash escapes (`\X` emits literal `X`; `\<ws>` is removed)
//!
//! Out of scope for the slice (tracked in `docs/compat.md`):
//! enumerated lists, definition lists, field lists, tables, directives,
//! roles, references, substitutions, line blocks, block quotes,
//! transitions, sections, multi-line inline spans across paragraphs,
//! list-item continuation lines, nested lists.

use crate::doctree::{Doctree, NodeId, NodeKind};

pub fn parse_rst(source: &str) -> Doctree {
    let mut tree = Doctree::new_document("<string>");
    let document = tree.root();

    for block in split_blocks(source) {
        emit_block(&mut tree, document, block);
    }

    tree
}

// ────────────────────────────────────────────────────────────────────────────
// Block-level
// ────────────────────────────────────────────────────────────────────────────

enum Block {
    Paragraph(String),
    BulletList { bullet: char, items: Vec<String> },
}

fn split_blocks(source: &str) -> Vec<Block> {
    let lines: Vec<&str> = source.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        if let Some((bullet, _)) = bullet_marker(line) {
            let mut items = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    // Peek past blank lines: still part of the list only if
                    // the next non-blank line uses the same bullet.
                    let mut j = i + 1;
                    while j < lines.len() && lines[j].trim().is_empty() {
                        j += 1;
                    }
                    let cont =
                        j < lines.len() && bullet_marker(lines[j]).map(|(b, _)| b) == Some(bullet);
                    if cont {
                        i = j;
                        continue;
                    }
                    break;
                }
                if let Some((b, rest)) = bullet_marker(l) {
                    if b != bullet {
                        break;
                    }
                    items.push(rest.to_string());
                    i += 1;
                } else {
                    break;
                }
            }
            blocks.push(Block::BulletList { bullet, items });
            continue;
        }
        let mut buf: Vec<&str> = Vec::new();
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() || bullet_marker(l).is_some() {
                break;
            }
            buf.push(l);
            i += 1;
        }
        if !buf.is_empty() {
            blocks.push(Block::Paragraph(buf.join("\n")));
        }
    }
    blocks
}

/// `(bullet_char, rest_of_line)` if `line` starts with `-`, `*`, or `+`
/// followed by a space and at least one non-space character.
fn bullet_marker(line: &str) -> Option<(char, &str)> {
    let first = line.chars().next()?;
    if !matches!(first, '-' | '*' | '+') {
        return None;
    }
    let after = &line[first.len_utf8()..];
    let mut chars = after.chars();
    let second = chars.next()?;
    if second != ' ' {
        return None;
    }
    let rest = chars.as_str();
    if rest.trim().is_empty() {
        return None;
    }
    Some((first, rest))
}

fn emit_block(tree: &mut Doctree, parent: NodeId, block: Block) {
    match block {
        Block::Paragraph(text) => {
            let p = tree.append(parent, NodeKind::Paragraph);
            parse_inline(tree, p, &text);
        }
        Block::BulletList { bullet, items } => {
            let list = tree.append(parent, NodeKind::BulletList { bullet });
            for item in items {
                let li = tree.append(list, NodeKind::ListItem);
                let p = tree.append(li, NodeKind::Paragraph);
                parse_inline(tree, p, &item);
            }
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline
// ────────────────────────────────────────────────────────────────────────────

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
}

const ORDER: [InlineKind; 3] = [
    InlineKind::Strong,
    InlineKind::Literal,
    InlineKind::Emphasis,
];

/// Result of escape preprocessing.
struct Escaped {
    /// Text with escapes resolved: `\X` → `X`, `\<ws>` → "".
    text: String,
    /// One entry per byte of `text`. `true` means that byte originated from
    /// an escaped character and must never be treated as inline-markup
    /// syntax.
    escaped: Vec<bool>,
}

fn preprocess_escapes(input: &str) -> Escaped {
    let mut text = String::with_capacity(input.len());
    let mut escaped: Vec<bool> = Vec::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek().copied() {
                Some(nc) if nc.is_whitespace() => {
                    chars.next();
                }
                Some(nc) => {
                    chars.next();
                    text.push(nc);
                    escaped.resize(text.len(), true);
                }
                None => {
                    text.push('\\');
                    escaped.resize(text.len(), false);
                }
            }
            continue;
        }
        text.push(c);
        escaped.resize(text.len(), false);
    }
    Escaped { text, escaped }
}

fn parse_inline(tree: &mut Doctree, parent: NodeId, raw: &str) {
    let pre = preprocess_escapes(raw);
    let text = &pre.text;
    let bytes = text.as_bytes();
    let mut cursor = 0;
    let mut text_start = 0;

    while cursor < bytes.len() {
        if let Some((kind, end_after_close)) = try_match_inline(text, &pre.escaped, cursor) {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            let marker_len = kind.marker().len();
            let inner = &text[cursor + marker_len..end_after_close - marker_len];
            let node = tree.append(parent, kind.node());
            push_text(tree, node, inner);
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

fn try_match_inline(text: &str, escaped: &[bool], start: usize) -> Option<(InlineKind, usize)> {
    for &kind in &ORDER {
        let marker = kind.marker();
        if !text[start..].starts_with(marker) {
            continue;
        }
        if escaped[start..start + marker.len()].iter().any(|&e| e) {
            continue;
        }
        if !valid_start_context(text, start) {
            continue;
        }
        let content_start = start + marker.len();
        if content_start >= text.len() {
            continue;
        }
        let first = text[content_start..].chars().next()?;
        if first.is_whitespace() {
            continue;
        }
        if let Some(end_of_marker) = find_close(text, escaped, content_start, marker) {
            return Some((kind, end_of_marker));
        }
    }
    None
}

fn find_close(text: &str, escaped: &[bool], from: usize, marker: &str) -> Option<usize> {
    let mut search = from;
    while let Some(rel) = text[search..].find(marker) {
        let abs = search + rel;
        if escaped[abs..abs + marker.len()].iter().any(|&e| e) {
            search = abs + 1;
            continue;
        }
        let prev_char = text[..abs].chars().next_back()?;
        if prev_char.is_whitespace() {
            search = abs + marker.len();
            continue;
        }
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
        b if b < 0xC0 => 1,
        b if b < 0xE0 => 2,
        b if b < 0xF0 => 3,
        _ => 4,
    }
}
