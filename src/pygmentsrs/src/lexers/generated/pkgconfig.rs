#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:PkgConfigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:PkgConfigLexer:pkgconfig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pkgconfig
pub struct PkgconfigLexer;

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
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::bygroups(r"(?m)^(\w+)(=)", vec![Some(NAME_ATTRIBUTE), Some(OPERATOR)]),
            Rule::bygroups_to(
                r"(?m)^([\w.]+)(:)",
                vec![Some(NAME_TAG), Some(PUNCTUATION)],
                NewState::Push(vec![r"spvalue"]),
            ),
            Rule::token(r"(?m)\$\$", TEXT),
            Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[^${}#=:\n.]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"interp",
        vec![
            Rule::token(r"(?m)\$\$", TEXT),
            Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        ],
    );
    m.insert(
        r"curly",
        vec![
            Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
            Rule::token(r"(?m)\w+", NAME_ATTRIBUTE),
        ],
    );
    m.insert(
        r"spvalue",
        vec![
            Rule::token(r"(?m)\$\$", TEXT),
            Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
            Rule::token_to(r"(?m)#.*$", COMMENT_SINGLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[^${}#\n\s]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for PkgconfigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
