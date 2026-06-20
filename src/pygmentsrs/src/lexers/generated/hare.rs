#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.hare:HareLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.hare:HareLexer:hare

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hare
pub struct HareLexer;

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
        r"whitespace",
        vec![
            Rule::token(r"(?m)^use.*;", COMMENT_PREPROC),
            Rule::token(r"(?m)@[a-z]+", COMMENT_PREPROC),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        ],
    );
    m.insert(r"statements", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)`[^`]*`", STRING),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[LlUu]*", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[LlUu]*", NUMBER_HEX),
        Rule::token(r"(?m)0o[0-7]+[LlUu]*", NUMBER_OCT),
        Rule::token(r"(?m)\d+[zui]?(\d+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)(\.\.(?:(?:\.)?)|=>|(?:[ai])s)", OPERATOR),
        Rule::token(r"(?m)[()\[\],.{};]+", PUNCTUATION),
        Rule::token(r"(?m)(a(?:bort|l(?:ign|loc)|ppend|ssert)|c(?:ase|onst)|de(?:f(?:(?:er)?)|lete)|e(?:lse|num|xport)|f(?:n|or|ree)|if|le(?:[nt])|match|offset|return|s(?:t(?:atic|ruct)|witch)|type|union|va(?:arg|end|start)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(bool|int|uint|uintptr|u8|u16|u32|u64|i8|i16|i32|i64|f32|f64|null|done|never|void|nullable|rune|size|valist)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false|null)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)^use.*;", COMMENT_PREPROC),
        Rule::token(r"(?m)@[a-z]+", COMMENT_PREPROC),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)`[^`]*`", STRING),
        Rule::token(r"(?m)'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[LlUu]*", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[LlUu]*", NUMBER_HEX),
        Rule::token(r"(?m)0o[0-7]+[LlUu]*", NUMBER_OCT),
        Rule::token(r"(?m)\d+[zui]?(\d+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)(\.\.(?:(?:\.)?)|=>|(?:[ai])s)", OPERATOR),
        Rule::token(r"(?m)[()\[\],.{};]+", PUNCTUATION),
        Rule::token(r"(?m)(a(?:bort|l(?:ign|loc)|ppend|ssert)|c(?:ase|onst)|de(?:f(?:(?:er)?)|lete)|e(?:lse|num|xport)|f(?:n|or|ree)|if|le(?:[nt])|match|offset|return|s(?:t(?:atic|ruct)|witch)|type|union|va(?:arg|end|start)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(bool|int|uint|uintptr|u8|u16|u32|u64|i8|i16|i32|i64|f32|f64|null|done|never|void|nullable|rune|size|valist)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false|null)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    Table(m)
}

impl Lexer for HareLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
