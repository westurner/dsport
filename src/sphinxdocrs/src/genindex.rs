//! `sphinxdocrs::genindex` — Rust port of the general index page adapter
//! (`sphinx.environment.adapters.indexentries.IndexEntries`) plus the
//! Python module index (Tier **H5e**).
//!
//! Built over `env.indexentries` (populated by
//! [`crate::domains::scan::scan_index_entries`], **H3d**) and
//! `env.py_domain.get_objects()` (**H3b**).
//!
//! **Accepted deviation:** grouping/sorting mirrors
//! `IndexEntries.create_index`'s shape (alphabetical buckets, `pair`/
//! `triple` subentry nesting, `see`/`seealso` cross-links) but not every
//! upstream nuance — collapsing of near-duplicate keys
//! (`ignore_case`/normalization edge cases) and the `keyword`/`operator`
//! quirks specific to the `py`/`c`/`cpp` domains' generated index
//! entries aren't replicated. Anchors are page-level (no in-page id
//! tracking — see the `domains` module doc for the same deviation).

use std::collections::BTreeMap;

use crate::domains::IndexEntryKind;
use crate::environment::BuildEnvironment;

/// One grouped term in the generated index.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GenIndexTerm {
    /// The main index term as it should be displayed.
    pub name: String,
    /// Direct `(docname, anchor)` links for this term with no subentry.
    pub links: Vec<(String, String)>,
    /// `subterm -> [(docname, anchor), ...]`, sorted by subterm text.
    pub subterms: Vec<(String, Vec<(String, String)>)>,
    /// `(target term, is_seealso)` when this term is a cross-reference
    /// rather than a set of real links.
    pub see: Option<(String, bool)>,
}

/// Build the alphabetical general index from every document's
/// `.. index::` entries, bucketed by first letter (uppercased) or
/// `"Symbols"` for non-alphabetic leading characters.
///
/// Mirrors `IndexEntries.create_index`'s overall shape.
pub fn build_genindex(env: &BuildEnvironment) -> Vec<(String, Vec<GenIndexTerm>)> {
    // main term (case-insensitive key) -> GenIndexTerm being assembled.
    let mut terms: BTreeMap<String, GenIndexTerm> = BTreeMap::new();

    let mut docnames: Vec<&String> = env.indexentries.keys().collect();
    docnames.sort();

    for docname in docnames {
        let Some(entries) = env.indexentries.get(docname) else {
            continue;
        };
        for entry in entries {
            match entry.kind {
                IndexEntryKind::See | IndexEntryKind::SeeAlso => {
                    let key = entry.text.to_lowercase();
                    let is_seealso = matches!(entry.kind, IndexEntryKind::SeeAlso);
                    let term = terms.entry(key).or_insert_with(|| GenIndexTerm {
                        name: entry.text.clone(),
                        ..Default::default()
                    });
                    if let Some(target) = &entry.see_target {
                        term.see = Some((target.clone(), is_seealso));
                    }
                }
                _ => {
                    // `"main; sub"` (from a `single`-with-subentry, `pair`,
                    // or `triple` entry) splits at the first `"; "`; a
                    // plain entry has no subterm.
                    let (main, sub) = match entry.text.split_once("; ") {
                        Some((m, s)) => (m.trim().to_string(), Some(s.trim().to_string())),
                        None => (entry.text.trim().to_string(), None),
                    };
                    let key = main.to_lowercase();
                    let term = terms.entry(key).or_insert_with(|| GenIndexTerm {
                        name: main.clone(),
                        ..Default::default()
                    });
                    let link = (docname.clone(), String::new());
                    match sub {
                        Some(sub_text) => {
                            match term.subterms.iter_mut().find(|(s, _)| *s == sub_text) {
                                Some((_, links)) => links.push(link),
                                None => term.subterms.push((sub_text, vec![link])),
                            }
                        }
                        None => term.links.push(link),
                    }
                }
            }
        }
    }

    for term in terms.values_mut() {
        term.subterms.sort_by(|a, b| a.0.cmp(&b.0));
    }

    let mut buckets: BTreeMap<String, Vec<GenIndexTerm>> = BTreeMap::new();
    for term in terms.into_values() {
        let letter = term
            .name
            .chars()
            .next()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "Symbols".to_string());
        buckets.entry(letter).or_default().push(term);
    }
    buckets.into_iter().collect()
}

