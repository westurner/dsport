#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.x10:X10Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.x10:X10Lexer:x10

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: x10, xten
pub struct X10Lexer;

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
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)\b(as|assert|async|at|athome|ateach|atomic|break|case|catch|class|clocked|continue|def|default|do|else|final|finally|finish|for|goto|haszero|here|if|import|in|instanceof|interface|isref|new|offer|operator|package|return|struct|switch|throw|try|type|val|var|when|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(v|o|i|d)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(false|null|self|super|this|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\b(abstract|extends|implements|native|offers|private|property|protected|public|static|throws|transient)\b", KEYWORD_DECLARATION),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-fA-F]{4}'", STRING_CHAR),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for X10Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
