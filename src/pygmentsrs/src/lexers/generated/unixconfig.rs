//! AUTO-GENERATED from `pygments.pygments.lexers.configs:UnixConfigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:UnixConfigLexer:unixconfig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: unixconfig, linuxconfig
pub struct UnixconfigLexer;

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
        Rule::token(r"(?m)^#.*", COMMENT),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token(r"(?m)[0-9]+", NUMBER),
        Rule::token(r"(?m)((?!\n)[a-zA-Z0-9\_\-\s\(\),]){2,}", TEXT),
        Rule::token(r"(?m)[^:\n]+", STRING),
    ]);
    Table(m)
}

impl Lexer for UnixconfigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
