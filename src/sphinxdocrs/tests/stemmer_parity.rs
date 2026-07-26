//! Parity tests for [`sphinxdocrs::stemmer`] against Python's
//! `snowballstemmer`, the package every `sphinx.search.SearchLanguage` uses.
//!
//! Two layers:
//!
//! 1. **Always on** — [`fixture_parity_en`] replays the `en` section of
//!    `tests/fixtures/stemmer_parity.json` and must match exactly: English is
//!    Sphinx's default `search_language`, and [`sphinxdocrs::stemmer::ParityOverrides::builtin`]
//!    was built by diffing `rust_stemmers` against `snowballstemmer` over
//!    ~9800 words, so it is expected to be exact. [`other_languages_advisory`]
//!    reports (without failing) how the other fourteen languages compare over
//!    their own stopword vocabulary — see the accepted-deviation note in
//!    `sphinxdocrs::stemmer`.
//! 2. **`test-parity` feature** — [`live_parity`] regenerates the reference
//!    from the installed `snowballstemmer` over a large English vocabulary,
//!    catching drift between `rust_stemmers` and whichever `snowballstemmer`
//!    release is present.
//!
//! Any word the two disagree on can be pinned through
//! `stemmer::ParityOverrides` without forking the algorithm.

#![cfg(feature = "search-stemming")]

use std::collections::BTreeMap;

use rstest::*;
use sphinxdocrs::stemmer::{NON_SNOWBALL_LANGUAGES, ParityOverrides, SEARCH_LANGUAGES, Stemmer};

const FIXTURE: &str = include_str!("fixtures/stemmer_parity.json");

type Table = BTreeMap<String, BTreeMap<String, String>>;

#[fixture]
#[once]
fn reference() -> Table {
    serde_json::from_str(FIXTURE).expect("stemmer_parity.json is valid JSON")
}

/// Every Snowball language Sphinx ships must be represented in the fixture.
#[rstest]
fn fixture_covers_every_search_language(reference: &Table) {
    for (code, _) in SEARCH_LANGUAGES {
        assert!(reference.contains_key(*code), "missing language {code}");
    }
    assert_eq!(reference.len(), SEARCH_LANGUAGES.len());
}

