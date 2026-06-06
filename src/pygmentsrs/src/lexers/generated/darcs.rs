//! AUTO-GENERATED from `pygments.pygments.lexers.diff:DarcsPatchLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.diff:DarcsPatchLexer:darcs

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dpatch
pub struct DarcsLexer;

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
        Rule::token(r"(?m)<", OPERATOR),
        Rule::token(r"(?m)>", OPERATOR),
        Rule::token(r"(?m)\{", OPERATOR),
        Rule::token(r"(?m)\}", OPERATOR),
        Rule::bygroups(r"(?m)(\[)((?:TAG )?)(.*)(\n)(.*)(\*\*)(\d+)(\s?)(\])", vec![Some(OPERATOR), Some(KEYWORD), Some(NAME), Some(WHITESPACE), Some(NAME), Some(OPERATOR), Some(LITERAL_DATE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups_to(r"(?m)(\[)((?:TAG )?)(.*)(\n)(.*)(\*\*)(\d+)(\s?)", vec![Some(OPERATOR), Some(KEYWORD), Some(NAME), Some(WHITESPACE), Some(NAME), Some(OPERATOR), Some(LITERAL_DATE), Some(WHITESPACE)], NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)New patches:", GENERIC_HEADING),
        Rule::token(r"(?m)Context:", GENERIC_HEADING),
        Rule::token(r"(?m)Patch bundle hash:", GENERIC_HEADING),
        Rule::bygroups(r"(?m)(\s*)(hunk|addfile|adddir|rmfile|rmdir|move|replace)(.*)(\n)", vec![Some(WHITESPACE), Some(KEYWORD), Some(TEXT), Some(WHITESPACE)]),
        Rule::token_to(r"(?m)\+", GENERIC_INSERTED, NewState::Push(vec![r"insert"])),
        Rule::token_to(r"(?m)-", GENERIC_DELETED, NewState::Push(vec![r"delete"])),
        Rule::bygroups(r"(?m)(.*)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^\]].*\n", COMMENT),
        Rule::token_to(r"(?m)\]", OPERATOR, NewState::Pop(1)),
    ]);
    m.insert(r"specialText", vec![
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\[_[^_]*_]", OPERATOR),
    ]);
    m.insert(r"insert", vec![
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\[_[^_]*_]", OPERATOR),
        Rule::token(r"(?m)\[", GENERIC_INSERTED),
        Rule::token(r"(?m)[^\n\[]+", GENERIC_INSERTED),
    ]);
    m.insert(r"delete", vec![
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\[_[^_]*_]", OPERATOR),
        Rule::token(r"(?m)\[", GENERIC_DELETED),
        Rule::token(r"(?m)[^\n\[]+", GENERIC_DELETED),
    ]);
    Table(m)
}

impl Lexer for DarcsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
