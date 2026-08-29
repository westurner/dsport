//! Context-specific string transformations used by Sphinx builders.
//!
//! These helpers intentionally have explicit names. HTML, XML, JSON, RST,
//! PO, and terminal strings have different grammars, so a generic `escape`
//! function would make context confusion easy and recreate CWE-116-style
//! output neutralization bugs.

/// Escape text for an HTML document text node.
///
/// Quotes are escaped as well as the characters required in a text node. This
/// preserves the existing renderer output and is harmless in text context.
pub fn html_escape_text(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            _ => output.push(character),
        }
    }
    output
}

/// Escape a value for a double-quoted HTML attribute.
pub fn html_escape_attr(value: &str) -> String {
    html_escape_text(value)
}

/// Return whether an attribute name is acceptable for generated HTML.
///
/// Attribute values are escaped separately. Names need policy as well: a
/// syntactically valid `onerror` or `style` attribute can still introduce
/// executable behavior even when its value is quote-escaped.
pub fn is_safe_html_attribute_name(name: &str) -> bool {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || matches!(first, ':' | '_')) {
        return false;
    }
    if !characters.all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, ':' | '.' | '_' | '-')
    }) {
        return false;
    }
    let lower = name.to_ascii_lowercase();
    !lower.starts_with("on") && lower != "style" && lower != "srcdoc"
}

/// Escape text or attribute content for XML 1.0.
///
/// Characters forbidden by XML 1.0 are omitted rather than producing a
/// malformed document. The caller still owns URI/path validation.
pub fn xml_escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value
        .chars()
        .filter(|&character| is_xml10_character(character))
    {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(character),
        }
    }
    output
}

fn is_xml10_character(character: char) -> bool {
    // XML 1.0 permits TAB, LF, CR, and the listed Unicode scalar ranges.
    matches!(
        character as u32,
        0x9 | 0xA | 0xD | 0x20..=0xD7FF | 0xE000..=0xFFFD | 0x10000..=0x10FFFF
    )
}

/// Encode a Rust string as a complete JSON string literal.
pub fn json_escape(value: &str) -> String {
    serde_json::to_string(value).expect("serializing a Rust string cannot fail")
}

/// Encode a string as a JavaScript string literal safe for an HTML
/// `<script>` element.
pub fn html_javascript_escape(value: &str) -> String {
    json_escape(value)
        .chars()
        .map(|character| match character {
            '<' => "\\u003c".to_owned(),
            '>' => "\\u003e".to_owned(),
            '&' => "\\u0026".to_owned(),
            '\u{2028}' => "\\u2028".to_owned(),
            '\u{2029}' => "\\u2029".to_owned(),
            _ => character.to_string(),
        })
        .collect()
}

/// Sanitize untrusted text before inserting it into reStructuredText or
/// Markdown.
///
/// Every ASCII punctuation character and backslash is escaped, control
/// characters are removed, and leading whitespace is stripped from each line.
/// This prevents inline markup and block constructs including directives,
/// comments, lists, block quotes, definition lists, headers, footnotes,
/// citations, hyperlink targets, and substitutions.
pub fn rst_escape(text: &str) -> String {
    escape_plaintext_markup(text)
}

/// Sanitize untrusted text before inserting it into Markdown.
///
/// Markdown and reStructuredText have different parsers, but escaping every
/// ASCII punctuation character and removing indentation neutralizes the
/// syntax shared by CommonMark, GFM, and the MyST extensions supported by this
/// crate. This covers headings, lists, block quotes, fenced code, links,
/// emphasis, raw HTML, front matter, and MyST roles/directive fences.
pub fn markdown_escape(text: &str) -> String {
    escape_plaintext_markup(text)
}

/// Escape untrusted Markdown source after strict decoding with an explicit
/// source encoding.
pub fn markdown_escape_bytes(
    bytes: &[u8],
    encoding: &str,
) -> Result<String, docutilsrs::encoding::SourceDecodeError> {
    let text = docutilsrs::encoding::decode_source(bytes, encoding)?;
    Ok(markdown_escape(&text))
}

/// Escape untrusted Markdown source using Docutils-style encoding detection.
pub fn markdown_escape_bytes_auto(
    bytes: &[u8],
) -> Result<String, docutilsrs::encoding::SourceDecodeError> {
    let text = docutilsrs::encoding::decode_source_auto(bytes)?;
    Ok(markdown_escape(&text))
}

