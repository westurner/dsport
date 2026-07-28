//! `sphinxdocrs::domains::js_domain` — Rust port of
//! `sphinx.domains.javascript.JavaScriptDomain` (Tier **H3e**).
//!
//! Mechanically similar to [`super::py_domain`] but smaller: no
//! `classmethod`/`staticmethod`/`exception` object types, no
//! `currentclass` directive. Reuses the same
//! [`super::scan::scan_domain_objects`] nesting scanner, supplying a
//! manual (non-`ruff`) signature splitter since `ruff_python_parser`
//! only understands Python — JS signatures are parsed with a simple
//! paren-balance scan instead.
//!
//! **Deferred**: destructured/default-value parameter rendering beyond
//! raw text passthrough, JS-specific type annotations (JSDoc `@param`
//! comments aren't consulted — only the directive's own argument line).

use std::collections::HashMap;

use super::scan::{ScannedObject, scan_domain_objects};
use super::{Domain, ObjectEntry, XrefTarget, normalize_id};
use crate::environment::BuildEnvironment;

/// Directive types that set the ambient module context.
pub const MODULE_TYPES: &[&str] = &["module"];
/// The `js` domain has no `currentclass`-style directive.
pub const CLASS_CONTEXT_TYPES: &[&str] = &[];
/// Directive types that register an object and open a nested scope.
pub const CONTAINER_TYPES: &[&str] = &["class"];
/// Directive types that register a plain object with no nested scope.
pub const PLAIN_TYPES: &[&str] = &["function", "method", "attribute", "data"];

/// Which concrete object types a `:js:<reftype>:` role may resolve to.
fn reftype_objtypes(reftype: &str) -> &'static [&'static str] {
    match reftype {
        "func" => &["function"],
        "class" => &["class"],
        "meth" => &["method"],
        "attr" => &["attribute"],
        "data" => &["data"],
        "mod" => &["module"],
        _ => &[],
    }
}

/// Parse one `.. js:<type>:: args` directive line into `(local_name,
/// rendered_signature)` using a manual paren-balance scan (no `ruff`
/// involved — JS isn't Python).
pub fn js_signature_of(objtype: &str, args: &str) -> (String, String) {
    if !matches!(objtype, "function" | "method" | "class") {
        return (args.trim().to_string(), String::new());
    }
    split_name_and_parens(args)
}

/// Split `"name(args)"` into `(name, args)` by scanning for the first
/// `(` and its balanced matching `)`, tolerating nested parens (e.g. a
/// default value like `configure(timeout = (1000))`).
///
/// **Accepted deviation:** this is a plain bracket-balance scan, not a
/// real JS parser — string literals containing unbalanced parens (e.g.
/// `greet(msg = "(hi")`) would misparse. Good enough for the common
/// case; a real `ruff`-equivalent JS parser isn't available in this
/// workspace.
fn split_name_and_parens(raw: &str) -> (String, String) {
    let raw = raw.trim();
    let Some(open) = raw.find('(') else {
        return (raw.to_string(), String::new());
    };
    let name = raw[..open].trim().to_string();
    let bytes = raw.as_bytes();
    let mut depth = 0i32;
    let mut close = None;
    for (i, &b) in bytes.iter().enumerate().skip(open) {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(i);
                    break;
                }
            }
            _ => {}
        }
    }
    let signature = match close {
        Some(c) if c > open => raw[open + 1..c].trim().to_string(),
        _ => raw[open + 1..].trim().to_string(),
    };
    (name, signature)
}

/// Rust port of `sphinx.domains.javascript.JavaScriptDomain`.
#[derive(Debug, Clone, Default)]
pub struct JsDomain {
    /// `(objtype, fullname) -> (docname, anchor, rendered_signature)`.
    pub objects: HashMap<(String, String), (String, String, String)>,
}

impl JsDomain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Note one object. Mirrors `JavaScriptDomain.note_object`.
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

    /// Scan `source` for `.. js:...::` directives and note every object
    /// found, synthesizing an anchor of `js-{objtype}-{normalize_id(fullname)}`.
    pub fn note_source(&mut self, docname: &str, source: &str) {
        for ScannedObject {
            objtype,
            fullname,
            signature,
        } in scan_domain_objects(
            source,
            "js",
            MODULE_TYPES,
            CLASS_CONTEXT_TYPES,
            CONTAINER_TYPES,
            PLAIN_TYPES,
            js_signature_of,
        ) {
            let anchor = format!("js-{objtype}-{}", normalize_id(&fullname));
            self.note_object(objtype, fullname, docname, anchor, signature);
        }
    }

    fn lookup(&self, objtypes: &[&str], target: &str) -> Option<(String, String)> {
        for objtype in objtypes {
            if let Some((docname, anchor, _sig)) =
                self.objects.get(&(objtype.to_string(), target.to_string()))
            {
                return Some((docname.clone(), anchor.clone()));
            }
        }
        None
    }
}

impl Domain for JsDomain {
    fn name(&self) -> &'static str {
        "js"
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
        let mut domain = JsDomain::new();
        domain.note_source(
            "api",
            ".. js:module:: widgets\n\n.. js:function:: render(el, opts)\n",
        );
        assert!(
            domain
                .objects
                .contains_key(&("module".to_string(), "widgets".to_string()))
        );
        let (docname, _anchor, sig) = domain
            .objects
            .get(&("function".to_string(), "widgets.render".to_string()))
            .unwrap();
        assert_eq!(docname, "api");
        assert_eq!(sig, "el, opts");
    }

    #[test]
    fn note_source_nests_method_under_class() {
        let mut domain = JsDomain::new();
        domain.note_source(
            "api",
            ".. js:class:: Widget\n\n   .. js:method:: render()\n",
        );
        assert!(
            domain
                .objects
                .contains_key(&("method".to_string(), "Widget.render".to_string()))
        );
    }

    #[test]
    fn split_name_and_parens_handles_nested() {
        let (name, sig) = split_name_and_parens("configure(timeout = (1000))");
        assert_eq!(name, "configure");
        assert_eq!(sig, "timeout = (1000)");
    }

    #[test]
    fn resolve_func_xref() {
        let env = make_env();
        let mut domain = JsDomain::new();
        domain.note_object(
            "function",
            "widgets.render",
            "api",
            "js-function-render",
            "",
        );
        let target = domain
            .resolve_xref(&env, "index", "func", "widgets.render")
            .unwrap();
        assert_eq!(target.docname, "api");
        assert_eq!(target.anchor, "js-function-render");
    }

    #[test]
    fn resolve_unknown_is_none() {
        let env = make_env();
        let domain = JsDomain::new();
        assert!(domain.resolve_xref(&env, "index", "func", "nope").is_none());
    }

    #[test]
    fn clear_doc_removes_matching_entries() {
        let mut domain = JsDomain::new();
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
