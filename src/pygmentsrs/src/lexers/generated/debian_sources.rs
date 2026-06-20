#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.installers:DebianSourcesLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.installers:DebianSourcesLexer:debian_sources

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: debian.sources
pub struct DebianSourcesLexer;

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
            Rule::bygroups_to(
                r"(?m)^(Signed-By)(:)(\s*)",
                vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"signed-by"]),
            ),
            Rule::bygroups(
                r"(?m)^([a-zA-Z\-0-9\.]*?)(:)(\s*)(.*?)$",
                vec![
                    Some(KEYWORD),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
        ],
    );
    m.insert(
        r"signed-by",
        vec![
            Rule::token_to(
                r"(?m) -----END PGP PUBLIC KEY BLOCK-----\n",
                TEXT,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m).+\n", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for DebianSourcesLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
