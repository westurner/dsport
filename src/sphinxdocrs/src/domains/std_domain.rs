//! `sphinxdocrs::domains::std_domain` — Rust port of
//! `sphinx.domains.std.StandardDomain` (Tier **H3a**).
//!
//! Ported: labels/anonlabels (`:ref:`/`:numref:`/`:keyword:`), documents
//! (`:doc:`), glossary terms (`:term:`), and the generic `objects` table
//! backing them (mirrors `StandardDomain.objects`/`.note_object`).
//!
//! **Deferred**: `:option:`/`progoptions` (needs a `.. program::`
//! directive context, not modelled yet), `:token:`/`productionlist`,
//! `numfig`-aware `:numref:` title formatting (**H5d**), the three
//! virtual `genindex`/`modindex`/`search` document labels are seeded but
//! the pages themselves aren't generated yet (**H5e**/**H3d**).

use std::collections::HashMap;

use super::{Domain, ObjectEntry, XrefTarget, normalize_id};
use crate::environment::BuildEnvironment;

/// `label name -> (docname, labelid, sectionname)`.
pub type Labels = HashMap<String, (String, String, String)>;
/// `label name -> (docname, labelid)`, used when a `:ref:` supplies an
/// explicit title (`refexplicit` in upstream).
pub type AnonLabels = HashMap<String, (String, String)>;

/// Rust port of `sphinx.domains.std.StandardDomain`.
#[derive(Debug, Clone)]
pub struct StdDomain {
    pub labels: Labels,
    pub anonlabels: AnonLabels,
    /// `(objtype, name) -> (docname, labelid)`. Mirrors `StandardDomain.objects`.
    pub objects: HashMap<(String, String), (String, String)>,
    /// `term.lower() -> (docname, labelid)`. Mirrors `StandardDomain._terms`.
    pub terms: HashMap<String, (String, String)>,
}

impl Default for StdDomain {
    fn default() -> Self {
        // Mirrors `StandardDomain.initial_data`'s three virtual doc labels.
        let mut labels = Labels::new();
        let mut anonlabels = AnonLabels::new();
        for (name, docname, title) in [
            ("genindex", "genindex", "Index"),
            ("modindex", "py-modindex", "Module Index"),
            ("search", "search", "Search Page"),
        ] {
            labels.insert(
                name.to_string(),
                (docname.to_string(), String::new(), title.to_string()),
            );
            anonlabels.insert(name.to_string(), (docname.to_string(), String::new()));
        }
        Self {
            labels,
            anonlabels,
            objects: HashMap::new(),
            terms: HashMap::new(),
        }
    }
}

impl StdDomain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Note a hyperlink target for cross reference.
    ///
    /// Mirrors `StandardDomain.note_hyperlink_target`. `sectionname`
    /// empty means "not attached to a titled element" (still resolvable
    /// via an explicit-title `:ref:`, but not via a named one).
    pub fn note_label(
        &mut self,
        name: impl Into<String>,
        docname: impl Into<String>,
        labelid: impl Into<String>,
        sectionname: impl Into<String>,
    ) {
        let name = name.into();
        let docname = docname.into();
        let labelid = labelid.into();
        let sectionname = sectionname.into();
        self.anonlabels
            .insert(name.clone(), (docname.clone(), labelid.clone()));
        if !sectionname.is_empty() {
            self.labels.insert(name, (docname, labelid, sectionname));
        }
    }

    /// Note a generic object for cross reference. Mirrors
    /// `StandardDomain.note_object`.
    pub fn note_object(
        &mut self,
        objtype: impl Into<String>,
        name: impl Into<String>,
        docname: impl Into<String>,
        labelid: impl Into<String>,
    ) {
        let objtype = objtype.into();
        let name = name.into();
        self.objects
            .insert((objtype, name), (docname.into(), labelid.into()));
    }

    /// Note a glossary term. Mirrors `StandardDomain._note_term`.
    pub fn note_term(
        &mut self,
        term: impl Into<String>,
        docname: impl Into<String>,
        labelid: impl Into<String>,
    ) {
        let term = term.into();
        let docname = docname.into();
        let labelid = labelid.into();
        self.note_object("term", term.clone(), docname.clone(), labelid.clone());
        self.terms.insert(term.to_lowercase(), (docname, labelid));
    }

    /// Auto-derive a label id from a label name, mirroring the common
    /// case where docutils assigns the explicit target's own normalized
    /// name as its id. See the module-level accepted-deviation note in
    /// `domains::scan` for what this simplifies away.
    pub fn label_id(name: &str) -> String {
        normalize_id(name)
    }

    /// `_resolve_ref_xref` / `_resolve_numref_xref` / `_resolve_keyword_xref`.
    fn resolve_ref(&self, target: &str, explicit_title: bool) -> Option<(String, String, String)> {
        if explicit_title {
            self.anonlabels
                .get(target)
                .map(|(d, l)| (d.clone(), l.clone(), String::new()))
        } else {
            self.labels.get(target).cloned()
        }
    }

    /// `_resolve_doc_xref`.
    fn resolve_doc(
        &self,
        env: &BuildEnvironment,
        fromdocname: &str,
        target: &str,
    ) -> Option<String> {
        let docname = super::docname_join(fromdocname, target);
        if env.found_docs().contains(&docname) {
            Some(docname)
        } else {
            None
        }
    }

    /// `_resolve_term_xref` (falls back to a case-insensitive match).
    fn resolve_term(&self, target: &str) -> Option<(String, String)> {
        if let Some((d, l)) = self.objects.get(&("term".to_string(), target.to_string())) {
            return Some((d.clone(), l.clone()));
        }
        self.terms.get(&target.to_lowercase()).cloned()
    }
}

