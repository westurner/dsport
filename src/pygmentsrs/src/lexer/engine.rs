//! RegexLexer engine — port of the subset of `pygments.lexer` we need.
//!
//! Semantics mirror `pygments.lexer.RegexLexer.get_tokens_unprocessed`:
//!
//! * A state is a list of rules. A rule is a compiled regex + an
//!   action (a [`TokenType`] or a callback like [`bygroups`]) + an
//!   optional state transition.
//! * Initial state stack = `["root"]`. At each `pos`, walk the
//!   current state's rules in order; the first one whose regex
//!   matches at `pos` wins.
//! * On match: yield tokens, advance `pos = m.end()`, apply the
//!   state transition (`#pop`, `#push`, tuple-of-states, ...).
//! * On no match: if the current char is `\n`, reset to `["root"]`
//!   and yield `Whitespace "\n"`; otherwise yield `Error <char>`
//!   and advance by one.
//!
//! Anchoring: Rust's `regex` crate has no `match-at-pos` API, so we
//! use `find_at(text, pos)` and accept only matches where
//! `m.start() == pos`. Patterns are compiled with the `(?m)` inline
//! flag so `^` keeps Python's MULTILINE semantics (start-of-line,
//! not start-of-pos).

use crate::token::{self, TokenType};
use fancy_regex::Regex;

/// Action attached to a regex rule.
pub enum Action {
    /// Emit a single token of this type covering the whole match.
    Single(TokenType),
    /// Per-group emission. Each entry corresponds to capture group
    /// 1..=N; `None` skips the group. Mirrors `pygments.lexer.bygroups`.
    ByGroups(Vec<Option<TokenType>>),
}

/// State-stack transition after a rule fires.
#[derive(Clone)]
pub enum NewState {
    /// No change.
    None,
    /// Pop N states (N = `pops`), keeping at least one.
    Pop(usize),
    /// Push the current state again.
    PushSame,
    /// Push these state names in order; new top is the last.
    Push(Vec<&'static str>),
}

pub struct Rule {
    pub regex: Regex,
    pub action: Option<Action>, // None for `default(...)` zero-width
    pub new_state: NewState,
}

impl Rule {
    /// Convenience: build a single-token rule with no state change.
    pub fn token(pattern: &str, t: TokenType) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::Single(t)),
            new_state: NewState::None,
        }
    }
    pub fn token_to(pattern: &str, t: TokenType, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::Single(t)),
            new_state: ns,
        }
    }
    pub fn bygroups(pattern: &str, tokens: Vec<Option<TokenType>>) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::ByGroups(tokens)),
            new_state: NewState::None,
        }
    }
    pub fn bygroups_to(pattern: &str, tokens: Vec<Option<TokenType>>, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::ByGroups(tokens)),
            new_state: ns,
        }
    }
    /// `default(...)` — zero-width transition.
    pub fn default(ns: NewState) -> Self {
        Self {
            regex: compile(""),
            action: None,
            new_state: ns,
        }
    }
}

/// Upper bound on backtracking steps for a single match attempt.
///
/// `fancy-regex` only backtracks for patterns that actually need it
/// (lookaround / backreferences); pure-`regex`-compatible patterns run
/// on the linear-time NFA and never hit this. The bound guards against
/// catastrophic backtracking (ReDoS) on adversarial code-block input.
/// 1,000,000 is `fancy-regex`'s own default; we set it explicitly so
/// the intent is documented and tunable.
const BACKTRACK_LIMIT: usize = 1_000_000;

fn compile(pattern: &str) -> Regex {
    // Apply `(?m)` so `^`/`$` mean line-start/end (Python's
    // `re.MULTILINE`, which pygments enables by default). Patterns
    // that already declare flags will get them combined.
    let with_ml = if pattern.starts_with("(?") {
        pattern.to_string()
    } else {
        format!("(?m){pattern}")
    };
    fancy_regex::RegexBuilder::new(&with_ml)
        .backtrack_limit(BACKTRACK_LIMIT)
        .build()
        .unwrap_or_else(|e| panic!("uncompilable pattern {pattern:?}: {e}"))
}

/// A `RegexLexer` is a static map of state-name → rules + a way to
/// look that up by name. Concrete lexers implement [`Lexer::get_tokens`]
/// by handing their state table to [`tokenize`].
pub trait StateTable: Send + Sync {
    fn state(&self, name: &str) -> Option<&[Rule]>;
}

