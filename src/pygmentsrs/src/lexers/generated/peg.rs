//! AUTO-GENERATED from `pygments.pygments.lexers.grammar_notation:PegLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.grammar_notation:PegLexer:peg

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: peg
pub struct PegLexer;

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
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token(r"(?m)<-|[←:=/|&!?*+^↑~]", OPERATOR),
            Rule::token(r"(?m)[()]", PUNCTUATION),
            Rule::token(r"(?m)\.", KEYWORD),
            Rule::bygroups(
                r"(?m)(\[)([^\]]*(?:\\.[^\]\\]*)*)(\])",
                vec![Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)],
            ),
            Rule::token(r#"(?m)[a-z]?"[^"\\]*(?:\\.[^"\\]*)*"[a-z]*"#, STRING_DOUBLE),
            Rule::token(r"(?m)[a-z]?'[^'\\]*(?:\\.[^'\\]*)*'[a-z]*", STRING_SINGLE),
            Rule::token(r#"(?m)[^\s<←:=/|&!?*+\^↑~()\[\]"\'#]+"#, NAME_CLASS),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for PegLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
