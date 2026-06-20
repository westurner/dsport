#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.textedit:SedLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textedit:SedLexer:sed

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sed, gsed, ssed
pub struct SedLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)\$", OPERATOR),
        Rule::token(r"(?m)[{};,!]", PUNCTUATION),
        Rule::token(r"(?m)[dDFgGhHlnNpPqQxz=]", KEYWORD),
        Rule::bygroups(r"(?m)([berRtTvwW:])([^;\n]*)", vec![Some(KEYWORD), Some(STRING_SINGLE)]),
        Rule::bygroups(r"(?m)([aci])((?:.*?\\\n)*(?:.*?[^\\]$))", vec![Some(KEYWORD), Some(STRING_DOUBLE)]),
        Rule::bygroups(r"(?m)([qQ])([0-9]*)", vec![Some(KEYWORD), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?m)(/)((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(/)", vec![Some(PUNCTUATION), Some(STRING_REGEX), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\\(.))((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(\2)", vec![Some(PUNCTUATION), None, Some(STRING_REGEX), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(y)(.)((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(\2)((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(\2)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(STRING_SINGLE), Some(PUNCTUATION), Some(STRING_SINGLE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(s)(.)((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(\2)((?:(?:\\[^\n]|[^\\])*?\\\n)*?(?:\\.|[^\\])*?)(\2)((?:[gpeIiMm]|[0-9])*)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(STRING_REGEX), Some(PUNCTUATION), Some(STRING_SINGLE), Some(PUNCTUATION), Some(KEYWORD)]),
    ]);
    Table(m)
}

impl Lexer for SedLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
