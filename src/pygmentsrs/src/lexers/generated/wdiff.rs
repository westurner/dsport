//! AUTO-GENERATED from `pygments.pygments.lexers.diff:WDiffLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.diff:WDiffLexer:wdiff

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: wdiff
pub struct WdiffLexer;

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
        Rule::token_to(r"(?ms)\{\+", GENERIC_INSERTED, NewState::Push(vec![r"inserted"])),
        Rule::token_to(r"(?ms)\[\-", GENERIC_DELETED, NewState::Push(vec![r"deleted"])),
        Rule::token(r"(?ms)[^{}\[\]+-]+", TEXT),
        Rule::token(r"(?ms).", TEXT),
    ]);
    m.insert(r"inserted", vec![
        Rule::token_to(r"(?ms)\{\+", GENERIC_INSERTED, NewState::PushSame),
        Rule::token_to(r"(?ms)\[\-", GENERIC_INSERTED, NewState::PushSame),
        Rule::token_to(r"(?ms)\-\]", GENERIC_INSERTED, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\+\}", GENERIC_INSERTED, NewState::Pop(1)),
        Rule::token(r"(?ms)[^{}\[\]+-]+", GENERIC_INSERTED),
        Rule::token(r"(?ms).", GENERIC_INSERTED),
    ]);
    m.insert(r"deleted", vec![
        Rule::token_to(r"(?ms)\[\-", GENERIC_DELETED, NewState::PushSame),
        Rule::token_to(r"(?ms)\{\+", GENERIC_DELETED, NewState::PushSame),
        Rule::token_to(r"(?ms)\+\}", GENERIC_DELETED, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\-\]", GENERIC_DELETED, NewState::Pop(1)),
        Rule::token(r"(?ms)[^{}\[\]+-]+", GENERIC_DELETED),
        Rule::token(r"(?ms).", GENERIC_DELETED),
    ]);
    Table(m)
}

impl Lexer for WdiffLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
