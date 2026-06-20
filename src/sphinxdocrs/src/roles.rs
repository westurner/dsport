//! `sphinxdocrs::roles` — Rust port of `sphinx.roles`.
//!
//! Pure-algorithm and registry portions of Sphinx's role system.
//! Parts that require the live docutils pipeline (inliner, document,
//! environment) are deferred to when the full parser is wired.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! | --- | --- | --- |
//! | `generic_docroles` | [`GENERIC_DOCROLES`] | role-name → node-kind string |
//! | `specific_docroles` keys | [`SPECIFIC_DOCROLES`] | role names for special roles |
//! | `_format_rfc_target` | [`format_rfc_target`] | format RFC number with padding |
//! | `EmphasizedLiteral` parser | [`parse_emphasized_literal`] | split `{var}` in text |
//! | `XRefRole` config | [`XRefRoleConfig`] | options for cross-ref roles |
//! | `default_role` | [`DefaultRoleConfig`] | encapsulate the default-role name |
//!
//! **Deferred** (needs docutils pipeline): `XRefRole.run`, `CVE.run`,
//! `PEP.run`, `RFC.run`, `GUILabel.run`, `MenuSelection.run`,
//! `Abbreviation.run`, `Keyboard.run`, `Manpage.run`, `code_role`.

// use crate::addnodes::{DownloadReference, LiteralEmphasis, LiteralStrong, PendingXref};

// ── role name registries ──────────────────────────────────────────────────────

/// Mapping from generic role name to the node kind it produces.
///
/// Mirrors `sphinx.roles.generic_docroles`.
///
/// These roles wrap their content in the named node with a matching CSS class.
pub const GENERIC_DOCROLES: &[(&str, &str)] = &[
    ("command", "literal_strong"),
    ("dfn", "emphasis"),
    ("mailheader", "literal_emphasis"),
    ("makevar", "literal_strong"),
    ("mimetype", "literal_emphasis"),
    ("newsgroup", "literal_emphasis"),
    ("program", "literal_strong"),
    ("regexp", "literal"),
];

/// Names of the specific (non-generic) roles registered by `sphinx.roles`.
///
/// Mirrors the keys of `sphinx.roles.specific_docroles`.
pub const SPECIFIC_DOCROLES: &[&str] = &[
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

/// Return `true` if `name` is a built-in Sphinx role.
///
/// Covers both `generic_docroles` and `specific_docroles`.
pub fn is_builtin_role(name: &str) -> bool {
    GENERIC_DOCROLES.iter().any(|(n, _)| *n == name) || SPECIFIC_DOCROLES.contains(&name)
}

// ── _format_rfc_target ────────────────────────────────────────────────────────

/// Format an RFC number, padding it to at least 4 digits.
///
/// Mirrors `sphinx.roles._format_rfc_target`.
///
/// ```rust
/// use sphinxdocrs::roles::format_rfc_target;
/// assert_eq!(format_rfc_target("1"), "0001");
/// assert_eq!(format_rfc_target("42"), "0042");
/// assert_eq!(format_rfc_target("1234"), "1234");
/// assert_eq!(format_rfc_target("12345"), "12345");
/// // Anchors after '#' are preserved.
/// assert_eq!(format_rfc_target("1234#section-5"), "1234#section-5");
/// ```
pub fn format_rfc_target(target: &str) -> String {
    // Mirrors: if '#' in target: ... else: target.zfill(4)
    if let Some(pos) = target.find('#') {
        let (num, anchor) = target.split_at(pos);
        format!("{:0>4}{anchor}", num)
    } else {
        format!("{:0>4}", target)
    }
}

// ── EmphasizedLiteral parser ──────────────────────────────────────────────────

/// A span within an `EmphasizedLiteral` role text.
///
/// Mirrors the output of `EmphasizedLiteral._parse`.
#[derive(Debug, Clone, PartialEq)]
pub enum EmphasizedSpan {
    /// Plain text segment.
    Text(String),
    /// An `{emphasized}` variable segment.
    Var(String),
}

/// Parse an `EmphasizedLiteral` (`:samp:` / `:file:`) role text into spans.
///
/// Mirrors `sphinx.roles.EmphasizedLiteral._parse`:
/// - Text between `{` and `}` becomes a [`EmphasizedSpan::Var`].
/// - Everything else is a [`EmphasizedSpan::Text`].
/// - Incomplete `{var` (no closing `}`) is treated as plain text.
/// - Empty `{}` is treated as plain text.
///
/// ```rust
/// use sphinxdocrs::roles::{parse_emphasized_literal, EmphasizedSpan};
/// let spans = parse_emphasized_literal("print 1+{variable}");
/// assert_eq!(spans, vec![
///     EmphasizedSpan::Text("print 1+".into()),
///     EmphasizedSpan::Var("variable".into()),
/// ]);
///
/// let spans = parse_emphasized_literal("print {}");
/// assert_eq!(spans, vec![EmphasizedSpan::Text("print {}".into())]);
///
/// let spans = parse_emphasized_literal("no vars");
/// assert_eq!(spans, vec![EmphasizedSpan::Text("no vars".into())]);
/// ```
pub fn parse_emphasized_literal(text: &str) -> Vec<EmphasizedSpan> {
    let mut spans = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            // Collect until '}' or end of string.
            let mut var = String::new();
            let mut closed = false;
            for v in chars.by_ref() {
                if v == '}' {
                    closed = true;
                    break;
                }
                var.push(v);
            }
            if closed && !var.is_empty() {
                // Flush plain text.
                if !current.is_empty() {
                    spans.push(EmphasizedSpan::Text(std::mem::take(&mut current)));
                }
                spans.push(EmphasizedSpan::Var(var));
            } else {
                // Not closed or empty — treat as literal text.
                current.push('{');
                current.push_str(&var);
                if !closed {
                    // No closing brace found — include nothing extra.
                } else {
                    // Empty braces `{}` — add the `}` back.
                    current.push('}');
                }
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        spans.push(EmphasizedSpan::Text(current));
    }

    spans
}

// ── XRefRole config ───────────────────────────────────────────────────────────

/// Configuration for an `XRefRole` instance.
///
/// Mirrors the constructor arguments of `sphinx.roles.XRefRole`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XRefRoleConfig {
    /// Add `()` to the title for callable objects.
    pub fix_parens: bool,
    /// Lowercase the target.
    pub lowercase: bool,
    /// Emit a warning if the cross-reference cannot be resolved.
    pub warn_dangling: bool,
    /// The node class for the reference (default: `pending_xref`).
    pub nodeclass: &'static str,
    /// The inner node class for the text (default: `literal`).
    pub innernodeclass: &'static str,
}

