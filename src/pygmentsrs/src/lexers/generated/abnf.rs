#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.grammar_notation:AbnfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.grammar_notation:AbnfLexer:abnf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: abnf
pub struct AbnfLexer;

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
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r#"(?m)(%[si])?"[^"]*""#, LITERAL),
        Rule::token(r"(?m)%b[01]+\-[01]+\b", LITERAL),
        Rule::token(r"(?m)%b[01]+(\.[01]+)*\b", LITERAL),
        Rule::token(r"(?m)%d[0-9]+\-[0-9]+\b", LITERAL),
        Rule::token(r"(?m)%d[0-9]+(\.[0-9]+)*\b", LITERAL),
        Rule::token(r"(?m)%x[0-9a-fA-F]+\-[0-9a-fA-F]+\b", LITERAL),
        Rule::token(r"(?m)%x[0-9a-fA-F]+(\.[0-9a-fA-F]+)*\b", LITERAL),
        Rule::token(r"(?m)\b[0-9]+\*[0-9]+", OPERATOR),
        Rule::token(r"(?m)\b[0-9]+\*", OPERATOR),
        Rule::token(r"(?m)\b[0-9]+", OPERATOR),
        Rule::token(r"(?m)\*", OPERATOR),
        Rule::token(r"(?m)(ALPHA|BIT|C(?:HAR|R(?:(?:LF)?)|TL)|D(?:IGIT|QUOTE)|H(?:EXDIG|TAB)|L(?:F|WSP)|OCTET|SP|VCHAR|WSP)\b", KEYWORD),
        Rule::token(r"(?m)[a-zA-Z][a-zA-Z0-9-]*\b", NAME_CLASS),
        Rule::token(r"(?m)(=/|=|/)", OPERATOR),
        Rule::token(r"(?m)[\[\]()]", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for AbnfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
