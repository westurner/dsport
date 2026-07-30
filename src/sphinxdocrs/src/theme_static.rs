//! `sphinxdocrs::theme_static` — copy theme static assets from the installed
//! Sphinx / theme packages at build time.
//!
//! Instead of vendoring (forking) theme CSS/JS into sphinxdocrs, this module
//! locates the **active theme's static directories from the Sphinx source**
//! (and any external theme package such as `alabaster`) via a small PyO3
//! bootstrap, then:
//!
//! 1. Copies verbatim static files (`.js`, `.png`, `.svg`, plain `.css`) into
//!    `outdir/_static/`.
//! 2. Renders Jinja2 template assets (`*.jinja`, `*_t` — e.g. `basic.css.jinja`,
//!    `alabaster.css_t`, `documentation_options.js.jinja`) through the
//!    [`jinja2rs`] engine, stripping the template suffix.
//! 3. Copies the search-language stemmer JS (`base-stemmer.js`,
//!    `<lang>-stemmer.js`) from `sphinx/search/minified-js/`.
//! 4. Generates `pygments.css` via Pygments.
//!
//! Theme resolution (inheritance chain, `[options]`) is done in Python because
//! it must import theme packages and parse `theme.conf` / `theme.toml`; the
//! actual file copying and template rendering happen in Rust.
//!
//! All PyO3 work is best-effort: if Sphinx/theme packages are unavailable the
//! functions return `Ok(())` without copying, so a pure-Rust build still
//! succeeds (only the theme assets are absent).

use std::collections::BTreeMap;
use std::path::Path;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::config::SphinxConfig;

/// One theme in the resolved inheritance chain.
#[derive(Debug, Clone)]
struct ThemeLayer {
    /// Absolute path to this theme's `static/` directory (may not exist).
    static_dir: String,
}

/// Result of resolving a theme: static dirs (base-first) + merged options.
#[derive(Debug, Default)]
struct ResolvedTheme {
    layers: Vec<ThemeLayer>,
    /// Merged `theme_<option>` → value map (child overrides parent).
    options: BTreeMap<String, String>,
    /// Directory holding the minified stemmer JS (`sphinx/search/minified-js`).
    search_js_dir: String,
    /// Pygments stylesheet text from the PyO3 fallback (empty when unused).
    pygments_css: String,
    /// Pygments style name used by the theme (informational).
    #[allow(dead_code)]
    pygments_style: String,
    /// Theme directories (not `static/` subdirs), child-first — the order
    /// real Jinja2 template loading searches in (H6c/H6b), so a child
    /// theme's own template overrides its base's while unmodified
    /// templates fall through to the base.
    template_dirs: Vec<String>,
}

/// Python bootstrap that resolves a theme's inheritance chain, merged options,
/// the stemmer-JS directory, and generates the Pygments stylesheet.
///
/// Returns a JSON-ish `dict` extracted into [`ResolvedTheme`].
const RESOLVE_PY: &str = r#"
import os, json, configparser

def _locate_theme_dir(name, theme_path_dirs):
    # `html_theme_path` — project-local theme directories (e.g. sphinx/doc's
    # `_themes/sphinx13`) take priority, matching `Theme.load_extra_theme`'s
    # `html_theme_path` handling: a project can shadow a builtin theme name.
    for base in theme_path_dirs:
        candidate = os.path.join(base, name)
        if os.path.isdir(candidate):
            return candidate
    import sphinx
    builtin = os.path.join(os.path.dirname(sphinx.__file__), 'themes', name)
    if os.path.isdir(builtin):
        return builtin
    # External theme via entry points.
    try:
        from importlib.metadata import entry_points
        for ep in entry_points(group='sphinx.html_themes'):
            if ep.name == name:
                mod = ep.load()
                # Convention: module exposes get_path() returning the parent dir.
                if hasattr(mod, 'get_path'):
                    return os.path.join(mod.get_path(), name)
                return os.path.dirname(mod.__file__)
    except Exception:
        pass
    return None

def _parse_conf(theme_dir):
    inherit = None
    options = {}
    conf = os.path.join(theme_dir, 'theme.conf')
    toml = os.path.join(theme_dir, 'theme.toml')
    if os.path.isfile(conf):
        cp = configparser.RawConfigParser()
        cp.read(conf)
        if cp.has_option('theme', 'inherit'):
            inherit = cp.get('theme', 'inherit')
        if cp.has_section('options'):
            for k, v in cp.items('options'):
                options[k] = v
        pyg = cp.get('theme', 'pygments_style', fallback=None) if cp.has_section('theme') else None
    elif os.path.isfile(toml):
        try:
            import tomllib
            with open(toml, 'rb') as f:
                data = tomllib.load(f)
            theme = data.get('theme', {})
            inherit = theme.get('inherit')
            if isinstance(inherit, str) and inherit == 'none':
                inherit = None
            options = {str(k): str(v) for k, v in data.get('options', {}).items()}
            pyg = theme.get('pygments_style')
            if isinstance(pyg, dict):
                pyg = pyg.get('default')
        except Exception:
            pyg = None
    else:
        pyg = None
    if inherit == 'none':
        inherit = None
    return inherit, options, pyg

