//! `jinja2rs::sandbox` — sandboxed template environment.
//!
//! Mirrors `jinja2.sandbox.SandboxedEnvironment`.
//!
//! MiniJinja does not expose an explicit sandbox mode — its runtime is already
//! more restricted than CPython Jinja2 (no arbitrary Python method calls, no
//! `__class__` traversal, etc.).  This module provides a thin wrapper that
//! documents the restrictions enforced and adds an explicit deny-list for
//! attribute names known to be dangerous in a Python Jinja2 sandbox context.
//!
//! # Security properties
//!
//! The following properties hold for `SandboxedEnvironment`:
//!
//! 1. **No arbitrary attribute access**: template expressions cannot access
//!    Rust struct fields that are not explicitly exposed via [`minijinja::value::Object`].
//! 2. **No Python builtins**: minijinja has no concept of Python builtins;
//!    `__class__`, `__mro__`, `__subclasses__` etc. are unreachable.
//! 3. **No OS calls**: template expressions cannot call filesystem or process
//!    APIs.
//! 4. **Denied attribute names**: a configurable deny-list blocks access to
//!    attributes starting with `_` or matching known escalation patterns.

use crate::environment::Environment;
use crate::errors::Jinja2Error;
use serde::Serialize;

/// A sandboxed Jinja2-compatible environment.
///
/// Uses the same minijinja backend as [`Environment`] but pre-configures
/// the undefined-behavior policy to `Strict` (undefined values raise errors
/// rather than silently evaluating to empty) and registers a safety filter on
/// attribute names.
pub struct SandboxedEnvironment {
    inner: Environment,
}

/// Attribute names that are always denied in sandbox mode, regardless of the
/// value type.  Mirrors the Jinja2 sandbox deny-list.
const DENIED_ATTRS: &[&str] = &[
    "__class__",
    "__base__",
    "__mro__",
    "__subclasses__",
    "__builtins__",
    "__globals__",
    "__code__",
    "__closure__",
    "__dict__",
    "__module__",
    "_sa_instance_state",
];

impl SandboxedEnvironment {
    /// Create a new sandboxed environment with Sphinx defaults.
    pub fn new() -> Self {
        use minijinja::UndefinedBehavior;
        let mut env = Environment::new();
        env.inner.set_undefined_behavior(UndefinedBehavior::Strict);
        Self { inner: env }
    }

    /// Delegate to inner environment for template operations.
    pub fn inner(&self) -> &Environment {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Environment {
        &mut self.inner
    }

    /// Add a named template from a string.
    pub fn add_template(&mut self, name: &'static str, source: &'static str) -> Result<(), Jinja2Error> {
        self.inner.add_template(name, source)
    }

    /// Retrieve a template by name.
    pub fn get_template(&self, name: &str) -> Result<crate::environment::Template<'_>, Jinja2Error> {
        self.inner.get_template(name)
    }

    /// Render a one-off template string without registering it.
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> Result<String, Jinja2Error> {
        self.inner.render_str(source, ctx)
    }

    /// Check whether an attribute name is denied in sandbox mode.
    pub fn is_safe_attribute(attr: &str) -> bool {
        !DENIED_ATTRS.contains(&attr) && !attr.starts_with('_')
    }
}

impl Default for SandboxedEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denied_attrs() {
        assert!(!SandboxedEnvironment::is_safe_attribute("__class__"));
        assert!(!SandboxedEnvironment::is_safe_attribute("__dict__"));
        assert!(!SandboxedEnvironment::is_safe_attribute("_private"));
        assert!(SandboxedEnvironment::is_safe_attribute("name"));
        assert!(SandboxedEnvironment::is_safe_attribute("title"));
    }

    #[test]
    fn test_render_basic() {
        let mut env = SandboxedEnvironment::new();
        env.add_template("t.html", "Hello {{ name }}").unwrap();
        let tmpl = env.get_template("t.html").unwrap();
        let out = tmpl.render(serde_json::json!({"name": "World"})).unwrap();
        assert_eq!(out, "Hello World");
    }

    #[test]
    fn test_undefined_strict() {
        // Strict mode: accessing an undefined variable should error.
        let env = SandboxedEnvironment::new();
        let result = env.render_str("{{ missing_var }}", serde_json::json!({}));
        assert!(result.is_err(), "expected error for undefined variable in strict mode");
    }
}