/// `rust_stemmers` must reproduce `snowballstemmer` exactly for English, the
/// default `search_language` and the only algorithm audited against a large
/// (~9800-word) vocabulary.
#[rstest]
fn fixture_parity_en(reference: &Table) {
    let stemmer = Stemmer::english();
    let mut mismatches: Vec<String> = Vec::new();
    for (word, expected) in &reference["en"] {
        let got = stemmer.stem(word);
        if &got != expected {
            mismatches.push(format!("{word} -> {got} (python: {expected})"));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} English stem mismatches:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
}

/// Report how the other fourteen languages compare against their own stopword
/// vocabulary. Not a hard gate: those algorithms have not been individually
/// audited the way English has (see `sphinxdocrs::stemmer`'s accepted
/// deviations), and are expected to need project-supplied `ParityOverrides`
/// entries to reach full parity. Run with `--nocapture` to see the report.
#[rstest]
fn other_languages_advisory(reference: &Table) {
    for (code, words) in reference {
        if code == "en" {
            continue;
        }
        let stemmer = Stemmer::new(code);
        let mismatches: Vec<String> = words
            .iter()
            .filter_map(|(word, expected)| {
                let got = stemmer.stem(word);
                (&got != expected).then(|| format!("  {word} -> {got} (python: {expected})"))
            })
            .collect();
        println!(
            "{code}: {}/{} words match snowballstemmer",
            words.len() - mismatches.len(),
            words.len()
        );
        for line in &mismatches {
            println!("{line}");
        }
    }
}

/// Languages without a Snowball algorithm must not silently stem.
#[rstest]
#[case("ja", "ドキュメント")]
#[case("zh", "文档")]
fn non_snowball_languages_pass_their_own_script_through(#[case] code: &str, #[case] word: &str) {
    assert!(NON_SNOWBALL_LANGUAGES.contains(&code));
    assert_eq!(Stemmer::new(code).stem(word), word);
}

/// An override must win over the algorithm, which is what makes full parity
/// achievable even if a future `rust_stemmers` release drifts.
#[rstest]
fn overrides_restore_parity_for_a_diverging_word(reference: &Table) {
    let stemmer = Stemmer::english();
    assert_eq!(stemmer.stem("running"), "run");

    let mut overrides = ParityOverrides::new();
    overrides.insert("en", "running", "runn");
    let pinned = Stemmer::english().with_overrides(overrides);
    assert_eq!(pinned.stem("running"), "runn");

    // Unpinned words still follow the fixture.
    let expected = &reference["en"]["runs"];
    assert_eq!(&pinned.stem("runs"), expected);
}

// ── live parity against the installed snowballstemmer ─────────────────────────

/// Regenerate the reference table from Python and compare over a large
/// vocabulary. Requires `python` with `snowballstemmer` importable.
#[cfg(feature = "test-parity")]
#[test]
fn live_parity() {
    use std::process::Command;

    const SCRIPT: &str = r#"
import json, pathlib, re, sys
import snowballstemmer

LANGS = {
    "da": "danish", "de": "german", "en": "english", "es": "spanish",
    "fi": "finnish", "fr": "french", "hu": "hungarian", "it": "italian",
    "nl": "dutch", "no": "norwegian", "pt": "portuguese", "ro": "romanian",
    "ru": "russian", "sv": "swedish", "tr": "turkish",
}

root = pathlib.Path(sys.argv[1])
sw_dir = root / "src/sphinx/sphinx/search/_stopwords"

prose = set()
for p in (root / "src/sphinx/sphinx").rglob("*.py"):
    prose.update(w.lower() for w in re.findall(r"[^\W\d_]+", p.read_text(errors="ignore"), re.UNICODE))
prose = {w for w in prose if 1 <= len(w) <= 30}

out = {}
for code, algo in LANGS.items():
    vocab = set(prose)
    txt = sw_dir / (code + ".txt")
    if txt.exists():
        vocab.update(
            w.strip().lower()
            for w in txt.read_text(encoding="utf-8").split()
            if w.strip() and not w.startswith("|")
        )
    s = snowballstemmer.stemmer(algo)
    out[code] = {w: s.stemWord(w) for w in sorted(vocab)}

json.dump(out, sys.stdout, ensure_ascii=False)
"#;

    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let output = Command::new("python")
        .arg("-c")
        .arg(SCRIPT)
        .arg(root)
        .output()
        .expect("python must be on PATH for the test-parity feature");
    assert!(
        output.status.success(),
        "reference generation failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let reference: Table =
        serde_json::from_slice(&output.stdout).expect("reference table is valid JSON");

    // English is audited byte-for-byte: this is the vocabulary
    // `ParityOverrides::builtin` was derived from, so it must be exact.
    let mut en_mismatches: Vec<String> = Vec::new();
    let en_stemmer = Stemmer::english();
    let en_words = &reference["en"];
    for (word, expected) in en_words {
        let got = en_stemmer.stem(word);
        if &got != expected {
            en_mismatches.push(format!("en: {word} -> {got} (python: {expected})"));
        }
    }
    assert!(
        en_mismatches.is_empty(),
        "{} of {} English stems differ from snowballstemmer:\n{}",
        en_mismatches.len(),
        en_words.len(),
        en_mismatches
            .iter()
            .take(50)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );

    // The other fourteen languages have not been individually audited (see
    // `sphinxdocrs::stemmer`'s accepted deviations); report their live parity
    // rate without failing the build.
    for (code, words) in &reference {
        if code == "en" {
            continue;
        }
        let stemmer = Stemmer::new(code);
        let mismatches = words
            .iter()
            .filter(|(word, expected)| &stemmer.stem(word) != *expected)
            .count();
        println!(
            "{code}: {}/{} words match snowballstemmer",
            words.len() - mismatches,
            words.len()
        );
    }
}
