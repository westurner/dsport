//! Regression coverage for the source discovery exercised by Sphinx's
//! `test_build_all.py` fixture: configured suffixes and non-ASCII docnames.

use tempfile::TempDir;

use sphinxdocrs::builders::Builder;
use sphinxdocrs::builders::html::HtmlBuilder;
use sphinxdocrs::builders::pseudoxml::PseudoxmlBuilder;
use sphinxdocrs::builders::text::TextBuilder;
use sphinxdocrs::builders::xml::XmlBuilder;
use sphinxdocrs::config::{ConfigVal, SphinxConfig};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

#[test]
fn build_all_honors_configured_suffix_for_all_simple_builders() {
    let src = TempDir::new().unwrap();
    let unicode_docname = "日本語";
    std::fs::write(
        src.path().join("index.txt"),
        "Welcome\n=======\n\nHomepage.\n",
    )
    .unwrap();
    std::fs::write(
        src.path().join(format!("{unicode_docname}.txt")),
        "Non-ASCII page\n================\n",
    )
    .unwrap();

    let builders: [(&dyn Builder, &str); 4] = [
        (&HtmlBuilder::new(), ".html"),
        (&TextBuilder::new(), ".txt"),
        (&XmlBuilder::new(), ".xml"),
        (&PseudoxmlBuilder::new(), ".pseudoxml"),
    ];

    for (builder, output_suffix) in builders {
        let out = TempDir::new().unwrap();
        let mut config = SphinxConfig::new_defaults();
        config.set(
            "source_suffix",
            ConfigVal::List(vec![
                ConfigVal::Str(".rst".into()),
                ConfigVal::Str(".txt".into()),
            ]),
        );
        let project = EnvProject::new(src.path(), &[(".txt", "restructuredtext")]);
        let env = BuildEnvironment::new(config, project, src.path(), out.path());

        let result = builder
            .build_all(src.path(), out.path(), &env)
            .unwrap_or_else(|error| panic!("{} build failed: {error}", builder.name()));

        assert_eq!(
            result.written,
            2,
            "{} should build both documents",
            builder.name()
        );
        assert!(
            out.path().join(format!("index{output_suffix}")).exists(),
            "{} index output is missing",
            builder.name()
        );
        assert!(
            out.path()
                .join(format!("{unicode_docname}{output_suffix}"))
                .exists(),
            "{} Unicode output is missing",
            builder.name()
        );
    }
}
