#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:RedcodeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:RedcodeLexer:redcode

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: redcode
pub struct RedcodeLexer;

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
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m);.*$", COMMENT_SINGLE),
            Rule::token(
                r"(?m)\b(DAT|MOV|ADD|SUB|MUL|DIV|MOD|JMP|JMZ|JMN|DJN|CMP|SLT|SPL|ORG|EQU|END)\b",
                NAME_FUNCTION,
            ),
            Rule::token(r"(?m)\b(A|B|AB|BA|F|X|I)\b", NAME_DECORATOR),
            Rule::token(r"(?m)[A-Za-z_]\w+", NAME),
            Rule::token(r"(?m)[-+*/%]", OPERATOR),
            Rule::token(r"(?m)[#$@<>]", OPERATOR),
            Rule::token(r"(?m)[.,]", PUNCTUATION),
            Rule::token(r"(?m)[-+]?\d+", NUMBER_INTEGER),
        ],
    );
    Table(m)
}

impl Lexer for RedcodeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
