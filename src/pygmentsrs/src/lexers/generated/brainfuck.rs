//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:BrainfuckLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:BrainfuckLexer:brainfuck

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: brainfuck, bf
pub struct BrainfuckLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"common", vec![
        Rule::token(r"(?m)[.,]+", NAME_TAG),
        Rule::token(r"(?m)[+-]+", NAME_BUILTIN),
        Rule::token(r"(?m)[<>]+", NAME_VARIABLE),
        Rule::token(r"(?m)[^.,+\-<>\[\]]+", COMMENT),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::Push(vec![r"loop"])),
        Rule::token(r"(?m)\]", ERROR),
        Rule::token(r"(?m)[.,]+", NAME_TAG),
        Rule::token(r"(?m)[+-]+", NAME_BUILTIN),
        Rule::token(r"(?m)[<>]+", NAME_VARIABLE),
        Rule::token(r"(?m)[^.,+\-<>\[\]]+", COMMENT),
    ]);
    m.insert(r"loop", vec![
        Rule::token_to(r"(?m)\[", KEYWORD, NewState::PushSame),
        Rule::token_to(r"(?m)\]", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[.,]+", NAME_TAG),
        Rule::token(r"(?m)[+-]+", NAME_BUILTIN),
        Rule::token(r"(?m)[<>]+", NAME_VARIABLE),
        Rule::token(r"(?m)[^.,+\-<>\[\]]+", COMMENT),
    ]);
    Table(m)
}

impl Lexer for BrainfuckLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
