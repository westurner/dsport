//! AUTO-GENERATED from `pygments.pygments.lexers.console:VCTreeStatusLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.console:VCTreeStatusLexer:vctreestatus

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vctreestatus
pub struct VctreestatusLexer;

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
        Rule::token(r"(?m)^A  \+  C\s+", GENERIC_ERROR),
        Rule::token(r"(?m)^A\s+\+?\s+", STRING),
        Rule::token(r"(?m)^M\s+", GENERIC_INSERTED),
        Rule::token(r"(?m)^C\s+", GENERIC_ERROR),
        Rule::token(r"(?m)^D\s+", GENERIC_DELETED),
        Rule::token(r"(?m)^[?!]\s+", COMMENT_PREPROC),
        Rule::token(r"(?m)      >\s+.*\n", COMMENT_PREPROC),
        Rule::token(r"(?m)\S+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for VctreestatusLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