impl XRefRoleConfig {
    /// Construct with the same defaults as upstream `XRefRole()`.
    pub fn new() -> Self {
        Self {
            nodeclass: "pending_xref",
            innernodeclass: "literal",
            ..Default::default()
        }
    }
}

// ── DefaultRoleConfig ─────────────────────────────────────────────────────────

/// Configuration for the RST default role.
///
/// Mirrors `sphinx.util.rst.default_role` (the `name` argument).
/// The actual role function lookup is deferred until the role registry
/// is wired (G5 completion); this struct captures the configuration.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DefaultRoleConfig {
    /// The role name to use as the default (e.g. `"any"`, `"py:obj"`).
    /// If `None`, the default is unset (empty string in Python).
    pub name: Option<String>,
}

impl DefaultRoleConfig {
    pub fn new(name: impl Into<String>) -> Self {
        let s = name.into();
        if s.is_empty() {
            Self { name: None }
        } else {
            Self { name: Some(s) }
        }
    }

    /// Return `true` if a default role is configured.
    pub fn is_set(&self) -> bool {
        self.name.is_some()
    }
}

// ── inline tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── GENERIC_DOCROLES ──────────────────────────────────────────────────────

    #[test]
    fn generic_docroles_contains_command() {
        assert!(GENERIC_DOCROLES.iter().any(|(n, _)| *n == "command"));
    }

    #[test]
    fn generic_docroles_command_is_literal_strong() {
        let kind = GENERIC_DOCROLES
            .iter()
            .find(|(n, _)| *n == "command")
            .unwrap()
            .1;
        assert_eq!(kind, "literal_strong");
    }

    #[test]
    fn generic_docroles_dfn_is_emphasis() {
        let kind = GENERIC_DOCROLES
            .iter()
            .find(|(n, _)| *n == "dfn")
            .unwrap()
            .1;
        assert_eq!(kind, "emphasis");
    }

    #[test]
    fn generic_docroles_count() {
        assert_eq!(GENERIC_DOCROLES.len(), 8);
    }

    // ── SPECIFIC_DOCROLES ─────────────────────────────────────────────────────

    #[test]
    fn specific_docroles_contains_pep() {
        assert!(SPECIFIC_DOCROLES.contains(&"pep"));
    }

    #[test]
    fn specific_docroles_contains_rfc() {
        assert!(SPECIFIC_DOCROLES.contains(&"rfc"));
    }

    #[test]
    fn specific_docroles_contains_samp() {
        assert!(SPECIFIC_DOCROLES.contains(&"samp"));
    }

    // ── is_builtin_role ───────────────────────────────────────────────────────

    #[test]
    fn is_builtin_role_command() {
        assert!(is_builtin_role("command"));
    }

    #[test]
    fn is_builtin_role_pep() {
        assert!(is_builtin_role("pep"));
    }

    #[test]
    fn is_builtin_role_unknown() {
        assert!(!is_builtin_role("nonexistent_role"));
    }

    // ── format_rfc_target ─────────────────────────────────────────────────────

    #[test]
    fn format_rfc_target_short() {
        assert_eq!(format_rfc_target("1"), "0001");
    }

    #[test]
    fn format_rfc_target_two_digits() {
        assert_eq!(format_rfc_target("42"), "0042");
    }

    #[test]
    fn format_rfc_target_four_digits() {
        assert_eq!(format_rfc_target("1234"), "1234");
    }

    #[test]
    fn format_rfc_target_five_digits() {
        assert_eq!(format_rfc_target("12345"), "12345");
    }

    #[test]
    fn format_rfc_target_with_anchor() {
        assert_eq!(format_rfc_target("1234#section-5"), "1234#section-5");
    }

    #[test]
    fn format_rfc_target_short_with_anchor() {
        assert_eq!(format_rfc_target("42#s2"), "0042#s2");
    }

    // ── parse_emphasized_literal ──────────────────────────────────────────────

    #[test]
    fn emph_literal_basic() {
        // Mirrors: 'print 1+{variable}' → [Text('print 1+'), Var('variable')]
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
    fn emph_literal_two_vars() {
        // Mirrors: 'print {1}+{variable}'
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
    fn emph_literal_empty_braces() {
        // Mirrors: 'print 1+{}' → plain text
        let spans = parse_emphasized_literal("print 1+{}");
        assert_eq!(spans, vec![EmphasizedSpan::Text("print 1+{}".into())]);
    }

    #[test]
    fn emph_literal_half_opened() {
        // Mirrors: 'print 1+{variable' (no closing brace) → plain text
        let spans = parse_emphasized_literal("print 1+{variable");
        assert_eq!(
            spans,
            vec![EmphasizedSpan::Text("print 1+{variable".into())]
        );
    }

    #[test]
    fn emph_literal_no_vars() {
        let spans = parse_emphasized_literal("no vars here");
        assert_eq!(spans, vec![EmphasizedSpan::Text("no vars here".into())]);
    }

    #[test]
    fn emph_literal_only_var() {
        let spans = parse_emphasized_literal("{host}");
        assert_eq!(spans, vec![EmphasizedSpan::Var("host".into())]);
    }

    #[test]
    fn emph_literal_empty_string() {
        let spans = parse_emphasized_literal("");
        assert!(spans.is_empty());
    }

    // ── XRefRoleConfig ────────────────────────────────────────────────────────

    #[test]
    fn xref_role_defaults() {
        let r = XRefRoleConfig::new();
        assert!(!r.fix_parens);
        assert!(!r.lowercase);
        assert!(!r.warn_dangling);
        assert_eq!(r.nodeclass, "pending_xref");
        assert_eq!(r.innernodeclass, "literal");
    }

    #[test]
    fn xref_role_any_warns() {
        let r = XRefRoleConfig {
            warn_dangling: true,
            ..XRefRoleConfig::new()
        };
        assert!(r.warn_dangling);
    }

    // ── DefaultRoleConfig ─────────────────────────────────────────────────────

    #[test]
    fn default_role_unset() {
        let r = DefaultRoleConfig::default();
        assert!(!r.is_set());
    }

    #[test]
    fn default_role_set() {
        let r = DefaultRoleConfig::new("py:obj");
        assert!(r.is_set());
        assert_eq!(r.name.as_deref(), Some("py:obj"));
    }

    #[test]
    fn default_role_empty_string_unsets() {
        let r = DefaultRoleConfig::new("");
        assert!(!r.is_set());
    }
}
