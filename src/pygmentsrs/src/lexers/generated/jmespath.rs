#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jmespath:JMESPathLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jmespath:JMESPathLexer:jmespath

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jmespath, jp
pub struct JmespathLexer;

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
        vec![Rule::token(r"(?m)'(\\(.|\n)|[^'\\])*'", STRING)],
    );
    m.insert(
        r"punctuation",
        vec![Rule::token(
            r"(?m)(\[\?|[\.\*\[\],:\(\)\{\}\|])",
            PUNCTUATION,
        )],
    );
    m.insert(r"ws", vec![Rule::token(r"(?m) |\t|\n|\r", WHITESPACE)]);
    m.insert(
        r"dq-identifier",
        vec![
            Rule::token(r#"(?m)[^\\"]+"#, NAME_VARIABLE),
            Rule::token(r#"(?m)\\""#, NAME_VARIABLE),
            Rule::token_to(r"(?m).", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"identifier",
        vec![
            Rule::bygroups_to(
                r#"(?m)(&)?(")"#,
                vec![Some(NAME_VARIABLE), Some(PUNCTUATION)],
                NewState::Push(vec![r"dq-identifier"]),
            ),
            Rule::bygroups(
                r#"(?m)(")?(&?[A-Za-z][A-Za-z0-9_-]*)(")?"#,
                vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)],
            ),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m) |\t|\n|\r", WHITESPACE),
            Rule::token(r"(?m)'(\\(.|\n)|[^'\\])*'", STRING),
            Rule::token(r"(?m)(==|!=|<=|>=|<|>|&&|\|\||!)", OPERATOR),
            Rule::token(r"(?m)(\[\?|[\.\*\[\],:\(\)\{\}\|])", PUNCTUATION),
            Rule::token(r"(?m)@", NAME_VARIABLE_GLOBAL),
            Rule::bygroups(
                r"(?m)(&?[A-Za-z][A-Za-z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
            ),
            Rule::bygroups(r"(?m)(&)(\()", vec![Some(NAME_VARIABLE), Some(PUNCTUATION)]),
            Rule::bygroups_to(
                r#"(?m)(&)?(")"#,
                vec![Some(NAME_VARIABLE), Some(PUNCTUATION)],
                NewState::Push(vec![r"dq-identifier"]),
            ),
            Rule::bygroups(
                r#"(?m)(")?(&?[A-Za-z][A-Za-z0-9_-]*)(")?"#,
                vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)-?\d+", NUMBER),
            Rule::token_to(r"(?m)`", LITERAL, NewState::Push(vec![r"literal"])),
        ],
    );
    m.insert(
        r"literal",
        vec![
            Rule::token(r"(?m) |\t|\n|\r", WHITESPACE),
            Rule::token(r"(?m)'(\\(.|\n)|[^'\\])*'", STRING),
            Rule::token(r"(?m)(\[\?|[\.\*\[\],:\(\)\{\}\|])", PUNCTUATION),
            Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
            Rule::bygroups_to(
                r#"(?m)(&)?(")"#,
                vec![Some(NAME_VARIABLE), Some(PUNCTUATION)],
                NewState::Push(vec![r"dq-identifier"]),
            ),
            Rule::bygroups(
                r#"(?m)(")?(&?[A-Za-z][A-Za-z0-9_-]*)(")?"#,
                vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)-?\d+\.?\d*([eE][-+]\d+)?", NUMBER),
            Rule::token(r"(?m)\\`", LITERAL),
            Rule::token_to(r"(?m)`", LITERAL, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for JmespathLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
