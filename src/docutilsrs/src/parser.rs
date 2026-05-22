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

use std::collections::HashMap;

use crate::doctree::{Doctree, NodeId, NodeKind};

pub fn parse_rst(source: &str) -> Doctree {
    parse_rst_with_source(source, "<string>")
}

pub fn parse_rst_with_source(source: &str, source_path: &str) -> Doctree {
    let mut tree = Doctree::new_document(source_path);
    let document = tree.root();

    for block in split_blocks(source) {
        emit_block(&mut tree, document, block);
    }

    resolve_references(&mut tree);
    tree
}

// ────────────────────────────────────────────────────────────────────────────
// Block-level
// ────────────────────────────────────────────────────────────────────────────

enum Block {
    Paragraph(String),
    BulletList {
        bullet: char,
        items: Vec<String>,
    },
    EnumeratedList {
        enumtype: &'static str,
        prefix: String,
        suffix: String,
        start: Option<u32>,
        items: Vec<String>,
    },
    /// Explicit hyperlink target: `.. _name: refuri`.
    Target {
        name: String,
        refuri: String,
    },
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
        if let Some((name, refuri)) = explicit_target(line) {
            blocks.push(Block::Target { name, refuri });
            i += 1;
            continue;
        }
        if let Some((bullet, _)) = bullet_marker(line) {
            let mut items: Vec<String> = Vec::new();
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
                    // Continuation lines: subsequent lines indented past the
                    // bullet, with no new bullet marker, belong to the
                    // current item.
                    let indent_cols = 2usize; // "- " etc.
                    while i < lines.len() {
                        let cl = lines[i];
                        if cl.trim().is_empty() {
                            break;
                        }
                        if bullet_marker(cl).is_some() {
                            break;
                        }
                        let stripped = match cl.strip_prefix(&" ".repeat(indent_cols)) {
                            Some(s) => s,
                            None => break,
                        };
                        let last = items.last_mut().expect("just pushed");
                        last.push('\n');
                        last.push_str(stripped);
                        i += 1;
                    }
                } else {
                    break;
                }
            }
            blocks.push(Block::BulletList { bullet, items });
            continue;
        }
        if let Some((enumtype, prefix, suffix, first_value, _rest)) = enum_marker(line) {
            // Resolve ambiguity: a single-letter alpha value that is also a
            // valid roman digit (i, v, x, l, c, d, m / I, V, X, L, C, D, M)
            // is treated as roman if the next item's value is unambiguously
            // roman (multi-char). Otherwise we keep the alpha classification.
            let enumtype = disambiguate_alpha_roman(enumtype, &prefix, &suffix, &lines, i);
            let start = enumerator_value(enumtype, &first_value).filter(|&v| v != 1);
            let mut items: Vec<String> = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    let mut j = i + 1;
                    while j < lines.len() && lines[j].trim().is_empty() {
                        j += 1;
                    }
                    let cont = j < lines.len()
                        && enum_marker(lines[j])
                            .map(|m| {
                                m.1 == prefix && m.2 == suffix && value_matches(enumtype, &m.3)
                            })
                            .unwrap_or(false);
                    if cont {
                        i = j;
                        continue;
                    }
                    break;
                }
                let m = match enum_marker(l) {
                    Some(m) if m.1 == prefix && m.2 == suffix && value_matches(enumtype, &m.3) => m,
                    _ => break,
                };
                items.push(m.4.to_string());
                i += 1;
                // Continuation lines: indent must match the enumerator width
                // (prefix + value + suffix + space).
                let indent_cols = prefix.len() + m.3.len() + suffix.len() + 1;
                while i < lines.len() {
                    let cl = lines[i];
                    if cl.trim().is_empty()
                        || bullet_marker(cl).is_some()
                        || enum_marker(cl).is_some()
                    {
                        break;
                    }
                    let stripped = match cl.strip_prefix(&" ".repeat(indent_cols)) {
                        Some(s) => s,
                        None => break,
                    };
                    let last = items.last_mut().expect("just pushed");
                    last.push('\n');
                    last.push_str(stripped);
                    i += 1;
                }
            }
            blocks.push(Block::EnumeratedList {
                enumtype,
                prefix,
                suffix,
                start,
                items,
            });
            continue;
        }
        let mut buf: Vec<&str> = Vec::new();
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty()
                || bullet_marker(l).is_some()
                || enum_marker(l).is_some()
                || explicit_target(l).is_some()
            {
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

/// Try to match an enumerated-list marker at the start of `line`.
///
/// Returns `(enumtype, prefix, suffix, value, rest_of_line)`. Supported
/// enumerators: arabic (`1`), loweralpha (`a`), upperalpha (`A`),
/// lowerroman (`i`), upperroman (`I`), plus auto-enumerator `#`. Supported
/// prefix/suffix: bare or `(value)` or `value.` or `value)`.
fn enum_marker(line: &str) -> Option<(&'static str, String, String, String, &str)> {
    let bytes = line.as_bytes();
    let (prefix, value_start) = if bytes.first() == Some(&b'(') {
        ("(", 1)
    } else {
        ("", 0)
    };
    // Auto-enumerator `#` is a single-character value.
    let mut end = value_start;
    if bytes.get(end) == Some(&b'#') {
        end += 1;
    } else {
        while end < bytes.len() && bytes[end].is_ascii_alphanumeric() {
            end += 1;
        }
    }
    if end == value_start {
        return None;
    }
    let value = &line[value_start..end];
    let suffix_byte = *bytes.get(end)?;
    let suffix = match (prefix, suffix_byte) {
        ("(", b')') => ")",
        ("", b'.') => ".",
        ("", b')') => ")",
        _ => return None,
    };
    let after_suffix = end + 1;
    // Must be followed by a single space and at least one non-space char,
    // or be the whole line (empty item — not supported).
    let rest = line.get(after_suffix..)?;
    if !rest.starts_with(' ') {
        return None;
    }
    let body = &rest[1..];
    if body.trim().is_empty() {
        return None;
    }
    let enumtype = classify_enumerator(value)?;
    Some((
        enumtype,
        prefix.to_string(),
        suffix.to_string(),
        value.to_string(),
        body,
    ))
}

/// Numeric value of an enumerator (1-based). Returns `None` for `#`.
fn enumerator_value(enumtype: &'static str, value: &str) -> Option<u32> {
    if value == "#" {
        return Some(1);
    }
    match enumtype {
        "arabic" => value.parse().ok(),
        "loweralpha" => value
            .chars()
            .next()
            .filter(|c| c.is_ascii_lowercase())
            .map(|c| (c as u32) - (b'a' as u32) + 1),
        "upperalpha" => value
            .chars()
            .next()
            .filter(|c| c.is_ascii_uppercase())
            .map(|c| (c as u32) - (b'A' as u32) + 1),
        "lowerroman" => roman_to_int(&value.to_uppercase()),
        "upperroman" => roman_to_int(value),
        _ => None,
    }
}

fn roman_to_int(s: &str) -> Option<u32> {
    let mut total: i64 = 0;
    let mut prev: i64 = 0;
    for c in s.chars().rev() {
        let v: i64 = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return None,
        };
        if v < prev {
            total -= v;
        } else {
            total += v;
            prev = v;
        }
    }
    if total <= 0 {
        None
    } else {
        Some(total as u32)
    }
}

