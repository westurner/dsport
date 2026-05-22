//! rST parser slice covering phase 1 + phase 2.
//!
//! See `docs/compat.md` for the supported feature matrix and accepted
//! deviations. In particular, no syntax highlighting is applied to
//! `code`/`code-block` content (it is emitted as a plain literal block).

#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::enum_variant_names,
    dead_code
)]

use std::collections::HashMap;

use crate::doctree::{Doctree, NodeId, NodeKind};

pub fn parse_rst(source: &str) -> Doctree {
    parse_rst_with_source(source, "<string>")
}

pub fn parse_rst_with_source(source: &str, source_path: &str) -> Doctree {
    let mut tree = Doctree::new_document(source_path);
    let document = tree.root();

    // Pre-scan for substitution definitions so inline substitution
    // references can be expanded during inline parsing.
    let mut subs: HashMap<String, String> = HashMap::new();
    let lines: Vec<&str> = source.lines().collect();
    collect_substitutions(&lines, &mut subs);

    let blocks = parse_blocks(&lines, 0);
    let ctx = ParseCtx { subs };
    for block in blocks {
        emit_block(&mut tree, document, &ctx, block);
    }

    resolve_references(&mut tree);
    promote_document_title(&mut tree);
    promote_docinfo(&mut tree);
    tree
}

// ────────────────────────────────────────────────────────────────────────────
// Parser context shared across emit_block calls.
// ────────────────────────────────────────────────────────────────────────────

struct ParseCtx {
    subs: HashMap<String, String>,
}

// ────────────────────────────────────────────────────────────────────────────
// Block model
// ────────────────────────────────────────────────────────────────────────────

enum Block {
    Paragraph(String),
    BulletList {
        bullet: char,
        items: Vec<Vec<Block>>,
    },
    EnumeratedList {
        enumtype: &'static str,
        prefix: String,
        suffix: String,
        start: Option<u32>,
        items: Vec<Vec<Block>>,
    },
    DefinitionList {
        items: Vec<DefItem>,
    },
    FieldList {
        items: Vec<FieldItem>,
    },
    BlockQuote(Vec<Block>),
    LiteralBlock {
        text: String,
        classes: String,
    },
    Section {
        title: String,
        level: usize,
        children: Vec<Block>,
    },
    Transition,
    Target {
        name: String,
        refuri: String,
    },
    SubstitutionDefinition {
        name: String,
        text: String,
    },
    Comment(String),
    Admonition {
        kind: &'static str,
        children: Vec<Block>,
    },
    Image {
        uri: String,
        alt: Option<String>,
        width: Option<String>,
        height: Option<String>,
    },
    Raw {
        format: String,
        text: String,
    },
    Table(TableData),
}

struct DefItem {
    term: String,
    classifier: Option<String>,
    definition: Vec<Block>,
}

struct FieldItem {
    name: String,
    body: Vec<Block>,
    body_text: String,
}

struct TableData {
    /// Column widths in characters.
    cols: Vec<usize>,
    /// Optional header row(s): each cell is its own paragraph-string.
    head: Vec<Vec<String>>,
    body: Vec<Vec<String>>,
}

// ────────────────────────────────────────────────────────────────────────────
// Block-level parser
// ────────────────────────────────────────────────────────────────────────────