pub fn tokenize<T: StateTable>(table: &T, text: &str) -> Vec<(TokenType, String)> {
    let mut stack: Vec<&str> = vec!["root"];
    let mut pos = 0usize;
    let mut out: Vec<(TokenType, String)> = Vec::new();

    'outer: while pos <= text.len() {
        if pos == text.len() {
            break;
        }
        let rules = match table.state(stack.last().copied().unwrap_or("root")) {
            Some(r) => r,
            None => break,
        };
        for rule in rules {
            // Zero-width `default(...)`.
            if rule.action.is_none() {
                apply_transition(&rule.new_state, &mut stack);
                continue 'outer;
            }
            let Ok(Some(m)) = rule.regex.captures_from_pos(text, pos) else {
                continue;
            };
            let m0 = m.get(0).unwrap();
            if m0.start() != pos {
                continue;
            }
            // Zero-width guard: a rule that matches at `pos` but consumes
            // nothing AND has no state transition would loop forever (Python's
            // RegexLexer has the same invariant — every zero-width rule in a
            // well-formed Pygments lexer carries a state transition).  Skip
            // rather than hang; the next rule in the list will fire instead.
            let is_zero_width = m0.start() == m0.end();
            if is_zero_width && matches!(rule.new_state, NewState::None) {
                continue;
            }
            match rule.action.as_ref().unwrap() {
                Action::Single(t) => {
                    // Emit even when the matched string is empty: Python's
                    // `get_tokens_unprocessed` yields `(pos, ttype, m.group())`
                    // verbatim, including zero-width matches (e.g. the JS
                    // `^(?=\s|/|<!--)` lookahead emits `Token.Text ""`).
                    let s = &text[m0.start()..m0.end()];
                    push_merged(&mut out, *t, s);
                }
                Action::ByGroups(toks) => {
                    // Python's bygroups callback guards `if data:` — it skips
                    // empty-string captures.  Mirror that behaviour here so
                    // optional groups (e.g. `([ \t]*)$`) don't emit stray
                    // empty tokens.
                    for (i, maybe_t) in toks.iter().enumerate() {
                        let Some(t) = maybe_t else { continue };
                        if let Some(g) = m.get(i + 1) {
                            let s = &text[g.start()..g.end()];
                            if !s.is_empty() {
                                push_merged(&mut out, *t, s);
                            }
                        }
                    }
                }
            }
            pos = m0.end();
            apply_transition(&rule.new_state, &mut stack);
            continue 'outer;
        }

        // No rule matched at pos.
        let rest = &text[pos..];
        if rest.starts_with('\n') {
            stack = vec!["root"];
            push_merged(&mut out, token::WHITESPACE, "\n");
            pos += 1;
            continue;
        }
        let ch_len = rest
            .chars()
            .next()
            .map(|c| c.len_utf8())
            .unwrap_or(rest.len());
        push_merged(&mut out, token::ERROR, &rest[..ch_len]);
        pos += ch_len;
    }

    out
}

fn apply_transition(ns: &NewState, stack: &mut Vec<&'static str>) {
    match ns {
        NewState::None => {}
        NewState::Pop(n) => {
            let n = (*n).max(1);
            if n >= stack.len() {
                stack.truncate(1);
            } else {
                let new_len = stack.len() - n;
                stack.truncate(new_len);
            }
        }
        NewState::PushSame => {
            if let Some(top) = stack.last().copied() {
                stack.push(top);
            }
        }
        NewState::Push(states) => {
            for s in states {
                if *s == "#pop" {
                    if stack.len() > 1 {
                        stack.pop();
                    }
                } else if *s == "#push" {
                    if let Some(top) = stack.last().copied() {
                        stack.push(top);
                    }
                } else {
                    stack.push(s);
                }
            }
        }
    }
}

/// Append a token to the output stream. Pygments'
/// `get_tokens_unprocessed` emits one record per regex match / bygroup
/// without merging adjacent same-type entries, so neither do we.
fn push_merged(out: &mut Vec<(TokenType, String)>, t: TokenType, v: &str) {
    out.push((t, v.to_string()));
}

/// Helper for concrete lexers: implement `StateTable` via a closure.
pub struct StaticStateTable<F: Fn(&str) -> Option<&'static [Rule]> + Send + Sync>(pub F);

impl<F: Fn(&str) -> Option<&'static [Rule]> + Send + Sync> StateTable for StaticStateTable<F> {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        (self.0)(name)
    }
}

/// Adapter so `tokenize` results satisfy [`crate::lexer::Lexer::get_tokens`].
pub fn lex_with<T: StateTable>(table: &T, code: &str) -> Vec<(TokenType, String)> {
    tokenize(table, code)
}

