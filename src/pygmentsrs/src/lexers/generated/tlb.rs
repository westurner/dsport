#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.tlb:TlbLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tlb:TlbLexer:tlb

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tlb
pub struct TlbLexer;

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
            Rule::token(r"(?m)//.*", TokenType::new(&["Comment", "Singleline"])),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)[0-9]+", NUMBER),
            Rule::token(r"(?m)(!=|(?:[<=>])=|[*+\-.<=>?\^~])", OPERATOR),
            Rule::token(r"(?m)(\#(?:<=|[#<]))", NAME_TAG),
            Rule::token(r"(?m)#[0-9a-f]*_?", NAME_TAG),
            Rule::token(r"(?m)\$[01]*_?", NAME_TAG),
            Rule::token(r"(?m)[a-zA-Z_][0-9a-zA-Z_]*", NAME),
            Rule::token(r"(?m)[;():\[\]{}]", PUNCTUATION),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*", TokenType::new(&["Comment", "Singleline"])),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for TlbLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
