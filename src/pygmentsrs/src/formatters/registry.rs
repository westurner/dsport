//! Formatter name → formatted-output resolution.
//!
//! Mirrors `pygments.formatters.get_formatter_by_name` for the native
//! Rust path. The dispatcher in [`crate::highlight_with_backend`] uses
//! [`format_native`] for known names and falls through to the PyO3
//! bridge for everything else (under `Backend::Auto`).

use crate::formatters::html::HtmlFormatter;
use crate::formatters::terminal::{
    TerminalFormatter, Terminal256Formatter, TerminalTrueColorFormatter, IRCFormatter, BBCodeFormatter,
};
use crate::formatters::markup::{GroffFormatter, PangoMarkupFormatter, LatexFormatter, RtfFormatter};
use crate::formatters::svg::SvgFormatter;
use crate::formatters::trivial::{NullFormatter, RawTokenFormatter, TestcaseFormatter};
use crate::token::TokenType;

/// Format `tokens` with the named native formatter. `None` if no
/// native formatter is registered for `name`.
pub fn format_native(name: &str, tokens: &[(TokenType, String)]) -> Option<String> {
    match name {
        // HTML
        "html" => Some(HtmlFormatter.format(tokens)),
        
        // F0 — Trivial
        "text" => Some(NullFormatter.format(tokens)),
        "raw" | "tokens" => Some(RawTokenFormatter.format(tokens)),
        "testcase" => Some(TestcaseFormatter.format(tokens)),
        
        // F1 — Terminal ANSI
        "terminal" | "console" => Some(TerminalFormatter.format(tokens)),
        "terminal256" | "256" => Some(Terminal256Formatter.format(tokens)),
        "terminal16m" | "truecolor" => Some(TerminalTrueColorFormatter.format(tokens)),
        "irc" => Some(IRCFormatter.format(tokens)),
        "bbcode" => Some(BBCodeFormatter.format(tokens)),
        
        // F2 — Markup
        "groff" | "groff-256" => {
            let mut formatter = GroffFormatter::new();
            Some(formatter.format(tokens))
        }
        "pango" => Some(PangoMarkupFormatter.format(tokens)),
        "latex" | "tex" => Some(LatexFormatter.format(tokens)),
        "rtf" => {
            let mut formatter = RtfFormatter::new();
            Some(formatter.format(tokens))
        }
        
        // F3 — Vector
        "svg" => Some(SvgFormatter::new().format(tokens)),
        
        _ => None,
    }
}

/// Names of formatters with a native Rust implementation.
pub fn native_names() -> &'static [&'static str] {
    &[
        // HTML
        "html",
        
        // F0 — Trivial
        "text",
        "raw",
        "tokens",
        "testcase",
        
        // F1 — Terminal ANSI
        "terminal",
        "console",
        "terminal256",
        "256",
        "terminal16m",
        "truecolor",
        "irc",
        "bbcode",
        
        // F2 — Markup
        "groff",
        "groff-256",
        "pango",
        "latex",
        "tex",
        "rtf",
        
        // F3 — Vector
        "svg",
    ]
}

pub fn has_native(name: &str) -> bool {
    native_names().contains(&name)
}
