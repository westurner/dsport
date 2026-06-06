//! AUTO-GENERATED from `pygments.pygments.lexers.configs:PacmanConfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:PacmanConfLexer:pacmanconf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pacmanconf
pub struct PacmanconfLexer;

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
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)^(\s*)(\[.*?\])(\s*)$", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\w+)(\s*)(=)", vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)^(\s*)(\w+)(\s*)$", vec![Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(WHITESPACE)]),
        Rule::token(r"(?m)(\$(?:arch|repo)|%(?:[ou]))\b", NAME_VARIABLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for PacmanconfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
