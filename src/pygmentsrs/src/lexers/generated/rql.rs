//! AUTO-GENERATED from `pygments.pygments.lexers.sql:RqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:RqlLexer:rql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rql
pub struct RqlLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)(DELETE|SET|INSERT|UNION|DISTINCT|WITH|WHERE|BEING|OR|AND|NOT|GROUPBY|HAVING|ORDERBY|ASC|DESC|LIMIT|OFFSET|TODAY|NOW|TRUE|FALSE|NULL|EXISTS)\b", KEYWORD),
        Rule::token(r"(?im)[+*/<>=%-]", OPERATOR),
        Rule::token(r"(?im)(Any|is|instance_of|CWEType|CWRelation)\b", NAME_BUILTIN),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)[A-Z_]\w*\??", NAME),
        Rule::token(r"(?im)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r#"(?im)"(""|[^"])*""#, STRING_SINGLE),
        Rule::token(r"(?im)[;:()\[\],.]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for RqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