/// Whether `value` is a valid enumerator value for `enumtype`.
/// `#` matches any enumerator type (auto-numbering).
fn value_matches(enumtype: &'static str, value: &str) -> bool {
    if value == "#" {
        return true;
    }
    match enumtype {
        "arabic" => value.chars().all(|c| c.is_ascii_digit()),
        "loweralpha" => value.len() == 1 && value.chars().next().unwrap().is_ascii_lowercase(),
        "upperalpha" => value.len() == 1 && value.chars().next().unwrap().is_ascii_uppercase(),
        "lowerroman" => value
            .chars()
            .all(|c| matches!(c, 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm')),
        "upperroman" => value
            .chars()
            .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M')),
        _ => false,
    }
}

fn classify_enumerator(value: &str) -> Option<&'static str> {
    if value == "#" {
        return Some("arabic");
    }
    if value.chars().all(|c| c.is_ascii_digit()) {
        return Some("arabic");
    }
    if value.len() == 1 {
        let c = value.chars().next().unwrap();
        if c.is_ascii_lowercase() {
            return Some("loweralpha");
        }
        if c.is_ascii_uppercase() {
            return Some("upperalpha");
        }
    }
    if value
        .chars()
        .all(|c| matches!(c, 'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm'))
    {
        return Some("lowerroman");
    }
    if value
        .chars()
        .all(|c| matches!(c, 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'))
    {
        return Some("upperroman");
    }
    None
}

/// If `enumtype` came out as `loweralpha`/`upperalpha` but the first item's
/// value is a roman digit AND the next list item's value is unambiguously
/// roman (multi-letter), reclassify as `lowerroman`/`upperroman`.
fn disambiguate_alpha_roman(
    enumtype: &'static str,
    prefix: &str,
    suffix: &str,
    lines: &[&str],
    first_idx: usize,
) -> &'static str {
    let target_roman = match enumtype {
        "loweralpha" => "lowerroman",
        "upperalpha" => "upperroman",
        _ => return enumtype,
    };
    // First item's value must itself be a roman digit.
    let first_value = match enum_marker(lines[first_idx]) {
        Some(m) => m.3,
        None => return enumtype,
    };
    if first_value.len() != 1 {
        return enumtype;
    }
    let roman_chars: &[char] = if target_roman == "lowerroman" {
        &['i', 'v', 'x', 'l', 'c', 'd', 'm']
    } else {
        &['I', 'V', 'X', 'L', 'C', 'D', 'M']
    };
    if !roman_chars.contains(&first_value.chars().next().unwrap()) {
        return enumtype;
    }
    // Scan to next item with same prefix/suffix.
    let mut j = first_idx + 1;
    while j < lines.len() && lines[j].trim().is_empty() {
        j += 1;
    }
    if j >= lines.len() {
        return enumtype;
    }
    let next = match enum_marker(lines[j]) {
        Some(m) if m.1 == prefix && m.2 == suffix => m,
        _ => return enumtype,
    };
    if next.3.len() < 2 {
        return enumtype;
    }
    if next.3.chars().all(|c| roman_chars.contains(&c)) {
        target_roman
    } else {
        enumtype
    }
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
        Block::EnumeratedList {
            enumtype,
            prefix,
            suffix,
            start,
            items,
        } => {
            let list = tree.append(
                parent,
                NodeKind::EnumeratedList {
                    enumtype,
                    prefix,
                    suffix,
                    start,
                },
            );
            for item in items {
                let li = tree.append(list, NodeKind::ListItem);
                let p = tree.append(li, NodeKind::Paragraph);
                parse_inline(tree, p, &item);
            }
        }
        Block::Target { name, refuri } => {
            let ids = normalize_id(&name);
            tree.append(
                parent,
                NodeKind::Target {
                    ids,
                    names: name,
                    refuri,
                },
            );
        }
    }
}

/// Parse `.. _name: refuri` into `(name, refuri)`. Returns `None` if the
/// line is not an explicit target directive. Phrase names (with spaces or
/// containing escaped colons) are deferred.
fn explicit_target(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix(".. _")?;
    let colon = rest.find(':')?;
    let name = rest[..colon].trim();
    let refuri = rest[colon + 1..].trim();
    if name.is_empty() || refuri.is_empty() {
        return None;
    }
    // Simple-name only: alnum plus internal `.-_+:` (no spaces, no backticks).
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || "._-+:".contains(c))
    {
        return None;
    }
    Some((name.to_string(), refuri.to_string()))
}

