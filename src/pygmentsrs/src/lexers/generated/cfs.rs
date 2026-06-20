#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.templates:ColdfusionLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:ColdfusionLexer:cfs

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cfs
pub struct CfsLexer;

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
        Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?im)/\*(?:.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?im)\+\+|--", OPERATOR),
        Rule::token(r"(?im)[-+*/^&=!]", OPERATOR),
        Rule::token(r"(?im)<=|>=|<|>|==", OPERATOR),
        Rule::token(r"(?im)mod\b", OPERATOR),
        Rule::token(r"(?im)(eq|lt|gt|lte|gte|not|is|and|or)\b", OPERATOR),
        Rule::token(r"(?im)\|\||&&", OPERATOR),
        Rule::token(r"(?im)\?", OPERATOR),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)'.*?'", STRING_SINGLE),
        Rule::token(r"(?im)\d+", NUMBER),
        Rule::token(r"(?im)(if|else|len|var|xml|default|break|switch|component|property|function|do|try|catch|in|continue|for|return|while|required|any|array|binary|boolean|component|date|guid|numeric|query|string|struct|uuid|case)\b", KEYWORD),
        Rule::token(r"(?im)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?im)(application|session|client|cookie|super|this|variables|arguments)\b", NAME_CONSTANT),
        Rule::bygroups(r"(?im)([a-z_$][\w.]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?im)[a-z_$][\w.]*", NAME_VARIABLE),
        Rule::token(r"(?im)[()\[\]{};:,.\\]", PUNCTUATION),
        Rule::token(r"(?im)\s+", TEXT),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?im)"""#, STRING_DOUBLE),
            Rule::token(
                r"(?im)#.+?#",
                TokenType::new(&["Literal", "String", "Interp"]),
            ),
            Rule::token(r##"(?im)[^"#]+"##, STRING_DOUBLE),
            Rule::token(r"(?im)#", STRING_DOUBLE),
            Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for CfsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
