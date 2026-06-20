//! Vendored Jinja templates for `sphinx-apidoc`.
//!
//! Mirrors `sphinx.ext.apidoc._generate.{create_module_file,
//! create_package_file, create_modules_toc_file}` rendering logic.
//!
//! Templates are embedded via `include_str!` from `assets/apidoc/`.
//! Two extra filters are registered:
//! - `heading` — mirrors `sphinx.util.rst.heading`: appends an RST
//!   underline using `=`, `-`, `~` at level 1, 2, 3.
//! - `e` (escape) — HTML-escape for RST headings (minijinja `e` built-in
//!   is already present; this is a no-op alias in our context).
//! - `repr` — already registered in `QuickstartTemplates`; re-registered
//!   here for the independent environment.

use std::path::Path;

use jinja2rs::environment::Environment;
use jinja2rs::errors::Jinja2Error;

const MODULE_RST: &str = include_str!("../../assets/apidoc/module.rst.jinja");
const PACKAGE_RST: &str = include_str!("../../assets/apidoc/package.rst.jinja");
const TOC_RST: &str = include_str!("../../assets/apidoc/toc.rst.jinja");

/// RST heading filter at level 1 (``=`` underline). Mirrors `sphinx.util.rst.heading`.
fn heading_filter(text: minijinja::Value) -> minijinja::Value {
    heading_at_level(&text.to_string(), 1)
}

/// RST heading filter at level 2 (``-`` underline).
fn heading2_filter(text: minijinja::Value) -> minijinja::Value {
    heading_at_level(&text.to_string(), 2)
}

fn heading_at_level(s: &str, level: u32) -> minijinja::Value {
    let width = unicode_width::UnicodeWidthStr::width(s);
    let ch = match level {
        2 => '-',
        3 => '~',
        _ => '=',
    };
    minijinja::Value::from(format!("{s}\n{}", ch.to_string().repeat(width)))
}

/// Python-style `repr` filter — same as in `quickstart::templates`.
fn repr_filter(value: minijinja::Value) -> minijinja::Value {
    let s = value.to_string();
    let escaped = s.replace('\\', "\\\\").replace('\'', "\\'");
    minijinja::Value::from(format!("'{escaped}'"))
}

/// Template collection used by apidoc's generate functions.
pub struct ApidocTemplates {
    env: Environment,
    /// Optional on-disk override directory.
    pub templatedir: Option<std::path::PathBuf>,
}

impl ApidocTemplates {
    /// Load the embedded (vendored) templates and register custom filters.
    pub fn vendored() -> Self {
        let mut env = Environment::new();
        env.add_filter("heading", heading_filter);
        env.add_filter("heading2", heading2_filter);
        env.add_filter("repr", repr_filter);
        env.add_template("module.rst.jinja", MODULE_RST)
            .expect("bundled module.rst.jinja is valid");
        env.add_template("package.rst.jinja", PACKAGE_RST)
            .expect("bundled package.rst.jinja is valid");
        env.add_template("toc.rst.jinja", TOC_RST)
            .expect("bundled toc.rst.jinja is valid");
        Self {
            env,
            templatedir: None,
        }
    }

    /// Layer in an optional custom template directory.
    pub fn with_templatedir(templatedir: Option<impl AsRef<Path>>) -> Self {
        let mut t = Self::vendored();
        t.templatedir = templatedir.map(|p| p.as_ref().to_path_buf());
        t
    }

    fn has_custom(&self, name: &str) -> bool {
        self.templatedir
            .as_ref()
            .map(|d| d.join(name).is_file())
            .unwrap_or(false)
    }

    /// Render a named template with `ctx`.
    pub fn render<S: serde::Serialize>(&self, name: &str, ctx: &S) -> Result<String, Jinja2Error> {
        if self.has_custom(name) {
            let path = self.templatedir.as_ref().unwrap().join(name);
            let src = std::fs::read_to_string(&path).map_err(Jinja2Error::Io)?;
            self.env.render_str(&src, ctx)
        } else {
            let tmpl = self.env.get_template(name)?;
            tmpl.render(ctx)
        }
    }
}
