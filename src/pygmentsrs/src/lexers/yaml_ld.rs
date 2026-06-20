//! YAML lexer with YAML-LD (JSON-LD in YAML) and embedded-language support.
//!
//! This is a hand-crafted Rust lexer that covers the most common YAML
//! constructs and adds:
//!
//! 1. **YAML-LD keyword highlighting** — `@context`, `@id`, `@type`, etc.
//!    (the JSON-LD keyword set) are emitted as `Token.Name.Decorator` when
//!    they appear as mapping keys or quoted string values.
//! 2. **Embedded Markdown dispatch** — `|` literal block scalars whose key
//!    contains "description", "markdown", "body", "content", "text", or "md"
//!    (case-insensitive) are passed to the native `markdown` lexer.
//! 3. **Embedded HTML dispatch** — same trigger logic, but using `html`.
//! 4. **IRI highlighting** — string values that start with `http://`,
//!    `https://`, `_:`, or look like compact IRIs (`prefix:local`) are
//!    highlighted as `Token.Name.Other`.
//!
//! This lexer is _not_ byte-parity with `pygments.lexers.data.YamlLexer`
//! (which uses a complex indentation-tracking `ExtendedRegexLexer`). It is
//! instead a best-effort highlighter optimised for YAML-LD documents.
//!
//! Aliases: `yaml-ld`, `yamlld`, `yaml` (shadow — prefer native path when
//! the document contains `@context` or similar JSON-LD keywords).

use crate::lexer::Lexer;
use crate::token::{
    self, COMMENT_SINGLE, KEYWORD_CONSTANT, NAME_CONSTANT, NAME_DECORATOR, NAME_NAMESPACE,
    NAME_TAG, NUMBER_FLOAT, NUMBER_INTEGER, PUNCTUATION, PUNCTUATION_INDICATOR, SCALAR_PLAIN,
    STRING, STRING_DOUBLE, STRING_SINGLE, TokenType, WHITESPACE,
};

// ── JSON-LD keywords (the `@keyword` set from JSON-LD 1.1) ─────────────────

const JSON_LD_KEYWORDS: &[&str] = &[
    "@base",
    "@container",
    "@context",
    "@direction",
    "@graph",
    "@id",
    "@import",
    "@included",
    "@index",
    "@json",
    "@language",
    "@list",
    "@nest",
    "@none",
    "@prefix",
    "@propagate",
    "@protected",
    "@reverse",
    "@set",
    "@type",
    "@value",
    "@version",
    "@vocab",
];

fn is_ld_keyword(s: &str) -> bool {
    JSON_LD_KEYWORDS.contains(&s)
}

/// Keys that trigger embedded-Markdown dispatch for block scalars.
const MARKDOWN_KEY_HINTS: &[&str] = &[
    "description",
    "markdown",
    "body",
    "content",
    "text",
    "md",
    "readme",
    "notes",
    "summary",
    "message",
    "detail",
];

/// Keys that trigger embedded-HTML dispatch for block scalars.
const HTML_KEY_HINTS: &[&str] = &["html", "template", "snippet", "markup"];

fn is_markdown_key(k: &str) -> bool {
    let lower = k.to_lowercase();
    MARKDOWN_KEY_HINTS.iter().any(|h| {
        lower == *h || lower.ends_with(&format!("_{h}")) || lower.ends_with(&format!("-{h}"))
    })
}

fn is_html_key(k: &str) -> bool {
    let lower = k.to_lowercase();
    HTML_KEY_HINTS.iter().any(|h| {
        lower == *h || lower.ends_with(&format!("_{h}")) || lower.ends_with(&format!("-{h}"))
    })
}

// ── YAML scalar value classification helpers ────────────────────────────────

