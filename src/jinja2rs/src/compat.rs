//! `jinja2rs::compat` — Jinja2 vs minijinja compatibility modes.
//!
//! This module provides configuration for two compatibility modes:
//!
//! - **Jinja2 mode** (default): Drop-in compatible with Python Jinja2. Enables
//!   Python method syntax like `obj.items()`, `obj.values()`, `dict.get()`,
//!   `str.upper()`, etc. via the `minijinja-contrib` pycompat module.
//!
//! - **minijinja mode**: Uses minijinja's native filter-based approach.
//!   Methods like `.items()` are not available; use `|items` filter instead.
//!   This mode has lower overhead and is more explicit.

/// Compatibility mode configuration for the template environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatMode {
    /// Jinja2 compatibility mode (default).
    ///
    /// Enables Python method syntax via the `minijinja-contrib` pycompat module:
    /// - `dict.items()`, `dict.values()`, `dict.keys()`, `dict.get()`
    /// - `str.upper()`, `str.lower()`, `str.split()`, `str.format()`, etc.
    /// - `list.count()`
    ///
    /// This makes templates written for Python Jinja2 work without modification.
    Jinja2,

    /// minijinja compatibility mode (strict).
    ///
    /// Uses minijinja's native approach with no Python method support.
    /// Methods are not available; use filters instead:
    /// - Use `|items`, `|values`, `|keys` filters instead of `.items()`, etc.
    /// - Use `|upper`, `|lower` instead of `.upper()`, etc.
    ///
    /// This mode is more efficient and encourages explicit filter-based syntax.
    Minijinja,
}

impl Default for CompatMode {
    /// Default compatibility mode is Jinja2 (for maximum compatibility).
    fn default() -> Self {
        CompatMode::Jinja2
    }
}

impl CompatMode {
    /// Returns true if this is Jinja2 compatibility mode.
    pub fn is_jinja2(&self) -> bool {
        *self == CompatMode::Jinja2
    }

    /// Returns true if this is minijinja compatibility mode.
    pub fn is_minijinja(&self) -> bool {
        *self == CompatMode::Minijinja
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_jinja2() {
        assert_eq!(CompatMode::default(), CompatMode::Jinja2);
    }

    #[test]
    fn test_is_jinja2() {
        assert!(CompatMode::Jinja2.is_jinja2());
        assert!(!CompatMode::Minijinja.is_jinja2());
    }

    #[test]
    fn test_is_minijinja() {
        assert!(CompatMode::Minijinja.is_minijinja());
        assert!(!CompatMode::Jinja2.is_minijinja());
    }
}
