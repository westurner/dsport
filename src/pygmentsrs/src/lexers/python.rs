//! Subset of `pygments.lexers.python.PythonLexer` — covers everything
//! the docutilsrs `code`/`code-block`/`sourcecode` byte-parity gate
//! exercises today, no more. Full byte-parity for arbitrary Python
//! input is tracked in `src/pygmentsrs/docs/compat.md` and lands
//! incrementally as fixtures graduate.
//!
//! **Byte-parity (`tokens ✅`)** vs `pygments.lex(..., PythonLexer())`
//! for the 9 `code_block_python_*` fixtures in
//! `src/tests/test_parity_pseudoxml.py`:
//!
//! * `def NAME(...)` bygroups → `Keyword` + `Whitespace` + `Name.Function`
//! * `class NAME` bygroups → `Keyword` + `Whitespace` + `Name.Class`
//! * `from MOD import NAME` 7-group bygroups (whitespaces before
//!   each name are `Whitespace`, the whitespace *between* `import`
//!   and the imported name is plain `Text`, and the imported `NAME`
//!   is `Name`, not `Name.Namespace`) — matches upstream exactly
//! * bare `import MOD` / `from MOD` bygroups → `Keyword.Namespace`
//!   + `Whitespace` + `Name.Namespace`
//! * `True` / `False` / `None` → `Keyword.Constant` (ordered before
//!   the generic keyword rule so it wins)
//! * the standard `\b(?:and|as|...|yield)\b` keyword set
//! * single- and double-quoted strings (single-line, no escape
//!   handling — sufficient for the current fixtures)
//! * `\b[0-9]+\b` → `Number.Integer`
//! * arithmetic / comparison / bitwise operators
//! * punctuation `[()\[\]{},:;.]`
//! * `#`-to-EOL comments → `Comment.Single`
//! * **whitespace bifurcation matching upstream**: `\n` →
//!   `Token.Text.Whitespace`, horizontal `[ \t]+` → `Token.Text`.
//!   Without this split, indentation after a newline merges into
//!   the preceding whitespace token and breaks docutils' `<inline
//!   classes="whitespace">` boundaries.
//!
//! **Anything not in the list above** falls through to the bare-name
//! rule (`Name`) or — if no rule matches — to the engine's
//! `Error <char>` fallback. Known gaps (not yet exercised, will
//! diverge from upstream): f-strings and the `fstring`/`fstring_inner`
//! state machine, triple-quoted strings, raw/byte string prefixes,
//! escape sequences inside strings, numeric variants (hex/oct/bin/
//! float/complex), decorators, type-hint syntax, `match`/`case`
//! soft keywords, and line-continuation backslashes.

use crate::lexer::Lexer;
use crate::lexer::engine::{self, Rule, StateTable};
use crate::token::{self, TokenType};
use std::sync::OnceLock;

pub struct PythonLexer;

fn rules_root() -> &'static [Rule] {
    static R: OnceLock<Vec<Rule>> = OnceLock::new();
    R.get_or_init(|| {
        vec![
            // whitespace — match pygments: `\n` is Whitespace, but
            // bare horizontal whitespace is plain Text. Inline `def`/
            // `class` bygroups emit Whitespace explicitly for the
            // space between the keyword and the name (matches upstream).
            Rule::token(r"\n", token::WHITESPACE),
            Rule::token(r"[ \t]+", token::TEXT),
            // comments
            Rule::token(r"#[^\n]*", token::COMMENT_SINGLE),
            // `def NAME` and `class NAME` — emit keyword + ws + name
            Rule::bygroups(
                r"(def)([ \t]+)([A-Za-z_][A-Za-z0-9_]*)",
                vec![
                    Some(token::KEYWORD),
                    Some(token::WHITESPACE),
                    Some(token::NAME_FUNCTION),
                ],
            ),
            Rule::bygroups(
                r"(class)([ \t]+)([A-Za-z_][A-Za-z0-9_]*)",
                vec![
                    Some(token::KEYWORD),
                    Some(token::WHITESPACE),
                    Some(token::NAME_CLASS),
                ],
            ),
            // `from MOD import NAME` — multi-bygroups matching
            // upstream's tokenization (whitespace=Whitespace for the
            // first two gaps, but the gap *after* the inner `import`
            // is plain Text, and the imported `NAME` is `Name`, not
            // `Name.Namespace`).
            Rule::bygroups(
                r"(from)([ \t]+)([A-Za-z_][A-Za-z0-9_.]*)([ \t]+)(import)([ \t]+)([A-Za-z_][A-Za-z0-9_]*)",
                vec![
                    Some(token::KEYWORD_NAMESPACE),
                    Some(token::WHITESPACE),
                    Some(token::NAME_NAMESPACE),
                    Some(token::WHITESPACE),
                    Some(token::KEYWORD_NAMESPACE),
                    Some(token::TEXT),
                    Some(token::NAME),
                ],
            ),
            // bare `import os` / `from os` — whitespace gap is
            // `Token.Text.Whitespace` (upstream short-name "whitespace").
            Rule::bygroups(
                r"(import|from)([ \t]+)([A-Za-z_][A-Za-z0-9_.]*)",
                vec![
                    Some(token::KEYWORD_NAMESPACE),
                    Some(token::WHITESPACE),
                    Some(token::NAME_NAMESPACE),
                ],
            ),
            // keyword constants (must precede the general keyword rule
            // so that `True`/`False`/`None` map to `Keyword.Constant`,
            // matching pygments' PythonLexer).
            Rule::token(r"\b(?:True|False|None)\b", token::KEYWORD_CONSTANT),
            // keywords (Keyword)
            Rule::token(
                r"\b(?:False|None|True|and|as|assert|async|await|break|class|continue|def|del|elif|else|except|finally|for|from|global|if|import|in|is|lambda|nonlocal|not|or|pass|raise|return|try|while|with|yield)\b",
                token::KEYWORD,
            ),
            // strings (single line, no escapes — Phase 1 limitation)
            Rule::token(r#""[^"\n]*""#, token::STRING_DOUBLE),
            Rule::token(r#"'[^'\n]*'"#, token::STRING_SINGLE),
            // numbers
            Rule::token(r"\b[0-9]+\b", token::NUMBER_INTEGER),
            // operators
            Rule::token(r"==|!=|<=|>=|->|\*\*|//|<<|>>|[+\-*/%<>=&|^~@]", token::OPERATOR),
            // punctuation
            Rule::token(r"[()\[\]{},:;.]", token::PUNCTUATION),
            // bare names
            Rule::token(r"[A-Za-z_][A-Za-z0-9_]*", token::NAME),
        ]
    })
}

struct Table;
impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        match name {
            "root" => Some(rules_root()),
            _ => None,
        }
    }
}

impl Lexer for PythonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        engine::tokenize(&Table, code)
    }
}

// (end)
