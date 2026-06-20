//! Vendored Jinja templates from `sphinx/templates/quickstart/`,
//! embedded at compile time with `include_str!`.
//!
//! The four canonical templates from upstream sphinx are stored in
//! `assets/quickstart/`. A custom `templatedir` at runtime can override
//! any individual template — matching the `_has_custom_template` /
//! `render` logic of `QuickstartRenderer`.

use std::path::Path;

use jinja2rs::environment::Environment;
use jinja2rs::errors::Jinja2Error;

/// Python-style `repr` filter for Jinja2 templates.
///
/// Mirrors Jinja2's built-in `|repr` which calls Python `repr()` on a value.
/// For strings, wraps in single-quotes, escaping backslashes and single-quotes.
fn repr_filter(value: minijinja::Value) -> minijinja::Value {
    let s = value.to_string();
    let escaped = s.replace('\\', "\\\\").replace('\'', "\\'");
    minijinja::Value::from(format!("'{escaped}'"))
}

/// The embedded Jinja template sources. Names must match upstream
/// template keys used in `generate()`.
const CONF_PY: &str = include_str!("../../assets/quickstart/conf.py.jinja");
const ROOT_DOC: &str = include_str!("../../assets/quickstart/root_doc.rst.jinja");
const MAKEFILE: &str = include_str!("../../assets/quickstart/Makefile.new.jinja");
const MAKE_BAT: &str = include_str!("../../assets/quickstart/make.bat.new.jinja");

/// Template collection used by [`super::generate::generate`].
pub struct QuickstartTemplates {
    env: Environment,
    /// Optional on-disk override dir (mirrors `QuickstartRenderer.templatedir`).
    pub templatedir: Option<std::path::PathBuf>,
}

impl QuickstartTemplates {
    /// Load the embedded (vendored) templates.
    pub fn vendored() -> Self {
        let mut env = Environment::new();
        // Register the `repr` filter — Jinja2 built-in, not in minijinja core.
        env.add_filter("repr", repr_filter);
        env.add_template("conf.py.jinja", CONF_PY)
            .expect("bundled conf.py.jinja is valid");
        env.add_template("root_doc.rst.jinja", ROOT_DOC)
            .expect("bundled root_doc.rst.jinja is valid");
        env.add_template("Makefile.new.jinja", MAKEFILE)
            .expect("bundled Makefile.new.jinja is valid");
        env.add_template("make.bat.new.jinja", MAKE_BAT)
            .expect("bundled make.bat.new.jinja is valid");
        Self {
            env,
            templatedir: None,
        }
    }

    /// Load embedded templates and optionally layer in a custom `templatedir`.
    pub fn with_templatedir(templatedir: Option<impl AsRef<Path>>) -> Self {
        let mut t = Self::vendored();
        t.templatedir = templatedir.map(|p| p.as_ref().to_path_buf());
        t
    }

    /// Check whether `templatedir` contains a custom override for the
    /// given basename. Mirrors `_has_custom_template`.
    pub fn has_custom(&self, basename: &str) -> bool {
        self.templatedir
            .as_ref()
            .map(|d| d.join(basename).is_file())
            .unwrap_or(false)
    }

    /// Render a named template with the given context. If a custom
    /// override exists on disk it is used; otherwise the embedded
    /// template is rendered.
    pub fn render<S: serde::Serialize>(&self, name: &str, ctx: &S) -> Result<String, Jinja2Error> {
        if self.has_custom(name) {
            let custom_path = self.templatedir.as_ref().unwrap().join(name);
            let src = std::fs::read_to_string(&custom_path).map_err(Jinja2Error::Io)?;
            let result = self.env.render_str(&src, ctx)?;
            Ok(result)
        } else {
            let tmpl = self.env.get_template(name)?;
            tmpl.render(ctx)
        }
    }

    /// Render a raw template string (used for `conf.py.jinja` which is
    /// read from disk or bundled via `conf_text` in upstream).
    pub fn render_str<S: serde::Serialize>(
        &self,
        src: &str,
        ctx: &S,
    ) -> Result<String, Jinja2Error> {
        self.env.render_str(src, ctx)
    }
}
