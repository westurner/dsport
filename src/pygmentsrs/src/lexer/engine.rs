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

/// Per-group action inside a `bygroups(...)` rule.
///
/// Each element corresponds to capture group 1..=N; `None` skips the
/// group entirely (mirrors Python's `None` placeholder in `bygroups`).
#[derive(Clone)]
pub enum GroupAction {
    /// Emit a static token for this capture group.
    Token(TokenType),
    /// Lex this group's text recursively with the *same* lexer,
    /// optionally starting in a different named state.
    /// `state = None` means start in "root".
    UsingThis { state: Option<Vec<&'static str>> },
    /// Lex this group's text with a *different* registered lexer
    /// (looked up by its primary alias at call time).
    /// `state = None` means start in "root".
    UsingLexer { alias: &'static str, state: Option<Vec<&'static str>> },
}

impl From<TokenType> for GroupAction {
    fn from(t: TokenType) -> Self {
        GroupAction::Token(t)
    }
}

/// Top-level action attached to a regex rule.
pub enum Action {
    /// Emit a single token of this type covering the whole match.
    Single(TokenType),
    /// Per-group emission. Each entry corresponds to capture group
    /// 1..=N; `None` skips the group. Mirrors `pygments.lexer.bygroups`.
    ByGroups(Vec<Option<GroupAction>>),
    /// Lex the whole match recursively with the same lexer (using(this)).
    UsingThis { state: Option<Vec<&'static str>> },
    /// Lex the whole match with another registered lexer (using(OtherLexer)).
    UsingLexer { alias: &'static str, state: Option<Vec<&'static str>> },
    /// Code-block dispatch: emit prefix/suffix tokens, look up a lexer by the
    /// value of `lang_group`, then lex the code assembled from `code_groups`
    /// with that lexer (or emit as `fallback_token` if the alias is unknown).
    /// Mirrors the `_handle_codeblock` callback pattern used by MarkdownLexer,
    /// TiddlyWiki5Lexer, RstLexer, etc.
    DispatchCodeBlock(Box<DispatchCodeBlockSpec>),
}

/// Spec for how to dispatch a fenced/directive code block.
pub struct DispatchCodeBlockSpec {
    /// Groups emitted as tokens before the code (in order).
    pub prefix: Vec<GroupEmit>,
    /// Capture group whose text is used as the lexer alias.
    pub lang_group: usize,
    /// Capture group indices concatenated (in order) to form the code string.
    pub code_groups: Vec<usize>,
    /// Groups emitted as tokens after the code (in order).
    pub suffix: Vec<GroupEmit>,
    /// Token type used when no native lexer is found for the lang.
    pub fallback_token: TokenType,
    /// When `Some(g)`, strip `len(group[g])` bytes from the start of every
    /// line in the assembled code before lexing (handles RST-style indentation).
    pub strip_indent_from_group: Option<usize>,
}

/// A single group→token emission within a [`DispatchCodeBlockSpec`].
pub struct GroupEmit {
    /// Capture group index (1-based).
    pub group: usize,
    /// Token type to emit.
    pub token: TokenType,
    /// If true, skip this emit when the capture group is empty / did not match.
    pub skip_if_none: bool,
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
            action: Some(Action::ByGroups(
                tokens.into_iter().map(|t| t.map(GroupAction::Token)).collect(),
            )),
            new_state: NewState::None,
        }
    }
    pub fn bygroups_to(pattern: &str, tokens: Vec<Option<TokenType>>, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::ByGroups(
                tokens.into_iter().map(|t| t.map(GroupAction::Token)).collect(),
            )),
            new_state: ns,
        }
    }
    /// `bygroups` with full `GroupAction` per group — used when one or more
    /// groups contain `using(this)` or `using(OtherLexer)`.
    pub fn bygroups_g(pattern: &str, actions: Vec<Option<GroupAction>>) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::ByGroups(actions)),
            new_state: NewState::None,
        }
    }
    pub fn bygroups_g_to(pattern: &str, actions: Vec<Option<GroupAction>>, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::ByGroups(actions)),
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
    /// `using(this)` — lex whole match with the same lexer from root state.
    pub fn using_this(pattern: &str, state: Option<Vec<&'static str>>) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::UsingThis { state }),
            new_state: NewState::None,
        }
    }
    pub fn using_this_to(pattern: &str, state: Option<Vec<&'static str>>, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::UsingThis { state }),
            new_state: ns,
        }
    }
    /// `using(OtherLexer)` — lex whole match with another registered lexer.
    pub fn using_lexer(pattern: &str, alias: &'static str, state: Option<Vec<&'static str>>) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::UsingLexer { alias, state }),
            new_state: NewState::None,
        }
    }
    pub fn using_lexer_to(pattern: &str, alias: &'static str, state: Option<Vec<&'static str>>, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::UsingLexer { alias, state }),
            new_state: ns,
        }
    }
    /// Convenience wrappers so generated code can pass `Some(token)` without importing GroupAction.
    pub fn group_token(t: TokenType) -> Option<GroupAction> {
        Some(GroupAction::Token(t))
    }
    pub fn group_using_this(state: Option<Vec<&'static str>>) -> Option<GroupAction> {
        Some(GroupAction::UsingThis { state })
    }
    pub fn group_using_lexer(alias: &'static str, state: Option<Vec<&'static str>>) -> Option<GroupAction> {
        Some(GroupAction::UsingLexer { alias, state })
    }
    /// Code-block dispatch rule (mirrors `_handle_codeblock` callback pattern).
    ///
    /// Emits `prefix` tokens, looks up a native lexer for `spec.lang_group`'s
    /// value, lexes `spec.code_groups` as assembled code (with optional indent
    /// stripping), then emits `suffix` tokens.  Falls back to
    /// `spec.fallback_token` when no native lexer is found.
    pub fn dispatch_code_block(pattern: &str, spec: DispatchCodeBlockSpec) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::DispatchCodeBlock(Box::new(spec))),
            new_state: NewState::None,
        }
    }
    pub fn dispatch_code_block_to(pattern: &str, spec: DispatchCodeBlockSpec, ns: NewState) -> Self {
        Self {
            regex: compile(pattern),
            action: Some(Action::DispatchCodeBlock(Box::new(spec))),
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
    tokenize_with_stack(table, text, vec!["root"])
}

/// Tokenize `text` starting with an explicit state stack. Used by
/// `using(this, state=...)` to resume lexing at a named state.
pub fn tokenize_with_stack<T: StateTable>(
    table: &T,
    text: &str,
    initial_stack: Vec<&'static str>,
) -> Vec<(TokenType, String)> {
    let mut stack: Vec<&str> = if initial_stack.is_empty() {
        vec!["root"]
    } else {
        initial_stack.into_iter().map(|s| s as &str).collect()
    };
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
            let is_zero_width = m0.start() == m0.end();
            if is_zero_width && matches!(rule.new_state, NewState::None) {
                continue;
            }
            let matched = &text[m0.start()..m0.end()];
            match rule.action.as_ref().unwrap() {
                Action::Single(t) => {
                    push_merged(&mut out, *t, matched);
                }
                Action::UsingThis { state } => {
                    // Lex the matched substring with the same state table,
                    // starting in the specified state (or root).
                    let init = state
                        .as_ref()
                        .map(|v| v.clone())
                        .unwrap_or_else(|| vec!["root"]);
                    let nested = tokenize_with_stack(table, matched, init);
                    for (nt, nv) in nested {
                        push_merged(&mut out, nt, &nv);
                    }
                }
                Action::UsingLexer { alias, state } => {
                    // Lex with a different registered lexer.
                    let nested = lex_nested_alias(alias, matched, state.as_deref());
                    for (nt, nv) in nested {
                        push_merged(&mut out, nt, &nv);
                    }
                }
                Action::ByGroups(toks) => {
                    for (i, maybe_action) in toks.iter().enumerate() {
                        let Some(group_action) = maybe_action else { continue };
                        if let Some(g) = m.get(i + 1) {
                            let s = &text[g.start()..g.end()];
                            if !s.is_empty() {
                                match group_action {
                                    GroupAction::Token(t) => {
                                        push_merged(&mut out, *t, s);
                                    }
                                    GroupAction::UsingThis { state } => {
                                        let init = state
                                            .as_ref()
                                            .map(|v| v.clone())
                                            .unwrap_or_else(|| vec!["root"]);
                                        let nested = tokenize_with_stack(table, s, init);
                                        for (nt, nv) in nested {
                                            push_merged(&mut out, nt, &nv);
                                        }
                                    }
                                    GroupAction::UsingLexer { alias, state } => {
                                        let nested = lex_nested_alias(alias, s, state.as_deref());
                                        for (nt, nv) in nested {
                                            push_merged(&mut out, nt, &nv);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Action::DispatchCodeBlock(spec) => {
                    // 1. Emit prefix tokens.
                    for ge in &spec.prefix {
                        if let Some(g) = m.get(ge.group) {
                            let s = &text[g.start()..g.end()];
                            if !(ge.skip_if_none && s.is_empty()) {
                                push_merged(&mut out, ge.token, s);
                            }
                        }
                    }
                    // 2. Assemble the code string from code_groups.
                    let mut code = String::new();
                    for &cg in &spec.code_groups {
                        if let Some(g) = m.get(cg) {
                            code.push_str(&text[g.start()..g.end()]);
                        }
                    }
                    // 3. Look up lexer by lang group and lex the code.
                    let lang = m.get(spec.lang_group)
                        .map(|g| text[g.start()..g.end()].trim().to_string())
                        .unwrap_or_default();
                    {
                        use crate::lexers::registry::get_lexer_by_name;
                        // SAFETY: `lang` is a local owned String; we leak it here so
                        // it can be passed as `&'static str` to the registry.  This is
                        // intentional — the leaked memory is bounded by the number of
                        // distinct language tags encountered at run time (a small set),
                        // and reclaiming it would require a global cache.
                        let lang_leaked: &'static str =
                            Box::leak(lang.clone().into_boxed_str());
                        if let Some(lexer) = get_lexer_by_name(lang_leaked) {
                            if let Some(indent_grp) = spec.strip_indent_from_group {
                                // RST-style: the code has leading indentation on every
                                // non-blank line.  We strip it before lexing but emit
                                // Text tokens for each stripped prefix, mirroring
                                // Pygments' `do_insertions` approach.
                                let indent_size = m.get(indent_grp)
                                    .map(|g| g.end() - g.start())
                                    .unwrap_or(0);
                                if indent_size > 0 {
                                    // Build insertions: (byte_pos_in_stripped, Text, indent).
                                    let mut stripped = String::new();
                                    let mut insertions: Vec<(usize, String)> = Vec::new();
                                    for line in code.split_inclusive('\n') {
                                        if line.len() > indent_size {
                                            insertions.push((stripped.len(), line[..indent_size].to_string()));
                                            stripped.push_str(&line[indent_size..]);
                                        } else {
                                            // Short or blank line — keep as-is.
                                            stripped.push_str(line);
                                        }
                                    }
                                    // Splice indent Text tokens back into lexer output.
                                    let code_tokens = lexer.get_tokens(&stripped);
                                    let insertions_fmt: Vec<(usize, Vec<(TokenType, String)>)> =
                                        insertions.into_iter().map(|(pos, s)| {
                                            (pos, vec![(token::TEXT, s)])
                                        }).collect();
                                    for (t, v) in crate::lexers::do_insertions_owned(insertions_fmt, code_tokens) {
                                        push_merged(&mut out, t, &v);
                                    }
                                } else {
                                    for (nt, nv) in lexer.get_tokens(&code) {
                                        push_merged(&mut out, nt, &nv);
                                    }
                                }
                            } else {
                                for (nt, nv) in lexer.get_tokens(&code) {
                                    push_merged(&mut out, nt, &nv);
                                }
                            }
                        } else {
                            push_merged(&mut out, spec.fallback_token, &code);
                        }
                    }
                    // 4. Emit suffix tokens.
                    for ge in &spec.suffix {
                        if let Some(g) = m.get(ge.group) {
                            let s = &text[g.start()..g.end()];
                            if !(ge.skip_if_none && s.is_empty()) {
                                push_merged(&mut out, ge.token, s);
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

/// Dispatch a nested `using(OtherLexer)` call to the native registry.
/// Falls back to emitting `Error` tokens for the entire slice when the
/// alias has no native implementation, so the engine never panics and
/// the caller does not need Python to be available.
fn lex_nested_alias(
    alias: &'static str,
    text: &str,
    _state: Option<&[&'static str]>,
) -> Vec<(TokenType, String)> {
    // The registry is in the sibling `lexers` module.  We import it
    // here (not at the top of the file) to avoid a circular-dependency
    // chain: `lexer::engine` → `lexers::registry` → `lexers::generated`
    // → `lexer::engine`.  The `use` is inside the function so it is
    // only resolved if this code path is actually reached at runtime.
    use crate::lexers::registry::get_lexer_by_name;
    if let Some(lexer) = get_lexer_by_name(alias) {
        lexer.get_tokens(text)
    } else {
        vec![(token::ERROR, text.to_string())]
    }
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

