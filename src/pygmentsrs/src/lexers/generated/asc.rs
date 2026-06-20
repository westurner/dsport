//! AUTO-GENERATED from `pygments.pygments.lexers.asc:AscLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asc:AscLexer:asc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: asc, pem
pub struct AscLexer;

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
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)^-----BEGIN [^\n]+-----$",
                GENERIC_HEADING,
                NewState::Push(vec![r"data"]),
            ),
            Rule::token(r"(?m)\S+", COMMENT),
        ],
    );
    m.insert(
        r"data",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)^([^:]+)(:)([ \t]+)(.*)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::token_to(
                r"(?m)^-----END [^\n]+-----$",
                GENERIC_HEADING,
                NewState::Push(vec![r"root"]),
            ),
            Rule::token(r"(?m)\S+", STRING),
        ],
    );
    Table(m)
}

impl Lexer for AscLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