fn is_yaml_null(s: &str) -> bool {
    matches!(s, "null" | "Null" | "NULL" | "~")
}
fn is_yaml_bool(s: &str) -> bool {
    matches!(
        s,
        "true"
            | "True"
            | "TRUE"
            | "false"
            | "False"
            | "FALSE"
            | "yes"
            | "Yes"
            | "YES"
            | "no"
            | "No"
            | "NO"
            | "on"
            | "On"
            | "ON"
            | "off"
            | "Off"
            | "OFF"
    )
}
fn is_yaml_int(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    let s = s.strip_prefix(['+', '-']).unwrap_or(s);
    if s.starts_with("0x") || s.starts_with("0X") {
        return s[2..].chars().all(|c| c.is_ascii_hexdigit());
    }
    if s.starts_with("0o") || s.starts_with("0O") {
        return s[2..].chars().all(|c| matches!(c, '0'..='7'));
    }
    if s.starts_with("0b") || s.starts_with("0B") {
        return s[2..].chars().all(|c| matches!(c, '0' | '1'));
    }
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}
fn is_yaml_float(s: &str) -> bool {
    let s = s.trim();
    matches!(
        s,
        ".inf"
            | ".Inf"
            | ".INF"
            | "-.inf"
            | "-.Inf"
            | "-.INF"
            | "+.inf"
            | "+.Inf"
            | "+.INF"
            | ".nan"
            | ".NaN"
            | ".NAN"
    ) || {
        let s2 = s.strip_prefix(['+', '-']).unwrap_or(s);
        s2.contains('.')
            && s2
                .chars()
                .all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-'))
    }
}

/// Is this a plain scalar that looks like an IRI / URL?
fn is_iri_like(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("_:")
        || s.starts_with("urn:")
        || s.starts_with("doi:")
        || (s.contains(':')
            && !s.contains(' ')
            && s.chars().next().map_or(false, |c| c.is_alphabetic()))
}

// ── Token type for IRI values ───────────────────────────────────────────────
fn iri_token() -> TokenType {
    TokenType::new(&["Name", "Other"])
}

// ── The lexer struct ────────────────────────────────────────────────────────

/// YAML-LD lexer with embedded Markdown/HTML support.
pub struct YamlLdLexer;

