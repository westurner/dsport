//! AUTO-GENERATED from `pygments.pygments.lexers.markup:BBCodeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:BBCodeLexer:bbcode

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bbcode
pub struct BbcodeLexer;

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
        Rule::token(r"(?m)[^\[]+", TEXT),
        Rule::token_to(r"(?m)\[/?\w+", KEYWORD, NewState::Push(vec![r"tag"])),
        Rule::token(r"(?m)\[", TEXT),
    ]);
    m.insert(r"tag", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::bygroups(r#"(?m)(\w+)(=)("?[^\s"\]]+"?)"#, vec![Some(NAME_ATTRIBUTE), Some(OPERATOR), Some(STRING)]),
        Rule::bygroups(r#"(?m)(=)("?[^\s"\]]+"?)"#, vec![Some(OPERATOR), Some(STRING)]),
        Rule::token_to(r"(?m)\]", KEYWORD, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for BbcodeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
