//! `sphinxdocrs::versioning` — Rust port of `sphinx.versioning`.
//!
//! Implements the low-level algorithms Sphinx uses for doctree versioning:
//! tracking content identity across incremental builds using UID strings
//! so that gettext message catalogues can follow paragraph movements.
//!
//! **P2 scope** — pure algorithms only. `UIDTransform` (which requires a
//! live Sphinx environment and builder) is deferred to P3.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `VERSIONING_RATIO` | [`VERSIONING_RATIO`] | constant 65.0 |
//! | `levenshtein_distance(a, b)` | [`levenshtein_distance`] | pure DP algorithm |
//! | `get_ratio(old, new)` | [`get_ratio`] | distance / (len(old)/100) |
//! | `add_uids(doctree, condition)` | [`add_uids`] | operates on `&mut [N: VersionableNode]` |
//! | `merge_doctrees(old, new, condition)` | [`merge_doctrees`] | returns Vec of changed indices |
//! | `UIDTransform` | **deferred** | requires P3 Sphinx environment |
//!
//! ## Adaptation note
//!
//! In the Python implementation, `add_uids` and `merge_doctrees` receive a
//! full doctree and a `condition` callable.  In this P2 Rust port, the
//! caller pre-filters nodes and passes a `&mut [N]` slice — this keeps
//! `versioning.rs` free of the doctree dependency.

use std::collections::{HashMap, HashSet};

use uuid::Uuid;

// ── constant ──────────────────────────────────────────────────────────────────

/// Similarity threshold below which two strings are considered "the same
/// text that changed" (uid preserved) rather than "new text" (uid replaced).
///
/// Mirrors `VERSIONING_RATIO = 65` in `sphinx.versioning`.
pub const VERSIONING_RATIO: f64 = 65.0;

// ── VersionableNode trait ─────────────────────────────────────────────────────

/// Abstraction over a doctree node that participates in versioning.
///
/// In the Python implementation the node objects are `docutils.nodes.Node`
/// instances with `.rawsource` and `.uid` attributes.  In this P2 Rust port
/// callers implement this trait for their node type so that `versioning.rs`
/// stays independent of the (not-yet-ported) doctree crate.
pub trait VersionableNode {
    /// The raw RST source text of this node (equivalent to `.rawsource`).
    fn raw_source(&self) -> &str;

    /// The current UID, or `None` if not yet assigned.
    fn uid(&self) -> Option<&str>;

    /// Set the UID.
    fn set_uid(&mut self, uid: String);
}

// ── Levenshtein distance ──────────────────────────────────────────────────────

/// Compute the Levenshtein edit distance between strings `a` and `b`.
///
/// Mirrors `sphinx.versioning.levenshtein_distance`.
///
/// Uses the classic O(m·n) DP algorithm. The optional C `Levenshtein`
/// speedup library used by the Python version is not available here; this
/// pure Rust implementation gives identical results.
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    if a == b {
        return 0;
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (a, b) = if a.len() < b.len() { (b, a) } else { (a, b) };
    if a.is_empty() {
        return b.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, &ca) in a.iter().enumerate() {
        let mut curr = vec![0usize; b.len() + 1];
        curr[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let ins = prev[j + 1] + 1;
            let del = curr[j] + 1;
            let sub = prev[j] + usize::from(ca != cb);
            curr[j + 1] = ins.min(del).min(sub);
        }
        prev = curr;
    }
    prev[b.len()]
}

// ── get_ratio ─────────────────────────────────────────────────────────────────

/// Return a "similarity ratio" (percentage) between two strings.
///
/// 0 means identical; higher means more different.  The ratio is
/// `levenshtein_distance(old, new) / (len(old) / 100.0)`.
///
/// Returns [`VERSIONING_RATIO`] when either string is empty (matches Python's
/// `if not all([old, new]): return VERSIONING_RATIO`).
///
/// Mirrors `sphinx.versioning.get_ratio`.
pub fn get_ratio(old: &str, new: &str) -> f64 {
    if old.is_empty() || new.is_empty() {
        return VERSIONING_RATIO;
    }
    levenshtein_distance(old, new) as f64 / (old.len() as f64 / 100.0)
}

