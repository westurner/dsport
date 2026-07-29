//! `sphinxdocrs::domains` — Rust port of `sphinx.domains` (Tier H3).
//!
//! Each domain owns a slice of cross-reference-able objects (labels,
//! terms, directive/role descriptions, ...) and knows how to resolve a
//! `reftype`/`target` pair recovered from an xref role into a concrete
//! document location.
//!
//! ## What is ported
//!
//! | upstream | Rust target | notes |
//! | --- | --- | --- |
//! | `sphinx.domains.Domain` (shape) | [`Domain`] trait | documents the contract; `BuildEnvironment` holds concrete fields rather than a `dyn Domain` registry |
//! | `sphinx.domains.std.StandardDomain` | [`StdDomain`] | **H3a** — labels, `:ref:`/`:doc:`/`:term:`/`:numref:`/`:keyword:`, glossary |
//! | `sphinx.domains.rst.ReSTDomain` | [`RstDomain`] | **H3c** — `rst:directive` / `rst:role` object descriptions |
//! | `sphinx.domains.python.PythonDomain` | [`py_domain::PyDomain`] | **H3b** — `py:module`/`function`/`class`/`method`/`attribute`/`data` |
//! | `sphinx.domains.javascript.JavaScriptDomain` | [`js_domain::JsDomain`] | **H3e** — `js:function`/`class`/`method`/`data`/`module` |
//! | `sphinx.util.nodes.docname_join` | [`docname_join`] | resolve a `:doc:` target relative to the referring document |
//! | `sphinx.util.nodes.process_index_entry` | [`scan::scan_index_entries`] | `.. index::` entry splitting (**H3d**/**H5e** input) |
//!
//! ## Accepted deviation (H5b)
//!
//! `docutilsrs`'s parser does not yet produce a structural `pending_xref`
//! node (`sphinx.addnodes.pending_xref`) — its role dispatch
//! (`emit_role` in `docutilsrs::parser`) only knows about plain docutils
//! roles. Rather than widen `docutilsrs::doctree::NodeKind` (a
//! cross-cutting change touching every writer), cross-references are
//! recovered with a text-level scan of the RST source
//! ([`scan::scan_xref_roles`]), mirroring the precedent set by
//! `environment::scan_toctree_entries` for **H2c**. Real AST-based xref
//! nodes are deferred until `docutilsrs` grows a structural role
//! registry.
//!
//! **Deferred** (not yet ported): `c`/`cpp` domains (**H3f** —
//! recorded as a **keep-python** decision, see the port plan §3),
//! `:option:`/`:token:`/`:productionlist:` resolution, `numfig`-aware
//! `:numref:` title formatting.

pub mod js_domain;
pub mod py_domain;
mod py_sig;
pub mod rst_domain;
pub mod scan;
pub mod std_domain;

pub use js_domain::JsDomain;
pub use py_domain::PyDomain;
pub use rst_domain::RstDomain;
pub use std_domain::StdDomain;

use serde::{Deserialize, Serialize};

use crate::environment::BuildEnvironment;

// ── (de)serialization helper for tuple-keyed maps (H8b) ──────────────────

/// `serde(with = ...)` helper for `HashMap<(String, String), V>` fields
/// (every domain's `objects` map): `serde_json` cannot serialize a map
/// whose keys aren't strings, so this round-trips the map through a
/// `Vec<((String, String), V)>` instead. Used by
/// [`std_domain::StdDomain`]/[`rst_domain::RstDomain`]/
/// [`py_domain::PyDomain`]/[`js_domain::JsDomain`] so `BuildEnvironment`
/// persistence (**H8b**) can derive `Serialize`/`Deserialize` directly on
/// the live domain structs instead of maintaining a parallel snapshot type.
pub mod tuple_key_map {
    use std::collections::HashMap;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, V>(
        map: &HashMap<(String, String), V>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        V: Serialize,
    {
        let entries: Vec<(&(String, String), &V)> = map.iter().collect();
        entries.serialize(serializer)
    }

    pub fn deserialize<'de, D, V>(deserializer: D) -> Result<HashMap<(String, String), V>, D::Error>
    where
        D: Deserializer<'de>,
        V: Deserialize<'de>,
    {
        let entries: Vec<((String, String), V)> = Vec::deserialize(deserializer)?;
        Ok(entries.into_iter().collect())
    }
}

// ── shared types ──────────────────────────────────────────────────────────────

/// Where a resolved cross-reference points.
///
/// Mirrors the `(docname, labelid, sectname)` tuples returned by
/// `StandardDomain._resolve_*_xref` before they are turned into a
/// `nodes.reference`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrefTarget {
    /// The document the reference resolves into.
    pub docname: String,
    /// The in-document anchor (empty string for a whole-document
    /// reference, e.g. `:doc:`).
    pub anchor: String,
    /// Display title to use when the role gave no explicit title.
    pub title: String,
}

