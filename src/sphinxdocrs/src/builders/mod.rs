//! `sphinxdocrs::builders` — Rust port of `sphinx.builders`.
//!
//! Builder trait + concrete builders.
//!
//! ## What is ported
//!
//! | upstream class | Rust type | notes |
//! | --- | --- | --- |
//! | `Builder` (ABC) | [`Builder`] | trait with `name`, `format`, `out_suffix`, `get_target_uri`, `build_doc`, `build_all` |
//! | `StandaloneHTMLBuilder` | [`html::HtmlBuilder`] | minimal RST → HTML5 file writer via `docutilsrs` |
//!
//! **Deferred** (requires full Sphinx app): theming, Jinja2 templates, CSS/JS
//! asset copying, search index, domain indices.

pub mod html;
pub mod json;
pub mod latex;
pub mod manpage;

use std::io;
use std::path::Path;

use crate::environment::BuildEnvironment;

// ── BuildError ────────────────────────────────────────────────────────────────

/// Error type for builder operations.
#[derive(Debug)]
pub enum BuildError {
    /// An I/O error (reading source, writing output).
    Io(io::Error),
    /// A builder-specific error message.
    Other(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Io(e) => write!(f, "I/O error: {e}"),
            BuildError::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<io::Error> for BuildError {
    fn from(e: io::Error) -> Self {
        BuildError::Io(e)
    }
}

// ── BuildResult ───────────────────────────────────────────────────────────────

/// Summary of a completed build.
#[derive(Debug, Clone, Default)]
pub struct BuildResult {
    /// Number of documents written.
    pub written: usize,
    /// Number of documents skipped (up-to-date).
    pub skipped: usize,
    /// Warnings collected during the build.
    pub warnings: Vec<String>,
}

// ── Builder trait ─────────────────────────────────────────────────────────────

/// Core builder interface.
///
/// Mirrors the abstract methods of `sphinx.builders.Builder` that are needed
/// for a minimal native build.
pub trait Builder {
    /// The builder's name (used in `sphinx-build -b <name>`).
    ///
    /// Mirrors `Builder.name`.
    fn name(&self) -> &str;

    /// The output format string.
    ///
    /// Mirrors `Builder.format`.
    fn format(&self) -> &str;

    /// The file suffix for output files (e.g. `".html"`).
    ///
    /// Mirrors `StandaloneHTMLBuilder.out_suffix`.
    fn out_suffix(&self) -> &str;

    /// Return the output URI for `docname`.
    ///
    /// Mirrors `Builder.get_target_uri`.
    fn get_target_uri(&self, docname: &str) -> String;

    /// Write a single document from RST `source` into `outdir`.
    ///
    /// Mirrors `Builder.write_doc`.
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError>;

    /// Build all documents in `srcdir` into `outdir`, guided by `env`.
    ///
    /// Discovers `.rst` files under `srcdir`, calls [`build_doc`](Builder::build_doc)
    /// for each, and returns a [`BuildResult`].
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError>;
}
