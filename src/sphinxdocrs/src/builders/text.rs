//! `sphinxdocrs::builders::text` — Rust port of
//! `sphinx.builders.text.TextBuilder`.
//!
//! Thin wrapper over `docutilsrs::text` (see that module's own doc comment
//! for the accepted simplifications: no line wrapping, no real table
//! drawing). One `<docname>.txt` file is written per document.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `TextBuilder.name` | `"text"` | constant |
//! | `TextBuilder.format` | `"text"` | constant |
//! | `TextBuilder.out_suffix` | `".txt"` | constant |
//! | `TextBuilder.get_target_uri` | [`TextBuilder::get_target_uri`] | returns `""` (matches upstream: text output has no meaningful cross-doc URIs) |
//! | `TextBuilder.write_doc` | [`TextBuilder::build_doc`] | parse → `docutilsrs::text` → write `<docname>.txt` |

use std::path::{Path, PathBuf};

use docutilsrs::{parse_rst_with_source, text};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Plain-text builder.
#[derive(Debug, Default)]
pub struct TextBuilder;

impl TextBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for TextBuilder {
    fn name(&self) -> &str {
        "text"
    }
    fn format(&self) -> &str {
        "text"
    }
    fn out_suffix(&self) -> &str {
        ".txt"
    }

    /// Mirrors `TextBuilder.get_target_uri`, which always returns `""`.
    fn get_target_uri(&self, _docname: &str) -> String {
        String::new()
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let output = text(&tree);
        let rel: PathBuf = docname.split('/').collect::<PathBuf>();
        let out_path = outdir.join(rel).with_extension("txt");
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
            let src_path = srcdir.join(format!("{docname}.rst"));
            let source =
                crate::environment::read_source_file(&src_path, &env.config.source_encoding())
                    .map_err(|e| {
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
    fn name_format_and_suffix() {
        let b = TextBuilder::new();
        assert_eq!(b.name(), "text");
        assert_eq!(b.format(), "text");
        assert_eq!(b.out_suffix(), ".txt");
    }

    #[test]
    fn get_target_uri_is_always_empty() {
        let b = TextBuilder::new();
        assert_eq!(b.get_target_uri("index"), "");
        assert_eq!(b.get_target_uri("guide/intro"), "");
    }

    #[test]
    fn build_doc_writes_txt_file_with_expected_content() {
        let tmp = TempDir::new().unwrap();
        TextBuilder::new()
            .build_doc("index", "Title\n=====\n\nHello *world*.\n", tmp.path())
            .unwrap();
        let out = std::fs::read_to_string(tmp.path().join("index.txt")).unwrap();
        assert!(out.starts_with("Title\n=====\n"));
        assert!(out.contains("Hello *world*."));
    }

    #[test]
    fn build_doc_creates_nested_directories() {
        let tmp = TempDir::new().unwrap();
        TextBuilder::new()
            .build_doc("guide/intro", "Intro\n=====\n\nBody.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("guide/intro.txt").exists());
    }

    #[test]
    fn build_all_discovers_and_writes_every_doc() {
        let tmp = TempDir::new().unwrap();
        std::fs::write(tmp.path().join("index.rst"), "Index\n=====\n\nA.\n").unwrap();
        std::fs::write(tmp.path().join("other.rst"), "Other\n=====\n\nB.\n").unwrap();
        let outdir = tmp.path().join("_out");
        let config = crate::config::SphinxConfig::new_defaults();
        let project =
            crate::environment::EnvProject::new(tmp.path(), &[(".rst", "restructuredtext")]);
        let env = crate::environment::BuildEnvironment::new(config, project, tmp.path(), &outdir);
        let result = TextBuilder::new()
            .build_all(tmp.path(), &outdir, &env)
            .unwrap();
        assert_eq!(result.written, 2);
        assert!(outdir.join("index.txt").exists());
        assert!(outdir.join("other.txt").exists());
    }
}
