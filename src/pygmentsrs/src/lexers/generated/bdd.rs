//! AUTO-GENERATED from `pygments.pygments.lexers.bdd:BddLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.bdd:BddLexer:bdd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bdd
pub struct BddLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"comments", vec![
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
    ]);
    m.insert(r"miscellaneous", vec![
        Rule::token(r"(?m)(<|>|\[|\]|=|\||:|\(|\)|\{|\}|,|\.|;|-|_|\$)", PUNCTUATION),
        Rule::token(r"(?m)((?<=\<)[^\\>]+(?=\>))", NAME_VARIABLE),
        Rule::token(r#"(?m)"([^\"]*)""#, STRING),
        Rule::token(r"(?m)^@\S+", NAME_LABEL),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\n|\s+", WHITESPACE),
        Rule::token(r"(?m)Given|When|Then|Add|And|Feature|Scenario Outline|Scenario|Background|Examples|But", KEYWORD),
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
        Rule::token(r"(?m)(<|>|\[|\]|=|\||:|\(|\)|\{|\}|,|\.|;|-|_|\$)", PUNCTUATION),
        Rule::token(r"(?m)((?<=\<)[^\\>]+(?=\>))", NAME_VARIABLE),
        Rule::token(r#"(?m)"([^\"]*)""#, STRING),
        Rule::token(r"(?m)^@\S+", NAME_LABEL),
        Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER),
        Rule::token(r"(?m)\S+", TEXT),
    ]);
    Table(m)
}

impl Lexer for BddLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