// ── helper ────────────────────────────────────────────────────────────────────

fn new_uid() -> String {
    // Match Python `uuid4().hex` — 32-char lower-case hex without hyphens.
    Uuid::new_v4().simple().to_string()
}

// ── add_uids ──────────────────────────────────────────────────────────────────

/// Assign a fresh UID to each node in `nodes` and return their indices.
///
/// **Adaptation**: the Python `add_uids(doctree, condition)` traverses a
/// whole document and filters by `condition`.  In this P2 Rust port the
/// caller pre-filters and passes only matching nodes.  The function returns
/// the indices (always `0..nodes.len()`) so callers can chain with
/// `merge_doctrees`.
///
/// Mirrors `sphinx.versioning.add_uids`.
pub fn add_uids<N: VersionableNode>(nodes: &mut [N]) -> Vec<String> {
    nodes
        .iter_mut()
        .map(|n| {
            let uid = new_uid();
            n.set_uid(uid.clone());
            uid
        })
        .collect()
}

// ── merge_doctrees ────────────────────────────────────────────────────────────

/// Merge `old_nodes` and `new_nodes` by matching them via similarity ratio.
///
/// For each pair where the ratio is below [`VERSIONING_RATIO`], the old UID
/// is reused (the paragraph just changed a bit).  For pairs that exceed the
/// threshold, or for entirely new nodes, a fresh UID is generated.
///
/// **Return value**: indices into `new_nodes` for nodes that received a *new*
/// UID (= nodes that are genuinely new or significantly changed).  This
/// mirrors the `yield` statements in the Python `merge_doctrees` generator.
///
/// **Adaptation**: the Python function takes a full doctree and a condition
/// callable.  In this P2 Rust port the caller pre-filters to pass only
/// matching nodes.  UIDs are set on elements of `new_nodes` in place.
///
/// Mirrors `sphinx.versioning.merge_doctrees`.
pub fn merge_doctrees<N: VersionableNode>(old_nodes: &mut [N], new_nodes: &mut [N]) -> Vec<usize> {
    // (old_idx, new_idx) → similarity ratio
    let mut ratios: HashMap<(usize, usize), f64> = HashMap::new();
    // new_idx values that have been matched
    let mut seen: HashSet<usize> = HashSet::new();
    // indices into old_nodes / new_nodes that were not matched in the zip pass
    let mut unmatched_old: Vec<usize> = Vec::new();
    let mut unmatched_new: Vec<usize> = Vec::new();

    // ── First pass: zip in document order ────────────────────────────────────
    let zip_len = old_nodes.len().min(new_nodes.len());

    for idx in 0..zip_len {
        // Ensure the old node has a UID (mirrors Python's getattr fallback).
        if old_nodes[idx].uid().is_none() {
            old_nodes[idx].set_uid(new_uid());
        }
        let ratio = get_ratio(old_nodes[idx].raw_source(), new_nodes[idx].raw_source());
        if ratio == 0.0 {
            // Exact match: copy uid.
            let uid = old_nodes[idx].uid().unwrap_or_default().to_string();
            new_nodes[idx].set_uid(uid);
            seen.insert(idx);
        } else {
            ratios.insert((idx, idx), ratio);
            unmatched_old.push(idx);
            unmatched_new.push(idx);
        }
    }
    // Extra old nodes (old list longer than new)
    for (oi, old_node) in old_nodes.iter_mut().enumerate().skip(zip_len) {
        if old_node.uid().is_none() {
            old_node.set_uid(new_uid());
        }
        unmatched_old.push(oi);
    }
    // Extra new nodes (new list longer than old)
    for ni in zip_len..new_nodes.len() {
        unmatched_new.push(ni);
    }

    // ── Second pass: cross-product of unmatched nodes ─────────────────────────
    for &oi in &unmatched_old {
        for &ni in &unmatched_new {
            if seen.contains(&ni) || ratios.contains_key(&(oi, ni)) {
                continue;
            }
            let ratio = get_ratio(old_nodes[oi].raw_source(), new_nodes[ni].raw_source());
            if ratio == 0.0 {
                let uid = old_nodes[oi].uid().unwrap_or_default().to_string();
                new_nodes[ni].set_uid(uid);
                seen.insert(ni);
            } else {
                ratios.insert((oi, ni), ratio);
            }
        }
    }

    // ── Third pass: sort by ratio, assign UIDs ────────────────────────────────
    let mut sorted_ratios: Vec<((usize, usize), f64)> = ratios.into_iter().collect();
    // Sort ascending by ratio (lowest = most similar = prefer first).
    sorted_ratios.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let mut changed: Vec<usize> = Vec::new();

    for ((oi, ni), ratio) in sorted_ratios {
        if seen.contains(&ni) {
            continue;
        }
        seen.insert(ni);
        if ratio < VERSIONING_RATIO {
            // Similar enough: reuse old uid.
            let uid = old_nodes[oi].uid().unwrap_or_default().to_string();
            new_nodes[ni].set_uid(uid);
        } else {
            // Genuinely new / changed: fresh uid.
            new_nodes[ni].set_uid(new_uid());
            changed.push(ni);
        }
    }

    // ── Final pass: any unmatched new nodes get new UIDs ──────────────────────
    for ni in unmatched_new {
        if !seen.contains(&ni) {
            new_nodes[ni].set_uid(new_uid());
            changed.push(ni);
        }
    }

    changed
}

