//! AUTO-GENERATED from `pygments.pygments.lexers.rnc:RNCCompactLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rnc:RNCCompactLexer:rng_compact

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rng-compact, rnc
pub struct RngCompactLexer;

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
        Rule::token(r"(?m)namespace\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:default|datatypes)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)##.*$", COMMENT_PREPROC),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token_to(r"(?m)(?:element|attribute|mixed)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"variable"])),
        Rule::token_to(r"(?m)(text\b|xsd:[^ ]+)", KEYWORD_TYPE, NewState::Push(vec![r"maybe_xsdattributes"])),
        Rule::token(r"(?m)[,?&*=|~]|>>", OPERATOR),
        Rule::token(r"(?m)[(){}]", PUNCTUATION),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"variable", vec![
        Rule::token(r"(?m)[^{]+", NAME_VARIABLE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"maybe_xsdattributes", vec![
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"xsdattributes"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"xsdattributes", vec![
        Rule::token(r"(?m)[^ =}]", NAME_ATTRIBUTE),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r#"(?m)"[^"]*""#, STRING_DOUBLE),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for RngCompactLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
