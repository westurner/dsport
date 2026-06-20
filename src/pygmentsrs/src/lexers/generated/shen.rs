#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:ShenLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:ShenLexer:shen

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: shen
pub struct ShenLexer;

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
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?m)c#\d{1,3};", STRING_ESCAPE),
            Rule::token(r"(?m)~[ARS%]", STRING_INTERPOL),
            Rule::token(r"(?m)(?s).", STRING),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)(?s)\\\*.*?\*\\", COMMENT_MULTILINE),
            Rule::token(r"(?m)\\\\.*", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)_{5,}", PUNCTUATION),
            Rule::token(r"(?m)={5,}", PUNCTUATION),
            Rule::token(r"(?m)(;|:=|\||--?>|<--?)", PUNCTUATION),
            Rule::token(r"(?m)(:-|:|\{|\})", LITERAL),
            Rule::token(r"(?m)[+-]*\d*\.\d+(e[+-]?\d+)?", NUMBER_FLOAT),
            Rule::token(r"(?m)[+-]*\d+", NUMBER_INTEGER),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)[A-Z][\w!$%*+,<=>?/.\'@&#:-]*", NAME_VARIABLE),
            Rule::token(r"(?m)(true|false|<>|\[\])", KEYWORD_PSEUDO),
            Rule::token(
                r"(?m)[a-z!$%*+,<=>?/.\'@&#_-][\w!$%*+,<=>?/.\'@&#:-]*",
                LITERAL,
            ),
            Rule::token(r"(?m)(\[|\]|\(|\))", PUNCTUATION),
        ],
    );
    Table(m)
}

impl Lexer for ShenLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
