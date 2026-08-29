//! EPUB3 builder with deterministic, stored ZIP entries.

use std::path::Path;

use docutilsrs::{Doctree, html5, parse_rst_with_source, zip_writer::ZipBuilder};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;
use crate::util_strypes::xml_escape;

#[derive(Debug, Default)]
pub struct EpubBuilder;

impl EpubBuilder {
    pub fn new() -> Self {
        Self
    }

    fn document_xhtml(docname: &str, tree: &Doctree) -> String {
        let body = html5(
            &tree,
            &docutilsrs::cli::Html5Options::default(),
            &docutilsrs::cli::CommonOptions::default(),
        );
        let title = Self::title_text(tree).unwrap_or_else(|| docname.to_string());
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>{}</title></head><body>{body}</body></html>\n",
            xml_escape(&title)
        )
    }

    fn title_text(tree: &Doctree) -> Option<String> {
        fn visit(tree: &Doctree, id: usize, text: &mut String) -> bool {
            if matches!(tree.node(id).kind, docutilsrs::NodeKind::Title) {
                collect_text(tree, id, text);
                return !text.is_empty();
            }
            tree.node(id)
                .children
                .iter()
                .copied()
                .any(|child| visit(tree, child, text))
        }
        fn collect_text(tree: &Doctree, id: usize, text: &mut String) {
            if let docutilsrs::NodeKind::Text(value) = &tree.node(id).kind {
                text.push_str(value);
            }
            for child in tree.node(id).children.iter().copied() {
                collect_text(tree, child, text);
            }
        }
        let mut title = String::new();
        visit(tree, tree.root(), &mut title).then_some(title)
    }

    fn archive(
        documents: &[(String, String, String)],
        title: &str,
        author: &str,
        language: &str,
        uid: &str,
        description: &str,
        publisher: &str,
        rights: &str,
    ) -> Vec<u8> {
        let mut zip = ZipBuilder::new();
        zip.add_file("mimetype", b"application/epub+zip");
        zip.add_file("META-INF/container.xml", br#"<?xml version="1.0" encoding="utf-8"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#);
        let manifest: String = documents.iter().enumerate().map(|(index, (docname, _, _))| format!("<item id=\"doc{index}\" href=\"{}.xhtml\" media-type=\"application/xhtml+xml\"/>", xml_escape(docname))).collect();
        let spine: String = (0..documents.len())
            .map(|index| format!("<itemref idref=\"doc{index}\"/>"))
            .collect();
        let opf = format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"pub-id\"><metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\"><dc:identifier id=\"pub-id\">{}</dc:identifier><dc:title>{}</dc:title><dc:creator>{}</dc:creator><dc:language>{}</dc:language><dc:description>{}</dc:description><dc:publisher>{}</dc:publisher><dc:rights>{}</dc:rights></metadata><manifest><item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/><item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\"/>{manifest}</manifest><spine toc=\"ncx\">{spine}</spine></package>\n",
            xml_escape(uid),
            xml_escape(title),
            xml_escape(author),
            xml_escape(language),
            xml_escape(description),
            xml_escape(publisher),
            xml_escape(rights)
        );
        zip.add_file("OEBPS/content.opf", opf.as_bytes());
        let nav: String = documents
            .iter()
            .map(|(docname, _, title)| {
                format!(
                    "<li><a href=\"{}.xhtml\">{}</a></li>",
                    xml_escape(docname),
                    xml_escape(title)
                )
            })
            .collect();
        zip.add_file("OEBPS/nav.xhtml", format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\"><head><title>{}</title></head><body><nav epub:type=\"toc\" id=\"toc\"><ol>{nav}</ol></nav></body></html>\n", xml_escape(title)).as_bytes());
        let nav_points: String = documents.iter().enumerate().map(|(index, (docname, _, title))| format!("<navPoint id=\"navPoint-{index}\" playOrder=\"{}\"><navLabel><text>{}</text></navLabel><content src=\"{}.xhtml\"/></navPoint>", index + 1, xml_escape(title), xml_escape(docname))).collect();
        zip.add_file("OEBPS/toc.ncx", format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<ncx xmlns=\"http://www.daisy.org/z3986/2005/ncx/\" version=\"2005-1\"><head><meta name=\"dtb:uid\" content=\"{}\"/></head><docTitle><text>{}</text></docTitle><navMap>{nav_points}</navMap></ncx>\n", xml_escape(uid), xml_escape(title)).as_bytes());
        for (docname, content, _) in documents {
            zip.add_file(&format!("OEBPS/{docname}.xhtml"), content.as_bytes());
        }
        zip.finish()
    }
}

