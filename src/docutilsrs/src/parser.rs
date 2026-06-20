//! rST parser slice covering phase 1 + phase 2.
//!
//! See `docs/compat.md` for the supported feature matrix and accepted
//! deviations. In particular, no syntax highlighting is applied to
//! `code`/`code-block` content (it is emitted as a plain literal block).

#![allow(
    clippy::needless_range_loop,
    clippy::collapsible_if,
    clippy::enum_variant_names,
    clippy::type_complexity,
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

    let blocks = parse_blocks(&lines, 0, 1);
    let mut ctx = ParseCtx {
        subs,
        anon_target_count: 0,
        anon_target_uris: Vec::new(),
        footnote_count: 0,
        citation_count: 0,
        footnote_ref_count: 0,
        citation_ref_count: 0,
        current_line: 0,
        inline_ref_sites: Vec::new(),
    };
    for block in blocks {
        emit_block(&mut tree, document, &mut ctx, block);
    }

    resolve_references(&mut tree, &ctx);
    promote_document_title(&mut tree);
    promote_docinfo(&mut tree);
    emit_unresolved_system_messages(&mut tree, &ctx);
    crate::plugins::apply_transforms(&mut tree);
    tree
}

// ────────────────────────────────────────────────────────────────────────────
// Parser context shared across emit_block calls.
// ────────────────────────────────────────────────────────────────────────────

pub struct ParseCtx {
    pub(crate) subs: HashMap<String, String>,
    pub(crate) anon_target_count: u32,
    pub(crate) anon_target_uris: Vec<String>,
    pub(crate) footnote_count: u32,
    pub(crate) citation_count: u32,
    pub(crate) footnote_ref_count: u32,
    pub(crate) citation_ref_count: u32,
    /// Source line of the paragraph currently being emitted, or 0 when
    /// unknown. Used to stamp `line=` on system messages for unresolved
    /// references discovered during inline parsing.
    pub(crate) current_line: u32,
    /// `(reference_node_id, source_line, name)` for every inline
    /// `Reference` produced; consulted post-resolution to emit
    /// `<problematic>` placeholders and a `system-messages` section for
    /// unresolved targets.
    pub(crate) inline_ref_sites: Vec<(NodeId, u32, String)>,
}

impl ParseCtx {
    fn next_anon_target(&mut self) -> u32 {
        self.anon_target_count += 1;
        self.anon_target_count
    }
}

// ────────────────────────────────────────────────────────────────────────────
// Block model
// ────────────────────────────────────────────────────────────────────────────

enum Block {
    /// `line` is the 1-based source line the paragraph starts on,
    /// or 0 when unknown (e.g. for content re-parsed from a nested
    /// context).
    Paragraph {
        text: String,
        line: u32,
    },
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
        tokens: Option<Vec<crate::code_block::Span>>,
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
        anonymous: bool,
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
    Figure {
        uri: String,
        alt: Option<String>,
        width: Option<String>,
        height: Option<String>,
        caption: Option<String>,
        legend: Vec<Block>,
    },
    Raw {
        format: String,
        text: String,
    },
    /// `.. math::` directive — accumulated LaTeX source. Routed through
    /// `mathrenderrs` at HTML-emit time.
    MathBlock {
        latex: String,
    },
    Footnote {
        label: String,
        body: Vec<Block>,
    },
    Citation {
        label: String,
        body: Vec<Block>,
    },
    /// Spliced result of a Python directive plugin: emitted as if its
    /// children were siblings of the directive site.
    PluginResult(Vec<Block>),
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
    /// Optional header row(s).
    head: Vec<Vec<Option<TableCell>>>,
    body: Vec<Vec<Option<TableCell>>>,
}

struct TableCell {
    /// Raw cell lines (between borders), dedented; may contain blank
    /// lines to introduce multiple paragraphs.
    lines: Vec<String>,
    morecols: u32,
    morerows: u32,
}

// ────────────────────────────────────────────────────────────────────────────
// Block-level parser
// ────────────────────────────────────────────────────────────────────────────

