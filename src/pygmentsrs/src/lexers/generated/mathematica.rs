#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.algebra:MathematicaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.algebra:MathematicaLexer:mathematica

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mathematica, mma, nb, wl, wolfram
pub struct MathematicaLexer;

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
        Rule::token(r"(?m)(?s)\(\*.*?\*\)", COMMENT),
        Rule::token(r"(?m)([a-zA-Z]+[A-Za-z0-9]*`)", NAME_NAMESPACE),
        Rule::token(r"(?m)([A-Za-z0-9]*_+[A-Za-z0-9]*)", NAME_VARIABLE),
        Rule::token(r"(?m)#\d*", NAME_VARIABLE),
        Rule::token(r"(?m)([a-zA-Z]+[a-zA-Z0-9]*)", NAME),
        Rule::token(r"(?m)-?\d+\.\d*", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d*\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(!===|\&\&|\->|/(?:[./;@])|:(?:[=>])|;;|<(?:[=>])|=(?:\.|==)|>=|@@(?:(?:@)?)|\|\||\~\~|[!&*+\-/<=>?@\^|])", OPERATOR),
        Rule::token(r"(?m)([(),;\[\]{}])", PUNCTUATION),
        Rule::token(r#"(?m)".*?""#, STRING),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for MathematicaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