impl Domain for StdDomain {
    fn name(&self) -> &'static str {
        "std"
    }

    fn resolve_xref(
        &self,
        env: &BuildEnvironment,
        fromdocname: &str,
        reftype: &str,
        target: &str,
    ) -> Option<XrefTarget> {
        match reftype {
            "ref" | "numref" | "keyword" => {
                // Upstream distinguishes `refexplicit` per-node; here we
                // simply prefer the named-label lookup and fall back to
                // anonlabels, since our text-scan doesn't track
                // `refexplicit` before calling in (see `resolve_xref_explicit`).
                let (docname, anchor, sectionname) = self.resolve_ref(target, false)?;
                let title = if sectionname.is_empty() {
                    env.get_title(&docname).unwrap_or(&docname).to_string()
                } else {
                    sectionname
                };
                Some(XrefTarget {
                    docname,
                    anchor,
                    title,
                })
            }
            "term" => {
                let (docname, anchor) = self.resolve_term(target)?;
                Some(XrefTarget {
                    docname,
                    anchor,
                    title: target.to_string(),
                })
            }
            "doc" => {
                let docname = self.resolve_doc(env, fromdocname, target)?;
                let title = env.get_title(&docname).unwrap_or(&docname).to_string();
                Some(XrefTarget {
                    docname,
                    anchor: String::new(),
                    title,
                })
            }
            _ => None,
        }
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
        self.labels.retain(|_, (d, _, _)| d != docname);
        self.anonlabels.retain(|_, (d, _)| d != docname);
        self.objects.retain(|_, (d, _)| d != docname);
        self.terms.retain(|_, (d, _)| d != docname);
    }
}

impl StdDomain {
    /// Like [`resolve_xref`](Domain::resolve_xref), but honours whether
    /// the role site gave an explicit title (`` `Title <target>` ``),
    /// matching upstream's `node['refexplicit']` branch for `ref`.
    pub fn resolve_xref_explicit(
        &self,
        env: &BuildEnvironment,
        fromdocname: &str,
        reftype: &str,
        target: &str,
        has_explicit_title: bool,
    ) -> Option<XrefTarget> {
        if matches!(reftype, "ref" | "numref") {
            let (docname, anchor, sectionname) = self.resolve_ref(target, has_explicit_title)?;
            let title = if sectionname.is_empty() {
                env.get_title(&docname).unwrap_or(&docname).to_string()
            } else {
                sectionname
            };
            return Some(XrefTarget {
                docname,
                anchor,
                title,
            });
        }
        self.resolve_xref(env, fromdocname, reftype, target)
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
    fn seeded_virtual_labels() {
        let domain = StdDomain::new();
        assert_eq!(
            domain.labels.get("genindex"),
            Some(&("genindex".to_string(), String::new(), "Index".to_string()))
        );
        assert!(domain.anonlabels.contains_key("search"));
    }

    #[test]
    fn note_label_and_resolve_ref() {
        let mut env = make_env();
        env.set_title("intro", "Introduction");
        env.std_domain
            .note_label("my-label", "intro", "my-label", "Introduction");
        let target = env
            .std_domain
            .resolve_xref(&env, "other", "ref", "my-label")
            .unwrap();
        assert_eq!(target.docname, "intro");
        assert_eq!(target.title, "Introduction");
    }

    #[test]
    fn resolve_ref_missing_is_none() {
        let env = make_env();
        assert!(
            env.std_domain
                .resolve_xref(&env, "other", "ref", "nope")
                .is_none()
        );
    }

    #[test]
    fn note_term_and_resolve_case_insensitive() {
        let mut env = make_env();
        env.std_domain
            .note_term("Widget", "glossary", "term-widget");
        let target = env
            .std_domain
            .resolve_xref(&env, "other", "term", "widget")
            .unwrap();
        assert_eq!(target.docname, "glossary");
        assert_eq!(target.anchor, "term-widget");
    }

    #[test]
    fn resolve_doc_relative_and_absolute() {
        let mut env = make_env();
        env.project.docnames.insert("guide/setup".to_string());
        env.project.docnames.insert("index".to_string());
        env.set_title("guide/setup", "Setup");

        let target = env
            .std_domain
            .resolve_xref(&env, "guide/intro", "doc", "setup")
            .unwrap();
        assert_eq!(target.docname, "guide/setup");
        assert_eq!(target.title, "Setup");

        let target = env
            .std_domain
            .resolve_xref(&env, "guide/intro", "doc", "/index")
            .unwrap();
        assert_eq!(target.docname, "index");
    }

    #[test]
    fn clear_doc_removes_all_entries() {
        let mut domain = StdDomain::new();
        domain.note_label("a", "docA", "a", "A");
        domain.note_term("t", "docA", "term-t");
        domain.clear_doc("docA");
        assert!(!domain.labels.contains_key("a"));
        assert!(!domain.anonlabels.contains_key("a"));
        assert!(domain.terms.is_empty());
    }

    #[test]
    fn explicit_title_ref_uses_anonlabels() {
        let mut env = make_env();
        // A label with no attached section (sectionname empty) is only
        // reachable via the explicit-title lookup path.
        env.std_domain
            .note_label("anon-target", "page", "anon-target", "");
        assert!(
            env.std_domain
                .resolve_xref(&env, "other", "ref", "anon-target")
                .is_none()
        );
        let target = env
            .std_domain
            .resolve_xref_explicit(&env, "other", "ref", "anon-target", true)
            .unwrap();
        assert_eq!(target.docname, "page");
    }
}
