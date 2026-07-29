//! `sphinxdocrs::autogen` — Rust port of `sphinx.ext.autosummary.generate`.
//!
//! C4 target from the CLI port plan.
//!
//! Architecture:
//! - **Native**: RST file scanning (`find_autosummary_in_lines`), argument
//!   parser, template rendering, output file writing, `--remove-old`.
//! - **PyO3 bridge** (**H9a/H9d**): object import/introspection now goes
//!   through `crate::autodoc_runtime` (`Python::attach` + `inspect`) via
//!   [`generate::StubContext::from_entry_runtime`],
//!   [`generate_stub_runtime`], and [`generate_stubs_runtime`]. The plain
//!   `generate_stub`/`generate_stubs` entry points keep the original
//!   heuristic-only (empty member list) behaviour for callers that don't
//!   need or want to import the target.

pub mod generate;
pub mod parser;
pub mod scan;
pub mod templates;

pub use generate::{
    ObjType, StubContext, generate_stub, generate_stub_runtime, generate_stubs,
    generate_stubs_runtime, infer_obj_type, split_fqn,
};
pub use parser::{AutogenArgs, build_parser, parse_args};
pub use scan::{AutosummaryEntry, find_autosummary_in_files, find_autosummary_in_lines};
pub use templates::AutogenTemplates;
