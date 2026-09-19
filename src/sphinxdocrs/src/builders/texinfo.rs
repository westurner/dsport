//! Minimal native texinfo builder.

use std::path::{Path, PathBuf};

use docutilsrs::{parse_rst_with_source, text};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

#[derive(Debug, Default)]
pub struct TexinfoBuilder;

impl TexinfoBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl Builder for TexinfoBuilder {
    fn name(&self) -> &str {
        "texinfo"
    }
    fn format(&self) -> &str {
        "texinfo"
    }
    fn out_suffix(&self) -> &str {
        ".texi"
    }
    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.texi")
    }

    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let body = text(&tree);
        let title = docname.rsplit('/').next().unwrap_or(docname);
        let output = format!(
            "\\input texinfo\n@setfilename {title}.info\n@settitle {title}\n\n{body}\n@bye\n"
        );
        let rel: PathBuf = docname
            .split('/')
            .collect::<PathBuf>()
            .with_extension("texi");
        let path = outdir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, output)?;
        Ok(())
    }

    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let docnames: Vec<String> = if env.all_docs.is_empty() {
            super::html::discover_docnames_pub(srcdir, &env.config)
        } else {
            env.all_docs.keys().cloned().collect()
        };
        std::fs::create_dir_all(outdir)?;
        for docname in &docnames {
            let path =
                super::html::src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
            let source = crate::environment::read_source_file(&path, &env.config.source_encoding())
                .map_err(|e| {
                    BuildError::Other(format!("failed to read {}: {e}", path.display()))
                })?;
            self.build_doc(docname, &source, outdir)?;
        }
        Ok(BuildResult {
            written: docnames.len(),
            ..BuildResult::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn writes_texinfo_header_and_body() {
        let out = TempDir::new().unwrap();
        TexinfoBuilder::new()
            .build_doc("index", "Title\n=====\n\nBody.\n", out.path())
            .unwrap();
        let output = std::fs::read_to_string(out.path().join("index.texi")).unwrap();
        assert!(output.starts_with("\\input texinfo\n"));
        assert!(output.contains("Body."));
        assert!(output.ends_with("@bye\n"));
    }

    #[test]
    fn trait_metadata_and_nested_build_doc_path() {
        let out = TempDir::new().unwrap();
        let builder = TexinfoBuilder::new();
        assert_eq!(builder.name(), "texinfo");
        assert_eq!(builder.format(), "texinfo");
        assert_eq!(builder.out_suffix(), ".texi");
        assert_eq!(builder.get_target_uri("guide/intro"), "guide/intro.texi");
        builder
            .build_doc("guide/intro", "Intro\n=====\n\nBody.\n", out.path())
            .unwrap();
        assert!(out.path().join("guide/intro.texi").is_file());
    }

    #[test]
    fn build_all_discovers_docs_uses_existing_docnames_and_reports_missing_sources() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::create_dir_all(src.path().join("guide")).unwrap();
        std::fs::write(src.path().join("index.rst"), "Index\n=====\n").unwrap();
        std::fs::write(src.path().join("guide/intro.rst"), "Intro\n=====\n").unwrap();
        let project = crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let mut env = BuildEnvironment::new(
            crate::config::SphinxConfig::new_defaults(),
            project,
            src.path(),
            out.path(),
        );
        let builder = TexinfoBuilder::new();
        assert_eq!(builder.build_all(src.path(), out.path(), &env).unwrap().written, 2);

        env.all_docs.insert("guide/intro".into(), 1);
        env.all_docs.insert("index".into(), 1);
        let second_out = TempDir::new().unwrap();
        assert_eq!(builder.build_all(src.path(), second_out.path(), &env).unwrap().written, 2);

        env.all_docs.insert("missing".into(), 1);
        assert!(builder.build_all(src.path(), second_out.path(), &env).is_err());
    }
}