def resolve(theme_name, theme_path_dirs):
    chain = []
    seen = set()
    name = theme_name
    pygments_style = None
    while name and name not in seen:
        seen.add(name)
        d = _locate_theme_dir(name, theme_path_dirs)
        if not d:
            break
        inherit, options, pyg = _parse_conf(d)
        if pyg and not pygments_style:
            pygments_style = pyg
        chain.append({'static': os.path.join(d, 'static'), 'options': options, 'dir': d})
        name = inherit
    chain.reverse()  # base-first
    merged = {}
    for c in chain:
        merged.update(c['options'])
    static_dirs = [c['static'] for c in chain]
    # Child-first (the order templates should be searched in, so a child
    # theme's own template overrides its base's, while unmodified templates
    # fall through to the base): reverse the now-base-first chain back.
    template_dirs = [c['dir'] for c in reversed(chain)]

    # Stemmer JS dir.
    import sphinx
    search_js = os.path.join(os.path.dirname(sphinx.__file__), 'search', 'minified-js')

    # Pygments stylesheet — FALLBACK only.  The primary path renders this from
    # the native `pygmentsrs` crate; this Python output is used only if the
    # native generator yields an empty string.
    pyg_css = ''
    try:
        from pygments.formatters import HtmlFormatter
        style = pygments_style or 'default'
        try:
            pyg_css = HtmlFormatter(style=style).get_style_defs('.highlight')
        except Exception:
            pyg_css = HtmlFormatter(style='default').get_style_defs('.highlight')
    except Exception:
        pyg_css = ''

    return {
        'static_dirs': static_dirs,
        'template_dirs': template_dirs,
        'options': {f'theme_{k}': v for k, v in merged.items()},
        'search_js_dir': search_js,
        'pygments_css': pyg_css,
        'pygments_style': pygments_style or 'default',
    }
"#;

/// Resolve the active theme via PyO3. `theme_path_dirs` are absolute
/// directories (already resolved against `confdir`) from the project's
/// `html_theme_path` config value, searched before builtin/entry-point
/// themes. Returns `None` if Python or Sphinx is unavailable (build
/// proceeds without theme assets).
fn resolve_theme(theme_name: &str, theme_path_dirs: &[String]) -> Option<ResolvedTheme> {
    Python::attach(|py| -> PyResult<ResolvedTheme> {
        let globals = PyDict::new(py);
        py.run(
            &std::ffi::CString::new(RESOLVE_PY).unwrap(),
            Some(&globals),
            None,
        )?;
        let resolve = globals.get_item("resolve")?.unwrap();
        let result = resolve.call1((theme_name, theme_path_dirs.to_vec()))?;
        let dict = result.cast::<PyDict>()?;

        let static_dirs: Vec<String> = dict
            .get_item("static_dirs")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();
        let template_dirs: Vec<String> = dict
            .get_item("template_dirs")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();
        let options: BTreeMap<String, String> = dict
            .get_item("options")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();
        let search_js_dir: String = dict
            .get_item("search_js_dir")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();
        let pygments_css: String = dict
            .get_item("pygments_css")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();
        let pygments_style: String = dict
            .get_item("pygments_style")?
            .map(|v| v.extract())
            .transpose()?
            .unwrap_or_default();

        Ok(ResolvedTheme {
            layers: static_dirs
                .into_iter()
                .map(|static_dir| ThemeLayer { static_dir })
                .collect(),
            options,
            search_js_dir,
            pygments_css,
            pygments_style,
            template_dirs,
        })
    })
    .ok()
}

/// Resolve the real theme's template directory chain (child-first) for
/// [`crate::theme_render`] to load actual Sphinx theme templates through
/// `jinja2rs`, plus its merged `theme_<option>` map. `confdir` and
/// `theme_path` mirror `html_theme_path` config entries (resolved relative
/// to `confdir`) so project-local themes (e.g. sphinx/doc's `_themes/
/// sphinx13`) can be located, not just builtin/installed ones. Returns
/// `None` if Python/Sphinx is unavailable or the theme can't be located —
/// callers should fall back to the embedded placeholder theme in that case.
pub fn resolve_theme_templates(
    theme_name: &str,
    confdir: &Path,
    theme_path: &[String],
) -> Option<(Vec<std::path::PathBuf>, BTreeMap<String, String>)> {
    let theme_path_dirs = resolve_theme_path_dirs(confdir, theme_path);
    let theme = resolve_theme(theme_name, &theme_path_dirs)?;
    if theme.template_dirs.is_empty() {
        return None;
    }
    Some((
        theme
            .template_dirs
            .into_iter()
            .map(std::path::PathBuf::from)
            .collect(),
        theme.options,
    ))
}

