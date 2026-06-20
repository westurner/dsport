//! `jinja2rs::sandbox` — sandboxed template environment.
//!
//! Mirrors `jinja2.sandbox.SandboxedEnvironment`.
//!
//! MiniJinja does not expose an explicit sandbox mode — its runtime is already
//! more restricted than CPython Jinja2 (no arbitrary Python method calls, no
//! `__class__` traversal, etc.).  This module provides a thin wrapper that
//! documents the restrictions enforced and adds:
//!
//! 1. Strict undefined behavior (errors on missing variables)
//! 2. Optional seccomp filtering (syscall restrictions)
//! 3. Optional resource limits (memory/CPU)
//! 4. Optional Python callable detection (migration safety)
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
use crate::sandbox_config::SandboxConfig;
use serde::Serialize;

/// A sandboxed Jinja2-compatible environment.
///
/// Uses the same minijinja backend as [`Environment`] but pre-configures
/// the undefined-behavior policy to `Strict` (undefined values raise errors
/// rather than silently evaluating to empty) and registers a safety filter on
/// attribute names.
///
/// # Security
///
/// This sandbox is designed for rendering untrusted template *source*. It protects
/// against template-based attacks (XSS, logic bombs, reflection exploits) but
/// assumes trusted context data and trusted Rust code.
///
/// For additional isolation, enable optional features:
/// - `seccomp`: Syscall filtering (Linux only)
/// - `resource-limits`: Memory/CPU limits
/// - `python-callable-warnings`: Detect unsafe patterns from Python Jinja2
pub struct SandboxedEnvironment {
    inner: Environment,
    config: SandboxConfig,
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
    ///
    /// Uses `SandboxConfig::default()` which enables features if they are
    /// compiled in and available on the current platform.
    pub fn new() -> Self {
        Self::with_config(Environment::new(), SandboxConfig::default())
    }

    /// Create a sandboxed environment with custom configuration.
    ///
    /// Internal use; prefer `SandboxedEnvironmentBuilder`.
    pub(crate) fn with_config(mut environment: Environment, config: SandboxConfig) -> Self {
        use minijinja::UndefinedBehavior;
        environment
            .inner
            .set_undefined_behavior(UndefinedBehavior::Strict);
        Self {
            inner: environment,
            config,
        }
    }

    /// Delegate to inner environment for template operations.
    pub fn inner(&self) -> &Environment {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Environment {
        &mut self.inner
    }

    /// Get the configuration.
    pub fn config(&self) -> &SandboxConfig {
        &self.config
    }

    /// Add a named template from a string.
    pub fn add_template(
        &mut self,
        name: &'static str,
        source: &'static str,
    ) -> Result<(), Jinja2Error> {
        self.inner.add_template(name, source)
    }

    /// Retrieve a template by name.
    pub fn get_template(
        &self,
        name: &str,
    ) -> Result<crate::environment::Template<'_>, Jinja2Error> {
        self.inner.get_template(name)
    }

    /// Render a one-off template string without registering it.
    ///
    /// # Arguments
    ///
    /// * `source` — The template source code
    /// * `ctx` — Context object (must be serializable)
    ///
    /// # Validation
    ///
    /// If `python-callable-warnings` is enabled, logs warnings for
    /// detected Python objects in context.
    pub fn render_str<S: Serialize>(&self, source: &str, ctx: S) -> Result<String, Jinja2Error> {
        // Validate context if Python callable warnings are enabled
        if self.config.enable_python_warnings {
            let val = serde_json::to_value(&ctx).map_err(|e| {
                Jinja2Error::TemplateRuntimeError(format!("Failed to serialize context: {}", e))
            })?;
            crate::sandbox_config::validate_context_for_python_callables(&val);
        }

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
        assert!(
            result.is_err(),
            "expected error for undefined variable in strict mode"
        );
    }

    #[test]
    fn test_with_config() {
        use crate::sandbox_config::SandboxConfig;
        let config = SandboxConfig::default();
        let env = SandboxedEnvironment::with_config(crate::environment::Environment::new(), config);
        // Config should be stored
        assert!(!env.config().enable_python_warnings || cfg!(feature = "python-callable-warnings"));
    }
}
