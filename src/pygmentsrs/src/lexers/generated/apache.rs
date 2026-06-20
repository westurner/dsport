#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:ApacheConfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:ApacheConfLexer:apache

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: apacheconf, aconf, apache
pub struct ApacheLexer;

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
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)#(.*\\\n)+.*$|(#.*?)$", COMMENT),
            Rule::bygroups(
                r"(?im)(<[^\s>/][^\s>]*)(?:(\s+)(.*))?(>)",
                vec![
                    Some(NAME_TAG),
                    Some(WHITESPACE),
                    Some(STRING),
                    Some(NAME_TAG),
                ],
            ),
            Rule::bygroups(r"(?im)(</[^\s>]+)(>)", vec![Some(NAME_TAG), Some(NAME_TAG)]),
            Rule::token_to(
                r"(?im)[a-z]\w*",
                NAME_BUILTIN,
                NewState::Push(vec![r"value"]),
            ),
            Rule::token(r"(?im)\.+", TEXT),
        ],
    );
    m.insert(r"value", vec![
        Rule::token(r"(?im)\\\n", TEXT),
        Rule::token_to(r"(?im)\n+", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?im)\\", TEXT),
        Rule::token(r"(?im)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?im)\d+\.\d+\.\d+\.\d+(?:/\d+)?", NUMBER),
        Rule::token(r"(?im)\d+", NUMBER),
        Rule::token(r"(?im)/([*a-z0-9][*\w./-]+)", STRING_OTHER),
        Rule::token(r"(?im)(on|off|none|any|all|double|email|dns|min|minimal|os|productonly|full|emerg|alert|crit|error|warn|notice|info|debug|registry|script|inetd|standalone|user|group)\b", KEYWORD),
        Rule::token(r#"(?im)"([^"\\]*(?:\\(.|\n)[^"\\]*)*)""#, STRING_DOUBLE),
        Rule::token(r#"(?im)[^\s"\\]+"#, TEXT),
    ]);
    Table(m)
}

impl Lexer for ApacheLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
