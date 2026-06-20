#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.gleam:GleamLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.gleam:GleamLexer:gleam

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gleam
pub struct GleamLexer;

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
        Rule::bygroups(r"(?m)(///.*?)(\n)", vec![Some(STRING_DOC), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?m)(a(?:s(?:(?:sert)?)|uto)|c(?:ase|onst)|de(?:(?:legat|riv)e)|e(?:cho|lse)|fn|i(?:f|mp(?:(?:lemen|or)t))|let|macro|opaque|p(?:anic|ub)|t(?:est|odo|ype)|use)\b", KEYWORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]+)(\.)", vec![Some(KEYWORD), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[()\[\]{}:;,@]+", PUNCTUATION),
        Rule::token(r"(?m)(#|!=|!|==|\|>|\|\||\||\->|<\-|&&|<<|>>|\.\.|\.|=)", PUNCTUATION),
        Rule::token(r"(?m)(<>|\+\.?|\-\.?|\*\.?|/\.?|%\.?|<=\.?|>=\.?|<\.?|>\.?|=)", OPERATOR),
        Rule::token(r#"(?m)"(\\"|[^"])*""#, STRING),
        Rule::bygroups(r"(?m)\b(let)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)\b(fn)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)[a-zA-Z_/]\w*", NAME),
        Rule::token(r"(?m)(\d+(_\d+)*\.(?!\.)(\d+(_\d+)*)?|\.\d+(_\d+)*)([eEf][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+(_\d+)*[eEf][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+(_[a-fA-F0-9]+)*(\.([a-fA-F0-9]+(_[a-fA-F0-9]+)*)?)?p[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0[bB][01]+(_[01]+)*", NUMBER_BIN),
        Rule::token(r"(?m)0[oO][0-7]+(_[0-7]+)*", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+(_[a-fA-F0-9]+)*", NUMBER_HEX),
        Rule::token(r"(?m)\d+(_\d+)*", NUMBER_INTEGER),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for GleamLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
