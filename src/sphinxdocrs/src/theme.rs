//! `sphinxdocrs::theme` — Jinja2 theme pipeline backed by [`jinja2rs`].
//!
//! Renders each page's HTML body into a full page using real Jinja2 templates
//! (`layout.html` → `page.html`) via the `jinja2rs` / `minijinja` engine, with
//! template inheritance (`{% extends %}`), blocks (`{% block %}`), and includes
//! (`{% include %}`) — the same mechanics `sphinx.jinja2glue` uses.
//!
//! ## Design
//!
//! Rather than porting Sphinx's 200-line `basic/layout.html` (which depends on
//! ~20 live template globals such as `pathto`, `hasdoc`, `toctree`, `css_tag`),
//! this module ships a **self-contained `sphinxdocrs_basic` theme**:
//!
//! - Templates are embedded in the binary (`include_str!`-style consts) and
//!   registered with the environment via `add_template`.
//! - All page-relative paths are **precomputed in Rust** and passed as plain
//!   context strings (`pathto_root`, `css_files`, `script_files`), so no live
//!   `pathto()` global is required — rendering is deterministic and portable.
//!
//! ## What is ported
//!
//! | upstream | Rust target | notes |
//! |----------|-------------|-------|
//! | `sphinx.jinja2glue.BuiltinTemplateLoader` | [`ThemeRenderer`] | env + render |
//! | `basic/layout.html` | [`LAYOUT_HTML`] | doctype, head, body scaffold |
//! | `basic/page.html` | [`PAGE_HTML`] | `{% extends "layout.html" %}` + body block |
//! | `basic/static/basic.css` | [`THEME_CSS`] | stylesheet written to `_static/` |

use serde::Serialize;

use jinja2rs::Environment;

/// The theme's base layout template.  Provides the HTML5 document scaffold and
/// the `body` block that `page.html` fills in.
pub const LAYOUT_HTML: &str = r#"<!DOCTYPE html>
<html lang="{{ language | default('en') }}">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
<title>{% block htmltitle %}{{ title }}{% if docstitle %} &#8212; {{ docstitle }}{% endif %}{% endblock %}</title>
<meta name="generator" content="sphinxdocrs" />
{%- for css in css_files %}
<link rel="stylesheet" type="text/css" href="{{ pathto_root | safe }}{{ css | safe }}" />
{%- endfor %}
{%- for js in script_files %}
<script src="{{ pathto_root | safe }}{{ js | safe }}"></script>
{%- endfor %}
{%- block extrahead %}{% endblock %}
</head>
<body>
<div class="document">
<div class="documentwrapper">
<div class="bodywrapper">
<main class="body" role="main">
{% block body %}{% endblock %}
</main>
</div>
</div>
{%- block sidebar %}
{%- if sidebars %}
<div class="sphinxsidebar" role="navigation" aria-label="main navigation">
<div class="sphinxsidebarwrapper">
{{ sidebars | safe }}
</div>
</div>
{%- endif %}
{%- endblock %}
</div>
<div class="footer" role="contentinfo">
{%- if show_copyright and copyright %}&#169; {{ copyright }}.{% endif %}
{%- if sourcename %}
<a href="{{ pathto_root | safe }}_sources/{{ sourcename | safe }}">Page source</a>
{%- endif %}
</div>
</body>
</html>
"#;

/// The page template: extends the layout and injects the rendered body.
pub const PAGE_HTML: &str = r#"{% extends "layout.html" %}
{% block body %}{{ body | safe }}{% endblock %}
"#;

/// Minimal theme stylesheet, written to `_static/basic.css`.
pub const THEME_CSS: &str = r#"/* sphinxdocrs basic theme */
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
       margin: 0; color: #333; background: #fff; }
