use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::epub::EpubBuilder;
use sphinxdocrs::config::{ConfigVal, SphinxConfig};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

fn read_epub_entries(bytes: &[u8]) -> Vec<(String, u16, Vec<u8>)> {
    pyo3::Python::attach(|py| -> pyo3::PyResult<_> {
        use pyo3::prelude::*;
        use pyo3::types::{PyBytes, PyModule};

        let zipfile = PyModule::import(py, "zipfile")?;
        let io = PyModule::import(py, "io")?;
        let buffer = io.getattr("BytesIO")?.call1((PyBytes::new(py, bytes),))?;
        let archive = zipfile.getattr("ZipFile")?.call1((buffer,))?;
        let names: Vec<String> = archive.call_method0("namelist")?.extract()?;
        names
            .into_iter()
            .map(|name| {
                let info = archive.call_method1("getinfo", (name.as_str(),))?;
                let method = info.getattr("compress_type")?.extract()?;
                let content = archive.call_method1("read", (name.as_str(),))?.extract()?;
                Ok((name, method, content))
            })
            .collect()
    })
    .unwrap()
}

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
    config.set(
        "epub_contributor",
        ConfigVal::Str("Proofreader & Editor".into()),
    );
    config.set("epub_uid", ConfigVal::Str("urn:example:guide".into()));
    config.set("epub_version", ConfigVal::Float(3.1));
    config.set("epub_scheme", ConfigVal::Str("ISBN".into()));
    config.set(
        "epub_identifier",
        ConfigVal::Str("urn:example:publication & edition".into()),
    );
    config.set("epub_basename", ConfigVal::Str("guide-book".into()));
    config.set("epub_writing_mode", ConfigVal::Str("vertical".into()));
    config.set(
        "epub_guide",
        ConfigVal::List(vec![ConfigVal::List(vec![
            ConfigVal::Str("cover".into()),
            ConfigVal::Str("index.xhtml#cover".into()),
            ConfigVal::Str("Cover & Page".into()),
        ])]),
    );
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
    assert!(out.path().join("genindex.xhtml").is_file());
    assert!(out.path().join(".buildinfo").is_file());
    assert!(out.path().join("mimetype").is_file());
    assert!(out.path().join("META-INF/container.xml").is_file());
    assert!(out.path().join("content.opf").is_file());
    assert!(out.path().join("toc.ncx").is_file());
    assert!(out.path().join("nav.xhtml").is_file());
    assert!(out.path().join("index.xhtml").is_file());
    assert!(out.path().join("_static/basic.css").is_file());
    assert!(out.path().join("_static/epub.css").is_file());
    assert!(out.path().join("_static/file.png").is_file());
    assert!(out.path().join("_static/minus.png").is_file());
    assert!(out.path().join("_static/plus.png").is_file());
    assert!(out.path().join("_static/pygments.css").is_file());
    let epub_css = std::fs::read_to_string(out.path().join("_static/epub.css")).unwrap();
    assert!(epub_css.contains("writing-mode: vertical-rl"), "{epub_css}");
    let archive = std::fs::read(out.path().join("guide-book.epub")).unwrap();
    assert_eq!(&archive[..4], b"PK\x03\x04");
    let entries = read_epub_entries(&archive);
    assert_eq!(entries.first().unwrap().0, "mimetype");
    assert_eq!(entries.first().unwrap().1, 0);
    assert!(entries.iter().skip(1).all(|(_, method, _)| *method == 8));
    assert_eq!(entries[0].2, b"application/epub+zip");
    for marker in [
        b"content.opf".as_slice(),
        b"index.xhtml".as_slice(),
        b"nav.xhtml".as_slice(),
        b"toc.ncx".as_slice(),
        b"properties=\"nav\"".as_slice(),
        b"page-progression-direction=\"rtl\"".as_slice(),
        b"reference type=\"cover\" title=\"Cover &amp; Page\" href=\"index.xhtml#cover\""
            .as_slice(),
        b"Guide &amp; Reference".as_slice(),
        b"A &lt;Writer&gt;".as_slice(),
        b"<dc:contributor>Proofreader &amp; Editor</dc:contributor>".as_slice(),
        b"unique-identifier=\"urn:example:guide\"".as_slice(),
        b"<dc:identifier id=\"urn:example:guide\" opf:scheme=\"ISBN\">urn:example:publication &amp; edition</dc:identifier>".as_slice(),
        b"<reference type=\"toc\" title=\"Table of Contents\" href=\"index.xhtml\" />".as_slice(),
    ] {
        assert!(
            entries
                .iter()
                .any(|(_, _, contents)| contents.windows(marker.len()).any(|window| window == marker)),
            "missing EPUB marker: {}",
            String::from_utf8_lossy(marker)
        );
    }
    assert!(
        !entries
            .iter()
            .flat_map(|(_, _, contents)| contents.windows(b"Reparsed title".len()))
            .any(|window| window == b"Reparsed title")
    );
}

#[test]
fn epub_use_index_overrides_html_use_index() {
    let src = TempDir::new().unwrap();
    let out_without = TempDir::new().unwrap();
    let out_with = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut config = SphinxConfig::new_defaults();
    config.set("html_use_index", ConfigVal::Bool(false));
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    EpubBuilder::new()
        .build_all(src.path(), out_without.path(), &env)
        .unwrap();
    assert!(!out_without.path().join("genindex.xhtml").exists());

    env.config.set("epub_use_index", ConfigVal::Bool(true));
    EpubBuilder::new()
        .build_all(src.path(), out_with.path(), &env)
        .unwrap();
    assert!(out_with.path().join("genindex.xhtml").is_file());
}

#[test]
fn epub_navigation_follows_resolved_toctree_order() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Index\n=====\n\n.. toctree::\n\n   guide\n\nRoot body.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide.rst"),
        "Guide\n=====\n\nChild body.\n",
    )
    .unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut env = BuildEnvironment::new(
        SphinxConfig::new_defaults(),
        project,
        src.path(),
        doctrees.path(),
    );
    env.find_files().unwrap();
    env.read_all().unwrap();

    EpubBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    let navigation = std::fs::read_to_string(out.path().join("nav.xhtml")).unwrap();
    assert!(
        navigation.find("index.xhtml").unwrap() < navigation.find("guide.xhtml").unwrap(),
        "navigation should start at the root and follow its toctree: {navigation}"
    );
    let index = std::fs::read_to_string(out.path().join("index.xhtml")).unwrap();
    assert!(
        index.contains("href=\"guide.xhtml\""),
        "EPUB toctree link: {index}"
    );
}

#[test]
fn default_epub_basename_uses_project_filename() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut config = SphinxConfig::new_defaults();
    config.set("project", ConfigVal::Str("Parity Matrix".into()));
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    let result = EpubBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();

    assert_eq!(result.written, 1);
    assert!(out.path().join("ParityMatrix.epub").is_file());
    assert!(!out.path().join("Parity Matrix.epub").exists());
}