impl Lexer for YamlLdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let mut out = Vec::new();
        let mut lines = code.split_inclusive('\n').peekable();

        while let Some(line) = lines.next() {
            let trimmed = line.trim_start();
            let indent_len = line.len() - trimmed.len();

            // ── Document markers ──────────────────────────────────────────
            if trimmed.starts_with("---") && trimmed.trim() == "---" {
                out.push((NAME_NAMESPACE, "---".to_string()));
                if let Some(rest) = trimmed.strip_prefix("---") {
                    if !rest.is_empty() {
                        out.push((WHITESPACE, rest.to_string()));
                    }
                }
                continue;
            }
            if trimmed.starts_with("...") && trimmed.trim() == "..." {
                out.push((NAME_NAMESPACE, "...".to_string()));
                continue;
            }

            // ── Comment lines ─────────────────────────────────────────────
            if trimmed.starts_with('#') {
                if indent_len > 0 {
                    out.push((WHITESPACE, line[..indent_len].to_string()));
                }
                out.push((COMMENT_SINGLE, trimmed.to_string()));
                continue;
            }

            // ── Empty / whitespace-only lines ────────────────────────────
            if trimmed.is_empty() || trimmed == "\n" || trimmed == "\r\n" {
                out.push((WHITESPACE, line.to_string()));
                continue;
            }

            // ── Sequence entry ('-' item) ─────────────────────────────────
            if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| {
                if trimmed == "-" || trimmed.starts_with("-\n") {
                    Some(&trimmed[1..])
                } else {
                    None
                }
            }) {
                if indent_len > 0 {
                    out.push((WHITESPACE, line[..indent_len].to_string()));
                }
                out.push((PUNCTUATION_INDICATOR, "- ".to_string()));
                let value_str = rest.trim_end_matches(['\n', '\r']);
                tokenize_scalar(value_str, &mut out);
                // trailing newline
                let after = &rest[value_str.len()..];
                if !after.is_empty() {
                    out.push((WHITESPACE, after.to_string()));
                }
                continue;
            }

            // ── Key: value mapping lines ──────────────────────────────────
            if let Some((key_part, val_part, block_indicator)) = parse_mapping_line(trimmed) {
                if indent_len > 0 {
                    out.push((WHITESPACE, line[..indent_len].to_string()));
                }

                // Emit the key
                let bare_key = bare_key_str(&key_part);
                tokenize_key(&key_part, &bare_key, &mut out);

                // `:` separator
                out.push((PUNCTUATION, ":".to_string()));

                if let Some(bi) = block_indicator {
                    // Block scalar (`|` or `>`)
                    out.push((WHITESPACE, " ".to_string()));
                    out.push((PUNCTUATION_INDICATOR, bi.to_string()));
                    if let Some(newline) = trimmed
                        .strip_prefix(&format!("{key_part}: {bi}"))
                        .map(str::to_string)
                        .or_else(|| Some("\n".to_string()))
                    {
                        out.push((WHITESPACE, newline));
                    }
                    // Collect block scalar body lines (indented more than key)
                    let scalar_content = collect_block_scalar(&mut lines, indent_len);

                    if !scalar_content.is_empty() {
                        if is_markdown_key(&bare_key) {
                            dispatch_embedded(
                                &scalar_content,
                                "markdown",
                                indent_len + 2,
                                &mut out,
                            );
                        } else if is_html_key(&bare_key) {
                            dispatch_embedded(&scalar_content, "html", indent_len + 2, &mut out);
                        } else {
                            // Emit as block scalar content
                            emit_block_scalar_content(&scalar_content, indent_len + 2, &mut out);
                        }
                    }
                    continue;
                }

                // Inline value
                if val_part.is_empty() {
                    // Key-only line (nested mapping follows)
                    // trailing newline
                    let newline = &line[indent_len + key_part.len() + 1..];
                    if !newline.is_empty() {
                        out.push((WHITESPACE, newline.to_string()));
                    }
                    continue;
                }

                out.push((WHITESPACE, " ".to_string()));
                let val_str = val_part.trim_end_matches(['\n', '\r']);
                tokenize_value(val_str, &mut out);
                let tail = &val_part[val_str.len()..];
                if !tail.is_empty() {
                    out.push((WHITESPACE, tail.to_string()));
                }
                continue;
            }

            // ── Fallback: emit as plain scalar / error ────────────────────
            if indent_len > 0 {
                out.push((WHITESPACE, line[..indent_len].to_string()));
            }
            let rest = trimmed.trim_end_matches(['\n', '\r']);
            if !rest.is_empty() {
                out.push((SCALAR_PLAIN, rest.to_string()));
            }
            let tail = &trimmed[rest.len()..];
            if !tail.is_empty() {
                out.push((WHITESPACE, tail.to_string()));
            }
        }

        merge_adjacent(out)
    }
}

// ── Parsing helpers ─────────────────────────────────────────────────────────

/// Parse a mapping line like `key: value` or `"key": |`.
/// Returns `(key_part, value_part, block_indicator)`.
fn parse_mapping_line(trimmed: &str) -> Option<(String, String, Option<char>)> {
    // Quoted key first
    if trimmed.starts_with('"') {
        let (quoted, rest) = read_dquoted(trimmed)?;
        let rest = rest.trim_start();
        let rest = rest.strip_prefix(':')?;
        let after = rest.trim_start_matches(' ');
        let after = after.trim_end_matches(['\n', '\r']);
        let bi = if after == "|" || after == ">" {
            after.chars().next()
        } else {
            None
        };
        let val = if bi.is_some() { "" } else { after };
        return Some((quoted, val.to_string(), bi));
    }
    if trimmed.starts_with('\'') {
        let (quoted, rest) = read_squoted(trimmed)?;
        let rest = rest.trim_start();
        let rest = rest.strip_prefix(':')?;
        let after = rest.trim_start_matches(' ');
        let after = after.trim_end_matches(['\n', '\r']);
        let bi = if after == "|" || after == ">" {
            after.chars().next()
        } else {
            None
        };
        let val = if bi.is_some() { "" } else { after };
        return Some((quoted, val.to_string(), bi));
    }
    // Plain key: scan until `:` not inside a value
    // A plain key cannot contain `: ` in its first part.
    let colon_pos = find_plain_key_colon(trimmed)?;
    let key = trimmed[..colon_pos].trim_end();
    let after = trimmed[colon_pos + 1..].trim_start_matches(' ');
    let after = after.trim_end_matches(['\n', '\r']);
    // Must not be a URL (which contains `:` but isn't a mapping)
    if key.contains("://") {
        return None;
    }
    let bi = if after == "|" || after == ">" {
        after.chars().next()
    } else {
        None
    };
    let val = if bi.is_some() { "" } else { after };
    Some((key.to_string(), val.to_string(), bi))
}

