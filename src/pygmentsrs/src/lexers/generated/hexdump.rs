//! AUTO-GENERATED from `pygments.pygments.lexers.hexdump:HexdumpLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.hexdump:HexdumpLexer:hexdump

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hexdump
pub struct HexdumpLexer;

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
        Rule::bygroups_to(r"(?m)^([0-9A-Ha-h]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)], NewState::Push(vec![r"offset-mode"])),
        Rule::token(r"(?m)^[0-9A-Ha-h]+", NAME_LABEL),
        Rule::bygroups(r"(?m)([0-9A-Ha-h]{2})(\-)([0-9A-Ha-h]{2})", vec![Some(NUMBER_HEX), Some(PUNCTUATION), Some(NUMBER_HEX)]),
        Rule::token(r"(?m)[0-9A-Ha-h]{2}", NUMBER_HEX),
        Rule::bygroups_to(r"(?m)(\s{2,3})(\>)(.{16})(\<)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)], NewState::Push(vec![r"bracket-strings"])),
        Rule::bygroups_to(r"(?m)(\s{2,3})(\|)(.{16})(\|)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)], NewState::Push(vec![r"piped-strings"])),
        Rule::bygroups(r"(?m)(\s{2,3})(\>)(.{1,15})(\<)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\s{2,3})(\|)(.{1,15})(\|)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\s{2,3})(.{1,15})$", vec![Some(WHITESPACE), Some(STRING)]),
        Rule::bygroups_to(r"(?m)(\s{2,3})(.{16}|.{20})$", vec![Some(WHITESPACE), Some(STRING)], NewState::Push(vec![r"nonpiped-strings"])),
        Rule::token(r"(?m)\s", WHITESPACE),
        Rule::token(r"(?m)^\*", PUNCTUATION),
    ]);
    m.insert(r"offset", vec![
        Rule::bygroups_to(r"(?m)^([0-9A-Ha-h]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)], NewState::Push(vec![r"offset-mode"])),
        Rule::token(r"(?m)^[0-9A-Ha-h]+", NAME_LABEL),
    ]);
    m.insert(r"offset-mode", vec![
        Rule::token_to(r"(?m)\s", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)[0-9A-Ha-h]+", NAME_LABEL),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    m.insert(r"piped-strings", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::bygroups_to(r"(?m)^([0-9A-Ha-h]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)], NewState::Push(vec![r"offset-mode"])),
        Rule::token(r"(?m)^[0-9A-Ha-h]+", NAME_LABEL),
        Rule::token(r"(?m)[0-9A-Ha-h]{2}", NUMBER_HEX),
        Rule::bygroups(r"(?m)(\s{2,3})(\|)(.{1,16})(\|)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\s", WHITESPACE),
        Rule::token(r"(?m)^\*", PUNCTUATION),
    ]);
    m.insert(r"bracket-strings", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::bygroups_to(r"(?m)^([0-9A-Ha-h]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)], NewState::Push(vec![r"offset-mode"])),
        Rule::token(r"(?m)^[0-9A-Ha-h]+", NAME_LABEL),
        Rule::token(r"(?m)[0-9A-Ha-h]{2}", NUMBER_HEX),
        Rule::bygroups(r"(?m)(\s{2,3})(\>)(.{1,16})(\<)$", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\s", WHITESPACE),
        Rule::token(r"(?m)^\*", PUNCTUATION),
    ]);
    m.insert(r"nonpiped-strings", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::bygroups_to(r"(?m)^([0-9A-Ha-h]+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)], NewState::Push(vec![r"offset-mode"])),
        Rule::token(r"(?m)^[0-9A-Ha-h]+", NAME_LABEL),
        Rule::bygroups(r"(?m)([0-9A-Ha-h]{2})(\-)([0-9A-Ha-h]{2})", vec![Some(NUMBER_HEX), Some(PUNCTUATION), Some(NUMBER_HEX)]),
        Rule::token(r"(?m)[0-9A-Ha-h]{2}", NUMBER_HEX),
        Rule::bygroups(r"(?m)(\s{19,})(.{1,20}?)$", vec![Some(WHITESPACE), Some(STRING)]),
        Rule::bygroups(r"(?m)(\s{2,3})(.{1,20})$", vec![Some(WHITESPACE), Some(STRING)]),
        Rule::token(r"(?m)\s", WHITESPACE),
        Rule::token(r"(?m)^\*", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for HexdumpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
