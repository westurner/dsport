//! `sphinxdocrs::autogen` — Rust port of `sphinx.ext.autosummary.generate`.
//!
//! C4 target from the CLI port plan.
//!
//! Architecture:
//! - **Native**: RST file scanning (`find_autosummary_in_lines`), argument
//!   parser, template rendering, output file writing, `--remove-old`.
//! - **Python bridge**: object import/introspection (`generate_autosummary_docs`
//!   body) stays Python because it needs `autodoc` import machinery.
//!   The binary delegates only the generation step, not the scanning.

pub mod generate;
pub mod parser;
pub mod scan;
pub mod templates;

pub use generate::{
    ObjType, StubContext, generate_stub, generate_stubs, infer_obj_type, split_fqn,
};
pub use parser::{AutogenArgs, build_parser, parse_args};
pub use scan::{AutosummaryEntry, find_autosummary_in_files, find_autosummary_in_lines};
pub use templates::AutogenTemplates;
