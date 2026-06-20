#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dns:DnsZoneLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dns:DnsZoneLexer:zone

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: zone
pub struct ZoneLexer;

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
        Rule::bygroups(r"(?m)([ \t]*)(;.*)(\n)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token_to(r"(?m)^\$ORIGIN\b", KEYWORD, NewState::Push(vec![r"values"])),
        Rule::token_to(r"(?m)^\$TTL\b", KEYWORD, NewState::Push(vec![r"values"])),
        Rule::token_to(r"(?m)^\$INCLUDE\b", COMMENT_PREPROC, NewState::Push(vec![r"include"])),
        Rule::token_to(r"(?m)^\$[A-Z]+\b", KEYWORD, NewState::Push(vec![r"values"])),
        Rule::bygroups_to(r"(?m)^(@)([ \t]+)(?:([0-9]+[smhdw]?)([ \t]+))?(?:(IN|CS|CH|HS)([ 	]+))?([A-Z]+)([ 	]+)", vec![Some(OPERATOR), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"values"])),
        Rule::bygroups_to(r"(?m)^([^ \t\n]*)([ \t]+)(?:([0-9]+[smhdw]?)([ \t]+))?(?:(IN|CS|CH|HS)([ 	]+))?([A-Z]+)([ 	]+)", vec![Some(NAME), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"values"])),
        Rule::bygroups_to(r"(?m)^(Operator)([ \t]+)(?:(IN|CS|CH|HS)([ 	]+))?(?:([0-9]+[smhdw]?)([ 	]+))?([A-Z]+)([ 	]+)", vec![Some(NAME), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"values"])),
        Rule::bygroups_to(r"(?m)^([^ \t\n]*)([ \t]+)(?:(IN|CS|CH|HS)([ 	]+))?(?:([0-9]+[smhdw]?)([ 	]+))?([A-Z]+)([ 	]+)", vec![Some(NAME), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"values"])),
    ]);
    m.insert(
        r"values",
        vec![
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"nested"])),
            Rule::bygroups(r"(?m)(;.*)", vec![Some(COMMENT_SINGLE)]),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)@\b", OPERATOR),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)[0-9]+[smhdw]?$", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)([0-9]+[smhdw]?)([ \t]+)",
                vec![Some(NUMBER_INTEGER), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\S+", LITERAL),
        ],
    );
    m.insert(
        r"simple-value",
        vec![
            Rule::bygroups(r"(?m)(;.*)", vec![Some(COMMENT_SINGLE)]),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)@\b", OPERATOR),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)[0-9]+[smhdw]?$", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)([0-9]+[smhdw]?)([ \t]+)",
                vec![Some(NUMBER_INTEGER), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\S+", LITERAL),
        ],
    );
    m.insert(
        r"nested",
        vec![
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(r"(?m)(;.*)", vec![Some(COMMENT_SINGLE)]),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)@\b", OPERATOR),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)[0-9]+[smhdw]?$", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)([0-9]+[smhdw]?)([ \t]+)",
                vec![Some(NUMBER_INTEGER), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\S+", LITERAL),
            Rule::token(r"(?m)[\n]+", WHITESPACE),
        ],
    );
    m.insert(
        r"multiple-simple-values",
        vec![
            Rule::bygroups(r"(?m)(;.*)", vec![Some(COMMENT_SINGLE)]),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)@\b", OPERATOR),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)[0-9]+[smhdw]?$", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)([0-9]+[smhdw]?)([ \t]+)",
                vec![Some(NUMBER_INTEGER), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\S+", LITERAL),
            Rule::token(r"(?m)[\n]+", WHITESPACE),
        ],
    );
    m.insert(
        r"include",
        vec![
            Rule::bygroups_to(
                r"(?m)([ \t]+)([^ \t\n]+)([ \t]+)([-\._a-zA-Z]+)([ \t]+)(;.*)?$",
                vec![
                    Some(WHITESPACE),
                    Some(COMMENT_PREPROCFILE),
                    Some(WHITESPACE),
                    Some(NAME),
                    Some(WHITESPACE),
                    Some(COMMENT_SINGLE),
                ],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)([ \t]+)([^ \t\n]+)([ \t\n]+)$",
                vec![
                    Some(WHITESPACE),
                    Some(COMMENT_PREPROCFILE),
                    Some(WHITESPACE),
                ],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)\\""#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for ZoneLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
