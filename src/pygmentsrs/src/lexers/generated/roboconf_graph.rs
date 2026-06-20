//! AUTO-GENERATED from `pygments.pygments.lexers.roboconf:RoboconfGraphLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.roboconf:RoboconfGraphLexer:roboconf_graph

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: roboconf-graph
pub struct RoboconfGraphLexer;

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
            Rule::token(r"(?im)=", OPERATOR),
            Rule::token(r"(?im)\b((?:face|impor)t)\s*\b", KEYWORD),
            Rule::token(
                r"(?im)\b(children|ex(?:(?:port|tend)s)|facets|i(?:mports|nstaller))\s*:?",
                NAME,
            ),
            Rule::token(r"(?im)#.*\n", COMMENT),
            Rule::token(r"(?im)[^#]", TEXT),
            Rule::token(r"(?im).*\n", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for RoboconfGraphLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
