#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.rita:RitaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rita:RitaLexer:rita

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rita
pub struct RitaLexer;

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
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#(.*?)\n", COMMENT_SINGLE),
            Rule::token(r"(?m)@(.*?)\n", OPERATOR),
            Rule::token(r#"(?m)"(\w|\d|\s|(\\")|[\'_\-./,\?\!])+?""#, LITERAL),
            Rule::token(r#"(?m)\'(\w|\d|\s|(\\\')|["_\-./,\?\!])+?\'"#, LITERAL),
            Rule::token(r"(?m)([A-Z_]+)", KEYWORD),
            Rule::token(r"(?m)([a-z0-9_]+)", NAME),
            Rule::token(r"(?m)((->)|[!?+*|=])", OPERATOR),
            Rule::token(r"(?m)[\(\),\{\}]", PUNCTUATION),
        ],
    );
    Table(m)
}

impl Lexer for RitaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
