//! Integration tests for `sphinxdocrs::versioning` — P2 port of
//! `sphinx.versioning`.
//!
//! Mirrors the behaviour of `src/sphinx/tests/test_versioning.py` at the
//! **algorithm level** (doctree-free). Upstream tests that require a live
//! `SphinxTestApp` / builder are deferred to P3.

use std::collections::HashSet;

use rstest::*;
use sphinxdocrs::versioning::{
    VERSIONING_RATIO, VersionableNode, add_uids, get_ratio, levenshtein_distance, merge_doctrees,
};

// ── test node ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Node {
    source: String,
    uid: Option<String>,
}

impl Node {
    fn new(s: &str) -> Self {
        Node {
            source: s.to_string(),
            uid: None,
        }
    }
    fn with_uid(s: &str, uid: &str) -> Self {
        Node {
            source: s.to_string(),
            uid: Some(uid.to_string()),
        }
    }
}

impl VersionableNode for Node {
    fn raw_source(&self) -> &str {
        &self.source
    }
    fn uid(&self) -> Option<&str> {
        self.uid.as_deref()
    }
    fn set_uid(&mut self, uid: String) {
        self.uid = Some(uid);
    }
}

fn nodes(sources: &[&str]) -> Vec<Node> {
    sources.iter().map(|s| Node::new(s)).collect()
}

fn stamp(ns: &mut [Node]) -> Vec<String> {
    add_uids(ns)
}

// ── paragraph fixtures ────────────────────────────────────────────────────────

const P1: &str = "This is the first paragraph.";
const P2: &str = "This is the second paragraph.";
const P3: &str = "This is the third paragraph.";
const P4: &str = "This is the fourth paragraph.";
const P_NEW: &str = "A brand new paragraph introduced in the middle.";
const P_SIMILAR: &str = "Anyway I need more";

// ── levenshtein_distance ──────────────────────────────────────────────────────

#[rstest]
#[case("", "", 0)]
#[case("abc", "abc", 0)]
#[case("abc", "ab", 1)]
#[case("ab", "abc", 1)]
#[case("abc", "axc", 1)]
#[case("kitten", "sitting", 3)]
#[case("saturday", "sunday", 3)]
fn levenshtein_cases(#[case] a: &str, #[case] b: &str, #[case] expected: usize) {
    assert_eq!(levenshtein_distance(a, b), expected, "lev({a:?},{b:?})");
}

#[test]
fn levenshtein_symmetric() {
    assert_eq!(
        levenshtein_distance("sphinx", "prefix"),
        levenshtein_distance("prefix", "sphinx")
    );
}

// ── get_ratio ─────────────────────────────────────────────────────────────────

#[test]
fn ratio_empty_old() {
    // Mirrors: assert get_ratio('', 'a')
    assert_eq!(get_ratio("", "a"), VERSIONING_RATIO);
}

#[test]
fn ratio_empty_new() {
    // Mirrors: assert get_ratio('a', '')
    assert_eq!(get_ratio("a", ""), VERSIONING_RATIO);
}

#[test]
fn ratio_identical_is_zero() {
    assert_eq!(get_ratio(P1, P1), 0.0);
}

#[test]
fn ratio_slightly_different_is_below_threshold() {
    // "second paragraph" vs "third paragraph" — minor edit, ratio should be
    // well below VERSIONING_RATIO (65).
    let r = get_ratio(P2, P3);
    assert!(
        r < VERSIONING_RATIO,
        "ratio {r} should be < {VERSIONING_RATIO}"
    );
}

#[test]
fn ratio_completely_different_is_above_threshold() {
    // Totally unrelated strings should score above the threshold.
    let r = get_ratio(P1, "xyz pqr");
    assert!(
        r > VERSIONING_RATIO,
        "ratio {r} should be > {VERSIONING_RATIO}"
    );
}

// ── add_uids ──────────────────────────────────────────────────────────────────

#[test]
fn add_uids_assigns_uid_to_all_nodes() {
    let mut ns = nodes(&[P1, P2, P3]);
    let uids = add_uids(&mut ns);
    assert_eq!(uids.len(), 3);
    for (i, n) in ns.iter().enumerate() {
        assert_eq!(n.uid().unwrap(), uids[i]);
    }
}

