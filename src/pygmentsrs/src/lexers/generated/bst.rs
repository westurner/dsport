#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.bibtex:BSTLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.bibtex:BSTLexer:bst

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bst, bst-pybtex
pub struct BstLexer;

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
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)%.*?$", COMMENT_SINGLE),
            Rule::token(r"(?im)(read|sort)", KEYWORD),
            Rule::token_to(
                r"(?im)(execute|i(?:ntegers|terate)|reverse|strings)",
                KEYWORD,
                NewState::Push(vec![r"group"]),
            ),
            Rule::token_to(
                r"(?im)(function|macro)",
                KEYWORD,
                NewState::Push(vec![r"group", r"group"]),
            ),
            Rule::token_to(
                r"(?im)(entry)",
                KEYWORD,
                NewState::Push(vec![r"group", r"group", r"group"]),
            ),
        ],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)%.*?$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"group",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)%.*?$", COMMENT_SINGLE),
            Rule::token_to(
                r"(?im)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"group-end", r"body"]),
            ),
        ],
    );
    m.insert(
        r"group-end",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)%.*?$", COMMENT_SINGLE),
            Rule::token_to(r"(?im)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"body",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)%.*?$", COMMENT_SINGLE),
            Rule::token(r#"(?im)\'[^#\"\{\}\s]+"#, NAME_FUNCTION),
            Rule::token(r#"(?im)[^#\"\{\}\s]+\$"#, NAME_BUILTIN),
            Rule::token(r#"(?im)[^#\"\{\}\s]+"#, NAME_VARIABLE),
            Rule::token(r#"(?im)"[^\"]*""#, STRING),
            Rule::token(r"(?im)#-?\d+", NUMBER),
            Rule::token_to(
                r"(?im)\{",
                PUNCTUATION,
                NewState::Push(vec![r"group-end", r"body"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for BstLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
