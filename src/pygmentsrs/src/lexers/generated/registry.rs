//! AUTO-GENERATED from `pygments.pygments.lexers.configs:RegeditLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:RegeditLexer:registry

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: registry
pub struct RegistryLexer;

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
        Rule::token(r"(?m)Windows Registry Editor.*", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[;#].*", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\[)(-?)(HKEY_[A-Z_]+)(.*?\])$", vec![Some(KEYWORD), Some(OPERATOR), Some(NAME_BUILTIN), Some(KEYWORD)]),
        Rule::bygroups_to(r#"(?m)("(?:\\"|\\\\|[^"])+")([ \t]*)(=)([ \t]*)"#, vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"value"])),
        Rule::bygroups_to(r"(?m)(.*?)([ \t]*)(=)([ \t]*)", vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)], NewState::Push(vec![r"value"])),
    ]);
    m.insert(r"value", vec![
        Rule::token_to(r"(?m)-", OPERATOR, NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(dword|hex(?:\([0-9a-fA-F]\))?)(:)([0-9a-fA-F,]+)", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(NUMBER)], NewState::Pop(1)),
        Rule::token_to(r"(?m).+", STRING, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for RegistryLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
