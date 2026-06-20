//! AUTO-GENERATED from `pygments.pygments.lexers.configs:LighttpdConfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:LighttpdConfLexer:lighttpd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: lighttpd, lighty
pub struct LighttpdLexer;

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
            Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)/\S*", NAME),
            Rule::token(r"(?m)[a-zA-Z._-]+", KEYWORD),
            Rule::token(r"(?m)\d+\.\d+\.\d+\.\d+(?:/\d+)?", NUMBER),
            Rule::token(r"(?m)[0-9]+", NUMBER),
            Rule::token(r"(?m)=>|=~|\+=|==|=|\+", OPERATOR),
            Rule::token(r"(?m)\$[A-Z]+", NAME_BUILTIN),
            Rule::token(r"(?m)[(){}\[\],]", PUNCTUATION),
            Rule::token(r#"(?m)"([^"\\]*(?:\\.[^"\\]*)*)""#, STRING_DOUBLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for LighttpdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