fn parse_blocks(lines: &[&str], base_indent: usize, base_line: u32) -> Vec<Block> {
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

        // Section (incl. overlined variant). Must be checked BEFORE the
        // transition rule, because an overline looks like a transition.
        if base_indent == 0
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
            let body = parse_blocks(&lines[body_start..end], 0, 0);
            blocks.push(Block::Section {
                title,
                level,
                children: body,
            });
            i = end;
            continue;
        }

        // Transition: 4+ identical punctuation chars on a line, surrounded by
        // blank lines. Cheap detection: at column 0, line is all one of
        // -=~`#"^*+<>:_'.
        if base_indent == 0 && is_transition(stripped) {
            blocks.push(Block::Transition);
            i += 1;
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
                    items.push(parse_blocks(&refs, item_indent, 0));
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
                items.push(parse_blocks(&refs, item_indent, 0));
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
            let children = parse_blocks(&quote_lines, qb_indent, 0);
            blocks.push(Block::BlockQuote(children));
            continue;
        }

        // Paragraph (with possible trailing `::` literal block).
        let para_start = i;
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
            let line = if base_line == 0 {
                0
            } else {
                base_line + para_start as u32
            };
            blocks.push(Block::Paragraph { text, line });
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
                        tokens: None,
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
    // ── overlined variant: `over` + title + `under` (same char) ─────
    if i + 2 < lines.len() {
        let over = lines[i];
        let title_o = lines[i + 1];
        let under_o = lines[i + 2];
        if !over.is_empty() && !title_o.trim().is_empty() && !under_o.is_empty() {
            let first = over.chars().next().unwrap();
            if is_section_char(first) {
                let over_trim = over.trim_end();
                let under_trim_o = under_o.trim_end();
                if over_trim.chars().all(|c| c == first)
                    && under_trim_o == over_trim
                    && over_trim.chars().count() >= title_o.trim_end().chars().count()
                {
                    // Overlined sections always get level 0 in docutils when
                    // they appear as the document title; nested overlines
                    // get their own level keyed on the char.
                    let level = match section_chars.iter().position(|&c| c == first) {
                        Some(idx) => idx,
                        None => {
                            section_chars.push(first);
                            section_chars.len() - 1
                        }
                    };
                    return Some((level, title_o.trim().to_string(), 3));
                }
            }
        }
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
        let body_blocks = parse_blocks(&refs, body_indent, 0);
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
        let definition = parse_blocks(&refs, def_indent, 0);
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
    line.starts_with(".. ") || line == ".." || line.starts_with("__ ")
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
    // Anonymous-target shorthand: a line beginning with `__ ` is treated as
    // `.. __ ...`.
    let rest = if let Some(r) = after.strip_prefix("__ ") {
        // Re-add the leading `__` since parse_target_inline expects it.
        // Use a leaked-ish approach: just build it here.
        let owned = format!("__ {r}");
        if let Some((name, refuri, anonymous)) = parse_target_inline(&owned) {
            *i_ref = i + 1;
            return Some(Block::Target {
                name,
                refuri,
                anonymous,
            });
        }
        // If parse_target_inline failed, fall through to comment.
        owned
    } else {
        after.strip_prefix(".. ").unwrap_or("").to_string()
    };
    let rest = rest.as_str();

    // Hyperlink target: `.. _name: refuri` or anonymous `.. __ refuri`.
    if let Some((name, refuri, anonymous)) = parse_target_inline(rest) {
        *i_ref = i + 1;
        return Some(Block::Target {
            name,
            refuri,
            anonymous,
        });
    }

    // Footnote / citation: `.. [label] body`.
    if let Some((label, first_line)) = parse_footnote_head(rest) {
        // Body: first line (after label) joined with subsequent indented
        // lines, then re-parsed as blocks.
        *i_ref = i + 1;
        let mut body_lines: Vec<String> = Vec::new();
        if !first_line.is_empty() {
            body_lines.push(first_line.to_string());
        }
        // Collect continuation lines indented past the explicit-markup
        // prefix (column `base_indent + 3`).
        let cont_indent = base_indent + 3;
        while *i_ref < lines.len() {
            let l = lines[*i_ref];
            if l.trim().is_empty() {
                body_lines.push(String::new());
                *i_ref += 1;
                continue;
            }
            if leading_spaces(l).unwrap_or(0) < cont_indent {
                break;
            }
            body_lines.push(l[cont_indent..].to_string());
            *i_ref += 1;
        }
        // Drop trailing blanks.
        while body_lines.last().map(|s| s.is_empty()).unwrap_or(false) {
            body_lines.pop();
        }
        let body_refs: Vec<&str> = body_lines.iter().map(|s| s.as_str()).collect();
        let body = parse_blocks(&body_refs, 0, 0);
        return Some(if is_citation_label(&label) {
            Block::Citation { label, body }
        } else {
            Block::Footnote { label, body }
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

/// `.. [label] body` → `(label, first_line_after_label)`.
fn parse_footnote_head(rest: &str) -> Option<(String, &str)> {
    let after = rest.strip_prefix('[')?;
    let close = after.find(']')?;
    let label = after[..close].to_string();
    if label.is_empty() {
        return None;
    }
    let tail = &after[close + 1..];
    // Must be followed by whitespace or end-of-line.
    if !tail.is_empty() && !tail.starts_with(' ') {
        return None;
    }
    Some((label, tail.trim_start()))
}

fn is_citation_label(label: &str) -> bool {
    // Numeric labels are footnotes; `#` autonumber / `*` autosymbol are
    // footnotes; everything else (e.g. `Knuth1986`) is a citation.
    if label.is_empty() {
        return false;
    }
    if label == "#" || label == "*" || label.starts_with("#") {
        return false;
    }
    !label.chars().all(|c| c.is_ascii_digit())
}

fn parse_target_inline(rest: &str) -> Option<(String, String, bool)> {
    // Anonymous target: `__ refuri` (no name, no colon).
    if let Some(after) = rest.strip_prefix("__") {
        let uri = after.trim();
        if uri.is_empty() {
            return None;
        }
        // Strip optional leading colon (rare form `.. __: uri`).
        let uri = uri.strip_prefix(':').map(str::trim_start).unwrap_or(uri);
        return Some((String::new(), uri.to_string(), true));
    }
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
    Some((name, refuri.to_string(), false))
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
                child_blocks.push(Block::Paragraph {
                    text: args.to_string(),
                    line: 0,
                });
            }
            *i_ref += 1;
            // Skip blank lines, then collect indented content.
            let content_indent = peek_inner_indent(lines, *i_ref, base_indent);
            if let Some(ci) = content_indent {
                let inner = consume_indented_lines(lines, i_ref, ci);
                let refs: Vec<&str> = inner.iter().map(|s| s.as_str()).collect();
                let mut more = parse_blocks(&refs, ci, 0);
                child_blocks.append(&mut more);
            }
            Block::Admonition {
                kind,
                children: child_blocks,
            }
        }
        "image" | "figure" => {
            let is_figure = name == "figure";
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
            let mut body_indent = None;
            if j < lines.len() {
                if let Some(ind) = leading_spaces(lines[j])
                    && ind > base_indent
                {
                    body_indent = Some(ind);
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
            if !is_figure {
                return Block::Image {
                    uri,
                    alt,
                    width,
                    height,
                };
            }
            // Figure body: skip blanks; if indented at `body_indent`,
            // collect lines until dedent and parse them as blocks. The
            // first block becomes the caption, the rest become the legend.
            let mut caption = None;
            let mut legend = Vec::new();
            let mut k = *i_ref;
            while k < lines.len() && lines[k].trim().is_empty() {
                k += 1;
            }
            if k < lines.len() {
                let ind =
                    body_indent.or_else(|| leading_spaces(lines[k]).filter(|n| *n > base_indent));
                if let Some(ind) = ind {
                    let start = k;
                    while k < lines.len() {
                        let l = lines[k];
                        if l.trim().is_empty() {
                            k += 1;
                            continue;
                        }
                        if leading_spaces(l).unwrap_or(0) < ind {
                            break;
                        }
                        k += 1;
                    }
                    let inner = parse_blocks(&lines[start..k], ind, 0);
                    *i_ref = k;
                    let mut iter = inner.into_iter();
                    if let Some(first) = iter.next() {
                        if let Block::Paragraph { text, .. } = first {
                            caption = Some(text);
                        } else {
                            legend.push(first);
                        }
                    }
                    for b in iter {
                        legend.push(b);
                    }
                }
            }
            Block::Figure {
                uri,
                alt,
                width,
                height,
                caption,
                legend,
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
            let text = inner.join("\n");
            let tokens = crate::code_block::tokenize(lang, &text);
            Block::LiteralBlock {
                text,
                classes,
                tokens,
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
        "math" => {
            // `.. math:: …` — args (if any) become the first line of the
            // LaTeX body; any following indented block is appended. The
            // body is emitted verbatim into a `<math_block>` and rendered
            // through `mathrenderrs` at HTML time.
            *i_ref += 1;
            let inner = consume_indented_text(lines, i_ref, base_indent);
            let args_t = args.trim();
            let body_joined = inner.join("\n");
            let latex = match (args_t.is_empty(), body_joined.is_empty()) {
                (true, _) => body_joined,
                (false, true) => args_t.to_string(),
                (false, false) => format!("{}\n{}", args_t, body_joined),
            };
            Block::MathBlock { latex }
        }
        _ => {
            // Unknown directive: consult the Python plugin registry. A
            // registered plugin receives `(args, body)` and returns a
            // replacement rST string which is re-parsed in place. If no
            // plugin is registered (or the callable fails), fall back to
            // the historical behaviour of swallowing as a comment.
            *i_ref += 1;
            let inner = consume_indented_text(lines, i_ref, base_indent);
            let body_text = inner.join("\n");
            if crate::plugins::has_plugin(&name) {
                if let Some(rst) = crate::plugins::invoke_plugin(&name, args, &body_text) {
                    let rst_lines: Vec<&str> = rst.lines().collect();
                    let blocks = parse_blocks(&rst_lines, 0, 0);
                    return Block::PluginResult(blocks);
                }
            }
            Block::Comment(body_text)
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

    // Build the table block as owned strings, all padded to the same width.
    let raw: Vec<&str> = (start..end).map(|k| &lines[k][base_indent..]).collect();
    let width = raw.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut block: Vec<Vec<u8>> = raw
        .iter()
        .map(|s| {
            let mut v = s.as_bytes().to_vec();
            if v.len() < width {
                v.resize(width, b' ');
            }
            v
        })
        .collect();
    let bottom_row = block.len() - 1;
    let right_col = width - 1;
    if block[0][0] != b'+' || block[0][right_col] != b'+' {
        return None;
    }
    if block[bottom_row][0] != b'+' || block[bottom_row][right_col] != b'+' {
        return None;
    }

    // Detect head/body separator (row of `=` on a separator line) and
    // rewrite it in-place as `-` chars so the scan_cell algorithm can
    // treat body cells uniformly (matches docutils tableparser).
    let mut head_body_sep: Option<usize> = None;
    for i in 1..bottom_row {
        if block[i][0] == b'+' {
            let mut has_eq = false;
            let mut ok = true;
            for c in 1..right_col {
                match block[i][c] {
                    b'=' => has_eq = true,
                    b'-' | b'+' => {}
                    _ => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok && has_eq {
                head_body_sep = Some(i);
                for c in 0..width {
                    if block[i][c] == b'=' {
                        block[i][c] = b'-';
                    }
                }
                break;
            }
        }
    }

    // BFS scan_cell starting from (0, 0).
    use std::collections::BTreeSet;
    let mut rowseps: BTreeSet<usize> = BTreeSet::new();
    let mut colseps: BTreeSet<usize> = BTreeSet::new();
    rowseps.insert(0);
    colseps.insert(0);
    let mut cells: Vec<(usize, usize, usize, usize)> = Vec::new();
    let mut done: Vec<i64> = vec![-1; width];

    let scan_up =
        |top: usize, left: usize, bottom: usize, block: &Vec<Vec<u8>>| -> Option<Vec<usize>> {
            let mut rs = Vec::new();
            let mut i = bottom;
            while i > top + 1 {
                i -= 1;
                match block[i][left] {
                    b'+' => rs.push(i),
                    b'|' => {}
                    _ => return None,
                }
            }
            Some(rs)
        };
    let scan_left = |top: usize,
                     left: usize,
                     bottom: usize,
                     right: usize,
                     block: &Vec<Vec<u8>>|
     -> Option<(Vec<usize>, Vec<usize>)> {
        let mut cs = Vec::new();
        let line = &block[bottom];
        let mut found_sep_eq = false;
        for i in (left + 1..right).rev() {
            match line[i] {
                b'+' => cs.push(i),
                b'-' => {}
                b'=' => {
                    found_sep_eq = true;
                }
                _ => return None,
            }
        }
        if line[left] != b'+' {
            return None;
        }
        let rs = scan_up(top, left, bottom, block)?;
        let _ = found_sep_eq;
        Some((rs, cs))
    };
    let scan_down = |top: usize,
                     left: usize,
                     right: usize,
                     block: &Vec<Vec<u8>>,
                     bottom_row: usize|
     -> Option<(usize, Vec<usize>, Vec<usize>)> {
        let mut rs = Vec::new();
        for i in top + 1..=bottom_row {
            match block[i][right] {
                b'+' => {
                    rs.push(i);
                    if let Some((newrs, cs)) = scan_left(top, left, i, right, block) {
                        rs.extend(newrs);
                        return Some((i, rs, cs));
                    }
                }
                b'|' => {}
                _ => return None,
            }
        }
        None
    };
    let scan_right = |top: usize,
                      left: usize,
                      block: &Vec<Vec<u8>>,
                      bottom_row: usize,
                      right_col: usize|
     -> Option<(usize, usize, Vec<usize>, Vec<usize>)> {
        let mut cs = Vec::new();
        let line = &block[top];
        for i in left + 1..=right_col {
            match line[i] {
                b'+' => {
                    cs.push(i);
                    if let Some((bottom, rs, newcs)) = scan_down(top, left, i, block, bottom_row) {
                        cs.extend(newcs);
                        return Some((bottom, i, rs, cs));
                    }
                }
                b'-' => {}
                _ => return None,
            }
        }
        None
    };

    let mut corners: Vec<(usize, usize)> = vec![(0, 0)];
    while !corners.is_empty() {
        let (top, left) = corners.remove(0);
        if top == bottom_row || left == right_col || (top as i64) <= done[left] {
            continue;
        }
        if block[top][left] != b'+' {
            continue;
        }
        let result = scan_right(top, left, &block, bottom_row, right_col);
        let Some((bottom, right, rs, cs)) = result else {
            continue;
        };
        for r in &rs {
            rowseps.insert(*r);
        }
        for c in &cs {
            colseps.insert(*c);
        }
        rowseps.insert(top);
        rowseps.insert(bottom);
        colseps.insert(left);
        colseps.insert(right);
        // mark_done: columns left..right consumed down to bottom-1.
        for col in left..right {
            done[col] = (bottom as i64) - 1;
        }
        cells.push((top, left, bottom, right));
        corners.push((top, right));
        corners.push((bottom, left));
        corners.sort();
    }

    let rowseps_v: Vec<usize> = rowseps.into_iter().collect();
    let colseps_v: Vec<usize> = colseps.into_iter().collect();
    if rowseps_v.len() < 2 || colseps_v.len() < 2 {
        return None;
    }
    let rowindex: std::collections::HashMap<usize, usize> =
        rowseps_v.iter().enumerate().map(|(i, &r)| (r, i)).collect();
    let colindex: std::collections::HashMap<usize, usize> =
        colseps_v.iter().enumerate().map(|(i, &c)| (c, i)).collect();
    let col_widths: Vec<usize> = (1..colseps_v.len())
        .map(|i| colseps_v[i] - colseps_v[i - 1] - 1)
        .collect();
    let num_rows = rowseps_v.len() - 1;
    let num_cols = colseps_v.len() - 1;
    let mut grid: Vec<Vec<Option<TableCell>>> = (0..num_rows)
        .map(|_| (0..num_cols).map(|_| None).collect())
        .collect();
    for (top, left, bottom, right) in cells {
        let rownum = *rowindex.get(&top)?;
        let colnum = *colindex.get(&left)?;
        let rownum_b = *rowindex.get(&bottom)?;
        let colnum_r = *colindex.get(&right)?;
        let morerows = rownum_b - rownum - 1;
        let morecols = colnum_r - colnum - 1;
        // Extract cell content: rows top+1..bottom, cols left+1..right.
        let mut cell_lines: Vec<String> = Vec::new();
        for r in top + 1..bottom {
            let slice = &block[r][left + 1..right];
            let s = std::str::from_utf8(slice)
                .unwrap_or("")
                .trim_end()
                .to_string();
            cell_lines.push(s);
        }
        // Dedent: compute min leading spaces of non-empty lines.
        let min_indent = cell_lines
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.chars().take_while(|&c| c == ' ').count())
            .min()
            .unwrap_or(0);
        if min_indent > 0 {
            for s in cell_lines.iter_mut() {
                if s.len() >= min_indent {
                    *s = s[min_indent..].to_string();
                }
            }
        }
        // Strip leading/trailing blank lines.
        while cell_lines.first().is_some_and(|s| s.is_empty()) {
            cell_lines.remove(0);
        }
        while cell_lines.last().is_some_and(|s| s.is_empty()) {
            cell_lines.pop();
        }
        grid[rownum][colnum] = Some(TableCell {
            lines: cell_lines,
            morecols: morecols as u32,
            morerows: morerows as u32,
        });
    }
    // Split into head/body by head_body_sep row index.
    let (head, body) = match head_body_sep {
        Some(hsep) => {
            let hi = *rowindex.get(&hsep).unwrap_or(&0);
            let mut h = grid;
            let b = h.split_off(hi);
            (h, b)
        }
        None => (Vec::new(), grid),
    };

    *i_ref = end;
    Some(TableData {
        cols: col_widths,
        head,
        body,
    })
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
    let split_row = |row: &str| -> Vec<Option<TableCell>> {
        let mut cells = Vec::with_capacity(col_widths.len());
        for (idx, &(s, e)) in cols.iter().enumerate() {
            let cell = if idx + 1 == cols.len() {
                row.get(s..).unwrap_or("").to_string()
            } else {
                row.get(s..e).unwrap_or("").to_string()
            };
            let trimmed = cell.trim().to_string();
            cells.push(Some(TableCell {
                lines: if trimmed.is_empty() {
                    Vec::new()
                } else {
                    vec![trimmed]
                },
                morecols: 0,
                morerows: 0,
            }));
        }
        cells
    };
    let (head, body): (Vec<Vec<Option<TableCell>>>, Vec<Vec<Option<TableCell>>>) =
        if sep_indices.len() >= 2 {
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

fn emit_block(tree: &mut Doctree, parent: NodeId, ctx: &mut ParseCtx, block: Block) {
    match block {
        Block::Paragraph { text, line } => {
            let prev_line = ctx.current_line;
            ctx.current_line = line;
            let p = tree.append(parent, NodeKind::Paragraph);
            parse_inline(tree, p, ctx, &text);
            ctx.current_line = prev_line;
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
        Block::BlockQuote(mut children) => {
            // Attribution: if the last child is a single Paragraph whose
            // text begins with `-- ` or `--- `, split it off as
            // <attribution>. Indentation/whitespace are stripped.
            let attribution_text = match children.last() {
                Some(Block::Paragraph { text, .. }) => {
                    let t = text.trim_start();
                    t.strip_prefix("--- ")
                        .or_else(|| t.strip_prefix("-- "))
                        .map(|rest| rest.to_string())
                }
                _ => None,
            };
            let attr = attribution_text.inspect(|_t| {
                children.pop();
            });
            let q = tree.append(parent, NodeKind::BlockQuote);
            for b in children {
                emit_block(tree, q, ctx, b);
            }
            if let Some(text) = attr {
                let a = tree.append(q, NodeKind::Attribution);
                parse_inline(tree, a, ctx, &text);
            }
        }
        Block::LiteralBlock {
            text,
            classes,
            tokens,
        } => {
            let lb = tree.append(parent, NodeKind::LiteralBlock { classes });
            match tokens {
                Some(spans) => {
                    for (class, value) in spans {
                        match class {
                            Some(cls) => {
                                let inl = tree.append(lb, NodeKind::Inline { classes: cls });
                                tree.append(inl, NodeKind::Text(value));
                            }
                            None => {
                                tree.append(lb, NodeKind::Text(value));
                            }
                        }
                    }
                }
                None => {
                    tree.append(lb, NodeKind::Text(text));
                }
            }
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
                    classes: String::new(),
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
        Block::Target {
            name,
            refuri,
            anonymous,
        } => {
            if anonymous {
                let n = ctx.next_anon_target();
                let ids = format!("target-{n}");
                tree.append(
                    parent,
                    NodeKind::Target {
                        ids,
                        names: String::new(),
                        refuri: refuri.clone(),
                        anonymous: true,
                    },
                );
                ctx.anon_target_uris.push(refuri);
                return;
            }
            let ids = normalize_id(&name);
            let names = if name.contains(' ') {
                name.split(' ')
                    .map(|w| w.to_ascii_lowercase())
                    .collect::<Vec<_>>()
                    .join("\\ ")
            } else {
                name.to_ascii_lowercase()
            };
            tree.append(
                parent,
                NodeKind::Target {
                    ids,
                    names,
                    refuri,
                    anonymous: false,
                },
            );
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
        Block::Figure {
            uri,
            alt,
            width,
            height,
            caption,
            legend,
        } => {
            let fig = tree.append(parent, NodeKind::Figure);
            tree.append(
                fig,
                NodeKind::Image {
                    uri,
                    alt,
                    width,
                    height,
                },
            );
            if let Some(text) = caption {
                let cap = tree.append(fig, NodeKind::Caption);
                parse_inline(tree, cap, ctx, &text);
            }
            if !legend.is_empty() {
                let leg = tree.append(fig, NodeKind::Legend);
                for b in legend {
                    emit_block(tree, leg, ctx, b);
                }
            }
        }
        Block::Raw { format, text } => {
            let r = tree.append(parent, NodeKind::Raw { format });
            tree.append(r, NodeKind::Text(text));
        }
        Block::MathBlock { latex } => {
            tree.append(parent, NodeKind::MathBlock { latex });
        }
        Block::Table(td) => {
            emit_table(tree, parent, ctx, td);
        }
        Block::Footnote { label, body } => {
            // Classify autonumber / autosymbol vs manual. Auto footnotes
            // get an `auto` marker plus placeholder ids/names that the
            // post-pass in resolve_footnotes rewrites once the document
            // order is known.
            let (ids, names, auto, label_text) = if label == "#" {
                ctx.footnote_count += 1;
                (
                    format!("footnote-{}", ctx.footnote_count),
                    String::new(),
                    Some("1"),
                    "#".to_string(),
                )
            } else if let Some(name) = label.strip_prefix('#') {
                if !name.is_empty() {
                    let n = name.to_ascii_lowercase();
                    (n.clone(), n, Some("1"), label.clone())
                } else {
                    ctx.footnote_count += 1;
                    (
                        format!("footnote-{}", ctx.footnote_count),
                        label.to_ascii_lowercase(),
                        None,
                        label.clone(),
                    )
                }
            } else if label == "*" {
                ctx.footnote_count += 1;
                (
                    format!("footnote-{}", ctx.footnote_count),
                    String::new(),
                    Some("*"),
                    "*".to_string(),
                )
            } else {
                ctx.footnote_count += 1;
                (
                    format!("footnote-{}", ctx.footnote_count),
                    label.to_ascii_lowercase(),
                    None,
                    label.clone(),
                )
            };
            let f = tree.append(
                parent,
                NodeKind::Footnote {
                    ids,
                    names,
                    backrefs: String::new(),
                    auto,
                },
            );
            let lbl = tree.append(f, NodeKind::Label);
            tree.append(lbl, NodeKind::Text(label_text));
            let _ = label;
            for b in body {
                emit_block(tree, f, ctx, b);
            }
        }
        Block::Citation { label, body } => {
            ctx.citation_count += 1;
            let ids = label.to_ascii_lowercase();
            let names = ids.clone();
            let c = tree.append(
                parent,
                NodeKind::Citation {
                    ids,
                    names,
                    backrefs: String::new(),
                },
            );
            let lbl = tree.append(c, NodeKind::Label);
            tree.append(lbl, NodeKind::Text(label));
            for b in body {
                emit_block(tree, c, ctx, b);
            }
        }
        Block::PluginResult(blocks) => {
            for b in blocks {
                emit_block(tree, parent, ctx, b);
            }
        }
    }
}

fn emit_table(tree: &mut Doctree, parent: NodeId, ctx: &mut ParseCtx, td: TableData) {
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
            for slot in row {
                let Some(cell) = slot else { continue };
                let e = tree.append(
                    r,
                    NodeKind::Entry {
                        morecols: cell.morecols,
                        morerows: cell.morerows,
                    },
                );
                emit_cell_content(tree, e, ctx, &cell.lines);
            }
        }
    }
    let tbody = tree.append(tgroup, NodeKind::Tbody);
    for row in td.body {
        let r = tree.append(tbody, NodeKind::Row);
        for slot in row {
            let Some(cell) = slot else { continue };
            let e = tree.append(
                r,
                NodeKind::Entry {
                    morecols: cell.morecols,
                    morerows: cell.morerows,
                },
            );
            emit_cell_content(tree, e, ctx, &cell.lines);
        }
    }
}

fn emit_cell_content(tree: &mut Doctree, entry: NodeId, ctx: &mut ParseCtx, lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    // If the cell is a single non-empty line, emit a single paragraph
    // with inline parsing (fast path; matches docutils on simple cells).
    let has_blank = lines.iter().any(|l| l.is_empty());
    if !has_blank && lines.len() == 1 {
        let p = tree.append(entry, NodeKind::Paragraph);
        parse_inline(tree, p, ctx, &lines[0]);
        return;
    }
    // Otherwise parse the cell content as a sub-block sequence so that
    // multi-paragraph cells produce one `<paragraph>` per blank-line
    // separated chunk.
    let refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    let blocks = parse_blocks(&refs, 0, 0);
    for b in blocks {
        emit_block(tree, entry, ctx, b);
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

pub(crate) fn resolve_references(tree: &mut Doctree, ctx: &ParseCtx) {
    let mut targets: HashMap<String, String> = HashMap::new();
    collect_targets(tree, tree.root(), &mut targets);
    let mut anon_iter = ctx.anon_target_uris.iter().cloned();
    apply_targets(tree, tree.root(), &targets, &mut anon_iter);
    resolve_footnotes(tree);
}

/// Walk `ctx.inline_ref_sites` for `Reference` nodes that remained
/// unresolved (empty `refuri`, non-anonymous), convert each into a
/// `<problematic>` placeholder, and append a `<section
/// classes="system-messages">` at the end of the document containing
/// one `<system_message>` per unresolved target.
pub(crate) fn emit_unresolved_system_messages(tree: &mut Doctree, ctx: &ParseCtx) {
    let mut unresolved: Vec<(NodeId, u32, String)> = Vec::new();
    for (id, line, name) in &ctx.inline_ref_sites {
        if let NodeKind::Reference {
            refuri, anonymous, ..
        } = &tree.node(*id).kind
        {
            if refuri.is_empty() && !anonymous && !name.is_empty() {
                unresolved.push((*id, *line, name.clone()));
            }
        }
    }
    if unresolved.is_empty() {
        return;
    }

    for (idx, (ref_id, _line, _name)) in unresolved.iter().enumerate() {
        let n = idx + 1;
        let ids = format!("problematic-{n}");
        let refid = format!("system-message-{n}");
        tree.set_kind(*ref_id, NodeKind::Problematic { ids, refid });
        // Append trailing underscore to the existing reference text so
        // the problematic renders as `name_`.
        let kids = tree.node(*ref_id).children.clone();
        if let Some(&first) = kids.first() {
            if let NodeKind::Text(t) = &tree.node(first).kind {
                let new_text = format!("{t}_");
                tree.set_kind(first, NodeKind::Text(new_text));
            }
        }
    }

    let root = tree.root();
    let sec = tree.append(
        root,
        NodeKind::Section {
            ids: String::new(),
            names: String::new(),
            classes: "system-messages".to_string(),
        },
    );
    let title = tree.append(sec, NodeKind::Title);
    tree.append(
        title,
        NodeKind::Text("Docutils System Messages".to_string()),
    );
    for (idx, (_ref_id, line, name)) in unresolved.iter().enumerate() {
        let n = idx + 1;
        let sm = tree.append(
            sec,
            NodeKind::SystemMessage {
                level: 3,
                line: if *line > 0 { Some(*line) } else { None },
                ty: "ERROR",
                ids: format!("system-message-{n}"),
                backrefs: format!("problematic-{n}"),
            },
        );
        let p = tree.append(sm, NodeKind::Paragraph);
        tree.append(
            p,
            NodeKind::Text(format!("Unknown target name: \"{}\".", name.to_lowercase())),
        );
    }
}

/// Map footnote labels → ids, then resolve `FootnoteReference.refid` and
/// append back-references into the corresponding footnote's `backrefs`.
fn resolve_footnotes(tree: &mut Doctree) {
    // label (name) → footnote id
    let mut label_to_id: HashMap<String, String> = HashMap::new();
    // id → list of reference ids targeting it
    let mut backrefs: HashMap<String, Vec<String>> = HashMap::new();
    collect_footnote_labels(tree, tree.root(), &mut label_to_id);
    // Auto pass first: rewrites Footnote.names + Label text and turns
    // auto FootnoteReference sentinels into real refids + display text.
    // Manual references that target an auto-named footnote (e.g.,
    // `[a]_` pointing at `.. [#a]`) are resolved via `label_to_id`
    // updated in place.
    resolve_auto_footnotes(tree, &mut label_to_id, &mut backrefs);
    apply_footnote_refs(tree, tree.root(), &label_to_id, &mut backrefs);
    apply_footnote_backrefs(tree, tree.root(), &backrefs);
}

const AUTOSYMBOL_CHARS: &[&str] = &[
    "*", "\u{2020}", "\u{2021}", "\u{a7}", "\u{b6}", "#", "\u{2660}", "\u{2665}", "\u{2666}",
    "\u{2663}",
];

fn autosymbol_label(n: usize) -> String {
    // 1-based n: positions 1..=10 use single chars, 11..=20 doubled, etc.
    let idx = (n - 1) % AUTOSYMBOL_CHARS.len();
    let reps = (n - 1) / AUTOSYMBOL_CHARS.len() + 1;
    AUTOSYMBOL_CHARS[idx].repeat(reps)
}

#[derive(Clone, Copy)]
enum AutoKind {
    Anon,
    Named,
    Sym,
}

fn resolve_auto_footnotes(
    tree: &mut Doctree,
    label_to_id: &mut HashMap<String, String>,
    backrefs: &mut HashMap<String, Vec<String>>,
) {
    // Collect in document order.
    let mut auto_defs: Vec<(NodeId, AutoKind)> = Vec::new();
    let mut auto_refs: Vec<(NodeId, AutoKind, Option<String>)> = Vec::new();
    let mut used_numbers: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    collect_auto_footnotes(
        tree,
        tree.root(),
        &mut auto_defs,
        &mut auto_refs,
        &mut used_numbers,
    );
    // Assign numeric labels to anon / named autos (skipping used numbers).
    let mut next_num: u32 = 1;
    let next_unused = |start: &mut u32, used: &std::collections::BTreeSet<u32>| -> u32 {
        while used.contains(start) {
            *start += 1;
        }
        let n = *start;
        *start += 1;
        n
    };
    // Per-kind ordered lists of (def_node, assigned_label_string).
    let mut anon_defs: Vec<(NodeId, String)> = Vec::new();
    let mut named_defs: Vec<(NodeId, String)> = Vec::new();
    let mut sym_defs: Vec<(NodeId, String)> = Vec::new();
    let mut sym_counter: usize = 0;
    for (nid, kind) in &auto_defs {
        match kind {
            AutoKind::Anon => {
                let n = next_unused(&mut next_num, &used_numbers);
                let s = n.to_string();
                anon_defs.push((*nid, s.clone()));
                // Update Footnote.names to assigned numeric string.
                if let NodeKind::Footnote { names, .. } = &mut tree.node_mut(*nid).kind {
                    *names = s.clone();
                }
                // names lookup for manual `[N]_` style refs (rare overlap).
                let ids = footnote_ids(tree, *nid);
                label_to_id.insert(s.clone(), ids);
                update_label_text(tree, *nid, &s);
            }
            AutoKind::Named => {
                let n = next_unused(&mut next_num, &used_numbers);
                let s = n.to_string();
                named_defs.push((*nid, s.clone()));
                update_label_text(tree, *nid, &s);
            }
            AutoKind::Sym => {
                sym_counter += 1;
                let s = autosymbol_label(sym_counter);
                sym_defs.push((*nid, s.clone()));
                update_label_text(tree, *nid, &s);
            }
        }
    }
    // Resolve references.
    let mut anon_iter = 0usize;
    let mut sym_iter = 0usize;
    // Build a map for named auto by lowercased name (= ids).
    let mut named_map: HashMap<String, (NodeId, String)> = HashMap::new();
    for (nid, label) in &named_defs {
        if let NodeKind::Footnote { ids, .. } = &tree.node(*nid).kind {
            named_map.insert(ids.clone(), (*nid, label.clone()));
        }
    }
    for (ref_nid, kind, name) in auto_refs {
        let (target_def, display) = match kind {
            AutoKind::Anon => {
                if anon_iter >= anon_defs.len() {
                    (None, String::new())
                } else {
                    let (nid, lbl) = &anon_defs[anon_iter];
                    anon_iter += 1;
                    (Some(*nid), lbl.clone())
                }
            }
            AutoKind::Sym => {
                if sym_iter >= sym_defs.len() {
                    (None, String::new())
                } else {
                    let (nid, lbl) = &sym_defs[sym_iter];
                    sym_iter += 1;
                    (Some(*nid), lbl.clone())
                }
            }
            AutoKind::Named => {
                if let Some(key) = name.as_ref() {
                    if let Some((nid, lbl)) = named_map.get(key) {
                        (Some(*nid), lbl.clone())
                    } else {
                        (None, String::new())
                    }
                } else {
                    (None, String::new())
                }
            }
        };
        let ref_ids = if let NodeKind::FootnoteReference { ids, .. } = &tree.node(ref_nid).kind {
            ids.clone()
        } else {
            continue;
        };
        if let Some(def_nid) = target_def {
            let def_ids = footnote_ids(tree, def_nid);
            if let NodeKind::FootnoteReference { refid, .. } = &mut tree.node_mut(ref_nid).kind {
                *refid = def_ids.clone();
            }
            backrefs.entry(def_ids).or_default().push(ref_ids);
            // Rewrite displayed Text child to the assigned label.
            let children = tree.node(ref_nid).children.clone();
            if let Some(text_id) = children.first() {
                if let NodeKind::Text(t) = &mut tree.node_mut(*text_id).kind {
                    *t = display;
                }
            }
        } else if let NodeKind::FootnoteReference { refid, .. } = &mut tree.node_mut(ref_nid).kind {
            refid.clear();
        }
    }
}

fn footnote_ids(tree: &Doctree, id: NodeId) -> String {
    if let NodeKind::Footnote { ids, .. } = &tree.node(id).kind {
        ids.clone()
    } else {
        String::new()
    }
}

fn update_label_text(tree: &mut Doctree, footnote: NodeId, new_text: &str) {
    let children = tree.node(footnote).children.clone();
    for c in children {
        if matches!(tree.node(c).kind, NodeKind::Label) {
            let label_children = tree.node(c).children.clone();
            if let Some(tid) = label_children.first() {
                if let NodeKind::Text(t) = &mut tree.node_mut(*tid).kind {
                    *t = new_text.to_string();
                }
            }
            break;
        }
    }
}

fn collect_auto_footnotes(
    tree: &Doctree,
    id: NodeId,
    defs: &mut Vec<(NodeId, AutoKind)>,
    refs: &mut Vec<(NodeId, AutoKind, Option<String>)>,
    used_numbers: &mut std::collections::BTreeSet<u32>,
) {
    match &tree.node(id).kind {
        NodeKind::Footnote {
            auto, names, ids, ..
        } => match auto {
            Some("*") => defs.push((id, AutoKind::Sym)),
            Some("1") => {
                // Anon auto has empty names at this point; named auto has
                // a non-empty alphabetic ids.
                if names.is_empty() {
                    defs.push((id, AutoKind::Anon));
                } else {
                    defs.push((id, AutoKind::Named));
                }
            }
            _ => {
                // Manual numeric → mark its label number as used.
                if let Ok(n) = names.parse::<u32>() {
                    used_numbers.insert(n);
                }
                let _ = ids;
            }
        },
        NodeKind::FootnoteReference { auto, refid, .. } => match auto {
            Some("*") => refs.push((id, AutoKind::Sym, None)),
            Some("1") => {
                if let Some(name) = refid.strip_prefix("__fnauto_named:") {
                    refs.push((id, AutoKind::Named, Some(name.to_string())));
                } else {
                    refs.push((id, AutoKind::Anon, None));
                }
            }
            _ => {}
        },
        _ => {}
    }
    let children = tree.node(id).children.clone();
    for c in children {
        collect_auto_footnotes(tree, c, defs, refs, used_numbers);
    }
}

fn collect_footnote_labels(tree: &Doctree, id: NodeId, out: &mut HashMap<String, String>) {
    if let NodeKind::Footnote { ids, names, .. } = &tree.node(id).kind {
        out.insert(names.clone(), ids.clone());
    }
    let children = tree.node(id).children.clone();
    for c in children {
        collect_footnote_labels(tree, c, out);
    }
}

fn apply_footnote_refs(
    tree: &mut Doctree,
    id: NodeId,
    labels: &HashMap<String, String>,
    backrefs: &mut HashMap<String, Vec<String>>,
) {
    if let NodeKind::FootnoteReference { ids, refid, .. } = &mut tree.node_mut(id).kind {
        if let Some(label) = refid.strip_prefix("__fnlabel:") {
            let key = label.to_ascii_lowercase();
            if let Some(target) = labels.get(&key) {
                *refid = target.clone();
                backrefs
                    .entry(target.clone())
                    .or_default()
                    .push(ids.clone());
            } else {
                // No matching footnote; leave refid empty so an unresolved
                // pass (future work) can flag it.
                refid.clear();
            }
        }
    }
    if let NodeKind::CitationReference { ids, refid } = &tree.node(id).kind {
        // Citation refs already carry the lowercased label as refid; just
        // record the backref so the citation can list us.
        backrefs.entry(refid.clone()).or_default().push(ids.clone());
    }
    let children = tree.node(id).children.clone();
    for c in children {
        apply_footnote_refs(tree, c, labels, backrefs);
    }
}

fn apply_footnote_backrefs(
    tree: &mut Doctree,
    id: NodeId,
    backrefs: &HashMap<String, Vec<String>>,
) {
    if let NodeKind::Footnote {
        ids, backrefs: br, ..
    } = &mut tree.node_mut(id).kind
    {
        if let Some(list) = backrefs.get(ids) {
            *br = list.join(" ");
        }
    }
    if let NodeKind::Citation {
        ids, backrefs: br, ..
    } = &mut tree.node_mut(id).kind
    {
        if let Some(list) = backrefs.get(ids) {
            *br = list.join(" ");
        }
    }
    let children = tree.node(id).children.clone();
    for c in children {
        apply_footnote_backrefs(tree, c, backrefs);
    }
}

fn collect_targets(tree: &Doctree, id: NodeId, out: &mut HashMap<String, String>) {
    if let NodeKind::Target {
        names,
        refuri,
        anonymous,
        ..
    } = &tree.node(id).kind
    {
        // Anonymous targets carry no names; they're resolved by FIFO order
        // against anonymous references instead.
        if !*anonymous {
            let key = names.replace("\\ ", " ");
            out.insert(key, refuri.clone());
        }
    }
    let children = tree.node(id).children.clone();
    for c in children {
        collect_targets(tree, c, out);
    }
}

fn apply_targets<I: Iterator<Item = String>>(
    tree: &mut Doctree,
    id: NodeId,
    targets: &HashMap<String, String>,
    anon_iter: &mut I,
) {
    if let NodeKind::Reference {
        name,
        refuri,
        anonymous,
    } = &mut tree.node_mut(id).kind
    {
        if refuri.is_empty() {
            if *anonymous {
                if let Some(uri) = anon_iter.next() {
                    *refuri = uri;
                }
            } else {
                let key = name.to_ascii_lowercase();
                if let Some(uri) = targets.get(&key) {
                    *refuri = uri.clone();
                }
            }
        }
    }
    let children = tree.node(id).children.clone();
    for c in children {
        apply_targets(tree, c, targets, anon_iter);
    }
}

/// Promote a leading section to document title (and optionally a second one
/// to subtitle), matching docutils' DocTitle transform.
pub(crate) fn promote_document_title(tree: &mut Doctree) {
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
        NodeKind::Section { ids, names, .. } => (ids.clone(), names.clone()),
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
            NodeKind::Section { ids, names, .. } => (ids.clone(), names.clone()),
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
pub(crate) fn promote_docinfo(tree: &mut Doctree) {
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

fn parse_inline(tree: &mut Doctree, parent: NodeId, ctx: &mut ParseCtx, raw: &str) {
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
        // Footnote / citation reference: `[label]_`.
        if let Some((label, end)) = try_match_footnote_reference(text, &pre.escaped, cursor) {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            if is_citation_label(&label) {
                ctx.citation_ref_count += 1;
                let ids = format!("citation-reference-{}", ctx.citation_ref_count);
                let refid = label.to_ascii_lowercase();
                let n = tree.append(parent, NodeKind::CitationReference { ids, refid });
                push_text(tree, n, &label);
            } else {
                ctx.footnote_ref_count += 1;
                let ids = format!("footnote-reference-{}", ctx.footnote_ref_count);
                // Classify autonumber / autosymbol vs manual numeric. Auto
                // refs get a sentinel refid + the matching `auto` marker;
                // the post-pass in resolve_footnotes assigns the real id
                // and rewrites the displayed label.
                let (refid, auto) = if label == "#" {
                    ("__fnauto_anon:".to_string(), Some("1"))
                } else if let Some(name) = label.strip_prefix('#') {
                    if !name.is_empty() {
                        (
                            format!("__fnauto_named:{}", name.to_ascii_lowercase()),
                            Some("1"),
                        )
                    } else {
                        (format!("__fnlabel:{}", label), None)
                    }
                } else if label == "*" {
                    ("__fnauto_sym:".to_string(), Some("*"))
                } else {
                    (format!("__fnlabel:{}", label), None)
                };
                let n = tree.append(parent, NodeKind::FootnoteReference { ids, refid, auto });
                push_text(tree, n, &label);
            }
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
        } else if let Some((name, embedded_uri, end, anonymous)) =
            try_match_phrase_reference(text, &pre.escaped, cursor)
        {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            let refuri = embedded_uri.clone().unwrap_or_default();
            let node = tree.append(
                parent,
                NodeKind::Reference {
                    name: name.clone(),
                    refuri: refuri.clone(),
                    anonymous,
                },
            );
            if embedded_uri.is_none() && !anonymous {
                ctx.inline_ref_sites
                    .push((node, ctx.current_line, name.clone()));
            }
            push_text(tree, node, &name);
            // For embedded URIs (non-anonymous), docutils also emits an
            // implicit Target sibling within the same paragraph.
            if !anonymous && embedded_uri.is_some() {
                let ids = normalize_id(&name);
                let names = if name.contains(' ') {
                    name.split(' ')
                        .map(|w| w.to_ascii_lowercase())
                        .collect::<Vec<_>>()
                        .join("\\ ")
                } else {
                    name.to_ascii_lowercase()
                };
                tree.append(
                    parent,
                    NodeKind::Target {
                        ids,
                        names,
                        refuri,
                        anonymous: false,
                    },
                );
            }
            cursor = end;
            text_start = cursor;
        } else if let Some((name, end, anonymous)) = try_match_reference(text, &pre.escaped, cursor)
        {
            if cursor > text_start {
                push_text(tree, parent, &text[text_start..cursor]);
            }
            let node = tree.append(
                parent,
                NodeKind::Reference {
                    name: name.clone(),
                    refuri: String::new(),
                    anonymous,
                },
            );
            if !anonymous {
                ctx.inline_ref_sites
                    .push((node, ctx.current_line, name.clone()));
            }
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
        "math" => {
            // Inline math role: `:math:`E=mc^2``. The renderer side
            // (html5_writer) routes this through `mathrenderrs` using
            // whichever backend the writer was configured with
            // (default RaTeX → SVG).
            tree.append(
                parent,
                NodeKind::Math {
                    latex: content.to_string(),
                },
            );
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

fn try_match_footnote_reference(
    text: &str,
    escaped: &[bool],
    start: usize,
) -> Option<(String, usize)> {
    if escaped.get(start).copied().unwrap_or(false) {
        return None;
    }
    if text.as_bytes().get(start)? != &b'[' {
        return None;
    }
    if !valid_start_context(text, start) {
        return None;
    }
    let after = &text[start + 1..];
    let close_rel = after.find(']')?;
    let abs_close = start + 1 + close_rel;
    if escaped[abs_close] {
        return None;
    }
    let label = &text[start + 1..abs_close];
    if label.is_empty() || label.contains('\n') {
        return None;
    }
    if text.as_bytes().get(abs_close + 1) != Some(&b'_') {
        return None;
    }
    if escaped[abs_close + 1] {
        return None;
    }
    Some((label.to_string(), abs_close + 2))
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

/// Returns `(name, embedded_uri, end, anonymous)`. `embedded_uri` is the
/// `<uri>` part inside the backticks, if present.
fn try_match_phrase_reference(
    text: &str,
    escaped: &[bool],
    start: usize,
) -> Option<(String, Option<String>, usize, bool)> {
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
    let mut abs_under_end = abs_close + 2; // past the first `_`
    let mut anonymous = false;
    if text.as_bytes().get(abs_under_end) == Some(&b'_') {
        anonymous = true;
        abs_under_end += 1;
    }
    if let Some(b) = text.as_bytes().get(abs_under_end)
        && b.is_ascii_alphanumeric()
    {
        return None;
    }
    let inner = &text[start + 1..abs_close];
    // Embedded URI: `text <uri>` where the URI must be at the end and
    // preceded by whitespace.
    let (name, uri) = if let Some(lt) = inner.rfind(" <")
        && inner.ends_with('>')
    {
        let name = inner[..lt].trim().to_string();
        let uri = inner[lt + 2..inner.len() - 1].trim().to_string();
        (name, Some(uri))
    } else {
        (inner.to_string(), None)
    };
    Some((name, uri, abs_under_end, anonymous))
}

fn try_match_reference(
    text: &str,
    escaped: &[bool],
    start: usize,
) -> Option<(String, usize, bool)> {
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
    let mut after = end + 1;
    let mut anonymous = false;
    if bytes.get(after) == Some(&b'_') {
        anonymous = true;
        after += 1;
    }
    if after < bytes.len() && bytes[after].is_ascii_alphanumeric() {
        return None;
    }
    let name = text[start..end].to_string();
    Some((name, after, anonymous))
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
