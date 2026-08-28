use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::epub::EpubBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_writes_one_epub_archive_with_documents() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(SphinxConfig::new_defaults(), project, src.path(), doctrees.path());

    let result = EpubBuilder::new().build_all(src.path(), out.path(), &env).unwrap();
    assert_eq!(result.written, 1);
    let archive = std::fs::read(out.path().join("Project name not set.epub")).unwrap();
    assert!(archive.windows(11).any(|window| window == b"content.opf"));
    assert!(archive.windows(11).any(|window| window == b"index.xhtml"));
}