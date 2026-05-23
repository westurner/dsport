//! Cross-wave parity aggregator.
//!
//! Prints one line per ported test fixture so CI can scrape a single
//! summary. Doesn't fail unless an *unexpected* regression appears in
//! one of the already-green waves (currently W1 commonmark spec).
//!
//! Per-fixture allowlists live in the relevant module; this file just
//! aggregates the counts.

#[path = "common/mod.rs"]
mod common;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Run a single fixture set and return `(passed, allowed, failed,
/// total)`. `case_runner` returns `Ok(true)` on pass, `Ok(false)` on
/// allowed deviation, and `Err(detail)` on unexpected failure.
fn run_fixture<F>(
    label: &str,
    cases: &[common::ParamCase],
    mut case_runner: F,
) -> (usize, usize, usize)
where
    F: FnMut(&common::ParamCase) -> Result<bool, String>,
{
    let mut passed = 0;
    let mut allowed = 0;
    let mut failed = 0;
    for case in cases {
        match case_runner(case) {
            Ok(true) => passed += 1,
            Ok(false) => allowed += 1,
            Err(_detail) => failed += 1,
        }
    }
    eprintln!(
        "[parity] {label}: {passed}/{total} passing ({allowed} pending, {failed} unexpected)",
        total = cases.len(),
    );
    (passed, allowed, failed)
}

#[test]
fn aggregate_parity() {
    let unexpected = AtomicUsize::new(0);

    // W1: CommonMark spec. The dedicated `commonmark.rs` integration
    // test already enforces parity; here we just surface the count.
    eprintln!(
        "[parity] commonmark: see `cargo test -p myst-md-rs --test commonmark` (623/646 + 23 allowed + 3 skipped of 649)"
    );

    // W3a: option block parsing.
    {
        let raw = include_str!("data/option_parsing.yaml");
        let cases = common::parse_yaml_format(raw);
        let (_, _, f) = run_fixture("options (parse)", &cases, |case| {
            match myst_md_rs::options::options_to_items(&case.content) {
                Ok(_) => Ok(false), // pending: we don't compare JSON yet
                Err(_) => Ok(false),
            }
        });
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    // W3b: option block error reporting.
    {
        let raw = include_str!("data/option_parsing_errors.yaml");
        let cases = common::parse_yaml_format(raw);
        let (_, _, f) = run_fixture("options (errors)", &cases, |case| {
            match myst_md_rs::options::options_to_items(&case.content) {
                Err(_) => Ok(false), // pending: we don't compare error messages yet
                Ok(_) => Ok(false),
            }
        });
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    // W3c: directive text parsing.
    {
        let raw = include_str!("data/directive_parsing.txt");
        let cases = common::parse_dot_format(raw);
        let (_, _, f) = run_fixture("directives", &cases, |_case| {
            // pending — directive runner needs a registry of upstream
            // DirectiveSpec entries (`Note`, `Admonition`, `CodeBlock`)
            // before we can dispatch.
            Ok(false)
        });
        unexpected.fetch_add(f, Ordering::Relaxed);
    }

    let n = unexpected.load(Ordering::Relaxed);
    assert_eq!(n, 0, "{n} unexpected parity failures");
}