/// A cross-reference recovered from an inline role during the read phase.
///
/// See the module-level accepted-deviation note for why this is a
/// text-scan result rather than a real `pending_xref` doctree node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingXref {
    /// Domain prefix (`"std"` for unprefixed roles like `:ref:`, `"rst"`
    /// for `:rst:dir:`, etc.).
    pub domain: String,
    /// Role name within the domain (`"ref"`, `"doc"`, `"term"`, `"dir"`, ...).
    pub reftype: String,
    /// The raw cross-reference target text.
    pub target: String,
    /// Explicit title given via the `` `Title <target>` `` phrase form.
    pub explicit_title: Option<String>,
    /// 1-based source line the role occurred on.
    pub line: usize,
    /// `true` when the role content had a leading `~` (e.g.
    /// `` :py:func:`~pkg.mod.func` ``), requesting that the resolved
    /// title show only the last dotted component. Mirrors the generic
    /// `~` truncation every `XRefRole` supports upstream
    /// (`sphinx.roles.XRefRole.__call__`), not just the `py` domain.
    pub shorten: bool,
}

/// Outcome of resolving one [`PendingXref`].
///
/// Mirrors `Domain.resolve_xref` returning either a `nodes.reference` or
/// `None` (in which case Sphinx logs one of `StandardDomain.dangling_warnings`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XrefResolution {
    Resolved {
        xref: PendingXref,
        target: XrefTarget,
    },
    Unresolved {
        xref: PendingXref,
        /// Warning text, mirroring `StandardDomain.dangling_warnings` /
        /// `'unknown document: %r'`-style upstream messages.
        warning: String,
    },
}

/// One object registered by a domain (`get_objects()`), consulted by
/// search-object population (**H3d**) and by per-domain indices (**H5e**).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectEntry {
    pub obj_type: String,
    pub name: String,
    pub docname: String,
    pub anchor: String,
}

/// The five `.. index::` entry forms recognized by
/// `sphinx.util.nodes.process_index_entry`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IndexEntryKind {
    Single,
    Pair,
    Triple,
    See,
    SeeAlso,
}

/// One `.. index::` entry recovered from a document (**H5e** input).
///
/// See [`scan::scan_index_entries`] for the accepted deviation (text-scan,
/// no in-page anchor tracking).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    pub kind: IndexEntryKind,
    /// The entry's display text. For `pair`/`triple` entries this is the
    /// `"; "`-joined form (`"first; second"`), matching
    /// `process_index_entry`'s own joining so up/downstream grouping code
    /// (genindex, search) doesn't need to special-case each kind.
    pub text: String,
    /// For `see`/`seealso` entries, the target term being redirected to.
    pub see_target: Option<String>,
    /// Whether this was a `!`-prefixed "main" entry.
    pub main: bool,
}

/// Shape shared by all domains (see the Tier H3 design note in
/// `docs/sphinxdocrs-port-plan.md`).
///
/// This trait documents the contract for future domains (`py`, `js`, ...);
/// it is not used for `dyn` dispatch today; [`BuildEnvironment`] holds
/// concrete [`StdDomain`]/[`RstDomain`] fields directly, the same way
/// upstream's `env.domains['std']` / `env.domains['rst']` are looked up
/// by name but implemented as concrete classes.
pub trait Domain {
    /// The domain's registration name (`"std"`, `"rst"`, `"py"`, ...).
    fn name(&self) -> &'static str;

    /// Resolve a `reftype`/`target` pair recovered from an xref role.
    /// Returns `None` when the target is unknown (a "dangling" xref).
    fn resolve_xref(
        &self,
        env: &BuildEnvironment,
        fromdocname: &str,
        reftype: &str,
        target: &str,
    ) -> Option<XrefTarget>;

    /// All objects this domain has registered, for search-index /
    /// index-page population.
    fn get_objects(&self) -> Vec<ObjectEntry>;

