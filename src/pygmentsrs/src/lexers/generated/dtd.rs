//! AUTO-GENERATED from `pygments.pygments.lexers.html:DtdLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.html:DtdLexer:dtd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dtd
pub struct DtdLexer;

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
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
        Rule::bygroups_to(r"(?ms)(<!ELEMENT)(\s+)(\S+)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_TAG)], NewState::Push(vec![r"element"])),
        Rule::bygroups_to(r"(?ms)(<!ATTLIST)(\s+)(\S+)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_TAG)], NewState::Push(vec![r"attlist"])),
        Rule::bygroups_to(r"(?ms)(<!ENTITY)(\s+)(\S+)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_ENTITY)], NewState::Push(vec![r"entity"])),
        Rule::bygroups_to(r"(?ms)(<!NOTATION)(\s+)(\S+)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_TAG)], NewState::Push(vec![r"notation"])),
        Rule::bygroups(r"(?ms)(<!\[)([^\[\s]+)(\s*)(\[)", vec![Some(KEYWORD), Some(NAME_ENTITY), Some(TEXT), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(<!DOCTYPE)(\s+)([^>\s]+)", vec![Some(KEYWORD), Some(TEXT), Some(NAME_TAG)]),
        Rule::token(r"(?ms)PUBLIC|SYSTEM", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[\[\]>]", KEYWORD),
    ]);
    m.insert(r"common", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?ms)[^-]+", COMMENT),
        Rule::token_to(r"(?ms)-->", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?ms)-", COMMENT),
    ]);
    m.insert(r"element", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
        Rule::token(r"(?ms)EMPTY|ANY|#PCDATA", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[^>\s|()?+*,]+", NAME_TAG),
        Rule::token_to(r"(?ms)>", KEYWORD, NewState::Pop(1)),
    ]);
    m.insert(r"attlist", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
        Rule::token(r"(?ms)CDATA|IDREFS|IDREF|ID|NMTOKENS|NMTOKEN|ENTITIES|ENTITY|NOTATION", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)#REQUIRED|#IMPLIED|#FIXED", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)xml:space|xml:lang", KEYWORD_RESERVED),
        Rule::token(r"(?ms)[^>\s|()?+*,]+", NAME_ATTRIBUTE),
        Rule::token_to(r"(?ms)>", KEYWORD, NewState::Pop(1)),
    ]);
    m.insert(r"entity", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
        Rule::token(r"(?ms)SYSTEM|PUBLIC|NDATA", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[^>\s|()?+*,]+", NAME_ENTITY),
        Rule::token_to(r"(?ms)>", KEYWORD, NewState::Pop(1)),
    ]);
    m.insert(r"notation", vec![
        Rule::token(r"(?ms)\s+", TEXT),
        Rule::token(r"(?ms)(%|&)[^;]*;", NAME_ENTITY),
        Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)[(|)*,?+]", OPERATOR),
        Rule::token(r#"(?ms)"[^"]*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)\'[^\']*\'", STRING_SINGLE),
        Rule::token(r"(?ms)SYSTEM|PUBLIC", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[^>\s|()?+*,]+", NAME_ATTRIBUTE),
        Rule::token_to(r"(?ms)>", KEYWORD, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for DtdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
