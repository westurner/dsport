//! `sphinxdocrs::build` — Rust port of `sphinx.cmd.build`.
//!
//! Phase split:
//! - **Native now**: argument parser, all `_parse_*` helpers,
//!   logging setup, make-mode dispatch (`make_mode` sub-module).
//! - **Delegated**: `build_main` body (constructing `Sphinx(...)` and
//!   running a builder) — kept as Python shim until builders land.

pub mod args;
pub mod logging;
pub mod make_mode;
pub mod parser;

pub use args::{BuildArgs, parse_args};
pub use make_mode::{BUILDERS, MakeMode};