/// Resolve `html_theme_path` entries (relative to `confdir`) into absolute,
/// existing-directory path strings for the embedded Python resolver.
/// Non-existent entries are skipped with a warning (matching
/// `copy_html_static_path`'s handling of missing `html_static_path` dirs).
fn resolve_theme_path_dirs(confdir: &Path, theme_path: &[String]) -> Vec<String> {
    theme_path
        .iter()
        .filter_map(|entry| {
            let dir = confdir.join(entry);
            if dir.is_dir() {
                Some(dir.to_string_lossy().into_owned())
            } else {
                eprintln!("Warning: html_theme_path entry {:?} does not exist", dir.display());
                None
            }
        })
        .collect()
}

/// Return `true` for asset file names that are Jinja2 templates.
fn is_template_asset(name: &str) -> bool {
    name.ends_with(".jinja") || name.ends_with("_t")
}

/// Strip a template suffix (`.jinja` or `_t`) from `name`.
fn strip_template_suffix(name: &str) -> String {
    if let Some(stripped) = name.strip_suffix(".jinja") {
        stripped.to_owned()
    } else if let Some(stripped) = name.strip_suffix("_t") {
        stripped.to_owned()
    } else {
        name.to_owned()
    }
}

/// Build the render context for theme templates: merged `theme_*` options plus
/// the standard variables Sphinx exposes to static templates.
fn build_render_context(
    config: &SphinxConfig,
    theme: &ResolvedTheme,
) -> BTreeMap<String, serde_json::Value> {
    use serde_json::Value;
    let mut ctx: BTreeMap<String, Value> = BTreeMap::new();
    for (k, v) in &theme.options {
        ctx.insert(k.clone(), Value::String(v.clone()));
    }
    ctx.insert("release".into(), Value::String(config.release()));
    ctx.insert("version".into(), Value::String(config.version()));
    ctx.insert("language".into(), Value::String(config.language()));
    ctx.insert("builder".into(), Value::String("html".into()));
    ctx.insert("file_suffix".into(), Value::String(".html".into()));
    ctx.insert("link_suffix".into(), Value::String(".html".into()));
    ctx.insert("sourcelink_suffix".into(), Value::String(".txt".into()));
    ctx.insert("has_source".into(), Value::Bool(true));
    ctx.insert("show_search_summary".into(), Value::Bool(true));
    ctx
}

/// Copy (and render) all static assets of the active theme into
/// `outdir/_static/`, plus stemmer JS and `pygments.css`.
///
/// `confdir` is the project's source/config directory, used (together with
/// `config.html_theme_path()`) to locate project-local themes such as
/// sphinx/doc's own `_themes/sphinx13`.
///
/// Best-effort: returns `Ok(())` even when Python/Sphinx are unavailable.
pub fn copy_theme_static_files(
    config: &SphinxConfig,
    outdir: &Path,
    confdir: &Path,
) -> std::io::Result<()> {
    let theme_name = config.html_theme();
    let theme_path_dirs = resolve_theme_path_dirs(confdir, &config.html_theme_path());
    let Some(theme) = resolve_theme(&theme_name, &theme_path_dirs) else {
        // No Python/Sphinx — skip theme assets silently.
        return Ok(());
    };

    let static_out = outdir.join("_static");
    std::fs::create_dir_all(&static_out)?;

    // Render context + jinja2rs environment (reused for every template asset).
    let ctx = build_render_context(config, &theme);
    let renderer = ThemeAssetRenderer::new();

    // Copy each theme layer's static dir (base-first so child files override).
    for layer in &theme.layers {
        let dir = Path::new(&layer.static_dir);
        if !dir.is_dir() {
            continue;
        }
        copy_static_dir(dir, &static_out, &renderer, &ctx)?;
    }

    // Stemmer JS: base-stemmer.js is always needed; english-stemmer.js covers
    // the default English search language.
    let search_dir = Path::new(&theme.search_js_dir);
    for js in ["base-stemmer.js", "english-stemmer.js"] {
        let from = search_dir.join(js);
        if from.is_file() {
            let _ = std::fs::copy(&from, static_out.join(js));
        }
    }

    // Pygments stylesheet — render from the native `pygmentsrs` crate; fall
    // back to the Python-generated CSS only if the native output is empty.
    let pygments_css = pygmentsrs::formatters::html::css_style_defs(".highlight");
    let pygments_css = if pygments_css.trim().is_empty() {
        theme.pygments_css.clone()
    } else {
        pygments_css
    };
    if !pygments_css.is_empty() {
        std::fs::write(static_out.join("pygments.css"), pygments_css.as_bytes())?;
    }

    Ok(())
}

