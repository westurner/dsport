//! `sphinxdocrs::builders::latex` — Rust port of
//! `sphinx.builders.latex.LaTeXBuilder` (minimal path).
//!
//! Reads RST source files, parses with `docutilsrs::parse_rst_with_source`,
//! renders to LaTeX via `docutilsrs::latex`, and writes `.tex` output files.

use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, LatexOptions};
use docutilsrs::{latex, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Minimal LaTeX builder.
///
/// Mirrors `sphinx.builders.latex.LaTeXBuilder` core path.
#[derive(Debug, Default)]
pub struct LatexBuilder {
    options: LatexOptions,
    common: CommonOptions,
}

impl LatexBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Builder for LatexBuilder {
    fn name(&self) -> &str {
        "latex"
    }
    fn format(&self) -> &str {
        "latex"
    }
    fn out_suffix(&self) -> &str {
        ".tex"
    }

    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.tex")
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let output = latex(&tree, &self.options, &self.common);
        let rel: PathBuf = docname
            .split('/')
            .collect::<PathBuf>()
            .with_extension("tex");
        let out_path = outdir.join(rel);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&out_path, output.as_bytes())?;
        Ok(())
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut result = BuildResult::default();
        let docnames: Vec<String> = if !env.all_docs.is_empty() {
            env.all_docs.keys().cloned().collect()
        } else {
            super::html::discover_rst_docnames_pub(srcdir)
        };
        std::fs::create_dir_all(outdir)?;
        for docname in &docnames {
            // Use string append, not with_extension — the latter strips any
            // existing dot in the final component (e.g. "0.1" → "0.rst").
            let src_path = srcdir.join(format!("{docname}.rst"));
            let source = std::fs::read_to_string(&src_path).map_err(|e| {
                BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
            })?;
            self.build_doc(docname, &source, outdir)?;
            result.written += 1;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn name_and_format() {
        let b = LatexBuilder::new();
        assert_eq!(b.name(), "latex");
        assert_eq!(b.format(), "latex");
        assert_eq!(b.out_suffix(), ".tex");
    }

    #[test]
    fn get_target_uri() {
        assert_eq!(LatexBuilder::new().get_target_uri("index"), "index.tex");
    }

    #[test]
    fn build_doc_creates_tex_file() {
        let tmp = TempDir::new().unwrap();
        LatexBuilder::new()
            .build_doc("index", "Title\n=====\n\nContent.\n", tmp.path())
            .unwrap();
        let out = tmp.path().join("index.tex");
        assert!(out.exists());
        let contents = std::fs::read_to_string(out).unwrap();
        assert!(contents.contains("\\documentclass") || contents.contains("section"));
    }
}
