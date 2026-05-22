//! Minimal Python lexer — Phase 1 starter.
//!
//! This is *not* a full port of `pygments.lexers.python.PythonLexer`
//! (1211 lines of patterns). It is a small subset covering the
//! handoff fixtures from `docs/handoff/pygments.md` so the engine
//! and the end-to-end pipeline can be exercised. Byte-parity for
//! arbitrary Python input is tracked in
//! `src/pygmentsrs/docs/compat.md` and lands incrementally as
//! fixtures move from accepted-deviation to byte-parity.
//!
//! Covers (matches `pygments.lex(..., PythonLexer())` byte-for-byte
//! for these shapes):
//!
//! * keywords (`def`, `return`, `if`, `else`, `for`, `in`, `not`,
//!   `and`, `or`, `class`, `import`, `from`, `as`, `pass`, `yield`,
//!   `lambda`, `True`, `False`, `None`)
//! * function-definition name (`def NAME` → `Name.Function`)
//! * integer literals
//! * single-quoted and double-quoted string literals (no escapes)
//! * `#` line comments
//! * whitespace, punctuation, operators
//!
//! Anything else falls through to `Name`. Gaps (f-strings, triple
//! strings, decorators, etc.) are explicit non-goals for this
//! starter and produce divergence from upstream — parity tests are
//! restricted to the covered fixture set.

use crate::lexer::Lexer;
use crate::lexer::engine::{self, Rule, StateTable};
use crate::token::{self, TokenType};
use std::sync::OnceLock;

pub struct PythonLexer;

fn rules_root() -> &'static [Rule] {
    static R: OnceLock<Vec<Rule>> = OnceLock::new();
    R.get_or_init(|| {
        vec![
            // whitespace
            Rule::token(r"[ \t]+", token::WHITESPACE),
            Rule::token(r"\n", token::WHITESPACE),
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
