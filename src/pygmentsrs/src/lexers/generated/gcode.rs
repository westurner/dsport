//! AUTO-GENERATED from `pygments.pygments.lexers.gcodelexer:GcodeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.gcodelexer:GcodeLexer:gcode

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gcode
pub struct GcodeLexer;

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
            Rule::token(r"(?m);.*\n", COMMENT),
            Rule::token(r"(?m)^[gmGM]\d{1,4}\s", NAME_BUILTIN),
            Rule::bygroups(
                r"(?m)([^gGmM])([+-]?\d*[.]?\d+)",
                vec![Some(KEYWORD), Some(NUMBER)],
            ),
            Rule::token(r"(?m)\s", WHITESPACE),
            Rule::token(r"(?m).*\n", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for GcodeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