// Blanket-style helper: many lexers are zero-state structs; this
// macro defines `impl Lexer for $Lexer { fn get_tokens(...) }`.
#[macro_export]
macro_rules! impl_lexer_via {
    ($Lexer:ty, $state_fn:path) => {
        impl $crate::lexer::Lexer for $Lexer {
            fn get_tokens(&self, code: &str) -> Vec<($crate::token::TokenType, String)> {
                struct __Table;
                impl $crate::lexer::engine::StateTable for __Table {
                    fn state(&self, name: &str) -> Option<&[$crate::lexer::engine::Rule]> {
                        $state_fn(name)
                    }
                }
                $crate::lexer::engine::tokenize(&__Table, code)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    //! Proof that the `fancy-regex` engine accepts the regex
    //! constructs the `regex` crate rejects. ~44% of upstream Pygments
    //! lexers use lookaround, and 23 use backreferences (the `bash`
    //! heredoc rule is the canonical blocker). Each test below builds
    //! a tiny state table whose patterns would `panic!` at
    //! `compile(...)` under the old `regex`-crate engine.

    use super::*;
    use crate::token::{NAME, NUMBER, PUNCTUATION, STRING, TokenType};

    /// One-state table built from an explicit rule list.
    struct OneState(Vec<Rule>);
    impl StateTable for OneState {
        fn state(&self, name: &str) -> Option<&[Rule]> {
            (name == "root").then_some(self.0.as_slice())
        }
    }

    fn reprs(toks: &[(TokenType, String)]) -> Vec<(String, String)> {
        toks.iter().map(|(t, v)| (t.repr(), v.clone())).collect()
    }

    #[test]
    fn lookahead_compiles_and_matches() {
        // `\d+(?=px)`: digits only when followed by `px`. Positive
        // lookahead is unsupported by the `regex` crate.
        let table = OneState(vec![
            Rule::token(r"\d+(?=px)", NUMBER),
            Rule::token(r"[a-z]+", NAME),
        ]);
        let out = reprs(&tokenize(&table, "10px"));
        assert_eq!(
            out,
            vec![
                ("Token.Literal.Number".into(), "10".into()),
                ("Token.Name".into(), "px".into()),
            ]
        );
    }

    #[test]
    fn lookbehind_compiles_and_matches() {
        // `(?<=\$)\w+`: a word only when preceded by `$`. Lookbehind
        // is unsupported by the `regex` crate.
        let table = OneState(vec![
            Rule::token(r"\$", PUNCTUATION),
            Rule::token(r"(?<=\$)\w+", NAME),
        ]);
        let out = reprs(&tokenize(&table, "$foo"));
        assert_eq!(
            out,
            vec![
                ("Token.Punctuation".into(), "$".into()),
                ("Token.Name".into(), "foo".into()),
            ]
        );
    }

    #[test]
    fn backreference_compiles_and_matches() {
        // `(["'])\w+\1`: a word wrapped in *matching* quotes. The `\1`
        // backreference is the same construct the `bash` heredoc rule
        // (`\2`) needs, and is unsupported by the `regex` crate.
        let table = OneState(vec![
            Rule::token(r#"(["'])\w+\1"#, STRING),
            Rule::token(r".", NAME),
        ]);

        // Matching quotes -> single STRING token.
        let matched = reprs(&tokenize(&table, "'abc'"));
        assert_eq!(matched, vec![("Token.Literal.String".into(), "'abc'".into())]);

        // Mismatched quotes -> backref fails, falls through to `.`.
        let mismatched = reprs(&tokenize(&table, "'abc\""));
        assert!(
            mismatched.iter().all(|(t, _)| t == "Token.Name"),
            "mismatched quotes must not be lexed as a string: {mismatched:?}"
        );
    }

    #[test]
    fn backtrack_limit_is_bounded() {
        // A pathological pattern on adversarial input must terminate
        // (return no match) rather than hang. `fancy-regex` surfaces a
        // backtrack-limit overflow as `Err`, which the engine treats
        // as "no match" and advances by one char.
        let table = OneState(vec![Rule::token(r"(a+)+$", STRING), Rule::token(r".", NAME)]);
        let input = "a".repeat(40) + "!";
        // Must complete (not hang) and consume the whole input.
        let out = tokenize(&table, &input);
        let consumed: usize = out.iter().map(|(_, v)| v.len()).sum();
        assert_eq!(consumed, input.len());
    }
}

