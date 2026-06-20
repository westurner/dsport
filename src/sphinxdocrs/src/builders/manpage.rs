//! `sphinxdocrs::builders::manpage` — Rust port of
//! `sphinx.builders.manpage.ManualPageBuilder` (minimal path).

use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, ManOptions};
use docutilsrs::{manpage, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

/// Minimal man-page builder.
#[derive(Debug, Default)]
pub struct ManpageBuilder {
    options: ManOptions,
    common: CommonOptions,
}

impl ManpageBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Builder for ManpageBuilder {
    fn name(&self) -> &str {
        "man"
    }
    fn format(&self) -> &str {
        "man"
    }
    fn out_suffix(&self) -> &str {
        ""
    }

    fn get_target_uri(&self, docname: &str) -> String {
        docname.to_string()
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let output = manpage(&tree, &self.options, &self.common);
        let rel: PathBuf = docname.split('/').collect::<PathBuf>();
        let out_path = outdir.join(rel).with_extension("1");
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
            let src_path = srcdir.join(
                docname
                    .split('/')
                    .collect::<PathBuf>()
                    .with_extension("rst"),
            );
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
        let b = ManpageBuilder::new();
        assert_eq!(b.name(), "man");
        assert_eq!(b.format(), "man");
    }

    #[test]
    fn build_doc_creates_man_file() {
        let tmp = TempDir::new().unwrap();
        ManpageBuilder::new()
            .build_doc("mycommand", "mycommand\n=========\n\nA tool.\n", tmp.path())
            .unwrap();
        assert!(tmp.path().join("mycommand.1").exists());
    }
}
