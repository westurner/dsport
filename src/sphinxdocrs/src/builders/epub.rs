//! Minimal EPUB3 builder with deterministic, stored ZIP entries.

use std::path::Path;

use docutilsrs::{html5, parse_rst_with_source, zip_writer::ZipBuilder};

use super::{BuildError, BuildResult, Builder};
use crate::environment::BuildEnvironment;

#[derive(Debug, Default)]
pub struct EpubBuilder;

impl EpubBuilder {
    pub fn new() -> Self { Self }

    fn document_xhtml(docname: &str, source: &str) -> String {
        let tree = parse_rst_with_source(source, docname);
        let body = html5(&tree, &docutilsrs::cli::Html5Options::default(), &docutilsrs::cli::CommonOptions::default());
        format!("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>{docname}</title></head><body>{body}</body></html>\n")
    }

    fn archive(docnames: &[String], documents: &[(String, String)]) -> Vec<u8> {
        let mut zip = ZipBuilder::new();
        zip.add_file("mimetype", b"application/epub+zip");
        zip.add_file("META-INF/container.xml", br#"<?xml version="1.0" encoding="utf-8"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#);
        let manifest: String = documents.iter().enumerate().map(|(index, (docname, _))| format!("<item id=\"doc{index}\" href=\"{docname}.xhtml\" media-type=\"application/xhtml+xml\"/>")).collect();
        let spine: String = (0..documents.len()).map(|index| format!("<itemref idref=\"doc{index}\"/>")).collect();
        zip.add_file("OEBPS/content.opf", format!("<?xml version=\"1.0\" encoding=\"utf-8\"?><package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\"><metadata><dc:title xmlns:dc=\"http://purl.org/dc/elements/1.1/\">Sphinx documentation</dc:title></metadata><manifest>{manifest}</manifest><spine>{spine}</spine></package>").as_bytes());
        let nav: String = docnames.iter().map(|docname| format!("<li><a href=\"{docname}.xhtml\">{docname}</a></li>")).collect();
        zip.add_file("OEBPS/toc.xhtml", format!("<?xml version=\"1.0\" encoding=\"utf-8\"?><html xmlns=\"http://www.w3.org/1999/xhtml\"><body><nav epub:type=\"toc\" xmlns:epub=\"http://www.idpf.org/2007/ops\"><ol>{nav}</ol></nav></body></html>").as_bytes());
        for (docname, content) in documents { zip.add_file(&format!("OEBPS/{docname}.xhtml"), content.as_bytes()); }
        zip.finish()
    }
}

impl Builder for EpubBuilder {
    fn name(&self) -> &str { "epub" }
    fn format(&self) -> &str { "epub" }
    fn out_suffix(&self) -> &str { ".epub" }
    fn get_target_uri(&self, docname: &str) -> String { format!("{docname}.xhtml") }
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        std::fs::create_dir_all(outdir)?;
        let path = outdir.join(format!("{docname}.epub"));
        std::fs::write(path, Self::archive(&[docname.to_owned()], &[(docname.to_owned(), Self::document_xhtml(docname, source))]))?;
        Ok(())
    }
    fn build_all(&self, srcdir: &Path, outdir: &Path, env: &BuildEnvironment) -> Result<BuildResult, BuildError> {
        let docnames: Vec<String> = if env.all_docs.is_empty() { super::html::discover_docnames_pub(srcdir, &env.config) } else { env.all_docs.keys().cloned().collect() };
        let mut documents = Vec::new();
        for docname in &docnames {
            let path = super::html::src_path_for_docname_with_suffixes(srcdir, docname, &env.config)?;
            let source = crate::environment::read_source_file(&path, &env.config.source_encoding()).map_err(|e| BuildError::Other(format!("failed to read {}: {e}", path.display())))?;
            documents.push((docname.clone(), Self::document_xhtml(docname, &source)));
        }
        std::fs::create_dir_all(outdir)?;
        std::fs::write(outdir.join(format!("{}.epub", env.config.project())), Self::archive(&docnames, &documents))?;
        Ok(BuildResult { written: docnames.len(), ..BuildResult::default() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn writes_epub_container_and_document() {
        let out = TempDir::new().unwrap();
        EpubBuilder::new().build_doc("index", "Title\n=====\n\nBody.\n", out.path()).unwrap();
        let bytes = std::fs::read(out.path().join("index.epub")).unwrap();
        assert!(bytes.windows(8).any(|window| window == b"mimetype"));
        assert!(bytes.windows(20).any(|window| window == b"application/epub+zip"));
        assert!(bytes.windows(11).any(|window| window == b"content.opf"));
    }
}