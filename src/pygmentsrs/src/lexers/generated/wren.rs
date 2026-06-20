#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.wren:WrenLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.wren:WrenLexer:wren

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: wren
pub struct WrenLexer;

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
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)[,\\\[\]{}]", PUNCTUATION),
        Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?ms)(?<!\.)(as|break|c(?:lass|on(?:struct|tinue))|else|for(?:(?:eign)?)|i(?:f|mport)|return|s(?:tatic|uper)|this|var|while)\b", KEYWORD),
        Rule::token(r"(?ms)(?<!\.)(false|null|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(?<!\.)(i(?:[ns]))\b", OPERATOR_WORD),
        Rule::token_to(r"(?ms)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?ms)#.*?(\(.*?\))?$", COMMENT_SPECIAL),
        Rule::token(r"(?ms)[!%&*+\-./:<=>?\\^|~]+", OPERATOR),
        Rule::token(r"(?ms)[a-z][a-zA-Z_0-9]*", NAME),
        Rule::token(r"(?ms)[A-Z][a-zA-Z_0-9]*", NAME_CLASS),
        Rule::token(r"(?ms)__[a-zA-Z_0-9]*", NAME_VARIABLE_CLASS),
        Rule::token(r"(?ms)_[a-zA-Z_0-9]*", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)\d+(\.\d+)?([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r#"(?ms)""".*?""""#, STRING),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token_to(r"(?ms)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?ms)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?ms)([^*/]|\*(?!/)|/(?!\*))+", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)\\[\\%"0abefnrtv]"#, STRING_ESCAPE),
            Rule::token(r"(?ms)\\x[a-fA-F0-9]{2}", STRING_ESCAPE),
            Rule::token(r"(?ms)\\u[a-fA-F0-9]{4}", STRING_ESCAPE),
            Rule::token(r"(?ms)\\U[a-fA-F0-9]{8}", STRING_ESCAPE),
            Rule::token_to(
                r"(?ms)%\(",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r#"(?ms)[^\\"%]+"#, STRING),
        ],
    );
    m.insert(r"interpolation", vec![
        Rule::token_to(r"(?ms)\)", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)[,\\\[\]{}]", PUNCTUATION),
        Rule::token_to(r"(?ms)\(", PUNCTUATION, NewState::Push(vec![r"root"])),
        Rule::token_to(r"(?ms)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?ms)(?<!\.)(as|break|c(?:lass|on(?:struct|tinue))|else|for(?:(?:eign)?)|i(?:f|mport)|return|s(?:tatic|uper)|this|var|while)\b", KEYWORD),
        Rule::token(r"(?ms)(?<!\.)(false|null|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(?<!\.)(i(?:[ns]))\b", OPERATOR_WORD),
        Rule::token_to(r"(?ms)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?ms)#.*?(\(.*?\))?$", COMMENT_SPECIAL),
        Rule::token(r"(?ms)[!%&*+\-./:<=>?\\^|~]+", OPERATOR),
        Rule::token(r"(?ms)[a-z][a-zA-Z_0-9]*", NAME),
        Rule::token(r"(?ms)[A-Z][a-zA-Z_0-9]*", NAME_CLASS),
        Rule::token(r"(?ms)__[a-zA-Z_0-9]*", NAME_VARIABLE_CLASS),
        Rule::token(r"(?ms)_[a-zA-Z_0-9]*", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)\d+(\.\d+)?([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r#"(?ms)""".*?""""#, STRING),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
    ]);
    Table(m)
}

impl Lexer for WrenLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