fn find_plain_key_colon(s: &str) -> Option<usize> {
    // A mapping key's `:` must be followed by space or end-of-line.
    let bytes = s.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b':' {
            if i + 1 >= bytes.len() || bytes[i + 1] == b' ' || bytes[i + 1] == b'\n' {
                // Exclude URLs: if we already passed `://`, skip
                if i >= 3 && &bytes[i - 2..i + 1] == b"://" {
                    continue;
                }
                return Some(i);
            }
        }
    }
    None
}

/// Read a double-quoted string from the start of `s`.  Returns (whole_token_including_quotes, rest).
fn read_dquoted(s: &str) -> Option<(String, &str)> {
    let mut i = 1; // skip opening `"`
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2; // skip escape
        } else if bytes[i] == b'"' {
            return Some((s[..i + 1].to_string(), &s[i + 1..]));
        } else {
            i += 1;
        }
    }
    None
}

/// Read a single-quoted string from the start of `s`. Returns (whole_token_including_quotes, rest).
fn read_squoted(s: &str) -> Option<(String, &str)> {
    let mut i = 1;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            // `''` is an escaped single quote inside single-quoted YAML
            if i + 1 < bytes.len() && bytes[i + 1] == b'\'' {
                i += 2;
            } else {
                return Some((s[..i + 1].to_string(), &s[i + 1..]));
            }
        } else {
            i += 1;
        }
    }
    None
}

/// Return the bare (unquoted) key string for matching against hints.
fn bare_key_str(key: &str) -> String {
    if (key.starts_with('"') && key.ends_with('"'))
        || (key.starts_with('\'') && key.ends_with('\''))
    {
        key[1..key.len() - 1].to_string()
    } else {
        key.to_string()
    }
}

// ── Token emitters ───────────────────────────────────────────────────────────

fn tokenize_key(key_text: &str, bare: &str, out: &mut Vec<(TokenType, String)>) {
    if is_ld_keyword(bare) {
        // Emit the whole key (including quotes if present) as Name.Decorator
        out.push((NAME_DECORATOR, key_text.to_string()));
        return;
    }
    if key_text.starts_with('"') {
        // Quoted key: String + possibly Name.Tag for the inner text
        // For simplicity we follow the Pygments convention of three tokens:
        // `"`, bare key, `"` — all as STRING.
        let inner = &key_text[1..key_text.len().saturating_sub(1)];
        out.push((STRING, "\"".to_string()));
        out.push((STRING, inner.to_string()));
        out.push((STRING, "\"".to_string()));
    } else if key_text.starts_with('\'') {
        let inner = &key_text[1..key_text.len().saturating_sub(1)];
        out.push((STRING_SINGLE, "'".to_string()));
        out.push((STRING_SINGLE, inner.to_string()));
        out.push((STRING_SINGLE, "'".to_string()));
    } else {
        out.push((NAME_TAG, key_text.to_string()));
    }
}

fn tokenize_value(val: &str, out: &mut Vec<(TokenType, String)>) {
    if val.is_empty() {
        return;
    }
    // Comment at end of value
    let (val, comment) = split_inline_comment(val);

    if val.starts_with('"') {
        tokenize_dquoted_value(val, out);
    } else if val.starts_with('\'') {
        tokenize_squoted_value(val, out);
    } else if val.starts_with('[') || val.starts_with('{') {
        tokenize_flow_collection(val, out);
    } else if is_yaml_null(val) || is_yaml_bool(val) {
        out.push((KEYWORD_CONSTANT, val.to_string()));
    } else if is_yaml_int(val) {
        out.push((NUMBER_INTEGER, val.to_string()));
    } else if is_yaml_float(val) {
        out.push((NUMBER_FLOAT, val.to_string()));
    } else if is_ld_keyword(val) {
        out.push((NAME_DECORATOR, val.to_string()));
    } else if is_iri_like(val) {
        out.push((iri_token(), val.to_string()));
    } else {
        out.push((SCALAR_PLAIN, val.to_string()));
    }

    if let Some(c) = comment {
        out.push((WHITESPACE, " ".to_string()));
        out.push((COMMENT_SINGLE, c.to_string()));
    }
}

