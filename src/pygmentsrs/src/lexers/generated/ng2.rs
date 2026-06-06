//! AUTO-GENERATED from `pygments.pygments.lexers.templates:Angular2Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:Angular2Lexer:ng2

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ng2
pub struct Ng2Lexer;

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
        Rule::token(r"(?m)[^{(\[*#]+", OTHER),
        Rule::bygroups_to(r"(?m)(\{\{)(\s*)", vec![Some(COMMENT_PREPROC), Some(TEXT)], NewState::Push(vec![r"ngExpression"])),
        Rule::bygroups_to(r"(?m)([(\[]+)([\w:.-]+)([\])]+)(\s*)(=)(\s*)", vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(PUNCTUATION), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"attr"])),
        Rule::bygroups(r"(?m)([(\[]+)([\w:.-]+)([\])]+)(\s*)", vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(PUNCTUATION), Some(TEXT)]),
        Rule::bygroups_to(r"(?m)([*#])([\w:.-]+)(\s*)(=)(\s*)", vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(TEXT), Some(OPERATOR), Some(TEXT)], NewState::Push(vec![r"attr"])),
        Rule::bygroups(r"(?m)([*#])([\w:.-]+)(\s*)", vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE), Some(TEXT)]),
    ]);
    m.insert(r"ngExpression", vec![
        Rule::token(r"(?m)\s+(\|\s+)?", TEXT),
        Rule::token_to(r"(?m)\}\}", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?m):?(true|false)", TokenType::new(&["Literal", "String", "Boolean"])),
        Rule::token(r#"(?m):?"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m):?'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?", NUMBER),
        Rule::token(r"(?m)[a-zA-Z][\w-]*(\(.*\))?", NAME_VARIABLE),
        Rule::token(r"(?m)\.[\w-]+(\(.*\))?", NAME_VARIABLE),
        Rule::bygroups(r"(?m)(\?)(\s*)([^}\s]+)(\s*)(:)(\s*)([^}\s]+)(\s*)", vec![Some(OPERATOR), Some(TEXT), Some(STRING), Some(TEXT), Some(OPERATOR), Some(TEXT), Some(STRING), Some(TEXT)]),
    ]);
    m.insert(r"attr", vec![
        Rule::token_to(r#"(?m)".*?""#, STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)'.*?'", STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)[^\s>]+", STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for Ng2Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
