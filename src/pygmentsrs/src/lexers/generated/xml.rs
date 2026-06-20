#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.html:XmlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.html:XmlLexer:xml

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: xml
pub struct XmlLexer;

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
            Rule::token(r"(?ms)[^<&\s]+", TEXT),
            Rule::token(r"(?ms)[^<&\S]+", WHITESPACE),
            Rule::token(r"(?ms)&\S*?;", NAME_ENTITY),
            Rule::token(r"(?ms)\<\!\[CDATA\[.*?\]\]\>", COMMENT_PREPROC),
            Rule::token(r"(?ms)<!--.*?-->", COMMENT_MULTILINE),
            Rule::token(r"(?ms)<\?.*?\?>", COMMENT_PREPROC),
            Rule::token(r"(?ms)<![^>]*>", COMMENT_PREPROC),
            Rule::token_to(r"(?ms)<\s*[\w:.-]+", NAME_TAG, NewState::Push(vec![r"tag"])),
            Rule::token(r"(?ms)<\s*/\s*[\w:.-]+\s*>", NAME_TAG),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token_to(
                r"(?ms)[\w.:-]+\s*=",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"attr"]),
            ),
            Rule::token_to(r"(?ms)/?\s*>", NAME_TAG, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"attr",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token_to(r#"(?ms)".*?""#, STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ms)'.*?'", STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ms)[^\s>]+", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for XmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