fn tokenize_scalar(val: &str, out: &mut Vec<(TokenType, String)>) {
    tokenize_value(val, out);
}

fn tokenize_dquoted_value(s: &str, out: &mut Vec<(TokenType, String)>) {
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        let inner = &s[1..s.len() - 1];
        // Check if bare inner is an @keyword
        if is_ld_keyword(inner) {
            out.push((NAME_DECORATOR, s.to_string()));
        } else if is_iri_like(inner) {
            out.push((STRING_DOUBLE, "\"".to_string()));
            out.push((iri_token(), inner.to_string()));
            out.push((STRING_DOUBLE, "\"".to_string()));
        } else {
            out.push((STRING_DOUBLE, s.to_string()));
        }
    } else {
        // Unterminated or malformed
        out.push((STRING_DOUBLE, s.to_string()));
    }
}

fn tokenize_squoted_value(s: &str, out: &mut Vec<(TokenType, String)>) {
    out.push((STRING_SINGLE, s.to_string()));
}

/// Very lightweight flow collection tokenizer (`[...]`, `{...}`).
fn tokenize_flow_collection(s: &str, out: &mut Vec<(TokenType, String)>) {
    // Just emit as a mix of punctuation and values without full parse.
    let mut i = 0;
    let bytes = s.as_bytes();
    let n = bytes.len();
    while i < n {
        match bytes[i] {
            b'[' | b']' | b'{' | b'}' | b',' => {
                out.push((PUNCTUATION, (bytes[i] as char).to_string()));
                i += 1;
            }
            b'"' => {
                // scan double-quoted string
                let mut j = i + 1;
                while j < n {
                    if bytes[j] == b'\\' {
                        j += 2;
                    } else if bytes[j] == b'"' {
                        j += 1;
                        break;
                    } else {
                        j += 1;
                    }
                }
                let tok = &s[i..j];
                tokenize_dquoted_value(tok, out);
                i = j;
            }
            b'\'' => {
                let mut j = i + 1;
                while j < n {
                    if bytes[j] == b'\'' {
                        if j + 1 < n && bytes[j + 1] == b'\'' {
                            j += 2;
                        } else {
                            j += 1;
                            break;
                        }
                    } else {
                        j += 1;
                    }
                }
                out.push((STRING_SINGLE, s[i..j].to_string()));
                i = j;
            }
            b' ' | b'\t' => {
                let mut j = i;
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t') {
                    j += 1;
                }
                out.push((WHITESPACE, s[i..j].to_string()));
                i = j;
            }
            _ => {
                // plain scalar token
                let mut j = i;
                while j < n && !matches!(bytes[j], b',' | b']' | b'}' | b' ' | b'\t') {
                    j += 1;
                }
                let tok = &s[i..j];
                if is_yaml_null(tok) || is_yaml_bool(tok) {
                    out.push((KEYWORD_CONSTANT, tok.to_string()));
                } else if is_yaml_int(tok) {
                    out.push((NUMBER_INTEGER, tok.to_string()));
                } else if is_yaml_float(tok) {
                    out.push((NUMBER_FLOAT, tok.to_string()));
                } else if is_ld_keyword(tok) {
                    out.push((NAME_DECORATOR, tok.to_string()));
                } else if is_iri_like(tok) {
                    out.push((iri_token(), tok.to_string()));
                } else {
                    out.push((SCALAR_PLAIN, tok.to_string()));
                }
                i = j;
            }
        }
    }
}

/// Split a scalar value at a ` # ` comment marker.
/// Returns `(value_part, comment_or_none)`.
fn split_inline_comment(s: &str) -> (&str, Option<&str>) {
    // A YAML inline comment starts with ` # ` (space hash space).
    if let Some(pos) = find_inline_comment(s) {
        (&s[..pos].trim_end(), Some(&s[pos..]))
    } else {
        (s, None)
    }
}

