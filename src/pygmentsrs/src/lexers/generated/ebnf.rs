#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.parsers:EbnfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.parsers:EbnfLexer:ebnf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ebnf
pub struct EbnfLexer;

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
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)\(\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)([a-zA-Z][\w \-]*)", KEYWORD),
            Rule::token_to(r"(?m)=", OPERATOR, NewState::Push(vec![r"production"])),
        ],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", TEXT)]);
    m.insert(
        r"comment_start",
        vec![Rule::token_to(
            r"(?m)\(\*",
            COMMENT_MULTILINE,
            NewState::Push(vec![r"comment"]),
        )],
    );
    m.insert(
        r"identifier",
        vec![Rule::token(r"(?m)([a-zA-Z][\w \-]*)", KEYWORD)],
    );
    m.insert(
        r"production",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)\(\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)([a-zA-Z][\w \-]*)", KEYWORD),
            Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
            Rule::token(r"(?m)'[^']*'", STRING_SINGLE),
            Rule::token(r"(?m)(\?[^?]*\?)", NAME_ENTITY),
            Rule::token(r"(?m)[\[\]{}(),|]", PUNCTUATION),
            Rule::token(r"(?m)-", OPERATOR),
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\.", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*)]", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?m)\(\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token_to(r"(?m)\*\)", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*)]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for EbnfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
