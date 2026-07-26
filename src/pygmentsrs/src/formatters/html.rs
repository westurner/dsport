//! `pygments.formatters.html.HtmlFormatter` — Phase 0 minimal port.
//!
//! Phase 0 emits the default-options shape only:
//!
//! ```html
//! <div class="highlight"><pre><span></span>...token spans...
//! </pre></div>
//! ```
//!
//! Each token becomes `<span class="SHORT">value</span>` where
//! `SHORT` is the short-name from `STANDARD_TYPES`. Unstyled tokens
//! (root `Token`, `Token.Text`) are emitted as raw escaped text.
//!
//! The byte-parity gate against pygments' own `HtmlFormatter` lands
//! with the Python lexer in Phase 1; the short-name table will be
//! expanded then to match `pygments.token.STANDARD_TYPES`.

use crate::token::TokenType;

pub struct HtmlFormatter;

impl HtmlFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::from("<div class=\"highlight\"><pre><span></span>");
        for (ttype, value) in tokens {
            let short = ttype.short_name();
            if short.is_empty() {
                out.push_str(&escape(value));
            } else {
                out.push_str("<span class=\"");
                out.push_str(&short);
                out.push_str("\">");
                out.push_str(&escape(value));
                out.push_str("</span>");
            }
        }
        out.push_str("</pre></div>\n");
        out
    }
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// Generate CSS style definitions for the default style, mirroring
/// `pygments.formatters.html.HtmlFormatter.get_style_defs`.
///
/// `selector` is the CSS scope prefix (e.g. `".highlight"`), so each rule is
/// emitted as `<selector> .<short> { … }`.  Only token types that carry a
/// non-empty style (via [`Style::from_token`]) produce a rule — matching how
/// Pygments omits rules for unstyled tokens.
///
/// The colour palette is the built-in default style shared with the terminal
/// and markup formatters, so highlighted HTML and `pygments.css` stay in sync.
pub fn css_style_defs(selector: &str) -> String {
    use crate::formatters::color::rgb_to_hex;
    use crate::formatters::style::Style;
    use crate::token::*;

    // Token types that the default style assigns a colour/emphasis to, in a
    // stable order.  Each entry pairs the token with the same short class name
    // the HTML formatter emits.
    const STYLED_TOKENS: &[TokenType] = &[
        COMMENT,
        COMMENT_SINGLE,
        COMMENT_MULTILINE,
        COMMENT_PREPROC,
        KEYWORD,
        KEYWORD_NAMESPACE,
        NAME,
        NAME_FUNCTION,
        STRING,
        STRING_DOUBLE,
        STRING_SINGLE,
        NUMBER,
        NUMBER_INTEGER,
        OPERATOR,
    ];

    let mut out = String::new();
    for &token in STYLED_TOKENS {
        let short = token.short_name();
        if short.is_empty() {
            continue;
        }
        let style = Style::from_token(token);
        let mut decls: Vec<String> = Vec::new();
        if let Some((r, g, b)) = style.fg_color {
            decls.push(format!("color: {}", rgb_to_hex(r, g, b)));
        }
        if let Some((r, g, b)) = style.bg_color {
            decls.push(format!("background-color: {}", rgb_to_hex(r, g, b)));
        }
        if style.bold {
            decls.push("font-weight: bold".to_string());
        }
        if style.italic {
            decls.push("font-style: italic".to_string());
        }
        if style.underline {
            decls.push("text-decoration: underline".to_string());
        }
        if decls.is_empty() {
            continue;
        }
        out.push_str(&format!("{selector} .{short} {{ {} }}\n", decls.join("; ")));
    }
    out
}


#[cfg(test)]
mod css_tests {
    use super::css_style_defs;

    #[test]
    fn emits_rules_for_styled_tokens() {
        let css = css_style_defs(".highlight");
        // Keyword is blue + bold.
        assert!(css.contains(".highlight .k { color: #0000ff; font-weight: bold }"), "got:\n{css}");
        // Comment is gray.
        assert!(css.contains(".highlight .c { color: #969696 }"), "got:\n{css}");
        // String is red.
        assert!(css.contains(".highlight .s { color: #c80000 }"), "got:\n{css}");
        // Every line is scoped to the selector.
        for line in css.lines().filter(|l| !l.is_empty()) {
            assert!(line.starts_with(".highlight ."), "unscoped rule: {line}");
        }
    }
}
