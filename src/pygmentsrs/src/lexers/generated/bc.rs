//! AUTO-GENERATED from `pygments.pygments.lexers.algebra:BCLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.algebra:BCLexer:bc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bc
pub struct BcLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r#"(?m)"(?:[^"\\]|\\.)*""#, STRING),
        Rule::token(r"(?m)[{}();,]", PUNCTUATION),
        Rule::token(r"(?m)(auto|break|continue|define|else|for|halt|if|l(?:ength|imits)|print|quit|re(?:ad|turn)|s(?:cale|qrt)|w(?:arranty|hile))\b", KEYWORD),
        Rule::token(r"(?m)\+\+|--|\|\||&&|([-<>+*%\^/!=])=?", OPERATOR),
        Rule::token(r"(?m)[0-9]+(\.[0-9]*)?", NUMBER),
        Rule::token(r"(?m)\.[0-9]+", NUMBER),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for BcLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