fn parse_blocks(lines: &[&str], base_indent: usize) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut i = 0;
    // Section detection: track underline level → integer.
    let mut section_chars: Vec<char> = Vec::new();
    while i < lines.len() {
        let line = lines[i];
        // Blank line.
        if line.trim().is_empty() {
            i += 1;
            continue;
        }
        // Line must start at our indent.
        if !at_indent(line, base_indent) {
            break;
        }
        let stripped = &line[base_indent..];

        // Transition: 4+ identical punctuation chars on a line, surrounded by
        // blank lines. Cheap detection: at column 0, line is all one of
        // -=~`#"^*+<>:_'.
        if base_indent == 0 && is_transition(stripped) {
            blocks.push(Block::Transition);
            i += 1;
            continue;
        }

        // Section: title line followed by underline of matching length using
        // one of the section punctuation characters. Only at column 0.
        if base_indent == 0
            && i + 1 < lines.len()
            && let Some((level, title, consumed)) = section_at(lines, i, &mut section_chars)
        {
            let body_start = i + consumed;
            // Section body extends until the next title underlined with a
            // section char already seen at this or a shallower level.
            let mut end = body_start;
            while end < lines.len() {
                if let Some((nl, _, _)) = section_at(lines, end, &mut section_chars.clone()) {
                    if nl <= level {
                        break;
                    }
                }
                end += 1;
            }
            let inner_lines: Vec<&str> = lines[body_start..end].to_vec();
            let children = parse_blocks(&inner_lines, 0);
            blocks.push(Block::Section {
                title,
                level,
                children,
            });
            i = end;
            continue;
        }

        // Explicit markup block: `.. name:: ...` / `.. _target: uri` /
        // `.. |name| directive::` / `.. comment text`.
        if let Some(b) = parse_explicit(lines, &mut i, base_indent) {
            blocks.push(b);
            continue;
        }

        // Grid / simple table.
        if let Some(tbl) = parse_table(lines, &mut i, base_indent) {
            blocks.push(Block::Table(tbl));
            continue;
        }

        // Bullet list.
        if let Some((bullet, _)) = bullet_marker(stripped) {
            let mut items: Vec<Vec<Block>> = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    let mut j = i + 1;
                    while j < lines.len() && lines[j].trim().is_empty() {
                        j += 1;
                    }
                    if j < lines.len()
                        && at_indent(lines[j], base_indent)
                        && bullet_marker(&lines[j][base_indent..]).map(|(b, _)| b) == Some(bullet)
                    {
                        i = j;
                        continue;
                    }
                    break;
                }
                if !at_indent(l, base_indent) {
                    break;
                }
                let after_indent = &l[base_indent..];
                if let Some((b, rest)) = bullet_marker(after_indent) {
                    if b != bullet {
                        break;
                    }
                    let item_indent = base_indent + 2;
                    // Build the item's content as text spanning subsequent
                    // continuation lines, then recursively parse it as
                    // blocks at `item_indent`.
                    let mut item_lines: Vec<String> = vec![format!(
                        "{indent}{rest}",
                        indent = " ".repeat(item_indent),
                        rest = rest
                    )];
                    i += 1;
                    while i < lines.len() {
                        let cl = lines[i];
                        if cl.trim().is_empty() {
                            item_lines.push(String::new());
                            i += 1;
                            continue;
                        }
                        if !at_indent(cl, item_indent) {
                            // Trim trailing blanks we added.
                            break;
                        }
                        if at_indent(cl, base_indent) && bullet_marker(&cl[base_indent..]).is_some()
                        {
                            break;
                        }
                        item_lines.push(cl.to_string());
                        i += 1;
                    }
                    while item_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
                        item_lines.pop();
                    }
                    let refs: Vec<&str> = item_lines.iter().map(|s| s.as_str()).collect();
                    items.push(parse_blocks(&refs, item_indent));
                } else {
                    break;
                }
            }
            blocks.push(Block::BulletList { bullet, items });
            continue;
        }

        // Enumerated list.
        if let Some((enumtype, prefix, suffix, first_value, _rest)) = enum_marker(stripped) {
            let enumtype =
                disambiguate_alpha_roman(enumtype, &prefix, &suffix, lines, i, base_indent);
            let start = enumerator_value(enumtype, &first_value).filter(|&v| v != 1);
            let mut items: Vec<Vec<Block>> = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    let mut j = i + 1;
                    while j < lines.len() && lines[j].trim().is_empty() {
                        j += 1;
                    }
                    let cont = j < lines.len()
                        && at_indent(lines[j], base_indent)
                        && enum_marker(&lines[j][base_indent..])
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
                if !at_indent(l, base_indent) {
                    break;
                }
                let after_indent = &l[base_indent..];
                let m = match enum_marker(after_indent) {
                    Some(m) if m.1 == prefix && m.2 == suffix && value_matches(enumtype, &m.3) => m,
                    _ => break,
                };
                let item_indent = base_indent + prefix.len() + m.3.len() + suffix.len() + 1;
                let mut item_lines: Vec<String> = vec![format!(
                    "{indent}{rest}",
                    indent = " ".repeat(item_indent),
                    rest = m.4
                )];
                i += 1;
                while i < lines.len() {
                    let cl = lines[i];
                    if cl.trim().is_empty() {
                        item_lines.push(String::new());
                        i += 1;
                        continue;
                    }
                    if !at_indent(cl, item_indent) {
                        break;
                    }
                    if at_indent(cl, base_indent) && enum_marker(&cl[base_indent..]).is_some() {
                        break;
                    }
                    item_lines.push(cl.to_string());
                    i += 1;
                }
                while item_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
                    item_lines.pop();
                }
                let refs: Vec<&str> = item_lines.iter().map(|s| s.as_str()).collect();
                items.push(parse_blocks(&refs, item_indent));
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

        // Field list (`:Name: value`).
        if let Some(fl) = parse_field_list(lines, &mut i, base_indent) {
            blocks.push(fl);
            continue;
        }

        // Definition list (`term\n    definition`).
        if let Some(dl) = parse_def_list(lines, &mut i, base_indent) {
            blocks.push(dl);
            continue;
        }

        // Block quote: an indented block when we are at base_indent==0 (or
        // at any depth, indentation strictly greater than base counts).
        if let Some(qb_indent) = leading_spaces(line).filter(|&n| n > base_indent) {
            let mut quote_lines: Vec<&str> = Vec::new();
            while i < lines.len() {
                let l = lines[i];
                if l.trim().is_empty() {
                    quote_lines.push("");
                    i += 1;
                    continue;
                }
                if leading_spaces(l).unwrap_or(0) < qb_indent {
                    break;
                }
                quote_lines.push(l);
                i += 1;
            }
            while quote_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
                quote_lines.pop();
            }
            let children = parse_blocks(&quote_lines, qb_indent);
            blocks.push(Block::BlockQuote(children));
            continue;
        }

        // Paragraph (with possible trailing `::` literal block).
        let mut buf: Vec<&str> = Vec::new();
        while i < lines.len() {
            let l = lines[i];
            if l.trim().is_empty() {
                break;
            }
            if !at_indent(l, base_indent) {
                break;
            }
            let after = &l[base_indent..];
            if bullet_marker(after).is_some()
                || enum_marker(after).is_some()
                || is_explicit_start(after)
                || field_marker(after).is_some()
            {
                break;
            }
            buf.push(l);
            i += 1;
        }
        if buf.is_empty() {
            i += 1;
            continue;
        }
        let mut text = buf
            .iter()
            .map(|l| &l[base_indent..])
            .collect::<Vec<&str>>()
            .join("\n");

        // `::` at end of paragraph → followed literal_block.
        let mut want_literal = false;
        if let Some(stripped) = text.strip_suffix("::") {
            want_literal = true;
            let before = stripped;
            if before.ends_with(' ') || before.is_empty() {
                // Drop the marker entirely.
                let cut = before.trim_end_matches(' ');
                if cut.is_empty() {
                    text.clear();
                } else {
                    text = cut.to_string();
                }
            } else {
                // Replace `::` with `:`.
                text = format!("{}:", before);
            }
        }
        if !text.is_empty() {
            blocks.push(Block::Paragraph(text));
        }
        if want_literal {
            // Skip blank lines, then consume indented block as literal.
            while i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
            if i < lines.len() {
                if let Some(lb_indent) = leading_spaces(lines[i]).filter(|&n| n > base_indent) {
                    let mut text_lines: Vec<String> = Vec::new();
                    while i < lines.len() {
                        let l = lines[i];
                        if l.trim().is_empty() {
                            text_lines.push(String::new());
                            i += 1;
                            continue;
                        }
                        if leading_spaces(l).unwrap_or(0) < lb_indent {
                            break;
                        }
                        text_lines.push(l[lb_indent..].to_string());
                        i += 1;
                    }
                    while text_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
                        text_lines.pop();
                    }
                    blocks.push(Block::LiteralBlock {
                        text: text_lines.join("\n"),
                        classes: String::new(),
                    });
                }
            }
        }
    }
    blocks
}

fn at_indent(line: &str, indent: usize) -> bool {
    if line.trim().is_empty() {
        return true;
    }
    line.len() >= indent && line[..indent].chars().all(|c| c == ' ')
}

fn leading_spaces(line: &str) -> Option<usize> {
    if line.trim().is_empty() {
        return None;
    }
    Some(line.chars().take_while(|&c| c == ' ').count())
}

// ────────────────────────────────────────────────────────────────────────────
// Sections / transitions
// ────────────────────────────────────────────────────────────────────────────

const SECTION_CHARS: &str = "=-`:.'\"~^_*+#<>";

fn is_section_char(c: char) -> bool {
    SECTION_CHARS.contains(c)
}

fn is_transition(line: &str) -> bool {
    let trimmed = line.trim_end();
    if trimmed.len() < 4 {
        return false;
    }
    let first = trimmed.chars().next().unwrap();
    if !is_section_char(first) {
        return false;
    }
    trimmed.chars().all(|c| c == first)
}

/// If `lines[i..]` begins with a title line + matching underline, return
/// `(level, title_text, lines_consumed)`. `section_chars` records the
/// punctuation per level encountered so far.
fn section_at(
    lines: &[&str],
    i: usize,
    section_chars: &mut Vec<char>,
) -> Option<(usize, String, usize)> {
    if i + 1 >= lines.len() {
        return None;
    }
    let title = lines[i];
    let under = lines[i + 1];
    if title.trim().is_empty() {
        return None;
    }
    if under.is_empty() {
        return None;
    }
    let first = under.chars().next()?;
    if !is_section_char(first) {
        return None;
    }
    let under_trim = under.trim_end();
    if !under_trim.chars().all(|c| c == first) {
        return None;
    }
    if under_trim.chars().count() < title.trim_end().chars().count() {
        return None;
    }
    let level = match section_chars.iter().position(|&c| c == first) {
        Some(idx) => idx,
        None => {
            section_chars.push(first);
            section_chars.len() - 1
        }
    };
    Some((level, title.trim().to_string(), 2))
}

// ────────────────────────────────────────────────────────────────────────────
// Bullet / enum markers
// ────────────────────────────────────────────────────────────────────────────

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