fn config_string(env: &BuildEnvironment, key: &str, fallback: &str) -> String {
    env.config
        .get(key)
        .and_then(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| fallback.to_string())
}

impl Builder for EpubBuilder {
    fn name(&self) -> &str {
        "epub"
    }
    fn format(&self) -> &str {
        "epub"
    }
    fn out_suffix(&self) -> &str {
        ".epub"
    }
    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.xhtml")
    }
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let document = Self::document_xhtml(docname, &tree);
        let title = Self::title_text(&tree).unwrap_or_else(|| docname.to_string());
        let path = outdir.join(format!("{docname}.epub"));
        std::fs::create_dir_all(outdir)?;
        std::fs::write(
            path,
            Self::archive(
                &[(docname.to_owned(), document, title)],
                "Sphinx documentation",
                "",
                "en",
                "unknown",
                "",
                "",
                "",
            ),
        )?;
        Ok(())
    }
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let mut docnames: Vec<String> = if env.all_docs.is_empty() {
            super::html::discover_docnames_pub(srcdir, &env.config)
        } else {
            env.all_docs.keys().cloned().collect()
        };
        docnames.sort();
        let mut documents = Vec::new();
        for docname in &docnames {
            let tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
                    let path = super::html::src_path_for_docname_with_suffixes(
                        srcdir,
                        docname,
                        &env.config,
                    )?;
                    let source =
                        crate::environment::read_source_file(&path, &env.config.source_encoding())
                            .map_err(|e| {
                                BuildError::Other(format!("failed to read {}: {e}", path.display()))
                            })?;
                    parse_rst_with_source(&source, docname)
                }
            };
            let title = Self::title_text(&tree).unwrap_or_else(|| docname.clone());
            documents.push((docname.clone(), Self::document_xhtml(docname, &tree), title));
        }
        std::fs::create_dir_all(outdir)?;
        let title = config_string(env, "epub_title", &env.config.project());
        let author = config_string(env, "epub_author", &env.config.author());
        let language = config_string(env, "epub_language", &env.config.language());
        let uid = config_string(env, "epub_uid", "unknown");
        let description = config_string(env, "epub_description", "unknown");
        let publisher = config_string(env, "epub_publisher", "");
        let rights = config_string(
            env,
            "epub_copyright",
            &env.config
                .get("project_copyright")
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default(),
        );
        let basename = config_string(env, "epub_basename", &env.config.project());
        std::fs::write(
            outdir.join(format!("{basename}.epub")),
            Self::archive(
                &documents,
                &title,
                &author,
                &language,
                &uid,
                &description,
                &publisher,
                &rights,
            ),
        )?;
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
    fn writes_epub_container_and_document() {
        let out = TempDir::new().unwrap();
        EpubBuilder::new()
            .build_doc("index", "Title\n=====\n\nBody.\n", out.path())
            .unwrap();
        let bytes = std::fs::read(out.path().join("index.epub")).unwrap();
        assert!(bytes.windows(8).any(|window| window == b"mimetype"));
        assert!(
            bytes
                .windows(20)
                .any(|window| window == b"application/epub+zip")
        );
        assert!(bytes.windows(11).any(|window| window == b"content.opf"));
    }
}
