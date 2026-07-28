//! `docutilsrs::roles` — Rust port of `docutils.parsers.rst.roles`.
//!
//! Upstream keeps two module-level dicts that map interpreted-text role names
//! onto role *functions*:
//!
//! - `_role_registry` — canonical name → role function
//! - `_roles` — local / language-dependent name → role function
//!
//! `docutilsrs` renders roles by name inside the parser rather than through
//! callables, so both registries store the **canonical role name** instead of
//! a function. That keeps the lookup semantics of
//! `roles.role(name, language, lineno, reporter)` — including the
//! resolve-then-cache behaviour — while leaving node construction where it
//! already lives.
//!
//! ## Ported surface
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `DEFAULT_INTERPRETED_ROLE` | [`DEFAULT_INTERPRETED_ROLE`] | `"title-reference"` |
//! | `languages.en.roles` | [`EN_ROLE_ALIASES`] | English name → canonical name |
//! | `_role_registry` | [`register_canonical_role`] | canonical names the parser can render |
//! | `_roles` | [`register_local_role`] / [`unregister_role`] | local aliases and the default role |
//! | `role()` | [`role`] | lookup with English fallback and caching |
//! | `DefaultRole` directive | [`set_default_role`] | also reachable via `.. default-role::` |
//!
//! ## Accepted deviations
//!
//! - Roles are canonical *names*, not callables, so
//!   `register_local_role('x', my_python_fn)` has no equivalent; custom
//!   rendering still goes through the parser's role dispatch.
//! - `set_implicit_options` has no analogue: role options are not modelled.
//! - The registries are process-global, exactly as upstream's module dicts
//!   are. Tests that mutate them must call [`reset_roles`] and cannot run in
//!   parallel with other role-sensitive tests.

use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, OnceLock};

/// Canonical name of the role used for interpreted text with no explicit role.
///
/// Mirrors `roles.DEFAULT_INTERPRETED_ROLE`.
pub const DEFAULT_INTERPRETED_ROLE: &str = "title-reference";

/// English role names mapped to canonical role names.
///
/// Exact mirror of the `roles` dict in
/// `docutils/parsers/rst/languages/en.py`, including the entries upstream
/// marks as "not implemented in Docutils".
pub const EN_ROLE_ALIASES: &[(&str, &str)] = &[
    ("abbreviation", "abbreviation"),
    ("ab", "abbreviation"),
    ("acronym", "acronym"),
    ("ac", "acronym"),
    ("code", "code"),
    ("emphasis", "emphasis"),
    ("literal", "literal"),
    ("math", "math"),
    ("pep-reference", "pep-reference"),
    ("pep", "pep-reference"),
    ("rfc-reference", "rfc-reference"),
    ("rfc", "rfc-reference"),
    ("strong", "strong"),
    ("subscript", "subscript"),
    ("sub", "subscript"),
    ("superscript", "superscript"),
    ("sup", "superscript"),
    ("title-reference", "title-reference"),
    ("title", "title-reference"),
    ("t", "title-reference"),
    ("raw", "raw"),
    // The following roles are not implemented in Docutils itself.
    ("index", "index"),
    ("i", "index"),
    ("anonymous-reference", "anonymous-reference"),
    ("citation-reference", "citation-reference"),
    ("footnote-reference", "footnote-reference"),
    ("named-reference", "named-reference"),
    ("substitution-reference", "substitution-reference"),
    ("uri-reference", "uri-reference"),
    ("uri", "uri-reference"),
    ("url", "uri-reference"),
    ("target", "target"),
];

/// Canonical roles registered by default.
///
/// Upstream builds this set through `register_generic_role` calls plus the
/// hand-written role functions at the bottom of `roles.py`.
pub const BUILTIN_CANONICAL_ROLES: &[&str] = &[
    "abbreviation",
    "acronym",
    "code",
    "emphasis",
    "literal",
    "math",
    "pep-reference",
    "raw",
    "rfc-reference",
    "strong",
    "subscript",
    "superscript",
    "title-reference",
];

/// `_role_registry`: canonical role names the parser knows how to render.
fn canonical_registry() -> &'static Mutex<HashSet<String>> {
    static REGISTRY: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        Mutex::new(
            BUILTIN_CANONICAL_ROLES
                .iter()
                .map(|name| (*name).to_owned())
                .collect(),
        )
    })
}

/// `_roles`: local / language-dependent names resolved to canonical names.
///
/// The empty key is the default role, exactly as upstream's `_roles['']`.
fn local_registry() -> &'static Mutex<HashMap<String, String>> {
    static ROLES: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
    ROLES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Resolve a role name to its canonical name.
///
/// Mirrors `roles.role()`: consult `_roles` first, fall back to the English
/// alias table, and only succeed if the resulting canonical name is present in
/// `_role_registry`. A successful lookup is cached back into `_roles`, as
/// upstream does. An empty `name` resolves the *default* role.
///
/// Returns `None` when the role is unknown, which the caller reports.
pub fn role(name: &str) -> Option<String> {
    let normname = name.to_lowercase();

    if let Some(canonical) = local_registry().lock().unwrap().get(&normname) {
        return Some(canonical.clone());
    }

    let canonical = if name.is_empty() {
        DEFAULT_INTERPRETED_ROLE.to_owned()
    } else {
        EN_ROLE_ALIASES
            .iter()
            .find(|(alias, _)| *alias == normname)
            .map_or_else(
                // "Trying <name> as canonical role name."
                || normname.clone(),
                |(_, canonical)| (*canonical).to_owned(),
            )
    };

    if canonical_registry().lock().unwrap().contains(&canonical) {
        // Upstream caches the resolution with `register_local_role`.
        local_registry()
            .lock()
            .unwrap()
            .insert(normname, canonical.clone());
        Some(canonical)
    } else {
        None
    }
}

