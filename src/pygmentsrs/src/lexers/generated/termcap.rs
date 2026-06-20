//! AUTO-GENERATED from `pygments.pygments.lexers.configs:TermcapLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:TermcapLexer:termcap

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: termcap
pub struct TermcapLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)^#.*", COMMENT),
            Rule::token_to(r"(?m)^[^\s#:|]+", NAME_TAG, NewState::Push(vec![r"names"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"names",
        vec![
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"defs"])),
            Rule::token(r"(?m)\|", PUNCTUATION),
            Rule::token(r"(?m)[^:|]+", NAME_ATTRIBUTE),
        ],
    );
    m.insert(
        r"defs",
        vec![
            Rule::bygroups(r"(?m)(\\)(\n[ \t]*)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token_to(r"(?m)\n[ \t]*", WHITESPACE, NewState::Pop(2)),
            Rule::bygroups(r"(?m)(#)([0-9]+)", vec![Some(OPERATOR), Some(NUMBER)]),
            Rule::token_to(r"(?m)=", OPERATOR, NewState::Push(vec![r"data"])),
            Rule::token(r"(?m):", PUNCTUATION),
            Rule::token(r"(?m)[^\s:=#]+", NAME_CLASS),
        ],
    );
    m.insert(
        r"data",
        vec![
            Rule::token(r"(?m)\\072", LITERAL),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[^:\\]+", LITERAL),
            Rule::token(r"(?m).", LITERAL),
        ],
    );
    Table(m)
}

impl Lexer for TermcapLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