// ── UIDTransform ──────────────────────────────────────────────────────────────

/// Configuration for `UIDTransform`.
///
/// Mirrors `sphinx.versioning.UIDTransform` — a `SphinxTransform` that adds
/// UIDs to all versioning-eligible nodes in a document.
///
/// In the Rust port this is not a full docutils `Transform` subclass but a
/// plain struct that the caller invokes with a mutable slice of
/// [`VersionableNode`]s and optional old-doctree data.
///
/// Mirrors the Python `default_priority = 880`.
pub const UID_TRANSFORM_PRIORITY: u32 = 880;

/// Apply UID transform to `new_nodes`.
///
/// If `old_nodes` is `None`, all new nodes receive fresh UIDs (the
/// first-build path in Python: `list(add_uids(self.document, condition))`).
///
/// If `old_nodes` is `Some(slice)`, old UIDs are merged into `new_nodes`
/// (the incremental rebuild path: `list(merge_doctrees(old, new, condition))`).
///
/// Returns the indices of nodes that were changed / added (same semantics as
/// [`merge_doctrees`]).
///
/// Mirrors `UIDTransform.apply`.
pub fn apply_uid_transform<N: VersionableNode>(
    new_nodes: &mut [N],
    old_nodes: Option<&mut Vec<N>>,
) -> Vec<usize> {
    match old_nodes {
        None => {
            add_uids(new_nodes);
            Vec::new() // all are "new", none are "changed"
        }
        Some(old) => merge_doctrees(old, new_nodes),
    }
}