/// Recursively copy a theme `static/` directory into `dst`, rendering template
/// assets through `renderer`.
fn copy_static_dir(
    src: &Path,
    dst: &Path,
    renderer: &ThemeAssetRenderer,
    ctx: &BTreeMap<String, serde_json::Value>,
) -> std::io::Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let from = entry.path();
        if from.is_dir() {
            let sub = dst.join(&name);
            std::fs::create_dir_all(&sub)?;
            copy_static_dir(&from, &sub, renderer, ctx)?;
            continue;
        }
        if is_template_asset(&name) {
            let out_name = strip_template_suffix(&name);
            let raw = std::fs::read_to_string(&from).unwrap_or_default();
            let rendered = renderer.render(&raw, ctx).unwrap_or_else(|_| {
                // Fall back to the raw template text so the filename still
                // exists; better a slightly-templated file than a missing one.
                raw.clone()
            });
            std::fs::write(dst.join(out_name), rendered.as_bytes())?;
        } else {
            std::fs::copy(&from, dst.join(&name))?;
        }
    }
    Ok(())
}

/// Thin wrapper around a `jinja2rs::Environment` for rendering theme asset
/// templates (CSS/JS) from source strings.
struct ThemeAssetRenderer {
    env: jinja2rs::Environment,
}

impl ThemeAssetRenderer {
    fn new() -> Self {
        let mut env = jinja2rs::Environment::new();
        // Sphinx theme templates use the `tobool` filter.  jinja2rs'
        // `add_filter` requires a `Value`-returning filter, so wrap the
        // bool-returning `tobool` in a closure.
        env.add_filter("tobool", |v: minijinja::Value| {
            minijinja::Value::from(jinja2rs::filters::tobool(v))
        });
        Self { env }
    }

    /// Render a template `source` with `ctx`.
    fn render(
        &self,
        source: &str,
        ctx: &BTreeMap<String, serde_json::Value>,
    ) -> Result<String, jinja2rs::Jinja2Error> {
        self.env.render_str(source, ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn template_suffix_detection() {
        assert!(is_template_asset("basic.css.jinja"));
        assert!(is_template_asset("alabaster.css_t"));
        assert!(!is_template_asset("doctools.js"));
        assert!(!is_template_asset("custom.css"));
    }

    #[test]
    fn strip_suffix_variants() {
        assert_eq!(strip_template_suffix("basic.css.jinja"), "basic.css");
        assert_eq!(strip_template_suffix("alabaster.css_t"), "alabaster.css");
        assert_eq!(strip_template_suffix("plain.css"), "plain.css");
    }

    #[test]
    fn renderer_substitutes_theme_options() {
        let r = ThemeAssetRenderer::new();
        let mut ctx: BTreeMap<String, serde_json::Value> = BTreeMap::new();
        ctx.insert(
            "theme_link".into(),
            serde_json::Value::String("#004B6B".into()),
        );
        let out = r.render("a { color: {{ theme_link }}; }", &ctx).unwrap();
        assert_eq!(out, "a { color: #004B6B; }");
    }

    #[test]
    fn renderer_handles_tobool_filter() {
        let r = ThemeAssetRenderer::new();
        let mut ctx: BTreeMap<String, serde_json::Value> = BTreeMap::new();
        ctx.insert(
            "theme_navigation_with_keys".into(),
            serde_json::Value::String("true".into()),
        );
        let out = r
            .render(
                "X: {{ 'true' if theme_navigation_with_keys|tobool else 'false' }}",
                &ctx,
            )
            .unwrap();
        assert_eq!(out, "X: true");
    }

    #[test]
    fn copy_is_ok_even_without_theme() {
        // With an obviously-nonexistent theme name, resolution may still find
        // basic via inheritance=None; the call must not error regardless.
        use tempfile::TempDir;
        let out = TempDir::new().unwrap();
        let mut raw = std::collections::HashMap::new();
        raw.insert(
            "html_theme".to_string(),
            crate::config::ConfigVal::Str("nonexistent_theme_xyz".into()),
        );
        let cfg = SphinxConfig::new(raw, std::collections::HashMap::new());
        // Must not panic or error.
        copy_theme_static_files(&cfg, out.path(), out.path()).unwrap();
    }
}
