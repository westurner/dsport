//! Markdown text utilities for `sphinxdocrs`.
//!
//! Markdown escaping is intentionally separate from [`crate::util_rst`]: the
//! two formats have different grammars and should not share a public generic
//! `escape` function. The implementation is centralized in
//! [`crate::util_strypes`] with the other context-specific transformations.

/// Sanitize untrusted text before inserting it into Markdown or MyST source.
///
/// This escapes ASCII punctuation and backslashes, removes control
/// characters, strips leading indentation, and normalizes Python-style line
/// boundaries. It prevents the input from becoming Markdown headings, lists,
/// block quotes, fenced code, links, emphasis, raw HTML, front matter, or
/// MyST roles and directive fences.
pub use crate::util_strypes::markdown_escape as escape;

/// Sanitize encoded Markdown after strict decoding with an explicit encoding.
pub use crate::util_strypes::markdown_escape_bytes as escape_bytes;

/// Sanitize encoded Markdown using Docutils-style encoding detection.
pub use crate::util_strypes::markdown_escape_bytes_auto as escape_bytes_auto;
