//! Inline MyST role and dollar-math detection inside text events.
//!
//! Roles look like `` {rolename}`content` ``. We split a text run into
//! interleaved plain text, role spans, and inline math spans so the renderer
//! can emit MyST-specific markup without touching the cmark inline parser.

/// One piece of a text run after MyST splitting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Piece<'a> {
    Text(&'a str),
    /// `` {name}`content` `` — content is raw, unescaped.
    Role {
        name: &'a str,
        content: &'a str,
    },
    /// `$content$` inline math.
    InlineMath(&'a str),
}

/// Split `text` into role / math / plain pieces. Always returns at least one
/// `Piece::Text` if `text` is non-empty and contains no MyST markers.
pub fn split_text(text: &str) -> Vec<Piece<'_>> {
    let mut out: Vec<Piece<'_>> = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut last = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'{' {
            if let Some((name, content, end)) = scan_role(text, i) {
                if last < i {
                    out.push(Piece::Text(&text[last..i]));
                }
                out.push(Piece::Role { name, content });
                i = end;
                last = end;
                continue;
            }
        } else if b == b'$' {
            // Avoid double-dollar (block math is handled in preprocess; an
            // inline `$$` is left as literal).
            if i + 1 < bytes.len() && bytes[i + 1] == b'$' {
                i += 2;
                continue;
            }
            if let Some((content, end)) = scan_inline_math(text, i) {
                if last < i {
                    out.push(Piece::Text(&text[last..i]));
                }
                out.push(Piece::InlineMath(content));
                i = end;
                last = end;
                continue;
            }
        }
        i += 1;
    }

    if last < text.len() {
        out.push(Piece::Text(&text[last..]));
    }
    if out.is_empty() {
        out.push(Piece::Text(text));
    }
    out
}

/// Try to parse `` {name}`content` `` starting at the `{` at `start`.
/// Returns (name, content, end_exclusive).
fn scan_role(text: &str, start: usize) -> Option<(&str, &str, usize)> {
    let bytes = text.as_bytes();
    debug_assert_eq!(bytes[start], b'{');
    let name_start = start + 1;
    let mut p = name_start;
    while p < bytes.len() {
        let c = bytes[p];
        // Role names: ASCII alnum, '-', '_', ':', '.'
        if c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b':' | b'.') {
            p += 1;
        } else {
            break;
        }
    }
    if p == name_start || p >= bytes.len() || bytes[p] != b'}' {
        return None;
    }
    let name = &text[name_start..p];
    let after_brace = p + 1;
    if after_brace >= bytes.len() || bytes[after_brace] != b'`' {
        return None;
    }
    // Count backtick run length on the opener.
    let mut ticks = 0;
    let mut q = after_brace;
    while q < bytes.len() && bytes[q] == b'`' {
        ticks += 1;
        q += 1;
    }
    let content_start = q;
    // Find a closing run of the same length.
    while q < bytes.len() {
        if bytes[q] == b'`' {
            let mut r = q;
            let mut run = 0;
            while r < bytes.len() && bytes[r] == b'`' {
                run += 1;
                r += 1;
            }
            if run == ticks {
                let content = &text[content_start..q];
                return Some((name, content, r));
            }
            q = r;
        } else {
            q += 1;
        }
    }
    None
}

/// Try to parse `$content$` starting at the `$` at `start`. Content must be
/// non-empty, not start or end with whitespace, and not contain a newline.
fn scan_inline_math(text: &str, start: usize) -> Option<(&str, usize)> {
    let bytes = text.as_bytes();
    debug_assert_eq!(bytes[start], b'$');
    let content_start = start + 1;
    if content_start >= bytes.len() {
        return None;
    }
    if matches!(bytes[content_start], b' ' | b'\t' | b'\n') {
        return None;
    }
    let mut p = content_start;
    while p < bytes.len() {
        let c = bytes[p];
        if c == b'\n' {
            return None;
        }
        if c == b'$' {
            if p > content_start && !matches!(bytes[p - 1], b' ' | b'\t' | b'\\') {
                let content = &text[content_start..p];
                return Some((content, p + 1));
            }
            return None;
        }
        p += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text() {
        assert_eq!(split_text("hello"), vec![Piece::Text("hello")]);
    }

    #[test]
    fn simple_role() {
        let pieces = split_text("see {ref}`label` now");
        assert_eq!(
            pieces,
            vec![
                Piece::Text("see "),
                Piece::Role {
                    name: "ref",
                    content: "label",
                },
                Piece::Text(" now"),
            ]
        );
    }

    #[test]
    fn inline_math() {
        let pieces = split_text("x $a+b$ y");
        assert_eq!(
            pieces,
            vec![
                Piece::Text("x "),
                Piece::InlineMath("a+b"),
                Piece::Text(" y"),
            ]
        );
    }

    #[test]
    fn role_with_multi_backticks() {
        let pieces = split_text("{code}``a`b``");
        assert_eq!(
            pieces,
            vec![Piece::Role {
                name: "code",
                content: "a`b",
            }]
        );
    }
}
