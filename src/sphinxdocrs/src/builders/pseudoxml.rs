//! `sphinxdocrs::builders::pseudoxml` — Rust port of
//! `sphinx.builders.xml.PseudoXMLBuilder`.
//!
//! Thin wrapper over `docutilsrs::pseudo_xml` (already fully implemented,
//! used elsewhere for `docutilsrs`'s own parity tests). One
//! `<docname>.pseudoxml` file is written per document.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `PseudoXMLBuilder.name` | `"pseudoxml"` | constant |
//! | `PseudoXMLBuilder.format` | `"pseudoxml"` | constant |
//! | `PseudoXMLBuilder.out_suffix` | `".pseudoxml"` | constant |
//! | `PseudoXMLBuilder.get_target_uri` | [`PseudoxmlBuilder::get_target_uri`] | `"{docname}.pseudoxml"`, matching upstream |
//! | `PseudoXMLBuilder.write_doc` | [`PseudoxmlBuilder::build_doc`] | parse → `docutilsrs::pseudo_xml` → write `<docname>.pseudoxml` |

use std::path::{Path, PathBuf};

use docutilsrs::{parse_rst_with_source, pseudo_xml};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Pseudo-XML builder.
#[derive(Debug, Default)]
pub struct PseudoxmlBuilder;

impl PseudoxmlBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for PseudoxmlBuilder {
    fn name(&self) -> &str {
        "pseudoxml"
    }
    fn format(&self) -> &str {
        "pseudoxml"
    }
    fn out_suffix(&self) -> &str {
        ".pseudoxml"
    }

    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.pseudoxml")
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let output = pseudo_xml(&tree);
        let rel: PathBuf = docname.split('/').collect::<PathBuf>();
        let out_path = outdir.join(rel).with_extension("pseudoxml");
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
    fn name_format_and_suffix() {
        let b = PseudoxmlBuilder::new();
        assert_eq!(b.name(), "pseudoxml");
        assert_eq!(b.format(), "pseudoxml");
        assert_eq!(b.out_suffix(), ".pseudoxml");
    }

    #[test]
    fn get_target_uri_appends_pseudoxml_suffix() {
        let b = PseudoxmlBuilder::new();
        assert_eq!(b.get_target_uri("index"), "index.pseudoxml");
        assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.pseudoxml");
    }

    #[test]
    fn build_doc_writes_pseudoxml_file() {
        let tmp = TempDir::new().unwrap();
        PseudoxmlBuilder::new()
            .build_doc("index", "Title\n=====\n\nHello world.\n", tmp.path())
            .unwrap();
        let out = std::fs::read_to_string(tmp.path().join("index.pseudoxml")).unwrap();
        assert!(out.contains("<document"));
        assert!(out.contains("<paragraph>"));
    }

    #[test]
    fn build_doc_creates_nested_directories() {
        let tmp = TempDir::new().unwrap();
        PseudoxmlBuilder::new()
            .build_doc("guide/intro", "Intro\n=====\n\nBody.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("guide/intro.pseudoxml").exists());
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
        let env =
            crate::environment::BuildEnvironment::new(config, project, tmp.path(), &outdir);
        let result = PseudoxmlBuilder::new()
            .build_all(tmp.path(), &outdir, &env)
            .unwrap();
        assert_eq!(result.written, 2);
        assert!(outdir.join("index.pseudoxml").exists());
        assert!(outdir.join("other.pseudoxml").exists());
    }
}
