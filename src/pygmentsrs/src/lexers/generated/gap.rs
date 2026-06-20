#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.algebra:GAPLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.algebra:GAPLexer:gap

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gap
pub struct GapLexer;

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
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token(r#"(?m)"(?:[^"\\]|\\.)*""#, STRING),
            Rule::token(r"(?m)\(|\)|\[|\]|\{|\}", PUNCTUATION),
            Rule::token(
                r"(?m)(?x)\b(?:
                if|then|elif|else|fi|
                for|while|do|od|
                repeat|until|
                break|continue|
                function|local|return|end|
                rec|
                quit|QUIT|
                IsBound|Unbind|
                TryNextMethod|
                Info|Assert
              )\b",
                KEYWORD,
            ),
            Rule::token(
                r"(?m)(?x)\b(?:
                true|false|fail|infinity
              )\b",
                NAME_CONSTANT,
            ),
            Rule::token(
                r"(?m)(?x)\b(?:
                (Declare|Install)([A-Z][A-Za-z]+)|
                   BindGlobal|BIND_GLOBAL
              )\b",
                NAME_BUILTIN,
            ),
            Rule::token(r"(?m)\.|,|:=|;|=|\+|-|\*|/|\^|>|<", OPERATOR),
            Rule::token(
                r"(?m)(?x)\b(?:
                and|or|not|mod|in
              )\b",
                OPERATOR_WORD,
            ),
            Rule::token(
                r"(?m)(?x)
              (?:\w+|`[^`]*`)
              (?:::\w+|`[^`]*`)*",
                NAME_VARIABLE,
            ),
            Rule::token(r"(?m)[0-9]+(?:\.[0-9]*)?(?:e[0-9]+)?", NUMBER),
            Rule::token(r"(?m)\.[0-9]+(?:e[0-9]+)?", NUMBER),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for GapLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
