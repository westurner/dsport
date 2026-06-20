//! `sphinxdocrs::quickstart` — Rust port of `sphinx.cmd.quickstart`.

pub mod generate;
pub mod parser;
pub mod settings;
pub mod templates;
pub mod validate;

pub use generate::{GenerateError, ask_user, generate, valid_dir};
pub use parser::{build_parser, is_fully_specified, parse_args};
pub use settings::QuickstartSettings;
pub use templates::QuickstartTemplates;
