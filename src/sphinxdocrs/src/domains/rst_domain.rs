//! `sphinxdocrs::domains::rst_domain` — Rust port of
//! `sphinx.domains.rst.ReSTDomain` (Tier **H3c**).
//!
//! Small, mostly self-documenting domain used by Sphinx's own docs to
//! describe reStructuredText directives and roles (`.. rst:directive::`,
//! `.. rst:role::`) and cross-reference them (`:rst:dir:`, `:rst:role:`).
//! A good second validation of the [`Domain`] trait shape after
//! [`super::StdDomain`].
//!
//! **Deferred**: `directive:option` sub-objects (`.. rst:directive:option::`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Domain, ObjectEntry, XrefTarget};
use crate::environment::BuildEnvironment;

/// Rust port of `sphinx.domains.rst.ReSTDomain`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RstDomain {
    /// `(objtype, name) -> (docname, labelid)`, objtype one of
    /// `"directive"` / `"role"`.
    #[serde(with = "crate::domains::tuple_key_map")]
    pub objects: HashMap<(String, String), (String, String)>,
}

impl RstDomain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Note a `.. rst:directive::`/`.. rst:role::` description. Mirrors
    /// `ReSTDomain.note_object`/the shared `ObjectDescription.add_target_and_index`.
    pub fn note_object(
        &mut self,
        objtype: impl Into<String>,
        name: impl Into<String>,
        docname: impl Into<String>,
        labelid: impl Into<String>,
    ) {
        self.objects.insert(
            (objtype.into(), name.into()),
            (docname.into(), labelid.into()),
        );
    }
}

impl Domain for RstDomain {
    fn name(&self) -> &'static str {
        "rst"
    }

    fn resolve_xref(
        &self,
        _env: &BuildEnvironment,
        _fromdocname: &str,
        reftype: &str,
        target: &str,
    ) -> Option<XrefTarget> {
        let objtype = match reftype {
            "dir" => "directive",
            "role" => "role",
            other => other,
        };
        // Directive cross-references may include a leading `.` or
        // trailing `::` (e.g. `` :rst:dir:`.. toctree::` ``); normalize
        // before lookup, mirroring `ReSTMarkup._object_hierarchy_parts`.
        let target = target
            .trim()
            .trim_start_matches('.')
            .trim()
            .trim_end_matches("::");
        let (docname, anchor) = self
            .objects
            .get(&(objtype.to_string(), target.to_string()))?;
        Some(XrefTarget {
            docname: docname.clone(),
            anchor: anchor.clone(),
            title: target.to_string(),
        })
    }

    fn get_objects(&self) -> Vec<ObjectEntry> {
        self.objects
            .iter()
            .map(|((obj_type, name), (docname, anchor))| ObjectEntry {
                obj_type: obj_type.clone(),
                name: name.clone(),
                docname: docname.clone(),
                anchor: anchor.clone(),
            })
            .collect()
    }

    fn clear_doc(&mut self, docname: &str) {
        self.objects.retain(|_, (d, _)| d != docname);
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
    fn note_and_resolve_directive() {
        let env = make_env();
        let mut domain = RstDomain::new();
        domain.note_object(
            "directive",
            "toctree",
            "usage/toctree",
            "rst-directive-toctree",
        );
        let target = domain
            .resolve_xref(&env, "index", "dir", "toctree")
            .unwrap();
        assert_eq!(target.docname, "usage/toctree");
    }

    #[test]
    fn resolve_strips_directive_punctuation() {
        let env = make_env();
        let mut domain = RstDomain::new();
        domain.note_object("directive", "toctree", "usage/toctree", "id");
        let target = domain
            .resolve_xref(&env, "index", "dir", ".. toctree::")
            .unwrap();
        assert_eq!(target.docname, "usage/toctree");
    }

    #[test]
    fn note_and_resolve_role() {
        let env = make_env();
        let mut domain = RstDomain::new();
        domain.note_object("role", "ref", "usage/roles", "rst-role-ref");
        let target = domain.resolve_xref(&env, "index", "role", "ref").unwrap();
        assert_eq!(target.anchor, "rst-role-ref");
    }

    #[test]
    fn resolve_unknown_is_none() {
        let env = make_env();
        let domain = RstDomain::new();
        assert!(domain.resolve_xref(&env, "index", "dir", "nope").is_none());
    }

    #[test]
    fn clear_doc_removes_matching_entries() {
        let mut domain = RstDomain::new();
        domain.note_object("directive", "toctree", "docA", "id1");
        domain.note_object("directive", "code-block", "docB", "id2");
        domain.clear_doc("docA");
        assert!(
            !domain
                .objects
                .contains_key(&("directive".to_string(), "toctree".to_string()))
        );
        assert!(
            domain
                .objects
                .contains_key(&("directive".to_string(), "code-block".to_string()))
        );
    }
}