fn find_inline_comment(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b' ' && bytes[i + 1] == b'#' {
            return Some(i + 1);
        }
        i += 1;
    }
    None
}

// ── Block scalar collection ─────────────────────────────────────────────────

/// Consume lines from `lines` that belong to a block scalar body.
/// Returns them as a concatenated string (preserving newlines).
fn collect_block_scalar<'a, I>(lines: &mut std::iter::Peekable<I>, key_indent: usize) -> String
where
    I: Iterator<Item = &'a str>,
{
    let mut buf = String::new();
    // The block scalar content must be indented more than the key.
    while let Some(line) = lines.peek() {
        let trimmed = line.trim_start();
        let line_indent = line.len() - trimmed.len();
        // An empty line or a line indented more than the key belongs to the scalar.
        if trimmed.is_empty() || trimmed == "\n" || trimmed == "\r\n" || line_indent > key_indent {
            buf.push_str(line);
            lines.next();
        } else {
            break;
        }
    }
    buf
}

/// Emit block scalar content as-is (as `Name.Constant` chunks, matching
/// the Pygments YAML lexer's convention).
fn emit_block_scalar_content(
    content: &str,
    scalar_indent: usize,
    out: &mut Vec<(TokenType, String)>,
) {
    for line in content.split_inclusive('\n') {
        if line.trim().is_empty() {
            out.push((WHITESPACE, line.to_string()));
        } else {
            let indent = line.len() - line.trim_start().len();
            if indent > 0 {
                out.push((WHITESPACE, line[..indent].to_string()));
            }
            let text = line.trim_end_matches(['\n', '\r']);
            let text = if text.len() > scalar_indent.min(indent) {
                &text[scalar_indent.min(indent)..]
            } else {
                text.trim_start()
            };
            if !text.is_empty() {
                out.push((NAME_CONSTANT, text.to_string()));
            }
            let tail = &line[indent + text.len()..];
            if !tail.is_empty() {
                out.push((WHITESPACE, tail.to_string()));
            }
        }
    }
}

/// Dispatch a block scalar to an embedded lexer (markdown or html).
/// The `scalar_indent` bytes are stripped from each non-empty line before
/// lexing, then re-emitted as `Text` (to mirror RST's do_insertions style).
fn dispatch_embedded(
    content: &str,
    lang: &str,
    scalar_indent: usize,
    out: &mut Vec<(TokenType, String)>,
) {
    use crate::lexers::registry::get_lexer_by_name;

    let lang_leaked: &'static str = Box::leak(lang.to_string().into_boxed_str());
    if let Some(lexer) = get_lexer_by_name(lang_leaked) {
        // Strip common indent and build insertion list.
        let mut stripped = String::new();
        let mut insertions: Vec<(usize, Vec<(TokenType, String)>)> = Vec::new();
        for line in content.split_inclusive('\n') {
            if line.trim().is_empty() {
                stripped.push_str(line);
            } else {
                let indent = line.len() - line.trim_start().len();
                let strip = indent.min(scalar_indent);
                if strip > 0 {
                    insertions.push((
                        stripped.len(),
                        vec![(token::WHITESPACE, line[..strip].to_string())],
                    ));
                }
                stripped.push_str(&line[strip..]);
            }
        }
        let code_tokens = lexer.get_tokens(&stripped);
        let result = crate::lexers::do_insertions_owned(insertions, code_tokens);
        out.extend(result);
    } else {
        // Lexer not available — fall back to plain block scalar.
        emit_block_scalar_content(content, scalar_indent, out);
    }
}

// ── Merge adjacent same-type tokens (matches Pygments' get_tokens() merging) ─

fn merge_adjacent(tokens: Vec<(TokenType, String)>) -> Vec<(TokenType, String)> {
    let mut out: Vec<(TokenType, String)> = Vec::with_capacity(tokens.len());
    for (t, v) in tokens {
        if v.is_empty() {
            continue;
        }
        if let Some(last) = out.last_mut() {
            if last.0 == t {
                last.1.push_str(&v);
                continue;
            }
        }
        out.push((t, v));
    }
    out
}
