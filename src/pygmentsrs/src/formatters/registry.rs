//! Formatter name → formatted-output resolution.
//!
//! Mirrors `pygments.formatters.get_formatter_by_name` for the native
//! Rust path. The dispatcher in [`crate::highlight_with_backend`] uses
//! [`format_native`] for known names and falls through to the PyO3
//! bridge for everything else (under `Backend::Auto`).

use crate::formatters::html::HtmlFormatter;
use crate::token::TokenType;

/// Format `tokens` with the named native formatter. `None` if no
/// native formatter is registered for `name`.
pub fn format_native(name: &str, tokens: &[(TokenType, String)]) -> Option<String> {
    match name {
        "html" => Some(HtmlFormatter.format(tokens)),
        _ => None,
    }
}

/// Names of formatters with a native Rust implementation.
pub fn native_names() -> &'static [&'static str] {
    &["html"]
}

pub fn has_native(name: &str) -> bool {
    native_names().contains(&name)
}