/// One module entry in the Python module index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModIndexEntry {
    pub name: String,
    pub docname: String,
    pub anchor: String,
}

/// Build the Python module index (`py-modindex`) from `env.py_domain`'s
/// registered `"module"` objects, bucketed the same way as
/// [`build_genindex`]. A distinct index from the general one, matching
/// upstream (`sphinx.domains.python.PythonModuleIndex`).
pub fn build_modindex(env: &BuildEnvironment) -> Vec<(String, Vec<ModIndexEntry>)> {
    use crate::domains::Domain as _;

    let mut modules: Vec<ModIndexEntry> = env
        .py_domain
        .get_objects()
        .into_iter()
        .filter(|e| e.obj_type == "module")
        .map(|e| ModIndexEntry {
            name: e.name,
            docname: e.docname,
            anchor: e.anchor,
        })
        .collect();
    modules.sort_by(|a, b| a.name.cmp(&b.name));

    let mut buckets: BTreeMap<String, Vec<ModIndexEntry>> = BTreeMap::new();
    for module in modules {
        let letter = module
            .name
            .chars()
            .next()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "Symbols".to_string());
        buckets.entry(letter).or_default().push(module);
    }
    buckets.into_iter().collect()
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

    fn single(text: &str) -> crate::domains::IndexEntry {
        crate::domains::IndexEntry {
            kind: IndexEntryKind::Single,
            text: text.to_string(),
            see_target: None,
            main: false,
        }
    }

    #[test]
    fn groups_by_first_letter() {
        let mut env = make_env();
        env.indexentries
            .insert("guide".to_string(), vec![single("Widget"), single("apple")]);
        let index = build_genindex(&env);
        let (letters, _): (Vec<&String>, Vec<_>) = index.iter().map(|(l, t)| (l, t)).unzip();
        assert!(letters.contains(&&"A".to_string()));
        assert!(letters.contains(&&"W".to_string()));
    }

    #[test]
    fn pair_entry_creates_subentry() {
        let mut env = make_env();
        env.indexentries.insert(
            "guide".to_string(),
            vec![crate::domains::IndexEntry {
                kind: IndexEntryKind::Pair,
                text: "Widget; rendering".to_string(),
                see_target: None,
                main: false,
            }],
        );
        let index = build_genindex(&env);
        let (_, terms) = index.iter().find(|(l, _)| l == "W").unwrap();
        assert_eq!(terms[0].name, "Widget");
        assert_eq!(terms[0].subterms.len(), 1);
        assert_eq!(terms[0].subterms[0].0, "rendering");
    }

    #[test]
    fn see_entry_records_cross_link() {
        let mut env = make_env();
        env.indexentries.insert(
            "guide".to_string(),
            vec![crate::domains::IndexEntry {
                kind: IndexEntryKind::See,
                text: "widget".to_string(),
                see_target: Some("gadget".to_string()),
                main: false,
            }],
        );
        let index = build_genindex(&env);
        let (_, terms) = index.iter().find(|(l, _)| l == "W").unwrap();
        assert_eq!(terms[0].see, Some(("gadget".to_string(), false)));
    }

    #[test]
    fn non_alphabetic_terms_bucket_under_symbols() {
        let mut env = make_env();
        env.indexentries
            .insert("guide".to_string(), vec![single("42-widget")]);
        let index = build_genindex(&env);
        assert!(index.iter().any(|(l, _)| l == "Symbols"));
    }

    #[test]
    fn modindex_groups_python_modules() {
        let mut env = make_env();
        env.py_domain
            .note_object("module", "pkg.alpha", "api", "py-module-pkg-alpha", "");
        env.py_domain
            .note_object("module", "pkg.beta", "api", "py-module-pkg-beta", "");
        let index = build_modindex(&env);
        let (_, mods) = index.iter().find(|(l, _)| l == "P").unwrap();
        assert_eq!(mods.len(), 2);
        assert_eq!(mods[0].name, "pkg.alpha");
    }

    #[test]
    fn modindex_ignores_non_module_objects() {
        let mut env = make_env();
        env.py_domain
            .note_object("function", "pkg.func", "api", "id", "");
        assert!(build_modindex(&env).is_empty());
    }
}
