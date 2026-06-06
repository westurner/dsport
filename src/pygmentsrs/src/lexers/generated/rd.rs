//! AUTO-GENERATED from `pygments.pygments.lexers.r:RdLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.r:RdLexer:rd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rd
pub struct RdLexer;

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
        Rule::token(r"(?m)\\[\\{}%]", STRING_ESCAPE),
        Rule::token(r"(?m)%.*$", COMMENT),
        Rule::token(r"(?m)\\(?:cr|l?dots|R|tab)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\\[a-zA-Z]+\b", KEYWORD),
        Rule::token(r"(?m)^\s*#(?:ifn?def|endif).*\b", COMMENT_PREPROC),
        Rule::token(r"(?m)[{}]", NAME_BUILTIN),
        Rule::token(r"(?m)[^\\%\n{}]+", TEXT),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for RdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
