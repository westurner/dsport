//! AUTO-GENERATED from `pygments.pygments.lexers.numbair:NumbaIRLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.numbair:NumbaIRLexer:numba_ir

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: numba_ir, numbair
pub struct NumbaIrLexer;

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
        Rule::bygroups(r"(?m)(label)(\ [0-9]+)(:)$", vec![Some(KEYWORD), Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)(\n|\s)+", WHITESPACE),
        Rule::token(r"(?m)(branch|call|del|jump) ", KEYWORD),
        Rule::token(r"(?m)\$[a-zA-Z0-9._]+", NAME_VARIABLE),
        Rule::bygroups(r"(?m)([a-zA-Z_]+[a-zA-Z0-9]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)([a-zA-Z_]+[a-zA-Z0-9]*)(\=)", vec![Some(NAME_ATTRIBUTE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)([a-zA-Z_]+[a-zA-Z0-9]*)", NAME_CONSTANT),
        Rule::token(r"(?m)[0-9]+", NUMBER),
        Rule::token(r"(?m)<[^>\n]*>", STRING),
        Rule::token(r"(?m)[=<>{}\[\]()*.,!\':]|x\b", PUNCTUATION),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)(\n|\s)+", WHITESPACE),
    ]);
    m.insert(r"keyword", vec![
        Rule::token(r"(?m)(branch|call|del|jump) ", KEYWORD),
    ]);
    Table(m)
}

impl Lexer for NumbaIrLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
