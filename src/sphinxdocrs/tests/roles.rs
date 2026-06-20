//! Integration tests for `sphinxdocrs::roles`.
//!
//! Mirrors the pure-algorithm subset of `sphinx/tests/test_roles.py`.

use rstest::*;

use sphinxdocrs::roles::{
    DefaultRoleConfig, EmphasizedSpan, GENERIC_DOCROLES, SPECIFIC_DOCROLES, XRefRoleConfig,
    format_rfc_target, is_builtin_role, parse_emphasized_literal,
};

// ── GENERIC_DOCROLES ──────────────────────────────────────────────────────────

#[test]
fn generic_docroles_all_present() {
    let expected = [
        ("command", "literal_strong"),
        ("dfn", "emphasis"),
        ("mailheader", "literal_emphasis"),
        ("makevar", "literal_strong"),
        ("mimetype", "literal_emphasis"),
        ("newsgroup", "literal_emphasis"),
        ("program", "literal_strong"),
        ("regexp", "literal"),
    ];
    for (name, kind) in &expected {
        let found = GENERIC_DOCROLES.iter().find(|(n, _)| n == name);
        assert!(found.is_some(), "missing generic role: {name}");
        assert_eq!(found.unwrap().1, *kind, "wrong kind for {name}");
    }
    assert_eq!(GENERIC_DOCROLES.len(), expected.len());
}

// ── SPECIFIC_DOCROLES ─────────────────────────────────────────────────────────

#[test]
fn specific_docroles_all_present() {
    let expected = [
        "download",
        "any",
        "cve",
        "cwe",
        "pep",
        "rfc",
        "guilabel",
        "menuselection",
        "file",
        "samp",
        "abbr",
        "kbd",
        "manpage",
    ];
    for name in &expected {
        assert!(
            SPECIFIC_DOCROLES.contains(name),
            "missing specific role: {name}"
        );
    }
}

// ── is_builtin_role ───────────────────────────────────────────────────────────

#[rstest]
#[case("command", true)]
#[case("dfn", true)]
#[case("pep", true)]
#[case("rfc", true)]
#[case("samp", true)]
#[case("kbd", true)]
#[case("manpage", true)]
#[case("nonexistent_role", false)]
#[case("", false)]
fn is_builtin_role_cases(#[case] name: &str, #[case] expected: bool) {
    assert_eq!(is_builtin_role(name), expected, "is_builtin_role({name:?})");
}

// ── format_rfc_target ─────────────────────────────────────────────────────────

// Mirrors test__format_rfc_target from test_roles.py.

#[rstest]
#[case("1", "0001")]
#[case("12", "0012")]
#[case("123", "0123")]
#[case("1234", "1234")]
#[case("12345", "12345")]
#[case("1234#s5", "1234#s5")]
#[case("42#anchor", "0042#anchor")]
fn format_rfc_target_cases(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(format_rfc_target(input), expected, "input={input:?}");
}

// ── EmphasizedLiteral — parse_emphasized_literal ──────────────────────────────

// Mirrors test_samp() from test_roles.py.

#[test]
fn samp_normal_case() {
    // 'print 1+{variable}' → [Text('print 1+'), Var('variable')]
    let spans = parse_emphasized_literal("print 1+{variable}");
    assert_eq!(
        spans,
        vec![
            EmphasizedSpan::Text("print 1+".into()),
            EmphasizedSpan::Var("variable".into()),
        ]
    );
}

#[test]
fn samp_two_vars() {
    // 'print {1}+{variable}'
    let spans = parse_emphasized_literal("print {1}+{variable}");
    assert_eq!(
        spans,
        vec![
            EmphasizedSpan::Text("print ".into()),
            EmphasizedSpan::Var("1".into()),
            EmphasizedSpan::Text("+".into()),
            EmphasizedSpan::Var("variable".into()),
        ]
    );
}

#[test]
fn samp_empty_braces_plain_text() {
    // 'print 1+{}' → plain text (empty variable = not a variable)
    let spans = parse_emphasized_literal("print 1+{}");
    assert_eq!(spans, vec![EmphasizedSpan::Text("print 1+{}".into())]);
}

#[test]
fn samp_unclosed_brace_plain_text() {
    // 'print 1+{variable' (no closing '}') → plain text
    let spans = parse_emphasized_literal("print 1+{variable");
    assert_eq!(
        spans,
        vec![EmphasizedSpan::Text("print 1+{variable".into())]
    );
}

#[test]
fn samp_only_var() {
    let spans = parse_emphasized_literal("{host}");
    assert_eq!(spans, vec![EmphasizedSpan::Var("host".into())]);
}

#[test]
fn samp_no_vars() {
    let spans = parse_emphasized_literal("plain text");
    assert_eq!(spans, vec![EmphasizedSpan::Text("plain text".into())]);
}

#[test]
fn samp_empty_input() {
    assert!(parse_emphasized_literal("").is_empty());
}

#[test]
fn samp_multiple_vars_no_text_between() {
    let spans = parse_emphasized_literal("{a}{b}");
    assert_eq!(
        spans,
        vec![
            EmphasizedSpan::Var("a".into()),
            EmphasizedSpan::Var("b".into()),
        ]
    );
}

// ── XRefRoleConfig ────────────────────────────────────────────────────────────

#[test]
fn xref_role_config_defaults() {
    let r = XRefRoleConfig::new();
    assert!(!r.fix_parens);
    assert!(!r.lowercase);
    assert!(!r.warn_dangling);
    assert_eq!(r.nodeclass, "pending_xref");
    assert_eq!(r.innernodeclass, "literal");
}

#[test]
fn xref_role_any_config() {
    let r = XRefRoleConfig {
        warn_dangling: true,
        ..XRefRoleConfig::new()
    };
    assert!(r.warn_dangling);
}

#[test]
fn xref_role_download_nodeclass() {
    let r = XRefRoleConfig {
        nodeclass: "download_reference",
        ..XRefRoleConfig::new()
    };
    assert_eq!(r.nodeclass, "download_reference");
}

// ── DefaultRoleConfig ─────────────────────────────────────────────────────────

#[test]
fn default_role_unset_by_default() {
    let r = DefaultRoleConfig::default();
    assert!(!r.is_set());
    assert!(r.name.is_none());
}

#[test]
fn default_role_set_to_any() {
    let r = DefaultRoleConfig::new("any");
    assert!(r.is_set());
    assert_eq!(r.name.as_deref(), Some("any"));
}

#[test]
fn default_role_empty_is_unset() {
    let r = DefaultRoleConfig::new("");
    assert!(!r.is_set());
}