#[test]
fn add_uids_returns_unique_ids() {
    let mut ns: Vec<Node> = (0..10).map(|i| Node::new(&i.to_string())).collect();
    let uids = add_uids(&mut ns);
    let uniq: HashSet<&str> = uids.iter().map(String::as_str).collect();
    assert_eq!(uniq.len(), 10);
}

#[test]
fn add_uids_uid_format_matches_python() {
    // Python uuid4().hex → 32 lower-case hex chars, no hyphens.
    let mut ns = vec![Node::new("x")];
    let uids = add_uids(&mut ns);
    assert_eq!(uids[0].len(), 32, "uid should be 32 chars: {}", uids[0]);
    assert!(
        uids[0].chars().all(|c| c.is_ascii_hexdigit()),
        "uid should be hex: {}",
        uids[0]
    );
}

// ── merge_doctrees — mirrors test_versioning.py cases ────────────────────────

#[test]
fn merge_modified_no_new_nodes() {
    // Mirrors test_modified: same content → 0 new nodes, same UIDs.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut modified = nodes(&[P1, P2, P3]);
    let changed = merge_doctrees(&mut orig, &mut modified);

    assert!(changed.is_empty(), "expected 0 new nodes, got {changed:?}");
    for (i, n) in modified.iter().enumerate() {
        assert_eq!(n.uid().unwrap(), orig_uids[i], "uid mismatch at index {i}");
    }
}

#[test]
fn merge_added_one_new_node() {
    // Mirrors test_added: [P1,P2,P3,P4] → 1 new node; first 3 uids preserved.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut added = nodes(&[P1, P2, P3, P4]);
    let changed = merge_doctrees(&mut orig, &mut added);

    assert_eq!(changed.len(), 1, "expected 1 new node");
    assert_eq!(added[0].uid().unwrap(), orig_uids[0]);
    assert_eq!(added[1].uid().unwrap(), orig_uids[1]);
    assert_eq!(added[2].uid().unwrap(), orig_uids[2]);
    // P4 has a fresh uid
    assert!(!orig_uids.contains(&added[3].uid().unwrap().to_string()));
}

#[test]
fn merge_deleted_zero_new_nodes() {
    // Mirrors test_deleted: [P1,P3] → 0 new nodes.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut deleted = nodes(&[P1, P3]);
    let changed = merge_doctrees(&mut orig, &mut deleted);

    assert!(changed.is_empty(), "expected 0 new nodes");
    // P1 exactly matched
    assert_eq!(deleted[0].uid().unwrap(), orig_uids[0]);
    // P3 matched (either to orig P2 or P3 — ratio keeps it below threshold)
    assert!(deleted[1].uid().is_some());
}

#[test]
fn merge_deleted_end_zero_new_nodes() {
    // Mirrors test_deleted_end: [P1,P2] → 0 new nodes; P1/P2 uids preserved.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut deleted_end = nodes(&[P1, P2]);
    let changed = merge_doctrees(&mut orig, &mut deleted_end);

    assert!(changed.is_empty());
    assert_eq!(deleted_end[0].uid().unwrap(), orig_uids[0]);
    assert_eq!(deleted_end[1].uid().unwrap(), orig_uids[1]);
}

#[test]
fn merge_insert_middle_one_new_node() {
    // Mirrors test_insert: [P1,P_NEW,P2,P3] → 1 new node; P1/P2/P3 preserved.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut inserted = nodes(&[P1, P_NEW, P2, P3]);
    let changed = merge_doctrees(&mut orig, &mut inserted);

    assert_eq!(changed.len(), 1, "expected 1 new node, got {changed:?}");
    assert_eq!(inserted[0].uid().unwrap(), orig_uids[0]);
    assert_eq!(inserted[2].uid().unwrap(), orig_uids[1]);
    assert_eq!(inserted[3].uid().unwrap(), orig_uids[2]);
}

