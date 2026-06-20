#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.pointless:PointlessLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.pointless:PointlessLexer:pointless

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pointless
pub struct PointlessLexer;

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
        Rule::token(r"(?m)[ \n\r]+", TEXT),
        Rule::token(r"(?m)--.*$", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"multiString"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[\[\](){}:;,.]", PUNCTUATION),
        Rule::token(r"(?m)(!=|%=|\*(?:\*=|[*=])|\+(?:[+=])|\-=|/=|<=|=(?:[=>])|>=|\|>|[$%*+\-/<=>])", OPERATOR),
        Rule::token(r"(?m)(a(?:nd|s)|c(?:a(?:se|tch)|ond)|else|for|i(?:[fn])|not|or|requires|t(?:h(?:en|row)|ry)|upval|w(?:he(?:n|re)|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?m)\d+|\d*\.\d+", NUMBER),
        Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Z][a-zA-Z0-9]*\b", STRING_SYMBOL),
        Rule::token(r"(?m)output\b", NAME_VARIABLE_MAGIC),
        Rule::token(r"(?m)(export|import)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)[a-z][a-zA-Z0-9]*\b", NAME_VARIABLE),
    ]);
    m.insert(
        r"multiString",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)""#, STRING),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?m)\n", ERROR),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for PointlessLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
