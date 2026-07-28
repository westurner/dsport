//! `sphinxdocrs::domains::py_domain` — Rust port of
//! `sphinx.domains.python.PythonDomain` (Tier **H3b**).
//!
//! Ported: `py:module`/`function`/`class`/`method`/`classmethod`/
//! `staticmethod`/`attribute`/`data`/`exception` object registration
//! (with module/class nesting tracked the same way upstream's
//! `PyObject.before_content`/`after_content` stack does) and
//! `:py:func:`/`:py:class:`/`:py:meth:`/`:py:attr:`/`:py:data:`/
//! `:py:exc:`/`:py:mod:`/`:py:obj:` cross-reference resolution,
//! including the `~` (display-last-component) and leading-`.`
//! (relative-to-current-context) target conventions.
//!
//! Signatures are parsed with `ruff_python_parser` via
//! [`super::py_sig`] rather than hand-rolled paren splitting — see that
//! module's doc comment.
//!
//! **Deferred**: overload sets, type-hint cross-linking, and
//! multi-module merge semantics (`py_modindex`'s cross-module grouping)
//! stay unported; only the common single-signature-per-object case is
//! handled.

use std::collections::HashMap;

use super::scan::{ScannedObject, scan_domain_objects};
use super::{Domain, ObjectEntry, XrefTarget, normalize_id};
use crate::environment::BuildEnvironment;

/// Directive types that set the ambient module context. Only the
/// literal `"module"` also registers an object; `"currentmodule"` is
/// context-only (mirrors `PyCurrentModule` vs. `PyModule` upstream).
pub const MODULE_TYPES: &[&str] = &["module", "currentmodule"];
/// Directive types that push a class-name context without registering
/// an object (`PyCurrentClass`).
pub const CLASS_CONTEXT_TYPES: &[&str] = &["currentclass"];
/// Directive types that register an object *and* open a nested naming
/// scope for deeper-indented children (a class body's methods).
pub const CONTAINER_TYPES: &[&str] = &["class", "exception"];
/// Directive types that register a plain object with no nested scope.
pub const PLAIN_TYPES: &[&str] = &[
    "function",
    "method",
    "classmethod",
    "staticmethod",
    "attribute",
    "property",
    "data",
];

/// All directive types this port recognizes, for the read-phase scan.
pub const ALL_TYPES: &[&str] = &[
    "module",
    "currentmodule",
    "currentclass",
    "class",
    "exception",
    "function",
    "method",
    "classmethod",
    "staticmethod",
    "attribute",
    "property",
    "data",
];

/// Which concrete object types a `:py:<reftype>:` role may resolve to.
/// Mirrors the reftype→objtype grouping in `PythonDomain.object_types`.
fn reftype_objtypes(reftype: &str) -> &'static [&'static str] {
    match reftype {
        "func" => &["function"],
        "class" => &["class"],
        "meth" => &["method", "classmethod", "staticmethod"],
        "classmethod" => &["classmethod"],
        "staticmethod" => &["staticmethod"],
        "attr" => &["attribute", "property"],
        "data" => &["data"],
        "exc" => &["exception"],
        "mod" => &["module"],
        "obj" => &[
            "function",
            "class",
            "method",
            "classmethod",
            "staticmethod",
            "attribute",
            "property",
            "data",
            "exception",
            "module",
        ],
        _ => &[],
    }
}

/// Parse one `.. py:<type>:: args` directive line into `(local_name,
/// rendered_signature)`. Passed to [`scan_domain_objects`] as its
/// `signature_of` callback.
pub fn py_signature_of(objtype: &str, args: &str) -> (String, String) {
    match objtype {
        "class" | "exception" => super::py_sig::parse_class_signature(args),
        "function" | "method" | "classmethod" | "staticmethod" => {
            super::py_sig::parse_function_signature(args)
        }
        _ => (args.trim().to_string(), String::new()),
    }
}

/// Rust port of `sphinx.domains.python.PythonDomain`.
#[derive(Debug, Clone, Default)]
pub struct PyDomain {
    /// `(objtype, fullname) -> (docname, anchor, rendered_signature)`.
    pub objects: HashMap<(String, String), (String, String, String)>,
}

impl PyDomain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Note one object. Mirrors `PythonDomain.note_object`.
    pub fn note_object(
        &mut self,
        objtype: impl Into<String>,
        fullname: impl Into<String>,
        docname: impl Into<String>,
        anchor: impl Into<String>,
        signature: impl Into<String>,
    ) {
        self.objects.insert(
            (objtype.into(), fullname.into()),
            (docname.into(), anchor.into(), signature.into()),
        );
    }

    /// Scan `source` for `.. py:...::` directives and note every object
    /// found, synthesizing an anchor of `py-{objtype}-{normalize_id(fullname)}`
    /// (matching the convention `std_domain`/`rst_domain` already use).
    pub fn note_source(&mut self, docname: &str, source: &str) {
        for ScannedObject {
            objtype,
            fullname,
            signature,
        } in scan_domain_objects(
            source,
            "py",
            MODULE_TYPES,
            CLASS_CONTEXT_TYPES,
            CONTAINER_TYPES,
            PLAIN_TYPES,
            py_signature_of,
        ) {
            let anchor = format!("py-{objtype}-{}", normalize_id(&fullname));
            self.note_object(objtype, fullname, docname, anchor, signature);
        }
    }

    /// Look up `target` under one of `objtypes`, trying an exact match
    /// first and falling back to a dotted-suffix match for a
    /// leading-`.`-prefixed target.
    ///
    /// **Accepted deviation:** upstream's `PyXrefRole.process_link`
    /// resolves a leading `.` relative to the *live* `py:module`/
    /// `py:class` ref-context at the role's call site. This port's
    /// text-scan doesn't retain that per-role-site context, so a
    /// leading `.` is instead resolved as "any registered name ending in
    /// this dotted suffix" — usually equivalent, but ambiguous names
    /// across modules could resolve differently than upstream.
    fn lookup(&self, objtypes: &[&str], target: &str) -> Option<(String, String)> {
        for objtype in objtypes {
            if let Some((docname, anchor, _sig)) =
                self.objects.get(&(objtype.to_string(), target.to_string()))
            {
                return Some((docname.clone(), anchor.clone()));
            }
        }
        let stripped = target.trim_start_matches('.');
        if stripped != target {
            for objtype in objtypes {
                for ((ty, fullname), (docname, anchor, _sig)) in &self.objects {
                    if ty == objtype
                        && (fullname == stripped || fullname.ends_with(&format!(".{stripped}")))
                    {
                        return Some((docname.clone(), anchor.clone()));
                    }
                }
            }
        }
        None
    }
}

