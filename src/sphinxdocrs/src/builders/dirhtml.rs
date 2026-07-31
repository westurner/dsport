//! `sphinxdocrs::builders::dirhtml` — Rust port of
//! `sphinx.builders.dirhtml.DirectoryHTMLBuilder`.
//!
//! Delegates every [`Builder`] method to an inner [`HtmlBuilder`]
//! constructed via `HtmlBuilder::new_dir_style()`, reusing the entire
//! theming / search-index / static-asset pipeline (**H6**) unchanged —
//! only the output *file layout* and [`Builder::get_target_uri`] differ
//! (`<docname>/index.html` instead of `<docname>.html`; see
//! `HtmlBuilder::PathStyle`'s doc comment for the recorded accepted
//! deviation about in-page navigation links).
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `DirectoryHTMLBuilder.name` | `"dirhtml"` | constant |
//! | `DirectoryHTMLBuilder.format` | `"html"` | constant (matches upstream: same output format, different layout) |
//! | `DirectoryHTMLBuilder.out_suffix` | `"/index.html"` | constant |
//! | `DirectoryHTMLBuilder.get_target_uri` | delegates to `HtmlBuilder::new_dir_style().get_target_uri` | |
//! | `DirectoryHTMLBuilder.write_doc`/`build_all` | delegates to the inner `HtmlBuilder` | full theme/search/static pipeline reused |

use std::path::Path;

use super::html::HtmlBuilder;
use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;
use docutilsrs::doctree::Doctree;

/// Directory-style HTML builder: `<docname>/index.html` per document.
#[derive(Default)]
pub struct DirhtmlBuilder(HtmlBuilder);

impl DirhtmlBuilder {
    pub fn new() -> Self {
        Self(HtmlBuilder::new_dir_style())
    }
}

impl Builder for DirhtmlBuilder {
    fn name(&self) -> &str {
        "dirhtml"
    }

    /// Mirrors upstream: same output *format* as `html`, different layout.
    fn format(&self) -> &str {
        "html"
    }

    fn out_suffix(&self) -> &str {
        self.0.out_suffix()
    }

    fn get_target_uri(&self, docname: &str) -> String {
        self.0.get_target_uri(docname)
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        self.0.build_doc(docname, source, outdir)
    }

    fn write_doc(&self, docname: &str, doctree: &Doctree, outdir: &Path) -> Result<(), BuildError> {
        self.0.write_doc(docname, doctree, outdir)
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        self.0.build_all(srcdir, outdir, env)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn name_format_and_suffix() {
        let b = DirhtmlBuilder::new();
        assert_eq!(b.name(), "dirhtml");
        assert_eq!(b.format(), "html");
        assert_eq!(b.out_suffix(), "/index.html");
    }

    #[test]
    fn get_target_uri_uses_directory_style() {
        let b = DirhtmlBuilder::new();
        assert_eq!(b.get_target_uri("index"), "");
        assert_eq!(b.get_target_uri("guide/index"), "guide/");
        assert_eq!(b.get_target_uri("guide/intro"), "guide/intro/");
    }

    #[test]
    fn build_doc_writes_docname_slash_index_html() {
        let tmp = TempDir::new().unwrap();
        DirhtmlBuilder::new()
            .build_doc("guide/intro", "Intro\n=====\n\nBody.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("guide/intro/index.html").exists());
        assert!(!tmp.path().join("guide/intro.html").exists());
    }

    #[test]
    fn build_doc_index_docname_stays_at_root() {
        let tmp = TempDir::new().unwrap();
        DirhtmlBuilder::new()
            .build_doc("index", "Welcome\n=======\n\nHome.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("index.html").exists());
        assert!(!tmp.path().join("index/index.html").exists());
    }

    #[test]
    fn build_all_writes_directory_layout_for_every_doc() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Welcome\n=======\n\nHomepage.\n",
        )
        .unwrap();
        std::fs::write(src.path().join("about.rst"), "About\n=====\n\nSome info.\n").unwrap();
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        let result = DirhtmlBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 2);
        assert!(out.path().join("index.html").exists());
        assert!(out.path().join("about/index.html").exists());
    }
}