fn escape_plaintext_markup(text: &str) -> String {
    python_splitlines(text)
        .into_iter()
        .map(sanitize_markup_line)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Sanitize one logical markup line after line-ending normalization.
fn sanitize_markup_line(line: String) -> String {
    let mut output = String::with_capacity(line.len());
    for character in line.trim_start().chars() {
        if character.is_control() {
            continue;
        }
        if character == '\\' || character.is_ascii_punctuation() {
            output.push('\\');
        }
        output.push(character);
    }
    output
}

/// Sanitize encoded RST source after decoding it with an explicit encoding.
///
/// Decoding is strict: malformed byte sequences and unknown encodings return
/// [`docutilsrs::encoding::SourceDecodeError`] instead of being replaced.
pub fn rst_escape_bytes(
    bytes: &[u8],
    encoding: &str,
) -> Result<String, docutilsrs::encoding::SourceDecodeError> {
    let text = docutilsrs::encoding::decode_source(bytes, encoding)?;
    Ok(rst_escape(&text))
}

/// Sanitize encoded RST source using Docutils-style encoding detection.
pub fn rst_escape_bytes_auto(
    bytes: &[u8],
) -> Result<String, docutilsrs::encoding::SourceDecodeError> {
    let text = docutilsrs::encoding::decode_source_auto(bytes)?;
    Ok(rst_escape(&text))
}

fn python_splitlines(text: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line_start = 0;
    let mut characters = text.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        let is_line_break = matches!(
            character,
            '\n' | '\r' | '\u{000B}' | '\u{000C}' | '\u{001C}'
                ..='\u{001E}' | '\u{0085}' | '\u{2028}' | '\u{2029}'
        );
        if !is_line_break {
            continue;
        }
        let mut line_end = index + character.len_utf8();
        if character == '\r' && text[line_end..].starts_with('\n') {
            characters.next();
            line_end += 1;
        }
        lines.push(text[line_start..index].to_owned());
        line_start = line_end;
    }
    if line_start < text.len() {
        lines.push(text[line_start..].to_owned());
    }
    lines
}

/// Decode C-style escape sequences used in PO catalog string tokens.
pub fn unescape_po_string(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut characters = value.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match characters.next() {
            Some('a') => output.push('\x07'),
            Some('b') => output.push('\x08'),
            Some('f') => output.push('\x0c'),
            Some('n') => output.push('\n'),
            Some('r') => output.push('\r'),
            Some('t') => output.push('\t'),
            Some('v') => output.push('\x0b'),
            Some('"') => output.push('"'),
            Some('\\') => output.push('\\'),
            Some('x') => {
                let mut digits = String::new();
                while digits.len() < 2 && characters.peek().is_some_and(|c| c.is_ascii_hexdigit()) {
                    digits.push(characters.next().expect("peeked character exists"));
                }
                if digits.is_empty() {
                    output.push('\\');
                    output.push('x');
                } else if let Ok(codepoint) = u8::from_str_radix(&digits, 16) {
                    output.push(codepoint as char);
                }
            }
            Some(character @ '0'..='7') => {
                let mut digits = String::from(character);
                while digits.len() < 3 && characters.peek().is_some_and(|c| matches!(c, '0'..='7'))
                {
                    digits.push(characters.next().expect("peeked character exists"));
                }
                if let Ok(codepoint) = u8::from_str_radix(&digits, 8) {
                    output.push(codepoint as char);
                }
            }
            Some(character) => {
                output.push('\\');
                output.push(character);
            }
            None => output.push('\\'),
        }
    }
    output
}

/// Decode a PO string and remove controls that could affect terminal or log
/// output. Newlines and tabs remain available for multiline translations.
pub fn unescape_po_string_safe(value: &str) -> String {
    let decoded = unescape_po_string(value);
    strip_escape_sequences(&decoded)
}

/// Remove ANSI/VT terminal controls from text written to logs or terminals.
///
/// This handles CSI, OSC, DCS, SOS, PM, APC, two-byte ESC sequences, C1
/// controls, and unsafe single-byte controls. Line feeds and tabs are kept so
/// log structure remains readable.
pub fn strip_escape_sequences(text: &str) -> String {
    let characters: Vec<char> = text.chars().collect();
    let mut output = String::with_capacity(text.len());
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if character == '\x1b' {
            index = skip_esc_sequence(&characters, index + 1);
        // C1 CSI (U+009B) is the single-byte form of ESC [.
        } else if character as u32 == 0x9B {
            index = skip_csi(&characters, index + 1);
        // C1 string controls: DCS, SOS, OSC, PM, and APC.
        } else if matches!(character as u32, 0x90 | 0x98 | 0x9D | 0x9E | 0x9F) {
            index = skip_string_control(&characters, index + 1);
        } else if character == '\n' || character == '\t' {
            output.push(character);
            index += 1;
        // Drop C0 controls and the remaining C1 controls, except LF and TAB.
        } else if (character as u32) < 0x20 || (0x7F..=0x9F).contains(&(character as u32)) {
            index += 1;
        } else {
            output.push(character);
            index += 1;
        }
    }
    output
}

