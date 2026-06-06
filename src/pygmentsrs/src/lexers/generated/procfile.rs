//! AUTO-GENERATED from `pygments.pygments.lexers.procfile:ProcfileLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.procfile:ProcfileLexer:procfile

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: procfile
pub struct ProcfileLexer;

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
        Rule::bygroups(r"(?m)^([a-z]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token(r"(?m)'[^']*'", STRING),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$[a-zA-Z_][\w]*", NAME_VARIABLE),
        Rule::bygroups(r"(?m)(\w+)(=)(\w+)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(STRING)]),
        Rule::token(r"(?m)([\w\-\./]+)", TEXT),
    ]);
    Table(m)
}

impl Lexer for ProcfileLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
