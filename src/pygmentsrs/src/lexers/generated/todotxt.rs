//! AUTO-GENERATED from `pygments.pygments.lexers.textfmts:TodotxtLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textfmts:TodotxtLexer:todotxt

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: todotxt
pub struct TodotxtLexer;

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
        Rule::bygroups_to(r"(?m)(x )(\d{4,}-\d{2}-\d{2})( )(\d{4,}-\d{2}-\d{2})", vec![Some(OPERATOR), Some(GENERIC_SUBHEADING), Some(OPERATOR), Some(GENERIC_SUBHEADING)], NewState::Push(vec![r"complete"])),
        Rule::bygroups_to(r"(?m)(x )(\d{4,}-\d{2}-\d{2})", vec![Some(OPERATOR), Some(GENERIC_SUBHEADING)], NewState::Push(vec![r"complete"])),
        Rule::bygroups_to(r"(?m)(\([A-Z]\))( )(\d{4,}-\d{2}-\d{2})", vec![Some(GENERIC_HEADING), Some(TEXT), Some(GENERIC_SUBHEADING)], NewState::Push(vec![r"incomplete"])),
        Rule::token_to(r"(?m)\([A-Z]\)", GENERIC_HEADING, NewState::Push(vec![r"incomplete"])),
        Rule::token_to(r"(?m)\d{4,}-\d{2}-\d{2}", GENERIC_SUBHEADING, NewState::Push(vec![r"incomplete"])),
        Rule::token_to(r"(?m)@\S+", STRING, NewState::Push(vec![r"incomplete"])),
        Rule::token_to(r"(?m)\+\S+", GENERIC_ERROR, NewState::Push(vec![r"incomplete"])),
        Rule::token_to(r"(?m)\S+", TEXT, NewState::Push(vec![r"incomplete"])),
    ]);
    m.insert(r"complete", vec![
        Rule::token_to(r"(?m)\s*\n", OPERATOR, NewState::Pop(1)),
        Rule::token(r"(?m)@\S+", STRING),
        Rule::token(r"(?m)\+\S+", GENERIC_ERROR),
        Rule::token(r"(?m)\S+", OPERATOR),
        Rule::token(r"(?m)\s+", OPERATOR),
    ]);
    m.insert(r"incomplete", vec![
        Rule::token_to(r"(?m)\s*\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)@\S+", STRING),
        Rule::token(r"(?m)\+\S+", GENERIC_ERROR),
        Rule::token(r"(?m)\S+", TEXT),
        Rule::token(r"(?m)\s+", TEXT),
    ]);
    Table(m)
}

impl Lexer for TodotxtLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
