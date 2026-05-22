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
use regex::Regex;

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
            regex: Regex::new("").unwrap(),
            action: None,
            new_state: ns,
        }
    }
}

fn compile(pattern: &str) -> Regex {
    // Apply `(?m)` so `^`/`$` mean line-start/end (Python's
    // `re.MULTILINE`, which pygments enables by default). Patterns
    // that already declare flags will get them combined.
    let with_ml = if pattern.starts_with("(?") {
        pattern.to_string()
    } else {
        format!("(?m){pattern}")
    };
    Regex::new(&with_ml).unwrap_or_else(|e| panic!("uncompilable pattern {pattern:?}: {e}"))
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
            let Some(m) = rule.regex.captures_at(text, pos) else {
                continue;
            };
            let m0 = m.get(0).unwrap();
            if m0.start() != pos {
                continue;
            }
            match rule.action.as_ref().unwrap() {
                Action::Single(t) => {
                    let s = &text[m0.start()..m0.end()];
                    if !s.is_empty() {
                        push_merged(&mut out, *t, s);
                    }
                }
                Action::ByGroups(toks) => {
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

/// Pygments merges adjacent tokens of the same type *implicitly* via
/// the consumer (`get_tokens` collapses them). Doing it here keeps
/// our output identical to `list(pygments.lex(...))`.
fn push_merged(out: &mut Vec<(TokenType, String)>, t: TokenType, v: &str) {
    if let Some(last) = out.last_mut()
        && last.0 == t
    {
        last.1.push_str(v);
        return;
    }
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
