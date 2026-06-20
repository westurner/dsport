#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.installers:SourcesListLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.installers:SourcesListLexer:debsources

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: debsources, sourceslist, sources.list
pub struct DebsourcesLexer;

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
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*?$", COMMENT),
            Rule::bygroups_to(
                r"(?m)^(deb(?:-src)?)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"distribution"]),
            ),
        ],
    );
    m.insert(
        r"distribution",
        vec![
            Rule::token_to(r"(?m)#.*?$", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)\$\(ARCH\)", NAME_VARIABLE),
            Rule::token(r"(?m)[^\s$\[]+", STRING),
            Rule::token_to(
                r"(?m)\[",
                STRING_OTHER,
                NewState::Push(vec![r"escaped-distribution"]),
            ),
            Rule::token(r"(?m)\$", STRING),
            Rule::token_to(r"(?m)\s+", WHITESPACE, NewState::Push(vec![r"components"])),
        ],
    );
    m.insert(
        r"escaped-distribution",
        vec![
            Rule::token_to(r"(?m)\]", STRING_OTHER, NewState::Pop(1)),
            Rule::token(r"(?m)\$\(ARCH\)", NAME_VARIABLE),
            Rule::token(r"(?m)[^\]$]+", STRING_OTHER),
            Rule::token(r"(?m)\$", STRING_OTHER),
        ],
    );
    m.insert(
        r"components",
        vec![
            Rule::token_to(r"(?m)#.*?$", COMMENT, NewState::Pop(2)),
            Rule::token_to(r"(?m)$", TEXT, NewState::Pop(2)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\S+", KEYWORD_PSEUDO),
        ],
    );
    Table(m)
}

impl Lexer for DebsourcesLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