fn enum_marker(line: &str) -> Option<(&'static str, String, String, String, &str)> {
    let bytes = line.as_bytes();
    let (prefix, value_start) = if bytes.first() == Some(&b'(') {
        ("(", 1)
    } else {
        ("", 0)
    };
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
    if total <= 0 { None } else { Some(total as u32) }
}

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

fn disambiguate_alpha_roman(
    enumtype: &'static str,
    prefix: &str,
    suffix: &str,
    lines: &[&str],
    first_idx: usize,
    base_indent: usize,
) -> &'static str {
    let target_roman = match enumtype {
        "loweralpha" => "lowerroman",
        "upperalpha" => "upperroman",
        _ => return enumtype,
    };
    let l0 = lines[first_idx];
    if !at_indent(l0, base_indent) {
        return enumtype;
    }
    let first_value = match enum_marker(&l0[base_indent..]) {
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
    let mut j = first_idx + 1;
    while j < lines.len() && lines[j].trim().is_empty() {
        j += 1;
    }
    if j >= lines.len() || !at_indent(lines[j], base_indent) {
        return enumtype;
    }
    let next = match enum_marker(&lines[j][base_indent..]) {
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

// ────────────────────────────────────────────────────────────────────────────
// Field lists
// ────────────────────────────────────────────────────────────────────────────

fn field_marker(line: &str) -> Option<(String, &str)> {
    if !line.starts_with(':') {
        return None;
    }
    let rest = &line[1..];
    // Field name terminates at unescaped `:`. We support the simple case
    // (no embedded backslashes / inline markup in name).
    let colon = rest.find(':')?;
    let name = &rest[..colon];
    if name.is_empty() || name.contains('\n') {
        return None;
    }
    let body = &rest[colon + 1..];
    // Field marker requires the value to be either empty or to begin with
    // whitespace. Otherwise we'd misread inline roles like
    // `:emphasis:`hi`` as field markers.
    if !body.is_empty() && !body.starts_with(char::is_whitespace) {
        return None;
    }
    let body = body.strip_prefix(' ').unwrap_or(body);
    Some((name.to_string(), body))
}

fn parse_field_list(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<Block> {
    let mut i = *i_ref;
    let l = lines[i];
    if !at_indent(l, base_indent) {
        return None;
    }
    let after = &l[base_indent..];
    field_marker(after)?;

    let mut items: Vec<FieldItem> = Vec::new();
    while i < lines.len() {
        let l = lines[i];
        if l.trim().is_empty() {
            // Field-list ends on blank line unless next non-blank is another
            // field at the same indent.
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len()
                && at_indent(lines[j], base_indent)
                && field_marker(&lines[j][base_indent..]).is_some()
            {
                i = j;
                continue;
            }
            break;
        }
        if !at_indent(l, base_indent) {
            break;
        }
        let after = &l[base_indent..];
        let (name, body) = match field_marker(after) {
            Some(v) => v,
            None => break,
        };
        let body_indent = base_indent + 4; // default content indent
        let mut body_lines: Vec<String> = if body.is_empty() {
            Vec::new()
        } else {
            vec![format!("{}{}", " ".repeat(body_indent), body)]
        };
        i += 1;
        while i < lines.len() {
            let cl = lines[i];
            if cl.trim().is_empty() {
                body_lines.push(String::new());
                i += 1;
                continue;
            }
            if leading_spaces(cl).unwrap_or(0) <= base_indent {
                break;
            }
            body_lines.push(cl.to_string());
            i += 1;
        }
        while body_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
            body_lines.pop();
        }
        let refs: Vec<&str> = body_lines.iter().map(|s| s.as_str()).collect();
        let body_blocks = parse_blocks(&refs, body_indent);
        let body_text = body_lines
            .iter()
            .map(|s| s.trim_start().to_string())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();
        items.push(FieldItem {
            name,
            body: body_blocks,
            body_text,
        });
    }
    *i_ref = i;
    Some(Block::FieldList { items })
}

// ────────────────────────────────────────────────────────────────────────────
// Definition lists
// ────────────────────────────────────────────────────────────────────────────

fn parse_def_list(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<Block> {
    let i_start = *i_ref;
    // A definition list item is a single text line (term) at base_indent
    // followed by an indented block (definition) on the next line.
    let term_line = lines[i_start];
    if !at_indent(term_line, base_indent) {
        return None;
    }
    let term_text = &term_line[base_indent..];
    if term_text.trim().is_empty() {
        return None;
    }
    if i_start + 1 >= lines.len() {
        return None;
    }
    let next = lines[i_start + 1];
    if next.trim().is_empty() {
        return None;
    }
    if leading_spaces(next).unwrap_or(0) <= base_indent {
        return None;
    }
    // Term must not match any other construct.
    if bullet_marker(term_text).is_some()
        || enum_marker(term_text).is_some()
        || field_marker(term_text).is_some()
        || is_explicit_start(term_text)
    {
        return None;
    }

    let mut i = i_start;
    let mut items: Vec<DefItem> = Vec::new();
    while i < lines.len() {
        let l = lines[i];
        if l.trim().is_empty() {
            // Lookahead: another term + indented def?
            let mut j = i + 1;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j + 1 < lines.len()
                && at_indent(lines[j], base_indent)
                && !lines[j][base_indent..].trim().is_empty()
                && leading_spaces(lines[j + 1]).unwrap_or(0) > base_indent
            {
                i = j;
                continue;
            }
            break;
        }
        if !at_indent(l, base_indent) {
            break;
        }
        let raw_term = l[base_indent..].to_string();
        // Optional classifier with `term : classifier`.
        let (term, classifier) = match raw_term.find(" : ") {
            Some(pos) => (
                raw_term[..pos].trim().to_string(),
                Some(raw_term[pos + 3..].trim().to_string()),
            ),
            None => (raw_term.trim().to_string(), None),
        };
        i += 1;
        // Definition must immediately follow.
        if i >= lines.len() || lines[i].trim().is_empty() {
            // Not actually a def-list item; abort.
            *i_ref = i_start;
            return None;
        }
        let def_indent = match leading_spaces(lines[i]) {
            Some(n) if n > base_indent => n,
            _ => {
                *i_ref = i_start;
                return None;
            }
        };
        let mut def_lines: Vec<String> = Vec::new();
        while i < lines.len() {
            let cl = lines[i];
            if cl.trim().is_empty() {
                def_lines.push(String::new());
                i += 1;
                continue;
            }
            if leading_spaces(cl).unwrap_or(0) < def_indent {
                break;
            }
            def_lines.push(cl.to_string());
            i += 1;
        }
        while def_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
            def_lines.pop();
        }
        let refs: Vec<&str> = def_lines.iter().map(|s| s.as_str()).collect();
        let definition = parse_blocks(&refs, def_indent);
        items.push(DefItem {
            term,
            classifier,
            definition,
        });
    }
    if items.is_empty() {
        *i_ref = i_start;
        return None;
    }
    *i_ref = i;
    Some(Block::DefinitionList { items })
}

// ────────────────────────────────────────────────────────────────────────────
// Explicit markup (`..` constructs): targets, directives, substitutions,
// comments.
// ────────────────────────────────────────────────────────────────────────────

fn is_explicit_start(line: &str) -> bool {
    line.starts_with(".. ") || line == ".."
}

fn parse_explicit(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<Block> {
    let i = *i_ref;
    let l = lines[i];
    if !at_indent(l, base_indent) {
        return None;
    }
    let after = &l[base_indent..];
    if !is_explicit_start(after) {
        return None;
    }
    let rest = after.strip_prefix(".. ").unwrap_or("");

    // Hyperlink target: `.. _name: refuri`.
    if let Some(t) = parse_target_inline(rest) {
        *i_ref = i + 1;
        return Some(Block::Target {
            name: t.0,
            refuri: t.1,
        });
    }

    // Substitution definition: `.. |name| directive:: args`.
    if let Some((subname, dir_rest)) = parse_substitution_head(rest) {
        let (directive, dargs) = split_directive(dir_rest)?;
        if directive == "replace" {
            let text = parse_directive_inline_or_indented(lines, i_ref, base_indent, dargs);
            return Some(Block::SubstitutionDefinition {
                name: subname,
                text,
            });
        }
        // Other substitution directives (e.g. image) are accepted-deviation
        // and dropped.
        *i_ref = i + 1;
        consume_indented(lines, i_ref, base_indent);
        return Some(Block::SubstitutionDefinition {
            name: subname,
            text: String::new(),
        });
    }

    // Directive: `name:: args`.
    if let Some((directive, dargs)) = split_directive(rest) {
        return Some(parse_directive(lines, i_ref, base_indent, directive, dargs));
    }

    // Comment: anything else under `..`.
    let mut text_lines: Vec<String> = Vec::new();
    if !rest.is_empty() {
        text_lines.push(rest.to_string());
    }
    *i_ref = i + 1;
    let inner = consume_indented_text(lines, i_ref, base_indent);
    for ln in inner {
        text_lines.push(ln);
    }
    Some(Block::Comment(text_lines.join("\n")))
}

fn parse_target_inline(rest: &str) -> Option<(String, String)> {
    let after = rest.strip_prefix('_')?;
    let colon = after.find(':')?;
    let name = after[..colon].trim();
    let refuri = after[colon + 1..].trim();
    if name.is_empty() || refuri.is_empty() {
        return None;
    }
    let name = if let Some(s) = name.strip_prefix('`') {
        s.strip_suffix('`')?.to_string()
    } else {
        // Allow letters, digits, spaces, and a small set of punctuation in
        // the name. Spaces yield phrase-reference targets.
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || " ._-+".contains(c))
        {
            return None;
        }
        name.to_string()
    };
    Some((name, refuri.to_string()))
}

fn parse_substitution_head(rest: &str) -> Option<(String, &str)> {
    let inner = rest.strip_prefix('|')?;
    let end = inner.find('|')?;
    let name = inner[..end].trim().to_string();
    if name.is_empty() {
        return None;
    }
    let tail = inner[end + 1..].trim_start();
    Some((name, tail))
}

fn split_directive(s: &str) -> Option<(String, &str)> {
    let end = s.find("::")?;
    let name = s[..end].trim();
    if name.is_empty() {
        return None;
    }
    let args = s[end + 2..].trim_start();
    Some((name.to_string(), args))
}

fn parse_directive_inline_or_indented(
    lines: &[&str],
    i_ref: &mut usize,
    base_indent: usize,
    inline_args: &str,
) -> String {
    if !inline_args.is_empty() {
        *i_ref += 1;
        return inline_args.to_string();
    }
    *i_ref += 1;
    let inner = consume_indented_text(lines, i_ref, base_indent);
    inner.join("\n")
}

fn parse_directive(
    lines: &[&str],
    i_ref: &mut usize,
    base_indent: usize,
    name: String,
    args: &str,
) -> Block {
    match name.as_str() {
        "note" | "warning" | "tip" | "hint" | "important" | "attention" | "caution" | "danger"
        | "error" => {
            let kind: &'static str = match name.as_str() {
                "note" => "note",
                "warning" => "warning",
                "tip" => "tip",
                "hint" => "hint",
                "important" => "important",
                "attention" => "attention",
                "caution" => "caution",
                "danger" => "danger",
                "error" => "error",
                _ => unreachable!(),
            };
            let mut child_blocks: Vec<Block> = Vec::new();
            if !args.is_empty() {
                child_blocks.push(Block::Paragraph(args.to_string()));
            }
            *i_ref += 1;
            // Skip blank lines, then collect indented content.
            let content_indent = peek_inner_indent(lines, *i_ref, base_indent);
            if let Some(ci) = content_indent {
                let inner = consume_indented_lines(lines, i_ref, ci);
                let refs: Vec<&str> = inner.iter().map(|s| s.as_str()).collect();
                let mut more = parse_blocks(&refs, ci);
                child_blocks.append(&mut more);
            }
            Block::Admonition {
                kind,
                children: child_blocks,
            }
        }
        "image" | "figure" => {
            let uri = args.trim().to_string();
            *i_ref += 1;
            // Consume option lines `:opt: val`.
            let mut alt = None;
            let mut width = None;
            let mut height = None;
            // Skip blank lines before options.
            let mut j = *i_ref;
            while j < lines.len() && lines[j].trim().is_empty() {
                j += 1;
            }
            if j < lines.len() {
                if let Some(ind) = leading_spaces(lines[j])
                    && ind > base_indent
                {
                    while j < lines.len() {
                        let l = lines[j];
                        if l.trim().is_empty() {
                            break;
                        }
                        if leading_spaces(l).unwrap_or(0) < ind {
                            break;
                        }
                        let stripped = &l[ind..];
                        if let Some((k, v)) = field_marker(stripped) {
                            match k.as_str() {
                                "alt" => alt = Some(v.to_string()),
                                "width" => width = Some(v.to_string()),
                                "height" => height = Some(v.to_string()),
                                _ => {}
                            }
                            j += 1;
                        } else {
                            break;
                        }
                    }
                    *i_ref = j;
                }
            }
            Block::Image {
                uri,
                alt,
                width,
                height,
            }
        }
        "code" | "code-block" | "sourcecode" => {
            let lang = args.trim();
            *i_ref += 1;
            let inner = consume_indented_text(lines, i_ref, base_indent);
            let classes = if lang.is_empty() {
                "code".to_string()
            } else {
                format!("code {}", lang)
            };
            Block::LiteralBlock {
                text: inner.join("\n"),
                classes,
            }
        }
        "raw" => {
            let format = args.trim().to_string();
            *i_ref += 1;
            let inner = consume_indented_text(lines, i_ref, base_indent);
            Block::Raw {
                format,
                text: inner.join("\n"),
            }
        }
        _ => {
            // Unknown directive: swallow as a comment to keep going.
            *i_ref += 1;
            let inner = consume_indented_text(lines, i_ref, base_indent);
            Block::Comment(inner.join("\n"))
        }
    }
}

/// After a directive header line at column `base_indent`, peek at the next
/// non-blank line and return its actual indent if it is greater than base.
fn peek_inner_indent(lines: &[&str], from: usize, base_indent: usize) -> Option<usize> {
    let mut j = from;
    while j < lines.len() && lines[j].trim().is_empty() {
        j += 1;
    }
    if j >= lines.len() {
        return None;
    }
    let ind = leading_spaces(lines[j])?;
    if ind > base_indent { Some(ind) } else { None }
}

/// Consume contiguous lines whose indent ≥ `indent`; return them stripped of
/// `indent` spaces. Skips leading blank lines and stops at the first line at
/// shallower indent. Trims trailing blanks.
fn consume_indented_lines(lines: &[&str], i_ref: &mut usize, indent: usize) -> Vec<String> {
    let mut j = *i_ref;
    while j < lines.len() && lines[j].trim().is_empty() {
        j += 1;
    }
    let mut out: Vec<String> = Vec::new();
    while j < lines.len() {
        let l = lines[j];
        if l.trim().is_empty() {
            out.push(String::new());
            j += 1;
            continue;
        }
        if leading_spaces(l).unwrap_or(0) < indent {
            break;
        }
        out.push(l.to_string());
        j += 1;
    }
    while out.last().map(|s| s.is_empty()).unwrap_or(false) {
        out.pop();
    }
    *i_ref = j;
    out
}

/// Like `consume_indented_lines` but returns lines with their common indent
/// stripped (for literal-block / raw / comment / directive content).
fn consume_indented_text(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Vec<String> {
    let ci = match peek_inner_indent(lines, *i_ref, base_indent) {
        Some(n) => n,
        None => return Vec::new(),
    };
    let raw = consume_indented_lines(lines, i_ref, ci);
    raw.into_iter()
        .map(|l| if l.is_empty() { l } else { l[ci..].to_string() })
        .collect()
}

fn consume_indented(lines: &[&str], i_ref: &mut usize, base_indent: usize) {
    let _ = consume_indented_text(lines, i_ref, base_indent);
}

// ────────────────────────────────────────────────────────────────────────────
// Tables
// ────────────────────────────────────────────────────────────────────────────

fn parse_table(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<TableData> {
    let i = *i_ref;
    let l = lines[i];
    if !at_indent(l, base_indent) {
        return None;
    }
    let stripped = &l[base_indent..];
    if stripped.starts_with('+') && stripped.chars().all(|c| c == '+' || c == '-') {
        return parse_grid_table(lines, i_ref, base_indent);
    }
    if stripped.starts_with('=') && stripped.chars().all(|c| c == '=' || c == ' ') {
        return parse_simple_table(lines, i_ref, base_indent);
    }
    None
}

fn parse_grid_table(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<TableData> {
    let start = *i_ref;
    // Find table extent: contiguous lines starting with `+` or `|` at base_indent.
    let mut end = start;
    while end < lines.len() {
        let l = lines[end];
        if l.trim().is_empty() {
            break;
        }
        if !at_indent(l, base_indent) {
            break;
        }
        let stripped = &l[base_indent..];
        let first = stripped.chars().next()?;
        if first != '+' && first != '|' {
            break;
        }
        end += 1;
    }
    if end - start < 3 {
        return None;
    }
    let top = &lines[start][base_indent..];
    // Column widths from top border: `+---+---+`
    let mut cols: Vec<usize> = Vec::new();
    let bytes = top.as_bytes();
    let mut col_start: Option<usize> = None;
    for (idx, &b) in bytes.iter().enumerate() {
        match b {
            b'+' => {
                if let Some(s) = col_start {
                    cols.push(idx - s - 1);
                }
                col_start = Some(idx);
            }
            b'-' => {}
            _ => return None,
        }
    }
    if cols.is_empty() {
        return None;
    }
    // Find header separator (`+===+`) if any.
    let mut head_end: Option<usize> = None;
    for k in start + 1..end {
        let l = &lines[k][base_indent..];
        if l.starts_with('+') && l.contains('=') {
            head_end = Some(k);
            break;
        }
    }
    // Collect rows: each text row is between two `+---+` separators.
    let collect_rows = |from: usize, to: usize| -> Vec<Vec<String>> {
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut buf: Vec<Vec<String>> = vec![Vec::new(); cols.len()];
        let mut have_buf = false;
        for k in from..to {
            let l = &lines[k][base_indent..];
            if l.starts_with('+') {
                if have_buf {
                    let cells: Vec<String> = buf
                        .iter()
                        .map(|cl| cl.join("\n").trim().to_string())
                        .collect();
                    rows.push(cells);
                    buf = vec![Vec::new(); cols.len()];
                    have_buf = false;
                }
                continue;
            }
            if !l.starts_with('|') {
                continue;
            }
            // Split by `|` and take cols.len() fields.
            let parts: Vec<&str> = l.split('|').collect();
            if parts.len() < cols.len() + 2 {
                continue;
            }
            for (ci, part) in parts[1..=cols.len()].iter().enumerate() {
                buf[ci].push(part.to_string());
            }
            have_buf = true;
        }
        rows
    };
    let (head, body) = match head_end {
        Some(h) => (collect_rows(start, h + 1), collect_rows(h, end)),
        None => (Vec::new(), collect_rows(start, end)),
    };
    *i_ref = end;
    Some(TableData { cols, head, body })
}

fn parse_simple_table(lines: &[&str], i_ref: &mut usize, base_indent: usize) -> Option<TableData> {
    let start = *i_ref;
    let top = &lines[start][base_indent..];
    // Column spans from top: runs of `=` separated by spaces.
    let mut cols: Vec<(usize, usize)> = Vec::new();
    let bytes = top.as_bytes();
    let mut k = 0;
    while k < bytes.len() {
        if bytes[k] == b'=' {
            let s = k;
            while k < bytes.len() && bytes[k] == b'=' {
                k += 1;
            }
            cols.push((s, k));
        } else {
            k += 1;
        }
    }
    if cols.is_empty() {
        return None;
    }
    let col_widths: Vec<usize> = cols.iter().map(|(s, e)| e - s).collect();
    // Find next `===` line: that ends the header (if there's more before bottom)
    // or the table bottom.
    let mut sep_indices: Vec<usize> = Vec::new();
    let mut j = start + 1;
    while j < lines.len() {
        let l = lines[j];
        if l.trim().is_empty() {
            break;
        }
        if !at_indent(l, base_indent) {
            break;
        }
        let stripped = &l[base_indent..];
        if stripped.starts_with('=') && stripped.chars().all(|c| c == '=' || c == ' ') {
            sep_indices.push(j);
        }
        j += 1;
    }
    if sep_indices.is_empty() {
        return None;
    }
    let end_line = *sep_indices.last().unwrap();
    let split_row = |row: &str| -> Vec<String> {
        let mut cells = Vec::with_capacity(col_widths.len());
        for (idx, &(s, e)) in cols.iter().enumerate() {
            let cell = if idx + 1 == cols.len() {
                row.get(s..).unwrap_or("").to_string()
            } else {
                row.get(s..e).unwrap_or("").to_string()
            };
            cells.push(cell.trim().to_string());
        }
        cells
    };
    let (head, body): (Vec<Vec<String>>, Vec<Vec<String>>) = if sep_indices.len() >= 2 {
        let head_end = sep_indices[0];
        let mut head_rows = Vec::new();
        for k in start + 1..head_end {
            let l = lines[k];
            if l.trim().is_empty() {
                continue;
            }
            head_rows.push(split_row(&l[base_indent..]));
        }
        let mut body_rows = Vec::new();
        for k in head_end + 1..end_line {
            let l = lines[k];
            if l.trim().is_empty() {
                continue;
            }
            body_rows.push(split_row(&l[base_indent..]));
        }
        (head_rows, body_rows)
    } else {
        let mut body_rows = Vec::new();
        for k in start + 1..end_line {
            let l = lines[k];
            if l.trim().is_empty() {
                continue;
            }
            body_rows.push(split_row(&l[base_indent..]));
        }
        (Vec::new(), body_rows)
    };
    *i_ref = end_line + 1;
    Some(TableData {
        cols: col_widths,
        head,
        body,
    })
}

// ────────────────────────────────────────────────────────────────────────────
// Substitution pre-scan
// ────────────────────────────────────────────────────────────────────────────

fn collect_substitutions(lines: &[&str], out: &mut HashMap<String, String>) {
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        if let Some(rest) = l.strip_prefix(".. ") {
            if let Some((name, dir_rest)) = parse_substitution_head(rest) {
                if let Some((directive, dargs)) = split_directive(dir_rest) {
                    if directive == "replace" {
                        let text = if dargs.is_empty() {
                            // Consume indented content.
                            let mut j = i + 1;
                            let mut text_lines: Vec<String> = Vec::new();
                            while j < lines.len() && lines[j].trim().is_empty() {
                                j += 1;
                            }
                            if j < lines.len() {
                                if let Some(ind) = leading_spaces(lines[j]) {
                                    if ind > 0 {
                                        while j < lines.len() {
                                            let ll = lines[j];
                                            if ll.trim().is_empty() {
                                                break;
                                            }
                                            if leading_spaces(ll).unwrap_or(0) < ind {
                                                break;
                                            }
                                            text_lines.push(ll[ind..].to_string());
                                            j += 1;
                                        }
                                    }
                                }
                            }
                            text_lines.join("\n")
                        } else {
                            dargs.to_string()
                        };
                        out.insert(name, text);
                    }
                }
            }
        }
        i += 1;
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Emit
// ────────────────────────────────────────────────────────────────────────────

fn emit_block(tree: &mut Doctree, parent: NodeId, ctx: &ParseCtx, block: Block) {
    match block {
        Block::Paragraph(text) => {
            let p = tree.append(parent, NodeKind::Paragraph);
            parse_inline(tree, p, ctx, &text);
        }
        Block::BulletList { bullet, items } => {
            let list = tree.append(parent, NodeKind::BulletList { bullet });
            for item in items {
                let li = tree.append(list, NodeKind::ListItem);
                for b in item {
                    emit_block(tree, li, ctx, b);
                }
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
                for b in item {
                    emit_block(tree, li, ctx, b);
                }
            }
        }
        Block::DefinitionList { items } => {
            let dl = tree.append(parent, NodeKind::DefinitionList);
            for it in items {
                let dli = tree.append(dl, NodeKind::DefinitionListItem);
                let term = tree.append(dli, NodeKind::Term);
                parse_inline(tree, term, ctx, &it.term);
                if let Some(c) = it.classifier {
                    let cl = tree.append(dli, NodeKind::Classifier);
                    parse_inline(tree, cl, ctx, &c);
                }
                let d = tree.append(dli, NodeKind::Definition);
                for b in it.definition {
                    emit_block(tree, d, ctx, b);
                }
            }
        }
        Block::FieldList { items } => {
            // Promote to docinfo if all field names are recognized
            // bibliographic fields and parent is document with no prior
            // children other than potentially a title/subtitle.
            let is_doc = matches!(&tree.node(parent).kind, NodeKind::Document { .. });
            let bibliographic = is_doc
                && tree.node(parent).children.iter().all(|&c| {
                    matches!(
                        &tree.node(c).kind,
                        NodeKind::Title | NodeKind::Subtitle { .. }
                    )
                })
                && items
                    .iter()
                    .all(|it| recognized_bibliographic(&it.name).is_some());
            if bibliographic {
                let docinfo = tree.append(parent, NodeKind::Docinfo);
                for it in items {
                    let tag = recognized_bibliographic(&it.name).unwrap();
                    let bib = tree.append(docinfo, NodeKind::Bibliographic { tag });
                    // Body is the raw field value text; inline-parse it.
                    if !it.body_text.is_empty() {
                        parse_inline(tree, bib, ctx, &it.body_text);
                    }
                }
            } else {
                let fl = tree.append(parent, NodeKind::FieldList);
                for it in items {
                    let f = tree.append(fl, NodeKind::Field);
                    let n = tree.append(f, NodeKind::FieldName);
                    parse_inline(tree, n, ctx, &it.name);
                    let b = tree.append(f, NodeKind::FieldBody);
                    for blk in it.body {
                        emit_block(tree, b, ctx, blk);
                    }
                }
            }
        }
        Block::BlockQuote(children) => {
            let q = tree.append(parent, NodeKind::BlockQuote);
            for b in children {
                emit_block(tree, q, ctx, b);
            }
        }
        Block::LiteralBlock { text, classes } => {
            let lb = tree.append(parent, NodeKind::LiteralBlock { classes });
            tree.append(lb, NodeKind::Text(text));
        }
        Block::Section {
            title,
            level: _,
            children,
        } => {
            let ids = normalize_id(&title);
            let sec = tree.append(
                parent,
                NodeKind::Section {
                    ids: ids.clone(),
                    names: title.to_ascii_lowercase(),
                },
            );
            let t = tree.append(sec, NodeKind::Title);
            parse_inline(tree, t, ctx, &title);
            for b in children {
                emit_block(tree, sec, ctx, b);
            }
        }
        Block::Transition => {
            tree.append(parent, NodeKind::Transition);
        }
        Block::Target { name, refuri } => {
            let ids = normalize_id(&name);
            let names = if name.contains(' ') {
                name.split(' ')
                    .map(|w| w.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .join("\\ ")
            } else {
                name.to_ascii_lowercase()
            };
            tree.append(parent, NodeKind::Target { ids, names, refuri });
        }
        Block::SubstitutionDefinition { name, text } => {
            let sd = tree.append(parent, NodeKind::SubstitutionDefinition { names: name });
            parse_inline(tree, sd, ctx, &text);
        }
        Block::Comment(text) => {
            let c = tree.append(parent, NodeKind::Comment);
            tree.append(c, NodeKind::Text(text));
        }
        Block::Admonition { kind, children } => {
            let a = tree.append(parent, NodeKind::Admonition { kind });
            for b in children {
                emit_block(tree, a, ctx, b);
            }
        }
        Block::Image {
            uri,
            alt,
            width,
            height,
        } => {
            tree.append(
                parent,
                NodeKind::Image {
                    uri,
                    alt,
                    width,
                    height,
                },
            );
        }
        Block::Raw { format, text } => {
            let r = tree.append(parent, NodeKind::Raw { format });
            tree.append(r, NodeKind::Text(text));
        }
        Block::Table(td) => {
            emit_table(tree, parent, ctx, td);
        }
    }
}

fn emit_table(tree: &mut Doctree, parent: NodeId, ctx: &ParseCtx, td: TableData) {
    let tbl = tree.append(parent, NodeKind::Table);
    let tgroup = tree.append(
        tbl,
        NodeKind::Tgroup {
            cols: td.cols.len() as u32,
        },
    );
    for w in &td.cols {
        tree.append(
            tgroup,
            NodeKind::Colspec {
                colwidth: *w as u32,
            },
        );
    }
    if !td.head.is_empty() {
        let thead = tree.append(tgroup, NodeKind::Thead);
        for row in td.head {
            let r = tree.append(thead, NodeKind::Row);
            for cell in row {
                let e = tree.append(r, NodeKind::Entry);
                if !cell.is_empty() {
                    let p = tree.append(e, NodeKind::Paragraph);
                    parse_inline(tree, p, ctx, &cell);
                }
            }
        }
    }
    let tbody = tree.append(tgroup, NodeKind::Tbody);
    for row in td.body {
        let r = tree.append(tbody, NodeKind::Row);
        for cell in row {
            let e = tree.append(r, NodeKind::Entry);
            if !cell.is_empty() {
                let p = tree.append(e, NodeKind::Paragraph);
                parse_inline(tree, p, ctx, &cell);
            }
        }
    }
}

fn recognized_bibliographic(name: &str) -> Option<&'static str> {
    match name.to_ascii_lowercase().as_str() {
        "author" => Some("author"),
        "authors" => Some("authors"),
        "organization" => Some("organization"),
        "address" => Some("address"),
        "contact" => Some("contact"),
        "version" => Some("version"),
        "revision" => Some("revision"),
        "status" => Some("status"),
        "date" => Some("date"),
        "copyright" => Some("copyright"),
        "abstract" => Some("topic"),
        "dedication" => Some("topic"),
        _ => None,
    }
}

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

// ────────────────────────────────────────────────────────────────────────────
// Post-processing
// ────────────────────────────────────────────────────────────────────────────

fn resolve_references(tree: &mut Doctree) {
    let mut targets: HashMap<String, String> = HashMap::new();
    collect_targets(tree, tree.root(), &mut targets);
    apply_targets(tree, tree.root(), &targets);
}

fn collect_targets(tree: &Doctree, id: NodeId, out: &mut HashMap<String, String>) {
    if let NodeKind::Target { names, refuri, .. } = &tree.node(id).kind {
        // `names` is the docutils-normalized form; the source name (with
        // backslash-escapes) is what references use. Convert by removing
        // `\ ` sequences.
        let key = names.replace("\\ ", " ");
        out.insert(key, refuri.clone());
    }
    let children = tree.node(id).children.clone();
    for c in children {
        collect_targets(tree, c, out);
    }
}

fn apply_targets(tree: &mut Doctree, id: NodeId, targets: &HashMap<String, String>) {
    if let NodeKind::Reference { name, refuri } = &mut tree.node_mut(id).kind {
        if refuri.is_empty() {
            let key = name.to_ascii_lowercase();
            if let Some(uri) = targets.get(&key) {
                *refuri = uri.clone();
            }
        }
    }
    let children = tree.node(id).children.clone();
    for c in children {
        apply_targets(tree, c, targets);
    }
}

/// Promote a leading section to document title (and optionally a second one
/// to subtitle), matching docutils' DocTitle transform.
fn promote_document_title(tree: &mut Doctree) {
    let root = tree.root();
    let children = tree.node(root).children.clone();
    let mut sec_idx = None;
    for (i, c) in children.iter().enumerate() {
        if matches!(&tree.node(*c).kind, NodeKind::Section { .. }) {
            sec_idx = Some(i);
            break;
        }
    }
    let i = match sec_idx {
        Some(i) => i,
        None => return,
    };
    // Must be exactly one top-level section (other siblings are non-section).
    let count = children
        .iter()
        .filter(|c| matches!(&tree.node(**c).kind, NodeKind::Section { .. }))
        .count();
    if count != 1 || i != 0 {
        return;
    }
    let sec_id = children[0];
    let (sec_ids, sec_names) = match &tree.node(sec_id).kind {
        NodeKind::Section { ids, names } => (ids.clone(), names.clone()),
        _ => return,
    };
    // Section children: first should be Title.
    let sec_children = tree.node(sec_id).children.clone();
    if sec_children.is_empty() {
        return;
    }
    let title_id = sec_children[0];
    if !matches!(&tree.node(title_id).kind, NodeKind::Title) {
        return;
    }
    // Extract title text.
    let title_text = collect_text(tree, title_id);
    // Move title to be a child of root; replace document's source attr with
    // ids/names/title.
    tree.detach(title_id);
    tree.detach(sec_id);
    if let NodeKind::Document {
        source: _,
        ids,
        names,
        title,
    } = &mut tree.node_mut(root).kind
    {
        *ids = sec_ids.clone();
        *names = sec_names.clone();
        *title = title_text;
    }
    // Re-attach title as first child of root.
    tree.node_mut(root).children.insert(0, title_id);
    tree.node_mut(title_id).parent = Some(root);
    // Hoist the section's other children to be children of root, in order.
    let mut insert_at = 1;
    for c in sec_children.iter().skip(1) {
        // Check: if exactly one section among them, lift it as subtitle.
        let _ = c;
    }
    let promoted_to_subtitle = {
        let body = &sec_children[1..];
        body.len() == 1 && matches!(&tree.node(body[0]).kind, NodeKind::Section { .. })
    };
    if promoted_to_subtitle {
        let sub_id = sec_children[1];
        let (sids, snames) = match &tree.node(sub_id).kind {
            NodeKind::Section { ids, names } => (ids.clone(), names.clone()),
            _ => unreachable!(),
        };
        let sub_children = tree.node(sub_id).children.clone();
        let sub_title = sub_children[0];
        let sub_title_text = collect_text(tree, sub_title);
        tree.detach(sub_id);
        // Build subtitle node.
        let subtitle = tree.append(
            root,
            NodeKind::Subtitle {
                ids: sids,
                names: snames,
            },
        );
        tree.append(subtitle, NodeKind::Text(sub_title_text));
        // Move the subtitle just after the title.
        let last_idx = tree.node(root).children.len() - 1;
        let id = tree.node_mut(root).children.remove(last_idx);
        tree.node_mut(root).children.insert(1, id);
        // Hoist the subtitle's remaining body to root.
        for c in sub_children.into_iter().skip(1) {
            tree.detach(c);
            tree.node_mut(root).children.push(c);
            tree.node_mut(c).parent = Some(root);
        }
    } else {
        for c in sec_children.into_iter().skip(1) {
            tree.detach(c);
            tree.node_mut(root).children.insert(insert_at, c);
            tree.node_mut(c).parent = Some(root);
            insert_at += 1;
        }
    }
}

fn collect_text(tree: &Doctree, id: NodeId) -> String {
    let mut out = String::new();
    walk_text(tree, id, &mut out);
    out
}

fn walk_text(tree: &Doctree, id: NodeId, out: &mut String) {
    if let NodeKind::Text(s) = &tree.node(id).kind {
        out.push_str(s);
    }
    for &c in &tree.node(id).children {
        walk_text(tree, c, out);
    }
}

/// After title promotion, convert a leading `<field_list>` of all
/// bibliographic fields into a `<docinfo>` block. Mirrors the docutils
/// DocInfo transform on a hoisted document.
fn promote_docinfo(tree: &mut Doctree) {
    let root = tree.root();
    let children = tree.node(root).children.clone();
    // Find the first FieldList that follows only Title/Subtitle siblings.
    let mut target_idx: Option<usize> = None;
    for (i, c) in children.iter().enumerate() {
        match &tree.node(*c).kind {
            NodeKind::Title | NodeKind::Subtitle { .. } => continue,
            NodeKind::FieldList => {
                target_idx = Some(i);
                break;
            }
            _ => return,
        }
    }
    let fl_idx = match target_idx {
        Some(i) => i,
        None => return,
    };
    let fl_id = children[fl_idx];
    // Collect (tag, body_text) for each field; bail if any is unrecognized.
    let fields: Vec<NodeId> = tree.node(fl_id).children.clone();
    let mut promoted: Vec<(&'static str, String)> = Vec::new();
    for field in &fields {
        let fc = tree.node(*field).children.clone();
        if fc.len() < 2 {
            return;
        }
        let name = collect_text(tree, fc[0]);
        let tag = match recognized_bibliographic(&name) {
            Some(t) => t,
            None => return,
        };
        // Body's first paragraph text.
        let body_id = fc[1];
        let body_text = collect_text(tree, body_id);
        promoted.push((tag, body_text));
    }
    // Replace field_list with docinfo.
    tree.detach(fl_id);
    // Insert docinfo at fl_idx.
    let docinfo = tree.append(root, NodeKind::Docinfo);
    let last = tree.node(root).children.len() - 1;
    let id = tree.node_mut(root).children.remove(last);
    tree.node_mut(root).children.insert(fl_idx, id);
    for (tag, body_text) in promoted {
        let bib = tree.append(docinfo, NodeKind::Bibliographic { tag });
        if !body_text.is_empty() {
            tree.append(bib, NodeKind::Text(body_text));
        }
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Inline
// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum InlineKind {
    Strong,
    Literal,
    Emphasis,
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

struct Escaped {
    text: String,
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
                    let prev = text.len();
                    text.push(nc);
                    while escaped.len() < text.len() {
                        escaped.push(true);
                    }
                    let _ = prev;
                }
                None => {
                    text.push('\\');
                    while escaped.len() < text.len() {
                        escaped.push(false);
                    }
                }
            }
            continue;
        }
        text.push(c);
        while escaped.len() < text.len() {
            escaped.push(false);
        }
    }
    Escaped { text, escaped }
}

fn parse_inline(tree: &mut Doctree, parent: NodeId, ctx: &ParseCtx, raw: &str) {
    let pre = preprocess_escapes(raw);
    let text = &pre.text;
    let bytes = text.as_bytes();
    let mut cursor = 0;
    let mut text_start = 0;

    while cursor < bytes.len() {
        // Substitution reference: `|name|`.
        if let Some((name, end)) = try_match_substitution(text, &pre.escaped, cursor) {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            if let Some(value) = ctx.subs.get(&name) {
                // Inline expansion: emit the substitution body as plain text.
                push_text(tree, parent, value);
            } else {
                let n = tree.append(parent, NodeKind::SubstitutionReference { refname: name });
                push_text(tree, n, "");
            }
            cursor = end;
            text_start = cursor;
            continue;
        }
        // Role + interpreted text: `:role:`text``.
        if let Some((role, content, end)) = try_match_role(text, &pre.escaped, cursor) {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            emit_role(tree, parent, &role, &content);
            cursor = end;
            text_start = cursor;
            continue;
        }
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
        } else if let Some((name, end)) = try_match_phrase_reference(text, &pre.escaped, cursor) {
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

fn emit_role(tree: &mut Doctree, parent: NodeId, role: &str, content: &str) {
    match role {
        "emphasis" => {
            let n = tree.append(parent, NodeKind::Emphasis);
            push_text(tree, n, content);
        }
        "strong" => {
            let n = tree.append(parent, NodeKind::Strong);
            push_text(tree, n, content);
        }
        "literal" | "code" => {
            let n = tree.append(parent, NodeKind::Literal);
            push_text(tree, n, content);
        }
        "title" | "title-reference" | "t" => {
            let n = tree.append(parent, NodeKind::TitleReference);
            push_text(tree, n, content);
        }
        _ => {
            let n = tree.append(
                parent,
                NodeKind::Inline {
                    classes: role.to_string(),
                },
            );
            push_text(tree, n, content);
        }
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

fn try_match_substitution(text: &str, escaped: &[bool], start: usize) -> Option<(String, usize)> {
    if escaped.get(start).copied().unwrap_or(false) {
        return None;
    }
    if text.as_bytes().get(start)? != &b'|' {
        return None;
    }
    if !valid_start_context(text, start) {
        return None;
    }
    let rest = &text[start + 1..];
    let end_rel = rest.find('|')?;
    let abs_end = start + 1 + end_rel;
    if escaped[abs_end] {
        return None;
    }
    let name = &text[start + 1..abs_end];
    if name.is_empty() || name.contains('\n') {
        return None;
    }
    Some((name.to_string(), abs_end + 1))
}

fn try_match_role(text: &str, escaped: &[bool], start: usize) -> Option<(String, String, usize)> {
    if escaped.get(start).copied().unwrap_or(false) {
        return None;
    }
    if text.as_bytes().get(start)? != &b':' {
        return None;
    }
    if !valid_start_context(text, start) {
        return None;
    }
    let after = &text[start + 1..];
    let end_role = after.find(':')?;
    let role = &after[..end_role];
    if role.is_empty()
        || !role
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
    {
        return None;
    }
    let abs_after_role = start + 1 + end_role + 1;
    if text.as_bytes().get(abs_after_role)? != &b'`' {
        return None;
    }
    let content_start = abs_after_role + 1;
    let rest = &text[content_start..];
    let end_rel = rest.find('`')?;
    let content = &rest[..end_rel];
    Some((
        role.to_string(),
        content.to_string(),
        content_start + end_rel + 1,
    ))
}

fn try_match_phrase_reference(
    text: &str,
    escaped: &[bool],
    start: usize,
) -> Option<(String, usize)> {
    if escaped.get(start).copied().unwrap_or(false) {
        return None;
    }
    if text.as_bytes().get(start)? != &b'`' {
        return None;
    }
    if !valid_start_context(text, start) {
        return None;
    }
    let after = &text[start + 1..];
    let end_rel = after.find('`')?;
    let abs_close = start + 1 + end_rel;
    let after_close = &text[abs_close + 1..];
    if !after_close.starts_with('_') {
        return None;
    }
    let abs_underscore = abs_close + 1;
    let after_under = abs_underscore + 1;
    if let Some(b) = text.as_bytes().get(after_under)
        && b.is_ascii_alphanumeric()
    {
        return None;
    }
    let name = text[start + 1..abs_close].to_string();
    Some((name, after_under))
}

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
            let next = bytes.get(end + 1).copied();
            if matches!(next, Some(n) if n.is_ascii_alphanumeric()) {
                end += 1;
                continue;
            }
            break;
        }
        break;
    }
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
