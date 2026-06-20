#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.textfmts:KernelLogLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textfmts:KernelLogLexer:kmsg

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: kmsg, dmesg
pub struct KmsgLexer;

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
            Rule::token_to(
                r"(?m)^[^:]+:debug : (?=\[)",
                TEXT,
                NewState::Push(vec![r"debug"]),
            ),
            Rule::token_to(
                r"(?m)^[^:]+:info  : (?=\[)",
                TEXT,
                NewState::Push(vec![r"info"]),
            ),
            Rule::token_to(
                r"(?m)^[^:]+:warn  : (?=\[)",
                TEXT,
                NewState::Push(vec![r"warn"]),
            ),
            Rule::token_to(
                r"(?m)^[^:]+:notice: (?=\[)",
                TEXT,
                NewState::Push(vec![r"warn"]),
            ),
            Rule::token_to(
                r"(?m)^[^:]+:err   : (?=\[)",
                TEXT,
                NewState::Push(vec![r"error"]),
            ),
            Rule::token_to(
                r"(?m)^[^:]+:crit  : (?=\[)",
                TEXT,
                NewState::Push(vec![r"error"]),
            ),
            Rule::token_to(r"(?m)^(?=\[)", TEXT, NewState::Push(vec![r"unknown"])),
        ],
    );
    m.insert(
        r"unknown",
        vec![
            Rule::token_to(
                r"(?m)^(?=.+(warning|notice|audit|deprecated))",
                TEXT,
                NewState::Push(vec![r"warn"]),
            ),
            Rule::token_to(
                r"(?m)^(?=.+(error|critical|fail|Bug))",
                TEXT,
                NewState::Push(vec![r"error"]),
            ),
            Rule::default(NewState::Push(vec![r"info"])),
        ],
    );
    m.insert(
        r"base",
        vec![
            Rule::token(r"(?m)\[[0-9. ]+\] ", NUMBER),
            Rule::token(r"(?m)(?<=\] ).+?:", KEYWORD),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"debug",
        vec![
            Rule::token(r"(?m)\[[0-9. ]+\] ", NUMBER),
            Rule::token(r"(?m)(?<=\] ).+?:", KEYWORD),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token_to(r"(?m).+\n", COMMENT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"info",
        vec![
            Rule::token(r"(?m)\[[0-9. ]+\] ", NUMBER),
            Rule::token(r"(?m)(?<=\] ).+?:", KEYWORD),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token_to(r"(?m).+\n", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"warn",
        vec![
            Rule::token(r"(?m)\[[0-9. ]+\] ", NUMBER),
            Rule::token(r"(?m)(?<=\] ).+?:", KEYWORD),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token_to(r"(?m).+\n", GENERIC_STRONG, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"error",
        vec![
            Rule::token(r"(?m)\[[0-9. ]+\] ", NUMBER),
            Rule::token(r"(?m)(?<=\] ).+?:", KEYWORD),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token_to(r"(?m).+\n", GENERIC_ERROR, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for KmsgLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
