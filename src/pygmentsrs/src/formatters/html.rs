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

#[derive(Default)]
pub struct HtmlFormatter;

impl HtmlFormatter {
    pub fn format(&self, tokens: &[(TokenType, String)]) -> String {
        let mut out = String::from("<div class=\"highlight\"><pre><span></span>");
        for (ttype, value) in tokens {
            let short = ttype.short_name();
            if short.is_empty() || short == "t" {
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