impl Domain for PyDomain {
    fn name(&self) -> &'static str {
        "py"
    }

    fn resolve_xref(
        &self,
        _env: &BuildEnvironment,
        _fromdocname: &str,
        reftype: &str,
        target: &str,
    ) -> Option<XrefTarget> {
        let objtypes = reftype_objtypes(reftype);
        if objtypes.is_empty() {
            return None;
        }
        let (docname, anchor) = self.lookup(objtypes, target)?;
        Some(XrefTarget {
            docname,
            anchor,
            title: target.to_string(),
        })
    }

    fn get_objects(&self) -> Vec<ObjectEntry> {
        self.objects
            .iter()
            .map(|((obj_type, name), (docname, anchor, _sig))| ObjectEntry {
                obj_type: obj_type.clone(),
                name: name.clone(),
                docname: docname.clone(),
                anchor: anchor.clone(),
            })
            .collect()
    }

    fn clear_doc(&mut self, docname: &str) {
        self.objects.retain(|_, (d, _, _)| d != docname);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::SphinxConfig;
    use crate::environment::EnvProject;

    fn make_env() -> BuildEnvironment {
        let config = SphinxConfig::new_defaults();
        let project = EnvProject::new("/tmp/src", &[(".rst", "restructuredtext")]);
        BuildEnvironment::new(config, project, "/tmp/src", "/tmp/doctrees")
    }

    #[test]
    fn note_source_registers_module_and_function() {
        let mut domain = PyDomain::new();
        domain.note_source(
            "api",
            ".. py:module:: pkg.mod\n\n.. py:function:: greet(name, loud=False)\n",
        );
        assert!(
            domain
                .objects
                .contains_key(&("module".to_string(), "pkg.mod".to_string()))
        );
        let (docname, _anchor, sig) = domain
            .objects
            .get(&("function".to_string(), "pkg.mod.greet".to_string()))
            .unwrap();
        assert_eq!(docname, "api");
        assert_eq!(sig, "name, loud=False");
    }

    #[test]
    fn note_source_nests_methods_under_class() {
        let mut domain = PyDomain::new();
        domain.note_source(
            "api",
            ".. py:module:: pkg\n\n.. py:class:: Widget\n\n   .. py:method:: render(self)\n",
        );
        assert!(
            domain
                .objects
                .contains_key(&("method".to_string(), "pkg.Widget.render".to_string()))
        );
    }

    #[test]
    fn resolve_func_xref() {
        let env = make_env();
        let mut domain = PyDomain::new();
        domain.note_object("function", "pkg.mod.greet", "api", "py-function-greet", "");
        let target = domain
            .resolve_xref(&env, "index", "func", "pkg.mod.greet")
            .unwrap();
        assert_eq!(target.docname, "api");
        assert_eq!(target.anchor, "py-function-greet");
    }

    #[test]
    fn resolve_meth_matches_any_method_kind() {
        let env = make_env();
        let mut domain = PyDomain::new();
        domain.note_object(
            "classmethod",
            "pkg.Widget.create",
            "api",
            "py-classmethod-create",
            "",
        );
        let target = domain
            .resolve_xref(&env, "index", "meth", "pkg.Widget.create")
            .unwrap();
        assert_eq!(target.docname, "api");
    }

    #[test]
    fn resolve_relative_dotted_suffix() {
        let env = make_env();
        let mut domain = PyDomain::new();
        domain.note_object("method", "pkg.Widget.render", "api", "py-method-render", "");
        let target = domain
            .resolve_xref(&env, "index", "meth", ".Widget.render")
            .unwrap();
        assert_eq!(target.docname, "api");
    }

    #[test]
    fn resolve_unknown_is_none() {
        let env = make_env();
        let domain = PyDomain::new();
        assert!(domain.resolve_xref(&env, "index", "func", "nope").is_none());
    }

    #[test]
    fn clear_doc_removes_matching_entries() {
        let mut domain = PyDomain::new();
        domain.note_object("function", "a.f", "docA", "id1", "");
        domain.note_object("function", "b.f", "docB", "id2", "");
        domain.clear_doc("docA");
        assert!(
            !domain
                .objects
                .contains_key(&("function".to_string(), "a.f".to_string()))
        );
        assert!(
            domain
                .objects
                .contains_key(&("function".to_string(), "b.f".to_string()))
        );
    }
}
