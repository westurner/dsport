//! `sphinxdocrs::apidoc` — Rust port of `sphinx.ext.apidoc`.
//!
//! Mirrors the C3 target from the CLI port plan.

pub mod generate;
pub mod parser;
pub mod settings;
pub mod templates;

pub use generate::{ApidocError, create_modules_toc_file, recurse_tree};
pub use parser::{build_parser, parse_args};
pub use settings::ApidocOptions;
pub use templates::ApidocTemplates;
