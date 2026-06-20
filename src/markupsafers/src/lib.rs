//! `markupsafers` — safe HTML/XML markup strings.
//!
//! A Rust port of the Python [`MarkupSafe`] library with an identical public
//! API surface, PyO3 bindings that make it a drop-in CPython replacement, and
//! optional [`minijinja`] integration so that [`Markup`] values bypass the
//! auto-escaper.
//!
//! [`MarkupSafe`]: https://markupsafe.palletsprojects.com/
//!
//! # Core concept
//!
//! HTML templates must escape user-controlled content before inserting it into
//! a page, but they must *not* double-escape content that is already safe HTML.
//! `MarkupSafe` solves this by wrapping safe strings in the newtype [`Markup`]:
//!
//! - Inserting a plain `&str`/`String` into a Jinja2 template auto-escapes it.
//! - Inserting a `Markup` value bypasses the auto-escaper.
//! - All string operations on `Markup` (concatenation, format, join, …)
//!   automatically escape their plain-string arguments before incorporating
//!   them, so the result remains safe.
//!
//! # Quick start
//!
//! ```rust
//! use markupsafers::{Markup, escape, escape_value};
//!
//! // Escape untrusted input.
//! let user_input = "<script>alert('xss')</script>";
//! let safe = escape(user_input);
//! assert_eq!(safe.as_str(), "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
//!
//! // Markup values are already safe — escape() is a no-op.
//! let html = Markup::from_safe("<b>bold</b>");
//! assert_eq!(escape_value(&html).as_str(), "<b>bold</b>");
//!
//! // Concatenation escapes the plain string, keeps the Markup as-is.
//! let result = html + Markup::escape(" & <entities>");
//! assert_eq!(result.as_str(), "<b>bold</b> &amp; &lt;entities&gt;");
//! ```

pub mod escape;
pub mod markup;

#[cfg(feature = "minijinja")]
pub mod minijinja_compat;

#[cfg(feature = "extension-module")]
mod bridge;

pub use escape::{MarkupEscapeWriter, escape, escape_silent, escape_value, soft_str};
pub use markup::Markup;