    /// Forget everything registered for `docname` (called before a
    /// document is re-read, mirroring `Domain.clear_doc`).
    fn clear_doc(&mut self, docname: &str);
}

// ── docname_join ──────────────────────────────────────────────────────────────

/// Resolve a `:doc:` cross-reference target relative to `base` (the
/// referring docname), mirroring `sphinx.util.osutil.docname_join`.
///
/// A target starting with `/` is absolute (relative to the source root);
/// otherwise it's resolved relative to `base`'s directory. `..`/`.`
/// components are normalized away (POSIX-style; docnames always use `/`).
///
/// ```rust
/// use sphinxdocrs::domains::docname_join;
/// assert_eq!(docname_join("guide/intro", "setup"), "guide/setup");
/// assert_eq!(docname_join("guide/intro", "/index"), "index");
/// assert_eq!(docname_join("guide/sub/page", "../other"), "guide/other");
/// ```
pub fn docname_join(base: &str, target: &str) -> String {
    if let Some(stripped) = target.strip_prefix('/') {
        return normalize_docpath(stripped);
    }
    let base_dir = match base.rfind('/') {
        Some(idx) => &base[..idx],
        None => "",
    };
    let joined = if base_dir.is_empty() {
        target.to_string()
    } else {
        format!("{base_dir}/{target}")
    };
    normalize_docpath(&joined)
}

fn normalize_docpath(path: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for comp in path.split('/') {
        match comp {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            other => stack.push(other),
        }
    }
    stack.join("/")
}

// ── normalize_id ──────────────────────────────────────────────────────────────

/// docutils-style identifier normalization: lowercase, collapse runs of
/// non-alphanumeric characters to a single `-`, trim trailing `-`.
///
/// Duplicated from `docutilsrs::parser`'s private `normalize_id` (not
/// exported) since label/term anchors need the same algorithm here.
///
/// ```rust
/// use sphinxdocrs::domains::normalize_id;
/// assert_eq!(normalize_id("My Label!"), "my-label");
/// assert_eq!(normalize_id("Über cool"), "ber-cool");
/// ```
pub fn normalize_id(name: &str) -> String {
    let lower = name.to_ascii_lowercase();
    let mut out = String::with_capacity(lower.len());
    let mut last_dash = true;
    for c in lower.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

// ── warning text ──────────────────────────────────────────────────────────────

/// Format the "dangling xref" warning for an unresolved [`PendingXref`].
///
/// Mirrors `StandardDomain.dangling_warnings` (and the `RstDomain`
/// fallback for its own roles).
pub fn dangling_warning(xref: &PendingXref) -> String {
    match (xref.domain.as_str(), xref.reftype.as_str()) {
        ("std", "term") => format!("term not in glossary: '{}'", xref.target),
        ("std", "ref" | "numref") => format!("undefined label: '{}'", xref.target),
        ("std", "keyword") => format!("unknown keyword: '{}'", xref.target),
        ("std", "doc") => format!("unknown document: '{}'", xref.target),
        ("std", "option") => format!("unknown option: '{}'", xref.target),
        ("rst", reftype) => format!("undefined rst {}: '{}'", reftype, xref.target),
        (domain, reftype) => format!(
            "{domain}:{reftype} reference target not found: '{}'",
            xref.target
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docname_join_relative() {
        assert_eq!(docname_join("guide/intro", "setup"), "guide/setup");
    }

    #[test]
    fn docname_join_absolute() {
        assert_eq!(docname_join("guide/intro", "/index"), "index");
    }

    #[test]
    fn docname_join_parent_traversal() {
        assert_eq!(docname_join("guide/sub/page", "../other"), "guide/other");
    }

    #[test]
    fn docname_join_top_level_base() {
        assert_eq!(docname_join("index", "setup"), "setup");
    }

    #[test]
    fn normalize_id_basic() {
        assert_eq!(normalize_id("My Label!"), "my-label");
        assert_eq!(normalize_id("already-slug"), "already-slug");
        assert_eq!(normalize_id("  spaced  out "), "spaced-out");
    }

    #[test]
    fn dangling_warning_text() {
        let xref = PendingXref {
            domain: "std".into(),
            reftype: "ref".into(),
            target: "missing".into(),
            explicit_title: None,
            line: 1,
            shorten: false,
        };
        assert_eq!(dangling_warning(&xref), "undefined label: 'missing'");
    }
}
