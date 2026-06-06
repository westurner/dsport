//! AUTO-GENERATED from `pygments.pygments.lexers.python:PythonTracebackLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.python:PythonTracebackLexer:pytb

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pytb, py3tb
pub struct PytbLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token_to(r"(?m)^(\^C)?Traceback \(most recent call last\):\n", GENERIC_TRACEBACK, NewState::Push(vec![r"intb"])),
        Rule::token(r"(?m)^During handling of the above exception, another exception occurred:\n\n", GENERIC_TRACEBACK),
        Rule::token(r"(?m)^The above exception was the direct cause of the following exception:\n\n", GENERIC_TRACEBACK),
        Rule::token_to(r#"(?m)^(?=  File "[^"]+", line \d+)"#, GENERIC_TRACEBACK, NewState::Push(vec![r"intb"])),
        Rule::token(r"(?m)^.*\n", OTHER),
    ]);
    m.insert(r"intb", vec![
        Rule::bygroups(r#"(?m)^(  File )("[^"]+")(, line )(\d+)(, in )(.+)(\n)"#, vec![Some(TEXT), Some(NAME_BUILTIN), Some(TEXT), Some(NUMBER), Some(TEXT), Some(NAME), Some(WHITESPACE)]),
        Rule::bygroups(r#"(?m)^(  File )("[^"]+")(, line )(\d+)(\n)"#, vec![Some(TEXT), Some(NAME_BUILTIN), Some(TEXT), Some(NUMBER), Some(WHITESPACE)]),
        Rule::bygroups_g_to(r"(?m)^(    )(.+)(\n)", vec![Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(WHITESPACE))], NewState::Push(vec![r"markers"])),
        Rule::bygroups(r"(?m)^([ \t]*)(\.\.\.)(\n)", vec![Some(WHITESPACE), Some(COMMENT), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)^([^:]+)(: )(.+)(\n)", vec![Some(GENERIC_ERROR), Some(TEXT), Some(NAME), Some(WHITESPACE)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)^([a-zA-Z_][\w.]*)(:?\n)", vec![Some(GENERIC_ERROR), Some(WHITESPACE)], NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"markers", vec![
        Rule::bygroups_to(r"(?m)^( {4,})([~^]+)(\n)", vec![Some(WHITESPACE), Some(PUNCTUATION_MARKER), Some(WHITESPACE)], NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for PytbLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