/// docutils' `nodes.fully_normalize_name` + `nodes.make_id` for simple
/// identifiers: lowercased, non-alnum runs collapsed to single hyphens,
/// leading/trailing hyphens stripped.
fn normalize_id(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut last_dash = true;
    for c in lower.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

/// Walk the tree and fill in `refuri` on every `Reference` whose name
/// matches a `Target`. Unresolved references keep `refuri = ""` — currently
/// untested (an accepted deviation; upstream emits a system_message).
fn resolve_references(tree: &mut Doctree) {
    let mut targets: HashMap<String, String> = HashMap::new();
    collect_targets(tree, tree.root(), &mut targets);
    apply_targets(tree, tree.root(), &targets);
}

fn collect_targets(tree: &Doctree, id: NodeId, out: &mut HashMap<String, String>) {
    if let NodeKind::Target { names, refuri, .. } = &tree.node(id).kind {
        out.insert(names.clone(), refuri.clone());
    }
    let children = tree.node(id).children.clone();
    for c in children {
        collect_targets(tree, c, out);
    }
}

fn apply_targets(tree: &mut Doctree, id: NodeId, targets: &HashMap<String, String>) {
    if let NodeKind::Reference { name, refuri } = &mut tree.node_mut(id).kind {
        if refuri.is_empty() {
            if let Some(uri) = targets.get(name) {
                *refuri = uri.clone();
            }
        }
    }
    let children = tree.node(id).children.clone();
    for c in children {
        apply_targets(tree, c, targets);
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
        } else if let Some((name, end)) = try_match_reference(text, &pre.escaped, cursor) {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            let node = tree.append(
                parent,
                NodeKind::Reference {
                    name: name.clone(),
                    refuri: String::new(),
                },
            );
            push_text(tree, node, &name);
            cursor = end;
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

/// Try to match a simple-name hyperlink reference `name_` starting at
/// `start`. Returns `(name, end_byte_index_after_underscore)`.
///
/// A simple name is `[A-Za-z0-9](?:[A-Za-z0-9]|[._+:-](?=[A-Za-z0-9]))*`.
/// The match requires a word-boundary start, an unescaped trailing `_`,
/// and that what follows the `_` is not alphanumeric.
fn try_match_reference(text: &str, escaped: &[bool], start: usize) -> Option<(String, usize)> {
    if escaped.get(start).copied().unwrap_or(false) {
        return None;
    }
    if !valid_start_context(text, start) {
        return None;
    }
    let bytes = text.as_bytes();
    let first = *bytes.get(start)?;
    if !first.is_ascii_alphanumeric() {
        return None;
    }
    let mut end = start + 1;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_alphanumeric() {
            end += 1;
            continue;
        }
        if matches!(b, b'.' | b'-' | b'_' | b'+' | b':') {
            // Underscore is only an internal char if followed by alnum;
            // otherwise it terminates the simple-name.
            let next = bytes.get(end + 1).copied();
            if matches!(next, Some(n) if n.is_ascii_alphanumeric()) {
                end += 1;
                continue;
            }
            break;
        }
        break;
    }
    // Must terminate with an unescaped `_`.
    if end >= bytes.len() || bytes[end] != b'_' {
        return None;
    }
    if escaped.get(end).copied().unwrap_or(false) {
        return None;
    }
    let after = end + 1;
    if after < bytes.len() && bytes[after].is_ascii_alphanumeric() {
        return None;
    }
    let name = text[start..end].to_string();
    Some((name, after))
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
