#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.spice:SpiceLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.spice:SpiceLexer:spice

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: spice, spicelang
pub struct SpiceLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(.*?)\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*]{2}(.|\n)*?[*](\\\n)?/", STRING_DOC),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(import|as)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(f|p|type|struct|interface|enum|alias|operator)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(assert|break|c(?:as(?:[et])|ontinue)|d(?:efault|o)|e(?:lse|xt)|f(?:allthrough|or(?:(?:each)?))|if|return|switch|(?:unsaf|whil)e)\b", KEYWORD),
        Rule::token(r"(?m)(co(?:mpose|nst)|heap|inline|public|(?:(?:un)?)signed)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(class|new|pick|s(?:tash|ync)|yield)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(true|false|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(b(?:ool|yte)|char|d(?:ouble|yn)|int|long|s(?:hort|tring))\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)(alignof|len|p(?:anic|rintf)|sizeof)\b(\()", vec![Some(NAME_BUILTIN), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[-]?[0-9]*[.][0-9]+([eE][+-]?[0-9]+)?", TokenType::new(&["Literal", "Number", "Double"])),
        Rule::token(r"(?m)0[bB][01]+[slu]?", NUMBER_BIN),
        Rule::token(r"(?m)0[oO][0-7]+[slu]?", NUMBER_OCT),
        Rule::token(r"(?m)0[xXhH][0-9a-fA-F]+[slu]?", NUMBER_HEX),
        Rule::token(r"(?m)(0[dD])?[0-9]+[slu]?", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)\'(\\\\|\\[^\\]|[^\'\\])\'", STRING_CHAR),
        Rule::token(r"(?m)<<=|>>=|<<|>>|<=|>=|\+=|-=|\*=|/=|\%=|\|=|&=|\^=|&&|\|\||&|\||\+\+|--|\%|\^|\~|==|!=|->|::|[.]{3}|#!|#|[+\-*/&]", OPERATOR),
        Rule::token(r"(?m)[|<>=!()\[\]{}.,;:\?]", PUNCTUATION),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_OTHER),
    ]);
    Table(m)
}

impl Lexer for SpiceLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
