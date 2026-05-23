//! CommonMark spec parity runner.
//!
//! Runs every example from `tests/data/commonmark.json` (CommonMark 0.29 as
//! dumped by upstream `commonmark/CommonMark`) through
//! `myst_md_rs::render_html` and prints a pass/fail summary. Failures are
//! collected into a single aggregated assertion so the suite reports
//! `N/M passing` instead of generating hundreds of independent test entries.
//!
//! Known deviations are recorded in `docs/compat.md` under the
//! `myst-md-rs::commonmark` heading; the `ALLOWLIST` constant below mirrors
//! that list so adding/removing a deviation is a single-file edit.

use std::collections::BTreeSet;

#[path = "common/mod.rs"]
#[allow(dead_code)]
mod common;

#[derive(serde::Deserialize)]
struct SpecCase {
    example: u32,
    markdown: String,
    html: String,
    section: String,
}

/// Examples upstream MyST skips outright. We honour the same skips because
/// the underlying disagreement is between CommonMark and the YAML-front-matter
/// flavour MyST ships with, not a pulldown-cmark vs commonmark.js issue.
const SKIP: &[u32] = &[
    14, // tests that `+++` is *not* a thematic break — conflicts with TOML fm.
    66, // thematic break on line 1 conflicts with `---` front matter.
    68, // same as 66.
];

/// Examples we expect to fail vs. CommonMark 0.29 because of well-understood
/// renderer/parser differences in our pulldown-cmark backend. Each entry has
/// a one-line entry in `docs/compat.md`. Keep this list sorted.
const ALLOWLIST: &[u32] = &[
    // pulldown-cmark omits a blank line inside HTML blocks split by list items.
    61, 144,
    // pulldown-cmark does not escape `"` in body text to `&quot;`.
    // (cmark-the-spec says yes, cmark-the-impl matches, pulldown-cmark differs
    // on purpose — see https://github.com/raphlinus/pulldown-cmark/issues/640.)
    178, 179, 180, 298, 300, 313, 327, 343, 352, 358, 362, 379, 384, 394, 504, 586, 615, 616, 620,
    623, 629,
];

#[test]
fn commonmark_spec_parity() {
    let raw = include_str!("data/commonmark.json");
    let cases: Vec<SpecCase> = serde_json::from_str(raw).expect("valid spec json");
    let total = cases.len();

    let skip: BTreeSet<u32> = SKIP.iter().copied().collect();
    let allow: BTreeSet<u32> = ALLOWLIST.iter().copied().collect();

    let mut passed = 0usize;
    let mut skipped = 0usize;
    let mut allowed = 0usize;
    let mut unexpected: Vec<(u32, String, String, String)> = Vec::new();

    for case in &cases {
        if skip.contains(&case.example) {
            skipped += 1;
            continue;
        }
        let out = myst_md_rs::render_html(&case.markdown);
        if out == case.html {
            passed += 1;
        } else if allow.contains(&case.example) {
            allowed += 1;
        } else {
            unexpected.push((case.example, case.section.clone(), case.html.clone(), out));
        }
    }

    // Always print the summary so `cargo test -- --nocapture` shows progress.
    eprintln!(
        "commonmark spec: {passed}/{run} passing ({skipped} skipped, {allowed} allowed-deviation, {fail} unexpected) out of {total}",
        run = total - skipped,
        fail = unexpected.len(),
    );

    if !unexpected.is_empty() {
        // Limit per-failure output so a fresh bootstrap run is readable.
        let preview = 10.min(unexpected.len());
        let mut msg = format!(
            "{} unexpected CommonMark spec failures (showing first {preview}):\n",
            unexpected.len()
        );
        for (ex, section, expected, actual) in unexpected.iter().take(preview) {
            msg.push_str(&format!(
                "\n--- example {ex} [{section}] ---\nexpected:\n{expected}\nactual:\n{actual}\n"
            ));
        }
        msg.push_str("\nIDs of all failing examples:\n");
        let ids: Vec<String> = unexpected.iter().map(|(e, ..)| e.to_string()).collect();
        msg.push_str(&ids.join(", "));
        panic!("{msg}");
    }
}
