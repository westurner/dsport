#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dylan:DylanLidLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dylan:DylanLidLexer:dylan_lid

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dylan-lid, lid
pub struct DylanLidLexer;

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
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?im)(//.*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"(?im)(.*?)(:)([ \t]*)(.*(?:\n[ \t].+)*)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
        ],
    );
    Table(m)
}

impl Lexer for DylanLidLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