/// Register `name` as a renderable canonical role.
///
/// Mirrors `roles.register_canonical_role`.
pub fn register_canonical_role(name: &str) {
    canonical_registry()
        .lock()
        .unwrap()
        .insert(name.to_lowercase());
}

/// Map a local or language-dependent `name` onto a canonical role.
///
/// Mirrors `roles.register_local_role`. Pass `""` as `name` to set the default
/// role, which is what `sphinx.util.rst.default_role` does.
pub fn register_local_role(name: &str, canonical: &str) {
    local_registry()
        .lock()
        .unwrap()
        .insert(name.to_lowercase(), canonical.to_lowercase());
}

/// Remove a local role mapping. Mirrors `del roles._roles[name]`.
pub fn unregister_role(name: &str) {
    local_registry()
        .lock()
        .unwrap()
        .remove(&name.to_lowercase());
}

/// Set (or with `None`, clear) the default interpreted-text role.
///
/// Mirrors the `.. default-role::` directive: an unknown role name is
/// rejected, leaving the previous default in place.
pub fn set_default_role(name: Option<&str>) -> Result<(), UnknownRole> {
    match name {
        None => {
            unregister_role("");
            Ok(())
        }
        Some(name) => {
            let canonical = role(name).ok_or_else(|| UnknownRole(name.to_owned()))?;
            register_local_role("", &canonical);
            Ok(())
        }
    }
}

/// The canonical name of the role currently used for bare interpreted text.
pub fn default_role() -> String {
    role("").unwrap_or_else(|| DEFAULT_INTERPRETED_ROLE.to_owned())
}

/// Restore both registries to their initial state.
///
/// Upstream has no equivalent — its dicts live for the process — but tests
/// need it because the registries are global.
pub fn reset_roles() {
    local_registry().lock().unwrap().clear();
    let mut canonical = canonical_registry().lock().unwrap();
    canonical.clear();
    canonical.extend(BUILTIN_CANONICAL_ROLES.iter().map(|n| (*n).to_owned()));
}

/// Returned when a role name cannot be resolved to a registered canonical role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownRole(pub String);

impl std::fmt::Display for UnknownRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown interpreted text role \"{}\".", self.0)
    }
}

impl std::error::Error for UnknownRole {}

/// Restore the "default" default role, as `Parser.parse` does once a document
/// has been parsed.
pub(crate) fn restore_default_role() {
    unregister_role("");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The registries are process-global, so every assertion that mutates them
    /// lives in one test to stay isolated from the parallel test runner.
    #[test]
    fn registry_lookup_matches_upstream_semantics() {
        reset_roles();

        // English aliases resolve to canonical names.
        assert_eq!(role("t").as_deref(), Some("title-reference"));
        assert_eq!(role("TITLE").as_deref(), Some("title-reference"));
        assert_eq!(role("sub").as_deref(), Some("subscript"));
        // A canonical name resolves to itself.
        assert_eq!(role("emphasis").as_deref(), Some("emphasis"));
        // Aliases upstream lists but does not implement are not renderable.
        assert_eq!(role("uri"), None);
        assert_eq!(role("nosuchrole"), None);

        // Empty name is the default role.
        assert_eq!(role("").as_deref(), Some(DEFAULT_INTERPRETED_ROLE));
        assert_eq!(default_role(), "title-reference");

        // `.. default-role:: emphasis`
        assert_eq!(set_default_role(Some("emphasis")), Ok(()));
        assert_eq!(default_role(), "emphasis");
        // Aliases are accepted and stored canonically.
        assert_eq!(set_default_role(Some("sub")), Ok(()));
        assert_eq!(default_role(), "subscript");
        // An unknown role leaves the previous default untouched.
        assert_eq!(
            set_default_role(Some("bogus")),
            Err(UnknownRole("bogus".to_owned()))
        );
        assert_eq!(default_role(), "subscript");
        // `.. default-role::` with no argument restores the built-in default.
        assert_eq!(set_default_role(None), Ok(()));
        assert_eq!(default_role(), "title-reference");

        // A locally registered alias wins over the English table.
        register_local_role("t", "strong");
        assert_eq!(role("t").as_deref(), Some("strong"));

        // New canonical roles become resolvable.
        assert_eq!(role("custom"), None);
        register_canonical_role("custom");
        assert_eq!(role("custom").as_deref(), Some("custom"));

        reset_roles();
        assert_eq!(role("t").as_deref(), Some("title-reference"));
    }

    #[test]
    fn unknown_role_error_matches_upstream_wording() {
        assert_eq!(
            UnknownRole("foo".to_owned()).to_string(),
            "Unknown interpreted text role \"foo\"."
        );
    }
}
