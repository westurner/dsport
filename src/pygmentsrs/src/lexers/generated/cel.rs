//! AUTO-GENERATED from `pygments.pygments.lexers.cel:CELLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.cel:CELLexer:cel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cel
pub struct CelLexer;

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
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::token_to(r#"(?m)(?:[bB][rR]|[rR][bB]|[rR])""""#, STRING, NewState::Push(vec![r"triple-double-raw"])),
        Rule::token_to(r"(?m)(?:[bB][rR]|[rR][bB]|[rR])'''", STRING, NewState::Push(vec![r"triple-single-raw"])),
        Rule::token_to(r#"(?m)[bB]?""""#, STRING, NewState::Push(vec![r"triple-double"])),
        Rule::token_to(r"(?m)[bB]?'''", STRING, NewState::Push(vec![r"triple-single"])),
        Rule::token(r#"(?m)(?:[bB][rR]|[rR][bB]|[rR])"[^"\r\n]*""#, STRING),
        Rule::token(r"(?m)(?:[bB][rR]|[rR][bB]|[rR])'[^'\r\n]*'", STRING),
        Rule::token_to(r#"(?m)[bB]?""#, STRING, NewState::Push(vec![r"double-string"])),
        Rule::token_to(r"(?m)[bB]?'", STRING, NewState::Push(vec![r"single-string"])),
        Rule::token(r"(?m)(?:0[xX][0-9a-fA-F]+|0|[1-9][0-9]*)[uU]", NUMBER_INTEGER),
        Rule::token(r"(?m)(?:[0-9]+\.[0-9]+(?:[eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+|\.[0-9]+(?:[eE][+-]?[0-9]+)?)", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+|0|[1-9][0-9]*", NUMBER_INTEGER),
        Rule::token(r"(?m)(as|break|con(?:st|tinue)|else|f(?:or|unction)|i(?:f|mport)|l(?:et|oop)|namespace|package|return|v(?:ar|oid)|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)in\b", KEYWORD),
        Rule::token(r"(?m)(false|null|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=", OPERATOR),
        Rule::token(r"(?m)[+\-*/%<>!?]", OPERATOR),
        Rule::token(r"(?m)[(){}\[\]]", PUNCTUATION),
        Rule::token(r"(?m)[,.]", PUNCTUATION),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token(r"(?m)[^\W\d]\w*", NAME),
    ]);
    m.insert(r"triple-double-raw", vec![
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)"(?!"")"#, STRING),
        Rule::token(r#"(?m)[^"]+"#, STRING),
    ]);
    m.insert(r"triple-single-raw", vec![
        Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)'(?!'')", STRING),
        Rule::token(r"(?m)[^']+", STRING),
    ]);
    m.insert(r"triple-double", vec![
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\(?:[abfnrtv\\"'`?]|[xX][0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|[0-3][0-7][0-7]|.)"#, STRING_ESCAPE),
        Rule::token(r#"(?m)"(?!"")"#, STRING),
        Rule::token(r#"(?m)[^"\\]+"#, STRING),
    ]);
    m.insert(r"triple-single", vec![
        Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\(?:[abfnrtv\\"'`?]|[xX][0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|[0-3][0-7][0-7]|.)"#, STRING_ESCAPE),
        Rule::token(r"(?m)'(?!'')", STRING),
        Rule::token(r"(?m)[^'\\]+", STRING),
    ]);
    m.insert(r"double-string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\(?:[abfnrtv\\"'`?]|[xX][0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|[0-3][0-7][0-7]|.)"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^"\\\r\n]+"#, STRING),
    ]);
    m.insert(r"single-string", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\(?:[abfnrtv\\"'`?]|[xX][0-9a-fA-F]{2}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|[0-3][0-7][0-7]|.)"#, STRING_ESCAPE),
        Rule::token(r"(?m)[^'\\\r\n]+", STRING),
    ]);
    Table(m)
}

impl Lexer for CelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
