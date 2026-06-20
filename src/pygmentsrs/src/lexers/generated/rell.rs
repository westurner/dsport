#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.rell:RellLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rell:RellLexer:rell

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rell
pub struct RellLexer;

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
        Rule::token(r"(?m)(b(?:ig_integer|oolean|yte_array)|decimal|gtv|integer|json|list|m(?:ap|utable)|set|text|virtual)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(entity|enum|namespace|object|struct)\b", KEYWORD_DECLARATION),
        Rule::token_to(r"(?m)(function|operation|query)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"function"])),
        Rule::token(r"(?m)(a(?:bstract|nd)|break|c(?:(?:ontinu|reat)e)|delete|else|for|i(?:mport|ndex|[fn])|key|limit|module|not|o(?:ffset|r|verride)|return|update|va(?:[lr])|wh(?:en|ile))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*(.|\n|\r)*?\*/", COMMENT_MULTILINE),
        Rule::token(r#"(?m)"(\\\\|\\"|[^"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)\'(\\\\|\\\'|[^\\\'])*\'", STRING_SINGLE),
        Rule::token(r"(?m)-?[0-9]*\.[0-9]+([eE][+-][0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?[0-9]+([eE][+-][0-9]+|[lL])?", NUMBER_INTEGER),
        Rule::token(r#"(?m)x(\'[a-fA-F0-9]*\'|"[a-fA-F0-9]*")"#, TokenType::new(&["Literal", "String", "Binary"])),
        Rule::bygroups(r"(?m)(\.)([ \n\t\r]*)([a-zA-Z_][a-zA-Z0-9_]*)", vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?m)[{}():;,]+", PUNCTUATION),
        Rule::token(r"(?m)[ \n\t\r]+", WHITESPACE),
        Rule::token(r"(?m)@[a-zA-Z_][a-zA-Z0-9_]*", NAME_DECORATOR),
        Rule::token(r"(?m)[~^*!%&\[\]<>|+=/?\-@\$]", PUNCTUATION_MARKER),
        Rule::token(r"(?m)[a-zA-Z_][a-zA-Z0-9_]*", NAME),
        Rule::token(r"(?m)(\.)+", PUNCTUATION),
    ]);
    m.insert(
        r"function",
        vec![
            Rule::token(r"(?m)[ \n\t\r]+", WHITESPACE),
            Rule::token_to(
                r"(?m)[a-zA-Z_][a-zA-Z0-9_]*",
                NAME_FUNCTION,
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for RellLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
