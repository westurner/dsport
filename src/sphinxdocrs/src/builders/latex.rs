//! `sphinxdocrs::builders::latex` — Rust port of
//! `sphinx.builders.latex.LaTeXBuilder` (minimal path).
//!
//! Reads RST source files, parses with `docutilsrs::parse_rst_with_source`,
//! renders to LaTeX via `docutilsrs::latex`, and writes `.tex` output files.

use std::path::{Path, PathBuf};

use docutilsrs::cli::{CommonOptions, LatexOptions};
use docutilsrs::{latex, parse_rst_with_source};

use super::{BuildError, BuildResult, Builder};
use crate::config::ConfigVal;
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

    fn write_support_files(outdir: &Path) -> Result<(), BuildError> {
        const STATIC_ASSETS: &[(&str, &[u8])] = &[
            (
                "LICRcyr2utf8.xdy",
                include_bytes!("../../../sphinx/sphinx/texinputs/LICRcyr2utf8.xdy"),
            ),
            (
                "LICRlatin2utf8.xdy",
                include_bytes!("../../../sphinx/sphinx/texinputs/LICRlatin2utf8.xdy"),
            ),
            (
                "LatinRules.xdy",
                include_bytes!("../../../sphinx/sphinx/texinputs/LatinRules.xdy"),
            ),
            (
                "python.ist",
                include_bytes!("../../../sphinx/sphinx/texinputs/python.ist"),
            ),
            (
                "sphinx.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinx.sty"),
            ),
            (
                "sphinx.xdy",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinx.xdy"),
            ),
            (
                "sphinxhowto.cls",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxhowto.cls"),
            ),
            (
                "sphinxlatexadmonitions.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexadmonitions.sty"),
            ),
            (
                "sphinxlatexcontainers.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexcontainers.sty"),
            ),
            (
                "sphinxlatexgraphics.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexgraphics.sty"),
            ),
            (
                "sphinxlatexindbibtoc.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexindbibtoc.sty"),
            ),
            (
                "sphinxlatexlists.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexlists.sty"),
            ),
            (
                "sphinxlatexliterals.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexliterals.sty"),
            ),
            (
                "sphinxlatexnumfig.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexnumfig.sty"),
            ),
            (
                "sphinxlatexobjects.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexobjects.sty"),
            ),
            (
                "sphinxlatexshadowbox.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexshadowbox.sty"),
            ),
            (
                "sphinxlatexstyleheadings.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexstyleheadings.sty"),
            ),
            (
                "sphinxlatexstylepage.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexstylepage.sty"),
            ),
            (
                "sphinxlatexstyletext.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatexstyletext.sty"),
            ),
            (
                "sphinxlatextables.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxlatextables.sty"),
            ),
            (
                "sphinxmanual.cls",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxmanual.cls"),
            ),
            (
                "sphinxoptionsgeometry.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxoptionsgeometry.sty"),
            ),
            (
                "sphinxoptionshyperref.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxoptionshyperref.sty"),
            ),
            (
                "sphinxpackageboxes.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxpackageboxes.sty"),
            ),
            (
                "sphinxpackagecyrillic.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxpackagecyrillic.sty"),
            ),
            (
                "sphinxpackagefootnote.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxpackagefootnote.sty"),
            ),
            (
                "sphinxpackagesubstitutefont.sty",
                include_bytes!("../../../sphinx/sphinx/texinputs/sphinxpackagesubstitutefont.sty"),
            ),
        ];
        for (name, contents) in STATIC_ASSETS {
            std::fs::write(outdir.join(name), contents)?;
        }

        let templates = [
            (
                "Makefile",
                include_str!("../../../sphinx/sphinx/texinputs/Makefile.jinja"),
            ),
            (
                "make.bat",
                include_str!("../../../sphinx/sphinx/texinputs/make.bat.jinja"),
            ),
            (
                "latexmkrc",
                include_str!("../../../sphinx/sphinx/texinputs/latexmkrc.jinja"),
            ),
            (
                "latexmkjarc",
                include_str!("../../../sphinx/sphinx/texinputs/latexmkjarc.jinja"),
            ),
        ];
        for (name, template) in templates {
            std::fs::write(
                outdir.join(name),
                Self::render_support_template(template).as_bytes(),
            )?;
        }
        Ok(())
    }

    fn render_support_template(template: &str) -> String {
        let mut output = String::new();
        let mut stack: Vec<(bool, bool)> = Vec::new();
        let mut active = true;
        for line in template.lines() {
            let directive = line
                .trim()
                .strip_prefix("{%")
                .and_then(|v| v.strip_suffix("%}"))
                .map(|v| v.trim_end_matches('-').trim());
            if let Some(expression) = directive.and_then(|v| v.strip_prefix("if ")) {
                let condition = Self::support_condition(expression.trim());
                stack.push((active, condition));
                active = active && condition;
            } else if let Some(expression) = directive.and_then(|v| v.strip_prefix("elif ")) {
                if let Some((parent, taken)) = stack.last_mut() {
                    let condition = !*taken && Self::support_condition(expression.trim());
                    active = *parent && condition;
                    *taken |= condition;
                }
            } else if directive == Some("else") {
                if let Some((parent, taken)) = stack.last_mut() {
                    active = *parent && !*taken;
                    *taken = true;
                }
            } else if directive == Some("endif") {
                active = stack.pop().map(|(parent, _)| parent).unwrap_or(true);
            } else if active {
                output.push_str(
                    &line
                        .replace("{{ latex_engine }}", "pdflatex")
                        .replace("{{ xindy_lang_option }}", "-L general -C utf8")
                        .replace("{{ xindy_use }}", "false")
                        .replace("{{ xindy_cyrillic }}", "false"),
                );
                output.push('\n');
            }
        }
        output
    }

    fn support_condition(expression: &str) -> bool {
        matches!(
            expression,
            "latex_engine == 'pdflatex'" | "latex_engine != 'xelatex'"
        )
    }

    fn configured_documents(env: &BuildEnvironment) -> Option<Vec<(String, String)>> {
        let Some(ConfigVal::List(entries)) = env.config.get("latex_documents") else {
            return None;
        };
        Some(
            entries
                .iter()
                .filter_map(|entry| {
                    let ConfigVal::List(fields) = entry else {
                        return None;
                    };
                    Some((
                        fields.first()?.as_str()?.to_owned(),
                        fields.get(1)?.as_str()?.to_owned(),
                    ))
                })
                .collect(),
        )
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
        Ok(latex(&tree, &self.options, &self.common))
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
            super::html::discover_docnames_pub(srcdir, &env.config)
        };
        std::fs::create_dir_all(outdir)?;

        if let Some(configured) = Self::configured_documents(env) {
            Self::write_support_files(outdir)?;
            let mut result = BuildResult::default();
            for (docname, targetname) in configured {
                let output = self.render_document(srcdir, env, &docname)?;
                let rel: PathBuf = targetname
                    .split('/')
                    .collect::<PathBuf>()
                    .with_extension("tex");
                let out_path = outdir.join(rel);
                if let Some(parent) = out_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(out_path, output.as_bytes())?;
                result.written += 1;
            }
            return Ok(result);
        }

        for docname in &docnames {
            // Use string append, not with_extension — the latter strips any
            // existing dot in the final component (e.g. "0.1" → "0.rst").
            let src_path =
                super::html::src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
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
