//! Tests for `sphinxdocrs::config::Config` (conf.py reader + math
//! renderer selection).
//!
//! These tests exercise the Rust API directly; the PyO3-exposed
//! `read_conf_py` function is a thin shim.

use std::io::Write;

use sphinxdocrs::config::{Config, DEFAULT_MATHJAX_PATH, MathRenderer};

fn write_conf(name: &str, body: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("sphinxdocrs-config-tests");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{name}.py"));
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(body.as_bytes()).unwrap();
    path
}

#[test]
fn defaults_match_sphinx() {
    let cfg = Config::defaults();
    assert_eq!(cfg.mathjax_path, DEFAULT_MATHJAX_PATH);
    assert_eq!(cfg.imgmath_image_format, "png");
    assert_eq!(cfg.imgmath_latex, "latex");
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::MathJax);
}

#[test]
fn empty_conf_uses_defaults() {
    let path = write_conf("empty", "");
    let cfg = Config::from_conf_py(&path).unwrap();
    assert!(cfg.extensions.is_empty());
    assert_eq!(cfg.mathjax_path, DEFAULT_MATHJAX_PATH);
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::MathJax);
}

#[test]
fn extensions_list_selects_mathjax() {
    let path = write_conf(
        "mathjax",
        "extensions = ['sphinx.ext.autodoc', 'sphinx.ext.mathjax']\n",
    );
    let cfg = Config::from_conf_py(&path).unwrap();
    assert_eq!(cfg.extensions.len(), 2);
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::MathJax);
}

#[test]
fn imgmath_extension_selects_imgmath_and_reads_options() {
    let body = r#"
extensions = ['sphinx.ext.imgmath']
imgmath_image_format = 'svg'
imgmath_latex = '/usr/bin/latex'
imgmath_dvisvgm = '/usr/bin/dvisvgm'
"#;
    let path = write_conf("imgmath", body);
    let cfg = Config::from_conf_py(&path).unwrap();
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::ImgMath);
    assert_eq!(cfg.imgmath_image_format, "svg");
    assert_eq!(cfg.imgmath_latex, "/usr/bin/latex");
    assert_eq!(cfg.imgmath_dvisvgm, "/usr/bin/dvisvgm");
}

#[test]
fn explicit_math_renderer_overrides_extensions() {
    let body = r#"
extensions = ['sphinx.ext.mathjax']
math_renderer = 'ratex'
"#;
    let path = write_conf("override", body);
    let cfg = Config::from_conf_py(&path).unwrap();
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::Ratex);
}

#[test]
fn mathjax_path_and_options_are_read() {
    let body = r#"
extensions = ['sphinx.ext.mathjax']
mathjax_path = 'https://example.com/mathjax.js'
mathjax_options = {'async': 'async', 'integrity': 'sha384-xxx'}
"#;
    let path = write_conf("mathjax_opts", body);
    let cfg = Config::from_conf_py(&path).unwrap();
    assert_eq!(cfg.mathjax_path, "https://example.com/mathjax.js");
    assert_eq!(
        cfg.mathjax_options.get("async").map(String::as_str),
        Some("async")
    );
    assert_eq!(
        cfg.mathjax_options.get("integrity").map(String::as_str),
        Some("sha384-xxx")
    );
}

#[test]
fn dsport_ratex_extension_selects_ratex() {
    let path = write_conf("ratex_ext", "extensions = ['dsport.ext.ratex']\n");
    let cfg = Config::from_conf_py(&path).unwrap();
    assert_eq!(cfg.effective_math_renderer(), MathRenderer::Ratex);
}

#[test]
fn invalid_math_renderer_errors() {
    let path = write_conf("bad_renderer", "math_renderer = 'nope'\n");
    let err = Config::from_conf_py(&path).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("math_renderer"), "msg: {msg}");
}

#[test]
fn syntax_error_in_conf_is_config_error() {
    let path = write_conf("broken", "extensions = [\n");
    let err = Config::from_conf_py(&path).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("conf.py failed"), "msg: {msg}");
}

// ═════════════════════════════════════════════════════════════════════════════
// SphinxConfig — full Config port tests
// Mirrors sphinx/tests/test_config/test_config.py
// ═════════════════════════════════════════════════════════════════════════════

use rstest::*;
use std::collections::HashMap;

use sphinxdocrs::config::{ConfigVal, RebuildKind, SphinxConfig, raw_config_from_conf_py};
use sphinxdocrs::environment::{BuildEnvironment, EnvProject};

// ── defaults ──────────────────────────────────────────────────────────────────

#[test]
fn sphinx_config_default_project() {
    let cfg = SphinxConfig::new_defaults();
    assert_eq!(cfg.project(), "Project name not set");
}

#[test]
fn sphinx_config_default_language() {
    let cfg = SphinxConfig::new_defaults();
    assert_eq!(cfg.language(), "en");
}

#[test]
fn sphinx_config_default_root_doc() {
    let cfg = SphinxConfig::new_defaults();
    assert_eq!(cfg.root_doc(), "index");
}

#[test]
fn sphinx_config_default_locale_dirs() {
    let cfg = SphinxConfig::new_defaults();
    let val = cfg.get("locale_dirs").unwrap();
    if let ConfigVal::List(items) = val {
        assert_eq!(items[0].as_str(), Some("locales"));
    } else {
        panic!("expected List");
    }
}

#[test]
fn sphinx_config_default_trim_footnote_false() {
    let cfg = SphinxConfig::new_defaults();
    assert_eq!(
        cfg.get("trim_footnote_reference_space")
            .and_then(|v| v.as_bool()),
        Some(false)
    );
}

