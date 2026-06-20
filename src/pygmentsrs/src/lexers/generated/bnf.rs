#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.grammar_notation:BnfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.grammar_notation:BnfLexer:bnf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bnf
pub struct BnfLexer;

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
            Rule::bygroups(
                r"(?m)(<)([ -;=?-~]+)(>)",
                vec![Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)::=", OPERATOR),
            Rule::token(r"(?m)[^<>:]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for BnfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