fn skip_esc_sequence(characters: &[char], mut index: usize) -> usize {
    let Some(&kind) = characters.get(index) else {
        return index;
    };
    index += 1;
    match kind {
        '[' => skip_csi(characters, index),
        ']' => skip_string_control(characters, index),
        'P' | 'X' | '^' | '_' => skip_string_control(characters, index),
        _ => index,
    }
}

fn skip_csi(characters: &[char], mut index: usize) -> usize {
    while let Some(&character) = characters.get(index) {
        index += 1;
        // CSI final bytes are the inclusive ASCII range 0x40 ('@') to 0x7E ('~').
        if (0x40..=0x7E).contains(&(character as u32)) {
            break;
        }
    }
    index
}

fn skip_string_control(characters: &[char], mut index: usize) -> usize {
    while let Some(&character) = characters.get(index) {
        // BEL and ST (C1 string terminator U+009C) terminate OSC/DCS-like strings.
        if character == '\x07' || character as u32 == 0x9C {
            return index + 1;
        }
        if character == '\x1B' && characters.get(index + 1) == Some(&'\\') {
            return index + 2;
        }
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_contexts_escape_markup_and_quotes() {
        assert_eq!(
            html_escape_text("<script>&\"'"),
            "&lt;script&gt;&amp;&quot;&#x27;"
        );
        assert_eq!(html_escape_attr("\" onclick=\"x"), "&quot; onclick=&quot;x");
    }

    #[test]
    fn attribute_names_reject_event_and_malformed_names() {
        assert!(is_safe_html_attribute_name("data-value"));
        assert!(!is_safe_html_attribute_name("onerror"));
        assert!(!is_safe_html_attribute_name("bad name"));
        assert!(!is_safe_html_attribute_name("style"));
    }

    #[test]
    fn xml_context_escapes_and_drops_invalid_controls() {
        assert_eq!(xml_escape("<&\"'\u{0001}"), "&lt;&amp;&quot;&apos;");
    }

    #[test]
    fn json_escape_returns_a_complete_literal() {
        assert_eq!(json_escape("a\"b\n"), "\"a\\\"b\\n\"");
    }

    #[test]
    fn html_javascript_escape_protects_html_script_context() {
        let value = "</script>\"\\&\u{2028}\u{2029}";
        let escaped = html_javascript_escape(value);
        assert_eq!(
            escaped,
            "\"\\u003c/script\\u003e\\\"\\\\\\u0026\\u2028\\u2029\""
        );
        assert_eq!(serde_json::from_str::<String>(&escaped).unwrap(), value);
    }

    #[test]
    fn rst_escape_escapes_all_rst_punctuation() {
        assert_eq!(rst_escape(":ref:`id`"), r"\:ref\:\`id\`");
        assert_eq!(rst_escape("sphinx.application"), r"sphinx\.application");
    }

    #[test]
    fn rst_escape_neutralizes_bol_markers_and_punctuation() {
        assert_eq!(rst_escape("* item *"), r"\* item \*");
        assert_eq!(rst_escape(".. note .. "), r"\.\. note \.\. ");
        assert_eq!(rst_escape(">>> prompt >>> "), r"\>\>\> prompt \>\>\> ");
        assert_eq!(rst_escape("plain [x] | `code`"), r"plain \[x\] \| \`code\`");
    }

    #[test]
    fn rst_escape_neutralizes_indirect_targets_and_indentation_blocks() {
        assert_eq!(rst_escape("__ target_"), r"\_\_ target\_");
        assert_eq!(
            rst_escape("term\n  definition\n    quoted"),
            "term\ndefinition\nquoted"
        );
    }

    #[test]
    fn rst_escape_removes_controls_and_neutralizes_existing_escapes() {
        assert_eq!(rst_escape("a\u{0000}\u{001B}b"), "ab");
        assert_eq!(rst_escape(r"\*"), r"\\\*");
    }

    #[test]
    fn rst_escape_neutralizes_headers_footnotes_and_citations() {
        let source = "Title\n=====\n\n[1]_ note\n.. [1] citation";
        let sanitized = rst_escape(source);
        assert_eq!(
            sanitized,
            "Title\n\\=\\=\\=\\=\\=\n\n\\[1\\]\\_ note\n\\.\\. \\[1\\] citation"
        );
        assert!(!sanitized.contains("\n====="));
        assert!(!sanitized.contains("\n.. "));
    }

    #[test]
    fn rst_escape_matches_python_splitlines_boundaries() {
        assert_eq!(
            rst_escape("one\r\ntwo\rthree\u{000B}f\u{000C}f\u{0085}g\u{2028}h"),
            "one\ntwo\nthree\nf\nf\ng\nh"
        );
        assert_eq!(rst_escape(""), "");
        assert_eq!(rst_escape("one\n"), "one");
    }

    #[test]
    fn rst_escape_bytes_decodes_strictly_before_sanitizing() {
        assert_eq!(
            rst_escape_bytes(b"caf\xe9", "latin-1").unwrap(),
            "caf\u{e9}"
        );
        assert!(rst_escape_bytes(b"caf\xff", "utf-8").is_err());
        assert_eq!(
            rst_escape_bytes_auto(b"\xef\xbb\xbf* item").unwrap(),
            r"\* item"
        );
    }

    #[test]
    fn markdown_escape_neutralizes_markdown_and_myst_constructs() {
        assert_eq!(
            markdown_escape("# heading\n- item\n> quote\n```rust\n{x}\n```"),
            r"\# heading
\- item
\> quote
\`\`\`rust
\{x\}
\`\`\`"
        );
        assert_eq!(
            markdown_escape("[label](https://example.test)"),
            r"\[label\]\(https\:\/\/example\.test\)"
        );
        assert_eq!(
            markdown_escape("{ref}`target`\n:::note"),
            "\\{ref\\}\\`target\\`\n\\:\\:\\:note"
        );
    }

    #[test]
    fn markdown_escape_neutralizes_html_front_matter_and_indentation() {
        assert_eq!(
            markdown_escape("<script>alert(1)</script>\n---\ntitle: value\n    indented"),
            r"\<script\>alert\(1\)\<\/script\>
\-\-\-
title\: value
indented"
        );
    }

    #[test]
    fn markdown_escape_removes_controls_and_normalizes_line_boundaries() {
        assert_eq!(
            markdown_escape("a\u{0000}\u{001B}b\r\nc\u{2028}d"),
            "ab\nc\nd"
        );
    }

    #[test]
    fn markdown_escape_bytes_decodes_strictly_before_sanitizing() {
        assert_eq!(
            markdown_escape_bytes(b"caf\xe9", "latin-1").unwrap(),
            "caf\u{e9}"
        );
        assert!(markdown_escape_bytes(b"caf\xff", "utf-8").is_err());
        assert_eq!(
            markdown_escape_bytes_auto(b"\xef\xbb\xbf# title").unwrap(),
            r"\# title"
        );
    }

    #[test]
    fn po_escape_decodes_standard_and_numeric_forms() {
        assert_eq!(unescape_po_string(r"\n\x41\101\a"), "\nAA\x07");
    }

    #[test]
    fn safe_po_escape_removes_terminal_and_control_sequences() {
        assert_eq!(
            unescape_po_string_safe(r"line\n\t\r\a\x1b[31mred\x1b[0m\x1b]title\x07"),
            "line\n\tred"
        );
    }

    #[test]
    fn markdown_escape_stays_plain_through_myst_parser() {
        let cases = [
            ("$x$", "class=\"math\""),
            ("{ref}`target`", "myst-role"),
            ("<script>alert(1)</script>", "<script"),
            ("<https://example.test>", "<a href="),
            (":::note\ncontent\n:::", "myst-directive"),
        ];
        for (source, forbidden) in cases {
            let rendered = myst_md_rs::render_html(&markdown_escape(source));
            assert!(
                !rendered.contains(forbidden),
                "escaped Markdown was reinterpreted: source={source:?}, rendered={rendered:?}"
            );
        }
    }

    #[test]
    fn terminal_sanitizer_removes_multiple_control_families() {
        let input = "ok\x1b[31m red\x1b[0m\x1b]0;title\x07\x1b[2J\r\x07done";
        assert_eq!(strip_escape_sequences(input), "ok reddone");
    }
}