#[test]
fn sphinx_config_contains_known() {
    let cfg = SphinxConfig::new_defaults();
    assert!(cfg.contains("project"));
    assert!(!cfg.contains("nonexisting_value"));
}

// ── overrides ─────────────────────────────────────────────────────────────────

#[test]
fn sphinx_config_override_root_doc() {
    let mut overrides = HashMap::new();
    overrides.insert("root_doc".into(), "root".into());
    let cfg = SphinxConfig::new(HashMap::new(), overrides);
    assert_eq!(cfg.root_doc(), "root");
}

#[test]
fn sphinx_config_override_csv_list() {
    let mut overrides = HashMap::new();
    overrides.insert("modindex_common_prefix".into(), "path1,path2".into());
    let cfg = SphinxConfig::new(HashMap::new(), overrides);
    if let ConfigVal::List(items) = cfg.get("modindex_common_prefix").unwrap() {
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].as_str(), Some("path1"));
    } else {
        panic!("expected List");
    }
}

#[test]
fn sphinx_config_override_bool_one() {
    let mut overrides = HashMap::new();
    overrides.insert("nitpicky".into(), "1".into());
    let cfg = SphinxConfig::new(HashMap::new(), overrides);
    assert!(cfg.nitpicky());
}

// ── add / duplicate ────────────────────────────────────────────────────────────

#[test]
fn sphinx_config_add_option_succeeds() {
    let mut cfg = SphinxConfig::new_defaults();
    cfg.add("myext_opt", ConfigVal::Bool(false), RebuildKind::Env, "")
        .unwrap();
    assert!(cfg.contains("myext_opt"));
}

#[test]
fn sphinx_config_add_duplicate_errors() {
    let mut cfg = SphinxConfig::new_defaults();
    let err = cfg
        .add(
            "project",
            ConfigVal::Str(String::new()),
            RebuildKind::None,
            "",
        )
        .unwrap_err();
    assert!(err.contains("already present"));
}

// ── set / alias ────────────────────────────────────────────────────────────────

#[test]
fn sphinx_config_set_syncs_alias() {
    let mut cfg = SphinxConfig::new_defaults();
    cfg.set("master_doc", ConfigVal::Str("contents".into()));
    assert_eq!(cfg.root_doc(), "contents");
}

#[test]
fn source_suffix_string_md_discovers_markdown_as_restructuredtext() {
    let conf = write_conf(
        "source_suffix_md_string",
        "source_suffix = '.md'\nproject = 'Markdown string'\n",
    );
    let config = SphinxConfig::new(raw_config_from_conf_py(&conf).unwrap(), HashMap::new());
    assert_eq!(
        config.source_suffix().get(".md").map(String::as_str),
        Some("restructuredtext")
    );

    let src = tempfile::TempDir::new().unwrap();
    std::fs::write(src.path().join("index.md"), "Title\n=====\n\nBody.\n").unwrap();
    let project = EnvProject::new(src.path(), &[(".md", "restructuredtext")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), src.path().join(".doctrees"));
    env.find_files().unwrap();

    assert!(env.found_docs().contains("index"));
    assert_eq!(env.doc2path("index"), src.path().join("index.md"));
}

#[test]
fn source_suffix_dict_md_selects_myst_and_renders_with_myst_md_rs() {
    let conf = write_conf(
        "source_suffix_md_dict",
        "source_suffix = {'.rst': 'restructuredtext', '.md': 'myst'}\n",
    );
    let config = SphinxConfig::new(raw_config_from_conf_py(&conf).unwrap(), HashMap::new());
    let suffixes = config.source_suffix();
    assert_eq!(
        suffixes.get(".rst").map(String::as_str),
        Some("restructuredtext")
    );
    assert_eq!(suffixes.get(".md").map(String::as_str), Some("myst"));

    let source = "# MyST title\n\nA **bold** paragraph.\n\n:::note\nA note.\n:::\n";
    let rendered = myst_md_rs::render_html(source);
    assert!(rendered.contains("<h1>MyST title</h1>"));
    assert!(rendered.contains("<strong>bold</strong>"));
    assert!(rendered.contains(r#"class="myst-directive""#));

    let src = tempfile::TempDir::new().unwrap();
    std::fs::write(src.path().join("index.rst"), "Index\n=====\n").unwrap();
    std::fs::write(src.path().join("guide.md"), source).unwrap();
    let project = EnvProject::new(src.path(), &[(".rst", "restructuredtext"), (".md", "myst")]);
    let mut env = BuildEnvironment::new(config, project, src.path(), src.path().join(".doctrees"));
    env.find_files().unwrap();

    assert!(env.found_docs().contains("index"));
    assert!(env.found_docs().contains("guide"));
    assert_eq!(env.doc2path("guide"), src.path().join("guide.md"));
}

// ── filter ────────────────────────────────────────────────────────────────────

#[test]
fn sphinx_config_filter_env_excludes_none_rebuild() {
    let cfg = SphinxConfig::new_defaults();
    let env_names: Vec<_> = cfg.filter(&RebuildKind::Env).map(|cv| cv.name).collect();
    assert!(!env_names.contains(&"needs_sphinx".to_string()));
    assert!(env_names.contains(&"project".to_string()));
}

// ── ConfigVal display ─────────────────────────────────────────────────────────

#[rstest]
#[case(ConfigVal::Bool(true), "True")]
#[case(ConfigVal::Bool(false), "False")]
#[case(ConfigVal::Null, "None")]
#[case(ConfigVal::Int(7), "7")]
fn sphinx_config_val_display(#[case] val: ConfigVal, #[case] expected: &str) {
    assert_eq!(val.display(), expected);
}
