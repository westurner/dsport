//! AUTO-GENERATED from `pygments.pygments.lexers.testing:TAPLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.testing:TAPLexer:tap

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tap
pub struct TapLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)^TAP version \d+\n", NAME_NAMESPACE),
            Rule::token_to(
                r"(?m)^1\.\.\d+",
                KEYWORD_DECLARATION,
                NewState::Push(vec![r"plan"]),
            ),
            Rule::bygroups_to(
                r"(?m)^(not ok)([^\S\n]*)(\d*)",
                vec![Some(GENERIC_ERROR), Some(TEXT), Some(NUMBER_INTEGER)],
                NewState::Push(vec![r"test"]),
            ),
            Rule::bygroups_to(
                r"(?m)^(ok)([^\S\n]*)(\d*)",
                vec![Some(KEYWORD_RESERVED), Some(TEXT), Some(NUMBER_INTEGER)],
                NewState::Push(vec![r"test"]),
            ),
            Rule::token(r"(?m)^#.*\n", COMMENT),
            Rule::token(r"(?m)^Bail out!.*\n", GENERIC_ERROR),
            Rule::token(r"(?m)^.*\n", TEXT),
        ],
    );
    m.insert(
        r"plan",
        vec![
            Rule::token(r"(?m)[^\S\n]+", TEXT),
            Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"directive"])),
            Rule::token_to(r"(?m)\n", COMMENT, NewState::Pop(1)),
            Rule::token_to(r"(?m).*\n", GENERIC_ERROR, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"test",
        vec![
            Rule::token(r"(?m)[^\S\n]+", TEXT),
            Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"directive"])),
            Rule::token(r"(?m)\S+", TEXT),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"directive",
        vec![
            Rule::token(r"(?m)[^\S\n]+", COMMENT),
            Rule::token(r"(?m)(?i)\bTODO\b", COMMENT_PREPROC),
            Rule::token(r"(?m)(?i)\bSKIP\S*", COMMENT_PREPROC),
            Rule::token(r"(?m)\S+", COMMENT),
            Rule::token_to(r"(?m)\n", COMMENT, NewState::Pop(2)),
        ],
    );
    Table(m)
}

impl Lexer for TapLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
