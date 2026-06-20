#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.roboconf:RoboconfInstancesLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.roboconf:RoboconfInstancesLexer:roboconf_instances

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: roboconf-instances
pub struct RoboconfInstancesLexer;

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
            Rule::token(r"(?im)\s+", TEXT),
            Rule::token(r"(?im)\b(i(?:mport|nstance\ of))\s*\b", KEYWORD),
            Rule::token(r"(?im)\b(count|name)s*:?", NAME),
            Rule::token(r"(?im)\s*[\w.-]+\s*:", NAME),
            Rule::token(r"(?im)#.*\n", COMMENT),
            Rule::token(r"(?im)[^#]", TEXT),
            Rule::token(r"(?im).*\n", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for RoboconfInstancesLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
