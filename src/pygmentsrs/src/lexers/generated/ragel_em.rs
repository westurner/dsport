#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.parsers:RagelEmbeddedLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.parsers:RagelEmbeddedLexer:ragel_em

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ragel-em
pub struct RagelEmLexer;

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
        Rule::token(r#"(?m)([^%\'"/#]+|%(?=[^%]|$)|"(\\\\|\\[^\\]|[^"\\])*"|'(\\\\|\\[^\\]|[^'\\])*'|/\*(.|\n)*?\*/|//.*$\n?|\#.*$\n?|/(?!\*)(\\\\|\\[^\\]|[^/\\])*/|/)+"#, OTHER),
        Rule::bygroups_g(r"(?m)(%%)(?![{%])(.*)($|;)(\n?)", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::UsingLexer { alias: "ragel", state: None }), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(TEXT))]),
        Rule::token_to(r"(?m)(%%%%|%%)\{", PUNCTUATION, NewState::Push(vec![r"multi-line-fsm"])),
    ]);
    m.insert(r"multi-line-fsm", vec![
        Rule::using_lexer(r#"(?m)(([^}\'"\[/#]|\}(?=[^%]|$)|\}%(?=[^%]|$)|[^\\]\\[{}]|(>|\$|%|<|@|<>)/|/(?!\*)(\\\\|\\[^\\]|[^/\\])*/\*|/(?=[^/*]|$))+|"(\\\\|\\[^\\]|[^"\\])*"|'(\\\\|\\[^\\]|[^'\\])*'|\[(\\\\|\\[^\\]|[^\]\\])*\]|/\*(.|\n)*?\*/|//.*$\n?|\#.*$\n?)+"#, "ragel", None),
        Rule::token_to(r"(?m)\}%%", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for RagelEmLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
