use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::texinfo::TexinfoBuilder;
use sphinxdocrs::config::{ConfigVal, SphinxConfig};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_writes_texinfo_documents() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(
        src.path().join("index.rst"),
        "Title\n=====\n\n.. toctree::\n   guide\n\nRoot body.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join("guide.rst"),
        "Guide\n=====\n\nChild body.\n",
    )
    .unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut config = SphinxConfig::new_defaults();
    config.set("project", ConfigVal::Str("Texinfo Project".into()));
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();

    let result = TexinfoBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 1);
    let output = std::fs::read_to_string(out.path().join("texinfoproject.texi")).unwrap();
    assert!(out.path().join("Makefile").is_file());
    assert!(output.contains("@setfilename texinfoproject.info"));
    assert!(output.contains("@direntry"));
    assert!(output.contains("@copying\nTexinfo Project\n\nAuthor name not set\n@end copying"));
    assert!(output.contains("Root body."));
    assert!(output.contains("Child body."));
    assert!(output.find("Root body.").unwrap() < output.find("Child body.").unwrap());
    assert!(output.ends_with("@bye\n"));
}

#[test]
fn build_all_uses_texinfo_documents_targets_and_respects_empty_list() {
    let src = TempDir::new().unwrap();
    let out = TempDir::new().unwrap();
    let doctrees = TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
    let mut config = SphinxConfig::new_defaults();
    config.set(
        "texinfo_documents",
        ConfigVal::List(vec![ConfigVal::List(vec![
            ConfigVal::Str("index".into()),
            ConfigVal::Str("custom-target".into()),
            ConfigVal::Str("Custom @Title {Guide}".into()),
            ConfigVal::Str("Ada {Lovelace} @".into()),
            ConfigVal::Str("Custom @Entry".into()),
            ConfigVal::Str("Custom @description".into()),
            ConfigVal::Str("Custom @category".into()),
        ])]),
    );
    let mut env = BuildEnvironment::new(config, project, src.path(), doctrees.path());
    env.find_files().unwrap();
    env.read_all().unwrap();
    let result = TexinfoBuilder::new()
        .build_all(src.path(), out.path(), &env)
        .unwrap();
    assert_eq!(result.written, 1);
    let output = std::fs::read_to_string(out.path().join("custom-target.texi")).unwrap();
    assert!(output.contains("@settitle Custom @@Title @{Guide@}"));
    assert!(
        output.contains("@copying\nCustom @@Title @{Guide@}\n\nAda @{Lovelace@} @@\n@end copying")
    );
    assert!(output.contains("@titlepage\n@title Custom @@Title @{Guide@}\n@insertcopying"));
    assert!(output.contains("* Custom @@Entry: (custom-target). Custom @@description"));
    assert!(output.contains("@dircategory Custom @@category"));

    let empty_out = TempDir::new().unwrap();
    env.config.set("texinfo_documents", ConfigVal::List(vec![]));
    let empty_result = TexinfoBuilder::new()
        .build_all(src.path(), empty_out.path(), &env)
        .unwrap();
    assert_eq!(empty_result.written, 0);
    assert!(empty_out.path().join("Makefile").is_file());
    assert!(!empty_out.path().join("custom-target.texi").exists());
}
