#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:ProtoBufLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:ProtoBufLexer:protobuf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: protobuf, proto
pub struct ProtobufLexer;

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
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)[,;{}\[\]()<>]", PUNCTUATION),
        Rule::token(r"(?m)/(\\\n)?/(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\b(ctype|default|extensions|import|max|o(?:neof|ption(?:(?:al)?))|packed|r(?:e(?:peated|quired|served|turns)|pc)|syntax|to)\b", KEYWORD),
        Rule::token(r"(?m)(b(?:ool|ytes)|double|f(?:ixed(?:32|64)|loat)|int(?:32|64)|s(?:fixed(?:32|64)|int(?:32|64)|tring)|uint(?:32|64))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?m)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"package"])),
        Rule::bygroups_to(r"(?m)(message|extend)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"message"])),
        Rule::bygroups_to(r"(?m)(enum|group|service)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"type"])),
        Rule::token(r#"(?m)\".*?\""#, STRING),
        Rule::token(r"(?m)\'.*?\'", STRING),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[LlUu]*", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\-?(inf|nan))\b", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[LlUu]*", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[LlUu]*", NUMBER_OCT),
        Rule::token(r"(?m)\d+[LlUu]*", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-=]", OPERATOR),
        Rule::bygroups(r"(?m)([a-zA-Z_][\w.]*)([ \t]*)(=)", vec![Some(NAME_ATTRIBUTE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[a-zA-Z_][\w.]*", NAME),
    ]);
    m.insert(
        r"package",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z_][\w\.]*", NAME_NAMESPACE, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"message",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for ProtobufLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
