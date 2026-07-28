//! `sphinxdocrs::builders::xml` — Rust port of `sphinx.builders.xml.XMLBuilder`.
//!
//! Thin wrapper over `docutilsrs::to_xml` (a well-formed, closed-tag XML
//! serialization of the doctree — see that module's own doc comment for
//! its accepted pretty-printing deviation). One `<docname>.xml` file is
//! written per document.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `XMLBuilder.name` | `"xml"` | constant |
//! | `XMLBuilder.format` | `"xml"` | constant |
//! | `XMLBuilder.out_suffix` | `".xml"` | constant |
//! | `XMLBuilder.get_target_uri` | [`XmlBuilder::get_target_uri`] | `"{docname}.xml"`, matching upstream |
//! | `XMLBuilder.write_doc` | [`XmlBuilder::build_doc`] | parse → `docutilsrs::to_xml` → write `<docname>.xml` |
//!
//! `PseudoXMLBuilder` (`sphinx.builders.xml.PseudoXMLBuilder`, which uses
//! `docutils.writers.pseudoxml` instead) is ported separately as
//! [`crate::builders::pseudoxml::PseudoxmlBuilder`].

use std::path::{Path, PathBuf};

use docutilsrs::{parse_rst_with_source, to_xml};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Docutils-XML builder.
#[derive(Debug, Default)]
pub struct XmlBuilder;

impl XmlBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for XmlBuilder {
    fn name(&self) -> &str {
        "xml"
    }
    fn format(&self) -> &str {
        "xml"
    }
    fn out_suffix(&self) -> &str {
        ".xml"
    }

    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.xml")
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let output = to_xml(&tree);
        let rel: PathBuf = docname.split('/').collect::<PathBuf>();
        let out_path = outdir.join(rel).with_extension("xml");
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
        let b = XmlBuilder::new();
        assert_eq!(b.name(), "xml");
        assert_eq!(b.format(), "xml");
        assert_eq!(b.out_suffix(), ".xml");
    }

    #[test]
    fn get_target_uri_appends_xml_suffix() {
        let b = XmlBuilder::new();
        assert_eq!(b.get_target_uri("index"), "index.xml");
        assert_eq!(b.get_target_uri("guide/intro"), "guide/intro.xml");
    }

    #[test]
    fn build_doc_writes_well_formed_xml() {
        let tmp = TempDir::new().unwrap();
        XmlBuilder::new()
            .build_doc("index", "Title\n=====\n\nHello world.\n", tmp.path())
            .unwrap();
        let out = std::fs::read_to_string(tmp.path().join("index.xml")).unwrap();
        assert!(out.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
        assert!(out.trim_end().ends_with("</document>"));
    }

    #[test]
    fn build_doc_creates_nested_directories() {
        let tmp = TempDir::new().unwrap();
        XmlBuilder::new()
            .build_doc("guide/intro", "Intro\n=====\n\nBody.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("guide/intro.xml").exists());
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
        let result = XmlBuilder::new()
            .build_all(tmp.path(), &outdir, &env)
            .unwrap();
        assert_eq!(result.written, 2);
        assert!(outdir.join("index.xml").exists());
        assert!(outdir.join("other.xml").exists());
    }
}
