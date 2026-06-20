#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.text:GroffLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.text:GroffLexer:groff

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: groff, nroff, man
pub struct GroffLexer;

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
            Rule::bygroups_to(
                r"(?m)(\.)(\w+)",
                vec![Some(TEXT), Some(KEYWORD)],
                NewState::Push(vec![r"request"]),
            ),
            Rule::token_to(r"(?m)\.", PUNCTUATION, NewState::Push(vec![r"request"])),
            Rule::token_to(r"(?m)[^\\\n]+", TEXT, NewState::Push(vec![r"textline"])),
            Rule::default(NewState::Push(vec![r"textline"])),
        ],
    );
    m.insert(
        r"textline",
        vec![
            Rule::token(r#"(?m)\\"[^\n]*"#, COMMENT),
            Rule::token(r"(?m)\\[fn]\w", STRING_ESCAPE),
            Rule::token(r"(?m)\\\(.{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\.\[.*\]", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r"(?m)\\\n", TEXT, NewState::Push(vec![r"request"])),
            Rule::token(r"(?m)[^\\\n]+", TEXT),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"escapes",
        vec![
            Rule::token(r#"(?m)\\"[^\n]*"#, COMMENT),
            Rule::token(r"(?m)\\[fn]\w", STRING_ESCAPE),
            Rule::token(r"(?m)\\\(.{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\.\[.*\]", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r"(?m)\\\n", TEXT, NewState::Push(vec![r"request"])),
        ],
    );
    m.insert(
        r"request",
        vec![
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token(r#"(?m)\\"[^\n]*"#, COMMENT),
            Rule::token(r"(?m)\\[fn]\w", STRING_ESCAPE),
            Rule::token(r"(?m)\\\(.{2}", STRING_ESCAPE),
            Rule::token(r"(?m)\\.\[.*\]", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r"(?m)\\\n", TEXT, NewState::Push(vec![r"request"])),
            Rule::token(r#"(?m)"[^\n"]+""#, STRING_DOUBLE),
            Rule::token(r"(?m)\d+", NUMBER),
            Rule::token(r"(?m)\S+", STRING),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for GroffLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
