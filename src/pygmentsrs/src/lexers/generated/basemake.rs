//! AUTO-GENERATED from `pygments.pygments.lexers.make:BaseMakefileLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.make:BaseMakefileLexer:basemake

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: basemake
pub struct BasemakeLexer;

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
        Rule::using_lexer(r"(?m)^(?:[\t ]+.*\n|\n)+", "bash", None),
        Rule::token(r"(?m)\$[<@$+%?|*]", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*?\n", COMMENT),
        Rule::bygroups_to(r"(?m)((?:un)?export)(\s+)(?=[\w${}\t -]+\n)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"export"])),
        Rule::token(r"(?m)(?:un)?export\s+", KEYWORD),
        Rule::bygroups_g(r"(?m)([\w${}().-]+)(\s*)([!?:+]?=)([ \t]*)((?:.*\\\n)+|.*\n)", vec![Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingLexer { alias: "bash", state: None })]),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::bygroups_to(r"(?m)([^\n:]+)(:+)([ \t]*)", vec![Some(NAME_FUNCTION), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"block-header"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"expansion"])),
    ]);
    m.insert(r"expansion", vec![
        Rule::token(r"(?m)[^\w$().-]+", TEXT),
        Rule::token(r"(?m)[\w.-]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$", KEYWORD),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::PushSame),
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
    ]);
    m.insert(r"export", vec![
        Rule::token(r"(?m)[\w${}-]+", NAME_VARIABLE),
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"block-header", vec![
        Rule::token(r"(?m)[,|]", PUNCTUATION),
        Rule::token_to(r"(?m)#.*?\n", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"expansion"])),
        Rule::token(r"(?m)[a-zA-Z_]+", NAME),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for BasemakeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
