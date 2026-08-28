use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::texinfo::TexinfoBuilder;
use sphinxdocrs::config::SphinxConfig;
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_writes_texinfo_documents() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let env = BuildEnvironment::new(SphinxConfig::new_defaults(), project, src.path(), doctrees.path());

    let result = TexinfoBuilder::new().build_all(src.path(), out.path(), &env).unwrap();
    assert_eq!(result.written, 1);
    let output = std::fs::read_to_string(out.path().join("index.texi")).unwrap();
    assert!(output.contains("@setfilename index.info"));
    assert!(output.ends_with("@bye\n"));
}