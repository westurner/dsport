//! minijinja integration for [`Markup`].
//!
//! When the `minijinja` feature is enabled, this module:
//!
//! 1. Implements [`minijinja::value::Object`] for `Markup` so that `Markup`
//!    values stored in a template context are rendered as-is (no extra escaping
//!    by the minijinja auto-escaper).
//!
//! 2. Provides [`markup_auto_escape_callback`] — a drop-in for
//!    [`minijinja::Environment::set_auto_escape_callback`] that enables HTML
//!    auto-escaping on `.html` / `.htm` / `.xml` templates.
//!
//! 3. Provides [`escape_filter`] — a minijinja filter that escapes its
//!    argument and returns a safe `Markup`-tagged value.

use minijinja::value::{Object, ObjectKind, StructObject, Value};
use minijinja::{Error, ErrorKind};
use std::fmt;
use std::sync::Arc;

use crate::escape::escape_to;
use crate::markup::Markup;

// ── Object impl ───────────────────────────────────────────────────────────────

/// Wraps a `Markup` so that minijinja treats it as pre-escaped HTML.
///
/// When a value implements the minijinja `__html__()` protocol via `Object`,
/// the auto-escaper will render it directly without further escaping.
#[derive(Debug, Clone)]
pub struct MarkupValue(pub Markup);

impl fmt::Display for MarkupValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Object for MarkupValue {
    fn kind(&self) -> ObjectKind<'_> {
        ObjectKind::Plain
    }

    fn call_method(
        self: &Arc<Self>,
        _state: &minijinja::State<'_, '_>,
        name: &str,
        _args: &[Value],
    ) -> Result<Value, Error> {
        match name {
            // Python MarkupSafe __html__() protocol — return the raw string.
            "__html__" => Ok(Value::from(self.0.as_str())),
            _ => Err(Error::new(
                ErrorKind::UnknownMethod,
                format!("Markup has no method '{}'", name),
            )),
        }
    }

    fn render(self: &Arc<Self>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // minijinja calls this when auto-escaping is active. Returning the
        // raw inner string bypasses the HTML escaper.
        f.write_str(self.0.as_str())
    }
}

/// Convert a `Markup` to a minijinja `Value` that will not be re-escaped.
///
/// ```rust,ignore
/// use markupsafers::{Markup, minijinja_compat::markup_to_value};
/// let val = markup_to_value(Markup::from_safe("<b>ok</b>"));
/// // When inserted into a minijinja context, renders as "<b>ok</b>".
/// ```
pub fn markup_to_value(m: Markup) -> Value {
    Value::from_object(MarkupValue(m))
}

// ── Auto-escape callback ──────────────────────────────────────────────────────

/// Auto-escape callback for minijinja that enables HTML escaping on `.html`,
/// `.htm`, and `.xml` templates — the same default as Python Jinja2 / MarkupSafe.
///
/// Pass to [`minijinja::Environment::set_auto_escape_callback`]:
///
/// ```rust,ignore
/// use markupsafers::minijinja_compat::markup_auto_escape_callback;
/// env.set_auto_escape_callback(markup_auto_escape_callback);
/// ```
pub fn markup_auto_escape_callback(name: &str) -> minijinja::AutoEscape {
    let ext = name.rsplit('.').next().unwrap_or("");
    match ext {
        "html" | "htm" | "xml" => minijinja::AutoEscape::Html,
        _ => minijinja::AutoEscape::None,
    }
}

// ── Filters ───────────────────────────────────────────────────────────────────

/// minijinja filter: escape a value and return a safe markup string.
///
/// Equivalent to `{{ value|e }}` / `{{ value|escape }}` in Jinja2.
///
/// Register with:
/// ```rust,ignore
/// use markupsafers::minijinja_compat::escape_filter;
/// env.add_filter("escape", escape_filter);
/// env.add_filter("e", escape_filter);
/// ```
pub fn escape_filter(val: Value) -> Value {
    // If the value is already a MarkupValue, return it unchanged.
    if let Some(obj) = val.as_object() {
        if obj.downcast_ref::<MarkupValue>().is_some() {
            return val;
        }
    }
    let s = val.to_string();
    let mut out = String::with_capacity(s.len() + 8);
    escape_to(&s, &mut out);
    markup_to_value(Markup::from_safe(out))
}

/// minijinja filter: mark a value as safe (no escaping).
///
/// Equivalent to `{{ value|safe }}` in Jinja2.
///
/// Register with:
/// ```rust,ignore
/// use markupsafers::minijinja_compat::safe_filter;
/// env.add_filter("safe", safe_filter);
/// ```
pub fn safe_filter(val: Value) -> Value {
    let s = val.to_string();
    markup_to_value(Markup::from_safe(s))
}

/// minijinja filter: force-escape a value, even if already marked safe.
///
/// Equivalent to `{{ value|forceescape }}` in Jinja2.
///
/// Register with:
/// ```rust,ignore
/// use markupsafers::minijinja_compat::force_escape_filter;
/// env.add_filter("forceescape", force_escape_filter);
/// ```
pub fn force_escape_filter(val: Value) -> Value {
    let s = val.to_string();
    let mut out = String::with_capacity(s.len() + 8);
    escape_to(&s, &mut out);
    markup_to_value(Markup::from_safe(out))
}
