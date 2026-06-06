//! AUTO-GENERATED from `pygments.pygments.lexers.configs:SystemdLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:SystemdLexer:systemd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: systemd
pub struct SystemdLexer;

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
        Rule::token(r"(?m)^[ \t]*\n", WHITESPACE),
        Rule::bygroups(r"(?m)^([;#].*)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\[[^\]\n]+\])(\n)", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)([^=]+)([ \t]*)(=)([ \t]*)([^\n]*)(\\)(\n)", vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(STRING), Some(TEXT), Some(WHITESPACE)], NewState::Push(vec![r"value"])),
        Rule::bygroups(r"(?m)([^=]+)([ \t]*)(=)([ \t]*)([^\n]*)(\n)", vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
    ]);
    m.insert(r"value", vec![
        Rule::bygroups(r"(?m)^([;#].*)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)([ \t]*)([^\n]*)(\\)(\n)", vec![Some(WHITESPACE), Some(STRING), Some(TEXT), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)([ \t]*)([^\n]*)(\n)", vec![Some(WHITESPACE), Some(STRING), Some(WHITESPACE)], NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for SystemdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
