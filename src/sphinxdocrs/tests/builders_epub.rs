use tempfile::TempDir;

use sphinxdocrs::builders::epub::EpubBuilder;
use sphinxdocrs::builders::Builder;
use sphinxdocrs::config::{ConfigVal, SphinxConfig};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_writes_one_epub_archive_with_documents() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut config = SphinxConfig::new_defaults();
    config.set("epub_title", ConfigVal::Str("Guide & Reference".into()));
    config.set("epub_author", ConfigVal::Str("A <Writer>".into()));
    config.set("epub_uid", ConfigVal::Str("urn:example:guide".into()));
    config.set("epub_basename", ConfigVal::Str("guide-book".into()));
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Reparsed title\n==============\n\nWrong body.\n",
    )
    .unwrap();

    let result = EpubBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 1);
    let archive = std::fs::read(out.path().join("guide-book.epub")).unwrap();
    assert_eq!(&archive[..4], b"PK\x03\x04");
    assert_eq!(&archive[8..10], &[0, 0]);
    let first_name_len = u16::from_le_bytes([archive[26], archive[27]]) as usize;
    assert!(archive[30 + first_name_len..].starts_with(b"application/epub+zip"));
    for marker in [
        b"content.opf".as_slice(),
        b"index.xhtml".as_slice(),
        b"nav.xhtml".as_slice(),
        b"toc.ncx".as_slice(),
        b"properties=\"nav\"".as_slice(),
        b"Guide &amp; Reference".as_slice(),
        b"A &lt;Writer&gt;".as_slice(),
    ] {
        assert!(archive.windows(marker.len()).any(|window| window == marker));
    }
    assert!(!archive
        .windows(b"Reparsed title".len())
        .any(|window| window == b"Reparsed title"));
}