.document { display: flex; max-width: 1100px; margin: 0 auto; }
.documentwrapper { flex: 1 1 auto; min-width: 0; }
.bodywrapper { padding: 1em 2em; }
.sphinxsidebar { width: 230px; padding: 1em; border-left: 1px solid #eee; }
main.body { line-height: 1.6; }
h1, h2, h3, h4 { color: #1a1a2e; }
a { color: #0057a8; text-decoration: none; }
a:hover { text-decoration: underline; }
pre, code { background: #f5f5f5; border-radius: 3px;
            font-family: 'SFMono-Regular', Consolas, monospace; }
pre { padding: 0.8em; overflow-x: auto; }
code { padding: 0.1em 0.3em; }
.footer { max-width: 1100px; margin: 1em auto; padding: 1em 2em;
          border-top: 1px solid #eee; color: #888; font-size: 0.9em; }
"#;

/// Context passed to the page template for a single document.
///
/// All path fields are **relative to the current page** and already account
/// for the page's directory depth (`pathto_root`).
#[derive(Debug, Clone, Serialize)]
pub struct PageContext {
    /// Document title (from the promoted RST title).
    pub title: String,
    /// The rendered HTML body fragment.
    pub body: String,
    /// Project name (`conf.py` `project`).
    pub project: String,
    /// Documentation title shown after the page title (usually the project).
    pub docstitle: String,
    /// Copyright string (may be empty).
    pub copyright: String,
    /// Whether to show the copyright line.
    pub show_copyright: bool,
    /// Document language code.
    pub language: String,
    /// Relative path prefix from this page to the output root (e.g. `"../"`).
    pub pathto_root: String,
    /// CSS hrefs relative to the output root (joined with `pathto_root`).
    pub css_files: Vec<String>,
    /// Script hrefs relative to the output root.
    pub script_files: Vec<String>,
    /// Source file name for the "Page source" link (e.g. `index.rst.txt`),
    /// empty when source links are disabled.
    pub sourcename: String,
    /// Rendered sidebar HTML (may be empty).
    pub sidebars: String,
}

impl PageContext {
    /// Build a context with sensible defaults for `docname`, `title`, `body`.
    ///
    /// `pathto_root` is derived from the docname's directory depth.
    pub fn new(docname: &str, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            project: String::new(),
            docstitle: String::new(),
            copyright: String::new(),
            show_copyright: false,
            language: "en".into(),
            pathto_root: pathto_root(docname),
            css_files: vec!["_static/basic.css".into()],
            script_files: Vec::new(),
            sourcename: format!("{docname}.rst.txt"),
            sidebars: String::new(),
        }
    }
}

/// Compute the relative path prefix from a page to the output root.
///
/// `index` → `""`, `guide/intro` → `"../"`, `a/b/c` → `"../../"`.
pub fn pathto_root(docname: &str) -> String {
    let depth = docname.matches('/').count();
    "../".repeat(depth)
}

/// A Jinja2 theme renderer wrapping a `jinja2rs::Environment` preloaded with
/// the embedded `sphinxdocrs_basic` theme templates.
pub struct ThemeRenderer {
    env: Environment,
}

impl ThemeRenderer {
    /// Construct the renderer and register the embedded theme templates.
    ///
    /// # Errors
    /// Returns a [`jinja2rs::Jinja2Error`] if a template fails to compile.
    pub fn new() -> Result<Self, jinja2rs::Jinja2Error> {
        let mut env = Environment::new();
        env.add_template("layout.html", LAYOUT_HTML)?;
        env.add_template("page.html", PAGE_HTML)?;
        Ok(Self { env })
    }

    /// Render `page.html` with the given [`PageContext`], returning the full
    /// HTML document.
    pub fn render_page(&self, ctx: &PageContext) -> Result<String, jinja2rs::Jinja2Error> {
        self.env.get_template("page.html")?.render(ctx)
    }

    /// Render an arbitrary registered template by name.
    pub fn render<S: Serialize>(
        &self,
        template: &str,
        ctx: S,
    ) -> Result<String, jinja2rs::Jinja2Error> {
        self.env.get_template(template)?.render(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pathto_root_depths() {
        assert_eq!(pathto_root("index"), "");
        assert_eq!(pathto_root("guide/intro"), "../");
        assert_eq!(pathto_root("a/b/c"), "../../");
    }

    #[test]
    fn renders_full_document() {
        let r = ThemeRenderer::new().unwrap();
        let ctx = PageContext::new("index", "My Title", "<p>Hello <b>world</b>.</p>");
        let html = r.render_page(&ctx).unwrap();
        assert!(html.starts_with("<!DOCTYPE html>"), "got:\n{html}");
        assert!(html.contains("<title>My Title</title>"));
        assert!(html.contains("<p>Hello <b>world</b>.</p>"));
        assert!(html.contains("_static/basic.css"));
    }

    #[test]
    fn docstitle_appended_to_title() {
        let r = ThemeRenderer::new().unwrap();
        let mut ctx = PageContext::new("index", "Home", "<p>x</p>");
        ctx.docstitle = "MyProject".into();
        let html = r.render_page(&ctx).unwrap();
        assert!(
            html.contains("<title>Home &#8212; MyProject</title>"),
            "got title in:\n{}",
            &html[..html.len().min(400)]
        );
    }

    #[test]
    fn subdir_page_uses_relative_paths() {
        let r = ThemeRenderer::new().unwrap();
        let ctx = PageContext::new("guide/intro", "Intro", "<p>body</p>");
        let html = r.render_page(&ctx).unwrap();
        // CSS href must be prefixed with the relative root.
        assert!(
            html.contains("href=\"../_static/basic.css\""),
            "expected relative css path; got:\n{}",
            &html[..html.len().min(500)]
        );
        // Source link also relative.
        assert!(html.contains("href=\"../_sources/guide/intro.rst.txt\""));
    }

    #[test]
    fn body_html_not_escaped() {
        let r = ThemeRenderer::new().unwrap();
        let ctx = PageContext::new("index", "T", "<div class=\"note\">Body</div>");
        let html = r.render_page(&ctx).unwrap();
        // The body must be emitted as raw HTML, not escaped.
        assert!(html.contains("<div class=\"note\">Body</div>"));
        assert!(!html.contains("&lt;div"));
    }

    #[test]
    fn sidebar_rendered_when_present() {
        let r = ThemeRenderer::new().unwrap();
        let mut ctx = PageContext::new("index", "T", "<p>b</p>");
        ctx.sidebars = "<ul><li>nav</li></ul>".into();
        let html = r.render_page(&ctx).unwrap();
        assert!(html.contains("sphinxsidebar"));
        assert!(html.contains("<ul><li>nav</li></ul>"));
    }

    #[test]
    fn no_sidebar_when_empty() {
        let r = ThemeRenderer::new().unwrap();
        let ctx = PageContext::new("index", "T", "<p>b</p>");
        let html = r.render_page(&ctx).unwrap();
        assert!(!html.contains("sphinxsidebar"));
    }

    #[test]
    fn copyright_shown_only_when_enabled() {
        let r = ThemeRenderer::new().unwrap();
        let mut ctx = PageContext::new("index", "T", "<p>b</p>");
        ctx.copyright = "2026, Me".into();
        ctx.show_copyright = true;
        let html = r.render_page(&ctx).unwrap();
        assert!(html.contains("2026, Me"));
    }
}
