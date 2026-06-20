//! Vendored Jinja templates for `sphinx-autogen`.
//!
//! The three RST stub templates from `sphinx/ext/autosummary/templates/autosummary/`
//! are embedded at compile time.
//!
//! Extra filters registered:
//! - `underline` — appends `=` underline matching the text display-width.
//!   Mirrors `sphinx.util.rst.heading(text, 1)` (the heading filter used in
//!   autosummary templates).
//! - `escape` — HTML/RST special-char escape (minijinja built-in `e`).

use std::path::Path;

use jinja2rs::environment::Environment;
use jinja2rs::errors::Jinja2Error;

const BASE_RST: &str = include_str!("../../assets/autosummary/base.rst");
const CLASS_RST: &str = include_str!("../../assets/autosummary/class.rst");
const MODULE_RST: &str = include_str!("../../assets/autosummary/module.rst");

/// RST `underline` filter: appends an `=` underline matching display width.
///
/// Used in autosummary templates as `{{ fullname | escape | underline }}`.
fn underline_filter(text: minijinja::Value) -> minijinja::Value {
    let s = text.to_string();
    let width = unicode_width::UnicodeWidthStr::width(s.as_str());
    minijinja::Value::from(format!("{s}\n{}", "=".repeat(width)))
}

/// Template collection for autogen stub generation.
pub struct AutogenTemplates {
    env: Environment,
    /// Optional on-disk override directory.
    pub templatedir: Option<std::path::PathBuf>,
}

impl AutogenTemplates {
    /// Load embedded (vendored) templates.
    pub fn vendored() -> Self {
        let mut env = Environment::new();
        env.add_filter("underline", underline_filter);
        env.add_template("base.rst", BASE_RST)
            .expect("bundled base.rst is valid");
        env.add_template("class.rst", CLASS_RST)
            .expect("bundled class.rst is valid");
        env.add_template("module.rst", MODULE_RST)
            .expect("bundled module.rst is valid");
        Self {
            env,
            templatedir: None,
        }
    }

    /// Layer in a custom template directory.
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

    /// Render a named template.
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
