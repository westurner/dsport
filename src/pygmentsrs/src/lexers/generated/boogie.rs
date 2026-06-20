#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.verification:BoogieLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.verification:BoogieLexer:boogie

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: boogie
pub struct BoogieLexer;

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
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//[/!](.*?)\n", TokenType::new(&["Comment", "Doc"])),
        Rule::token(r"(?m)//(.*?)\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)(axiom|break|call|e(?:lse|(?:nsure|xist)s)|f(?:orall|unction)|i(?:f|nvariant)|modifies|procedure|requires|then|var|while)\b", KEYWORD),
        Rule::token(r"(?m)(const)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(bool|int|ref)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)(>=|<=|:=|!=|==>|&&|\|\||[+/\-=>*<\[\]])", OPERATOR),
        Rule::token(r"(?m)\{.*?\}", GENERIC_EMPH),
        Rule::token(r"(?m)([{}():;,.])", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"numbers", vec![Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER)]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for BoogieLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
