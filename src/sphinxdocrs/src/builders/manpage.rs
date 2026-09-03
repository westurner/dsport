//! `sphinxdocrs::builders::manpage` — Rust port of
//! `sphinx.builders.manpage.ManualPageBuilder` (minimal path).

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, ManOptions};
use docutilsrs::{manpage, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::config::ConfigVal;
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

    fn configured_pages(
        env: &BuildEnvironment,
    ) -> Result<Option<Vec<(String, String, String)>>, BuildError> {
        let Some(ConfigVal::List(entries)) = env.config.get("man_pages") else {
            return Ok(None);
        };
        let mut pages = Vec::with_capacity(entries.len());
        let mut outputs = HashSet::new();
        for entry in entries {
            let ConfigVal::List(fields) = entry else {
                return Err(BuildError::Other(
                    "man_pages entries must be sequences".into(),
                ));
            };
            let docname = fields.first().and_then(ConfigVal::as_str).ok_or_else(|| {
                BuildError::Other("man_pages entry has no source document".into())
            })?;
            let name = fields
                .get(1)
                .and_then(ConfigVal::as_str)
                .ok_or_else(|| BuildError::Other("man_pages entry has no command name".into()))?;
            let section = fields
                .get(4)
                .and_then(ConfigVal::as_str)
                .ok_or_else(|| BuildError::Other("man_pages entry has no section".into()))?;
            if !outputs.insert((name.to_owned(), section.to_owned())) {
                return Err(BuildError::Other(format!(
                    "duplicate man page output: {name}.{section}"
                )));
            }
            pages.push((docname.to_owned(), name.to_owned(), section.to_owned()));
        }
        Ok(Some(pages))
    }

    fn render_document(
        &self,
        srcdir: &Path,
        env: &BuildEnvironment,
        docname: &str,
    ) -> Result<String, BuildError> {
        let tree = match env.get_and_resolve_doctree(docname) {
            Ok(tree) => tree,
            Err(_) => {
                let src_path =
                    super::html::src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
                let source =
                    crate::environment::read_source_file(&src_path, &env.config.source_encoding())
                        .map_err(|e| {
                            BuildError::Other(format!("failed to read {}: {e}", src_path.display()))
                        })?;
                docutilsrs::parse_rst_with_source(&source, docname)
            }
        };
        Ok(manpage(&tree, &self.options, &self.common))
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
        std::fs::create_dir_all(outdir)?;

        if let Some(configured) = Self::configured_pages(env)? {
            let mut result = BuildResult::default();
            for (docname, name, section) in configured {
                let output = self.render_document(srcdir, env, &docname)?;
                let rel: PathBuf = format!("{name}.{section}").split('/').collect();
                let out_path = outdir.join(rel);
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(out_path, output.as_bytes())?;
                result.written += 1;
            }
            return Ok(result);
        }
        Ok(BuildResult::default())
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
