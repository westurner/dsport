//! AUTO-GENERATED from `pygments.pygments.lexers.webmisc:CirruLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.webmisc:CirruLexer:cirru

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cirru
pub struct CirruLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^"\\\n]+"#, STRING),
        Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"escape", vec![
        Rule::token_to(r"(?m).", STRING_ESCAPE, NewState::Pop(1)),
    ]);
    m.insert(r"function", vec![
        Rule::token_to(r"(?m)\,", OPERATOR, NewState::Pop(1)),
        Rule::token_to(r#"(?m)[^\s"()]+"#, NAME_FUNCTION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\)", OPERATOR, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=\n)", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?m)\(", OPERATOR, NewState::PushSame),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"#pop", r"string"])),
        Rule::token(r"(?m)[ ]+", WHITESPACE),
    ]);
    m.insert(r"line", vec![
        Rule::token_to(r"(?m)(?<!\w)\$(?!\w)", OPERATOR, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)\(", OPERATOR, NewState::Push(vec![r"function"])),
        Rule::token(r"(?m)\)", OPERATOR),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[ ]+", WHITESPACE),
        Rule::token(r"(?m)[+-]?[\d.]+\b", NUMBER),
        Rule::token(r#"(?m)[^\s"()]+"#, NAME_VARIABLE),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)^\n+", WHITESPACE),
        Rule::default(NewState::Push(vec![r"line", r"function"])),
    ]);
    Table(m)
}

impl Lexer for CirruLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
