//! `sphinxdocrs::toctree` — Rust port of the `TocTree` env adapter
//! (`sphinx.environment.adapters.toctree.TocTree`) (Tier **H5d**).
//!
//! Built entirely over data the H2c read phase already recovers
//! (`env.toctree_includes`, `env.titles`/`env.longtitles`,
//! `env.numbered_toctrees`, `env.config.root_doc()`) — no new scanning
//! needed.
//!
//! **Accepted deviation:** [`TocEntry`] is a plain Rust struct standing
//! in for the `bullet_list`/`list_item`/`compact_paragraph` doctree
//! nodes `TocTree.resolve` builds upstream (there's no such node kind to
//! build them from — see the `domains` module doc for the same
//! deviation elsewhere). [`secnumbers`] assigns one chapter number per
//! *document* only: upstream also numbers each section inside a
//! document, which needs section-tree structure this port doesn't track
//! beyond the promoted document title — deferred alongside `numfig`.
//! `:glob:` toctree expansion (`env.glob_toctrees` is tracked but not
//! expanded against on-disk globs here) and `:hidden:`/`:includehidden:`
//! filtering are likewise not implemented.

use std::collections::{HashMap, HashSet};

use crate::environment::BuildEnvironment;

/// One resolved toctree entry: a document and its nested children.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TocEntry {
    pub docname: String,
    pub title: String,
    pub children: Vec<TocEntry>,
}

/// Recursively expand the toctree rooted at `docname`'s own
/// `.. toctree::` entries, following nested toctrees up to `max_depth`
/// levels (`0` = unlimited).
///
/// Mirrors `TocTree.resolve`/`_entries_from_toctree` for the subset of
/// behaviour this port supports (see the module accepted-deviation
/// note). A toctree that (directly or transitively) includes itself
/// stops expanding rather than recursing forever.
pub fn resolve(env: &BuildEnvironment, docname: &str, max_depth: usize) -> Vec<TocEntry> {
    resolve_inner(env, docname, max_depth, &mut HashSet::new())
}

fn resolve_inner(
    env: &BuildEnvironment,
    docname: &str,
    max_depth: usize,
    seen: &mut HashSet<String>,
) -> Vec<TocEntry> {
    let Some(entries) = env.toctree_includes.get(docname) else {
        return Vec::new();
    };
    if !seen.insert(docname.to_string()) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for entry in entries {
        let title = env
            .longtitles
            .get(entry)
            .or_else(|| env.titles.get(entry))
            .cloned()
            .unwrap_or_else(|| entry.clone());
        let children = if max_depth == 1 {
            Vec::new()
        } else {
            resolve_inner(env, entry, max_depth.saturating_sub(1), seen)
        };
        out.push(TocEntry {
            docname: entry.clone(),
            title,
            children,
        });
    }
    seen.remove(docname);
    out
}

/// The global toctree rooted at the project's `root_doc`. Mirrors
/// `TocTree.get_toctree_for(env.config.root_doc, ...)`.
pub fn global_toctree_for_doc(env: &BuildEnvironment, max_depth: usize) -> Vec<TocEntry> {
    resolve(env, &env.config.root_doc(), max_depth)
}

/// The toctree rooted at an arbitrary `docname` (not necessarily the
/// project root). Mirrors `TocTree.get_toc_for`.
pub fn get_toc_for(env: &BuildEnvironment, docname: &str) -> Vec<TocEntry> {
    resolve(env, docname, 0)
}

/// Assign chapter numbers (`"1"`, `"2"`, `"1.1"`, ...) to every document
/// reachable from `root_doc`'s global toctree, when `root_doc` (or a
/// container along the way) is marked `:numbered:` in
/// `env.numbered_toctrees`. Mirrors `TocTree.assign_section_numbers`,
/// scoped to whole-document numbering (see the module accepted-deviation
/// note).
pub fn secnumbers(env: &BuildEnvironment) -> HashMap<String, Vec<u32>> {
    let mut out = HashMap::new();
    let root = env.config.root_doc();
    if env.numbered_toctrees.contains(&root) {
        let entries = global_toctree_for_doc(env, 0);
        assign_numbers(&entries, &mut Vec::new(), &mut out);
    }
    out
}

fn assign_numbers(
    entries: &[TocEntry],
    prefix: &mut Vec<u32>,
    out: &mut HashMap<String, Vec<u32>>,
) {
    for (i, entry) in entries.iter().enumerate() {
        prefix.push((i + 1) as u32);
        out.insert(entry.docname.clone(), prefix.clone());
        assign_numbers(&entry.children, prefix, out);
        prefix.pop();
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
    fn resolve_nested_toctree() {
        let mut env = make_env();
        env.note_toctree("index", vec!["guide".to_string(), "reference".to_string()]);
        env.note_toctree("guide", vec!["guide/intro".to_string()]);
        env.set_title("guide", "Guide");
        env.set_title("reference", "Reference");
        env.set_title("guide/intro", "Introduction");

        let toc = resolve(&env, "index", 0);
        assert_eq!(toc.len(), 2);
        assert_eq!(toc[0].docname, "guide");
        assert_eq!(toc[0].title, "Guide");
        assert_eq!(toc[0].children.len(), 1);
        assert_eq!(toc[0].children[0].docname, "guide/intro");
        assert_eq!(toc[1].docname, "reference");
        assert!(toc[1].children.is_empty());
    }

    #[test]
    fn resolve_max_depth_stops_recursion() {
        let mut env = make_env();
        env.note_toctree("index", vec!["guide".to_string()]);
        env.note_toctree("guide", vec!["guide/intro".to_string()]);

        let toc = resolve(&env, "index", 1);
        assert_eq!(toc.len(), 1);
        assert!(toc[0].children.is_empty());
    }

    #[test]
    fn resolve_missing_toctree_is_empty() {
        let env = make_env();
        assert!(resolve(&env, "orphan", 0).is_empty());
    }

    #[test]
    fn resolve_self_referencing_toctree_does_not_recurse_forever() {
        let mut env = make_env();
        env.note_toctree("index", vec!["index".to_string()]);
        let toc = resolve(&env, "index", 0);
        assert_eq!(toc.len(), 1);
        assert!(toc[0].children.is_empty());
    }

    #[test]
    fn global_toctree_for_doc_uses_root_doc() {
        let mut env = make_env();
        // Default root_doc is "index".
        env.note_toctree("index", vec!["guide".to_string()]);
        let toc = global_toctree_for_doc(&env, 0);
        assert_eq!(toc.len(), 1);
        assert_eq!(toc[0].docname, "guide");
    }

    #[test]
    fn secnumbers_assigns_chapter_numbers_when_numbered() {
        let mut env = make_env();
        env.note_toctree("index", vec!["a".to_string(), "b".to_string()]);
        env.note_toctree("a", vec!["a/sub".to_string()]);
        env.numbered_toctrees.insert("index".to_string());

        let nums = secnumbers(&env);
        assert_eq!(nums.get("a"), Some(&vec![1]));
        assert_eq!(nums.get("a/sub"), Some(&vec![1, 1]));
        assert_eq!(nums.get("b"), Some(&vec![2]));
    }

    #[test]
    fn secnumbers_empty_when_not_numbered() {
        let mut env = make_env();
        env.note_toctree("index", vec!["a".to_string()]);
        assert!(secnumbers(&env).is_empty());
    }
}