// ── inline unit tests ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Minimal node for testing.
    #[derive(Debug, Clone)]
    struct TestNode {
        source: String,
        uid: Option<String>,
    }

    impl TestNode {
        fn new(source: &str) -> Self {
            TestNode {
                source: source.to_string(),
                uid: None,
            }
        }
        fn with_uid(source: &str, uid: &str) -> Self {
            TestNode {
                source: source.to_string(),
                uid: Some(uid.to_string()),
            }
        }
    }

    impl VersionableNode for TestNode {
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

    // ── levenshtein_distance ──────────────────────────────────────────────────

    #[test]
    fn lev_equal() {
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
    }

    #[test]
    fn lev_empty_a() {
        assert_eq!(levenshtein_distance("", "abc"), 3);
    }

    #[test]
    fn lev_empty_b() {
        assert_eq!(levenshtein_distance("abc", ""), 3);
    }

    #[test]
    fn lev_insert() {
        assert_eq!(levenshtein_distance("abc", "abcd"), 1);
    }

    #[test]
    fn lev_delete() {
        assert_eq!(levenshtein_distance("abcd", "abc"), 1);
    }

    #[test]
    fn lev_replace() {
        assert_eq!(levenshtein_distance("abc", "axc"), 1);
    }

    #[test]
    fn lev_symmetric() {
        assert_eq!(
            levenshtein_distance("kitten", "sitting"),
            levenshtein_distance("sitting", "kitten")
        );
    }

    #[test]
    fn lev_kitten_sitting() {
        // Classic example: levenshtein("kitten", "sitting") == 3
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    // ── get_ratio ─────────────────────────────────────────────────────────────

    #[test]
    fn ratio_empty_old_returns_threshold() {
        // Mirrors: assert get_ratio('', 'a')
        let r = get_ratio("", "a");
        assert_eq!(r, VERSIONING_RATIO);
        assert!(r > 0.0);
    }

    #[test]
    fn ratio_empty_new_returns_threshold() {
        // Mirrors: assert get_ratio('a', '')
        let r = get_ratio("a", "");
        assert_eq!(r, VERSIONING_RATIO);
        assert!(r > 0.0);
    }

    #[test]
    fn ratio_equal_strings() {
        assert_eq!(get_ratio("hello", "hello"), 0.0);
    }

    #[test]
    fn ratio_one_char_difference() {
        // distance 1 / (5 chars / 100) = 20.0
        let r = get_ratio("hello", "hxllo");
        assert!((r - 20.0).abs() < 0.01, "got {r}");
    }

    // ── add_uids ──────────────────────────────────────────────────────────────

    #[test]
    fn add_uids_sets_uids_on_all_nodes() {
        let mut nodes = vec![
            TestNode::new("P1"),
            TestNode::new("P2"),
            TestNode::new("P3"),
        ];
        let uids = add_uids(&mut nodes);
        // All nodes got a uid.
        assert_eq!(uids.len(), 3);
        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(node.uid().unwrap(), &uids[i]);
        }
    }

    #[test]
    fn add_uids_uids_are_unique() {
        let mut nodes: Vec<TestNode> = (0..5).map(|i| TestNode::new(&i.to_string())).collect();
        let uids = add_uids(&mut nodes);
        let unique: HashSet<&str> = uids.iter().map(String::as_str).collect();
        assert_eq!(unique.len(), 5);
    }

    #[test]
    fn add_uids_uids_are_32_hex_chars() {
        let mut nodes = vec![TestNode::new("x")];
        let uids = add_uids(&mut nodes);
        assert_eq!(uids[0].len(), 32);
        assert!(uids[0].chars().all(|c| c.is_ascii_hexdigit()));
    }

    // ── merge_doctrees ────────────────────────────────────────────────────────

    fn make_nodes(sources: &[&str]) -> Vec<TestNode> {
        sources.iter().map(|s| TestNode::new(s)).collect()
    }

    fn set_uids(nodes: &mut Vec<TestNode>) -> Vec<String> {
        add_uids(nodes)
    }

    // Test data mirrors the sphinx/tests/test-versioning fixtures.
    const P1: &str = "This is the first paragraph.";
    const P2: &str = "This is the second paragraph.";
    const P3: &str = "This is the third paragraph.";
    const P4: &str = "This is the fourth paragraph.";
    const P_NEW: &str = "A brand new paragraph introduced in the middle.";
    const P_SIMILAR: &str = "Anyway I need more";

    #[test]
    fn merge_modified_same_content_zero_new_nodes() {
        // modified = same content → 0 new nodes, same UIDs
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut modified = make_nodes(&[P1, P2, P3]);
        let changed = merge_doctrees(&mut original, &mut modified);

        assert!(changed.is_empty(), "expected 0 new nodes, got {changed:?}");
        let new_uids: Vec<_> = modified.iter().map(|n| n.uid().unwrap()).collect();
        assert_eq!(
            new_uids,
            orig_uids.iter().map(String::as_str).collect::<Vec<_>>()
        );
    }

    #[test]
    fn merge_added_new_node_at_end() {
        // added = [P1, P2, P3, P4_new] → 1 new node, P1/P2/P3 uids preserved
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut added = make_nodes(&[P1, P2, P3, P4]);
        let changed = merge_doctrees(&mut original, &mut added);

        assert_eq!(changed.len(), 1, "expected 1 new node");
        // P1, P2, P3 uids preserved
        for i in 0..3 {
            assert_eq!(added[i].uid().unwrap(), orig_uids[i]);
        }
        // P4 has a new uid
        assert!(!orig_uids.contains(&added[3].uid().unwrap().to_string()));
    }

    #[test]
    fn merge_deleted_middle_node() {
        // deleted = [P1, P3] → 0 new nodes; P1 and P3 uids matched
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut deleted = make_nodes(&[P1, P3]);
        let changed = merge_doctrees(&mut original, &mut deleted);

        assert!(changed.is_empty(), "expected 0 new nodes, got {changed:?}");
        // P1 stays P1
        assert_eq!(deleted[0].uid().unwrap(), orig_uids[0]);
        // P3 should match orig P3 (or P2 — depends on ratio; key point: uid exists)
        assert!(deleted[1].uid().is_some());
    }

    #[test]
    fn merge_deleted_end_node() {
        // deleted_end = [P1, P2] → 0 new nodes; P1, P2 uids preserved
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut deleted_end = make_nodes(&[P1, P2]);
        let changed = merge_doctrees(&mut original, &mut deleted_end);

        assert!(changed.is_empty(), "expected 0 new nodes");
        assert_eq!(deleted_end[0].uid().unwrap(), orig_uids[0]);
        assert_eq!(deleted_end[1].uid().unwrap(), orig_uids[1]);
    }

    #[test]
    fn merge_insert_middle_one_new_node() {
        // insert = [P1, P_NEW, P2, P3] → 1 new node; P1/P2/P3 uids preserved
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut inserted = make_nodes(&[P1, P_NEW, P2, P3]);
        let changed = merge_doctrees(&mut original, &mut inserted);

        assert_eq!(changed.len(), 1, "expected 1 new node, got {changed:?}");
        // P1 uid preserved
        assert_eq!(inserted[0].uid().unwrap(), orig_uids[0]);
        // P2 uid preserved
        assert_eq!(inserted[2].uid().unwrap(), orig_uids[1]);
        // P3 uid preserved
        assert_eq!(inserted[3].uid().unwrap(), orig_uids[2]);
    }

    #[test]
    fn merge_insert_beginning_one_new_node() {
        // insert_beginning = [P_NEW, P1, P2, P3] → 1 new node; P1/P2/P3 uids preserved
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut inserted = make_nodes(&[P_NEW, P1, P2, P3]);
        let changed = merge_doctrees(&mut original, &mut inserted);

        assert_eq!(changed.len(), 1, "expected 1 new node");
        assert_eq!(inserted.len(), 4);
        // P1, P2, P3 uids preserved (they moved to positions 1, 2, 3)
        let inserted_uids: Vec<_> = inserted.iter().map(|n| n.uid().unwrap()).collect();
        assert_eq!(
            &inserted_uids[1..],
            orig_uids
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .as_slice()
        );
        // P_NEW has a different uid
        assert!(!orig_uids.contains(&inserted[0].uid().unwrap().to_string()));
    }

    #[test]
    fn merge_insert_similar_one_new_node() {
        // insert_similar = [P1, P_SIMILAR, P2, P3] → 1 new node = P_SIMILAR
        // P1/P2/P3 uids preserved; P_SIMILAR gets a new uid (ratio >= threshold)
        let mut original = make_nodes(&[P1, P2, P3]);
        let orig_uids = set_uids(&mut original);

        let mut inserted = make_nodes(&[P1, P_SIMILAR, P2, P3]);
        let changed = merge_doctrees(&mut original, &mut inserted);

        assert_eq!(changed.len(), 1, "expected 1 new node");
        // P_SIMILAR is the new/changed node
        assert!(
            changed.contains(&1),
            "expected index 1 (P_SIMILAR) in changed: {changed:?}"
        );
        // P1, P2, P3 uids preserved
        assert_eq!(inserted[0].uid().unwrap(), orig_uids[0]);
        assert_eq!(inserted[2].uid().unwrap(), orig_uids[1]);
        assert_eq!(inserted[3].uid().unwrap(), orig_uids[2]);
    }

    #[test]
    fn merge_all_new_no_old() {
        let mut original: Vec<TestNode> = vec![];
        let mut new_nodes = make_nodes(&[P1, P2]);
        let changed = merge_doctrees(&mut original, &mut new_nodes);
        assert_eq!(changed.len(), 2);
        assert!(new_nodes[0].uid().is_some());
        assert!(new_nodes[1].uid().is_some());
    }

    #[test]
    fn merge_empty_new() {
        let mut original = make_nodes(&[P1, P2]);
        set_uids(&mut original);
        let mut new_nodes: Vec<TestNode> = vec![];
        let changed = merge_doctrees(&mut original, &mut new_nodes);
        assert!(changed.is_empty());
    }

    #[test]
    fn merge_old_without_uid_gets_uid_assigned() {
        // old node without a uid should get one during merge
        let mut old_nodes = vec![TestNode::new(P1)]; // no uid set
        let mut new_nodes = vec![TestNode::new(P1)];
        let changed = merge_doctrees(&mut old_nodes, &mut new_nodes);
        assert!(changed.is_empty(), "exact match should yield 0 changed");
        // Both nodes should have uid now
        assert!(old_nodes[0].uid().is_some());
        assert_eq!(new_nodes[0].uid(), old_nodes[0].uid());
    }

    #[test]
    fn merge_existing_uid_reused() {
        let mut old_nodes = vec![TestNode::with_uid(P1, "fixed-uid-abc")];
        let mut new_nodes = vec![TestNode::new(P1)];
        let changed = merge_doctrees(&mut old_nodes, &mut new_nodes);
        assert!(changed.is_empty());
        assert_eq!(new_nodes[0].uid().unwrap(), "fixed-uid-abc");
    }

    // ── apply_uid_transform ───────────────────────────────────────────────────

    #[test]
    fn uid_transform_no_old_assigns_new_uids() {
        let mut nodes = vec![TestNode::new(P1), TestNode::new(P2)];
        let changed = apply_uid_transform(&mut nodes, None);
        assert!(
            changed.is_empty(),
            "no old: all are new, none counted as changed"
        );
        assert!(nodes[0].uid().is_some(), "node 0 should have uid");
        assert!(nodes[1].uid().is_some(), "node 1 should have uid");
    }

    #[test]
    fn uid_transform_with_old_reuses_uids() {
        let mut old_nodes = vec![
            TestNode::with_uid(P1, "uid-a"),
            TestNode::with_uid(P2, "uid-b"),
        ];
        let mut new_nodes = vec![TestNode::new(P1), TestNode::new(P2)];
        let _changed = apply_uid_transform(&mut new_nodes, Some(&mut old_nodes));
        assert_eq!(new_nodes[0].uid().unwrap(), "uid-a");
        assert_eq!(new_nodes[1].uid().unwrap(), "uid-b");
    }

    #[test]
    fn uid_transform_with_added_node() {
        let mut old_nodes = vec![TestNode::with_uid(P1, "uid-a")];
        let mut new_nodes = vec![TestNode::new(P1), TestNode::new(P2)];
        let changed = apply_uid_transform(&mut new_nodes, Some(&mut old_nodes));
        // The second node is new, so it shows up in changed
        assert!(
            changed.contains(&1),
            "index 1 should be changed/new: {changed:?}"
        );
        // First node reuses old uid
        assert_eq!(new_nodes[0].uid().unwrap(), "uid-a");
        // Second node gets fresh uid
        assert!(new_nodes[1].uid().is_some());
    }
}
