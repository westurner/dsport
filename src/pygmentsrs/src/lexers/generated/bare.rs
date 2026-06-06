//! AUTO-GENERATED from `pygments.pygments.lexers.bare:BareLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.bare:BareLexer:bare

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bare
pub struct BareLexer;

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
        Rule::bygroups_to(r"(?m)(type)(\s+)([A-Z][a-zA-Z0-9]+)(\s+)(\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(TEXT)], NewState::Push(vec![r"struct"])),
        Rule::bygroups_to(r"(?m)(type)(\s+)([A-Z][a-zA-Z0-9]+)(\s+)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(TEXT)], NewState::Push(vec![r"union"])),
        Rule::bygroups_to(r"(?m)(type)(\s+)([A-Z][a-zA-Z0-9]+)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE)], NewState::Push(vec![r"typedef"])),
        Rule::bygroups_to(r"(?m)(enum)(\s+)([A-Z][a-zA-Z0-9]+)(\s+\{)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE)], NewState::Push(vec![r"enum"])),
        Rule::token(r"(?m)#.*?$", COMMENT),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"struct", vec![
        Rule::token_to(r"(?m)\{", TEXT, NewState::PushSame),
        Rule::token_to(r"(?m)\}", TEXT, NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)([a-zA-Z0-9]+)(:)(\s*)", vec![Some(NAME_ATTRIBUTE), Some(TEXT), Some(WHITESPACE)], NewState::Push(vec![r"typedef"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"union", vec![
        Rule::token_to(r"(?m)\)", TEXT, NewState::Pop(1)),
        Rule::bygroups(r"(?m)(\s*)(\|)(\s*)", vec![Some(WHITESPACE), Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)[A-Z][a-zA-Z0-9]+", NAME_CLASS),
        Rule::token(r"(?m)(bool|data|enum|f(?:32|64)|i(?:16|32|64|8|nt)|map|optional|string|type|u(?:16|32|64|8|int)|void)", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"typedef", vec![
        Rule::token(r"(?m)\[\]", TEXT),
        Rule::token_to(r"(?m)#.*?$", COMMENT, NewState::Pop(1)),
        Rule::bygroups(r"(?m)(\[)(\d+)(\])", vec![Some(TEXT), Some(LITERAL), Some(TEXT)]),
        Rule::token(r"(?m)<|>", TEXT),
        Rule::token_to(r"(?m)\(", TEXT, NewState::Push(vec![r"union"])),
        Rule::bygroups(r"(?m)(\[)([a-z][a-z-A-Z0-9]+)(\])", vec![Some(TEXT), Some(KEYWORD), Some(TEXT)]),
        Rule::bygroups(r"(?m)(\[)([A-Z][a-z-A-Z0-9]+)(\])", vec![Some(TEXT), Some(NAME_CLASS), Some(TEXT)]),
        Rule::token(r"(?m)([A-Z][a-z-A-Z0-9]+)", NAME_CLASS),
        Rule::token(r"(?m)(bool|data|enum|f(?:32|64)|i(?:16|32|64|8|nt)|map|optional|string|type|u(?:16|32|64|8|int)|void)", KEYWORD),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?m)\{", TEXT, NewState::Push(vec![r"struct"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+", LITERAL),
    ]);
    m.insert(r"enum", vec![
        Rule::token_to(r"(?m)\{", TEXT, NewState::PushSame),
        Rule::token_to(r"(?m)\}", TEXT, NewState::Pop(1)),
        Rule::bygroups(r"(?m)([A-Z][A-Z0-9_]*)(\s*=\s*)(\d+)", vec![Some(NAME_ATTRIBUTE), Some(TEXT), Some(LITERAL)]),
        Rule::bygroups(r"(?m)([A-Z][A-Z0-9_]*)", vec![Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)#.*?$", COMMENT),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for BareLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