#[test]
fn merge_insert_beginning_one_new_node() {
    // Mirrors test_insert_beginning: [P_NEW,P1,P2,P3] → 1 new node; P1/P2/P3 preserved.
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut inserted = nodes(&[P_NEW, P1, P2, P3]);
    let changed = merge_doctrees(&mut orig, &mut inserted);

    assert_eq!(changed.len(), 1, "expected 1 new node");
    assert_eq!(inserted.len(), 4);
    // P1/P2/P3 uids preserved (moved to positions 1..=3)
    assert_eq!(inserted[1].uid().unwrap(), orig_uids[0]);
    assert_eq!(inserted[2].uid().unwrap(), orig_uids[1]);
    assert_eq!(inserted[3].uid().unwrap(), orig_uids[2]);
    // P_NEW uid is new
    assert!(!orig_uids.contains(&inserted[0].uid().unwrap().to_string()));
}

#[test]
fn merge_insert_similar_one_new_node() {
    // Mirrors test_insert_similar: [P1,P_SIMILAR,P2,P3] → 1 new node (P_SIMILAR).
    let mut orig = nodes(&[P1, P2, P3]);
    let orig_uids = stamp(&mut orig);

    let mut inserted = nodes(&[P1, P_SIMILAR, P2, P3]);
    let changed = merge_doctrees(&mut orig, &mut inserted);

    assert_eq!(changed.len(), 1, "expected 1 new node, got {changed:?}");
    assert!(
        changed.contains(&1),
        "P_SIMILAR (index 1) should be the new node"
    );
    // P1/P2/P3 uids preserved
    assert_eq!(inserted[0].uid().unwrap(), orig_uids[0]);
    assert_eq!(inserted[2].uid().unwrap(), orig_uids[1]);
    assert_eq!(inserted[3].uid().unwrap(), orig_uids[2]);
}

// ── edge cases ────────────────────────────────────────────────────────────────

#[test]
fn merge_empty_old_all_new() {
    let mut orig: Vec<Node> = vec![];
    let mut new_ns = nodes(&[P1, P2]);
    let changed = merge_doctrees(&mut orig, &mut new_ns);
    assert_eq!(changed.len(), 2);
    assert!(new_ns[0].uid().is_some());
    assert!(new_ns[1].uid().is_some());
}

#[test]
fn merge_empty_new_no_changed() {
    let mut orig = nodes(&[P1, P2]);
    stamp(&mut orig);
    let mut new_ns: Vec<Node> = vec![];
    let changed = merge_doctrees(&mut orig, &mut new_ns);
    assert!(changed.is_empty());
}

#[test]
fn merge_old_node_without_uid_gets_uid() {
    let mut old_ns = vec![Node::new(P1)]; // no uid pre-assigned
    let mut new_ns = vec![Node::new(P1)];
    let changed = merge_doctrees(&mut old_ns, &mut new_ns);
    assert!(changed.is_empty(), "exact match → 0 changed");
    assert!(old_ns[0].uid().is_some());
    assert_eq!(new_ns[0].uid(), old_ns[0].uid());
}

#[test]
fn merge_preserved_uid_propagated() {
    let mut old_ns = vec![Node::with_uid(P1, "deadbeef00000000deadbeef00000000")];
    let mut new_ns = vec![Node::new(P1)];
    let changed = merge_doctrees(&mut old_ns, &mut new_ns);
    assert!(changed.is_empty());
    assert_eq!(new_ns[0].uid().unwrap(), "deadbeef00000000deadbeef00000000");
}

#[test]
fn merge_single_vs_single_identical() {
    let mut old_ns = nodes(&[P1]);
    stamp(&mut old_ns);
    let mut new_ns = nodes(&[P1]);
    let changed = merge_doctrees(&mut old_ns, &mut new_ns);
    assert!(changed.is_empty());
    assert_eq!(new_ns[0].uid(), old_ns[0].uid());
}

#[test]
fn merge_single_vs_single_completely_different() {
    let mut old_ns = nodes(&[P1]);
    stamp(&mut old_ns);
    let mut new_ns = nodes(&["xyz pqr completely unrelated content that is totally different"]);
    let changed = merge_doctrees(&mut old_ns, &mut new_ns);
    assert_eq!(changed.len(), 1, "completely different → 1 new node");
    assert!(!old_ns[0].uid().unwrap().eq(new_ns[0].uid().unwrap()));
}
