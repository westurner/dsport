#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.go:GoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.go:GoLexer:go

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: go, golang
pub struct GoLexer;

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
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(import|package)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(var|func|struct|map|chan|type|interface|const)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(break|c(?:(?:as|ontinu)e)|def(?:ault|er)|else|f(?:allthrough|or)|go(?:(?:to)?)|if|r(?:ange|eturn)|s(?:elect|witch))\b", KEYWORD),
        Rule::token(r"(?m)(true|false|iota|nil)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)(a(?:ny|ppend)|b(?:ool|yte)|c(?:ap|l(?:ear|ose)|o(?:mp(?:arable|lex(?:(?:128|64)?))|py))|delete|error|float(?:(?:32|64)?)|i(?:mag|nt(?:(?:16|32|64|8)?))|len|m(?:a(?:ke|x)|in)|new|p(?:anic|rint(?:(?:ln)?))|r(?:e(?:al|cover)|une)|string|uint(?:(?:16|32|64|8|ptr)?))\b(\()", vec![Some(NAME_BUILTIN), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(any|b(?:ool|yte)|comp(?:arable|lex(?:128|64))|error|float(?:(?:32|64)?)|int(?:(?:16|32|64|8)?)|rune|string|uint(?:(?:16|32|64|8|ptr)?))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\d+i", NUMBER),
        Rule::token(r"(?m)\d+\.\d*([Ee][-+]\d+)?i", NUMBER),
        Rule::token(r"(?m)\.\d+([Ee][-+]\d+)?i", NUMBER),
        Rule::token(r"(?m)\d+[Ee][-+]\d+i", NUMBER),
        Rule::token(r"(?m)\d+(\.\d+[eE][+\-]?\d+|\.\d*|[eE][+\-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)\.\d+([eE][+\-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)(0|[1-9][0-9]*)", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(\\['"\\abfnrtv]|\\x[0-9a-fA-F]{2}|\\[0-7]{1,3}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|[^\\])'"#, STRING_CHAR),
        Rule::token(r"(?m)`[^`]*`", STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(<<=|>>=|<<|>>|<=|>=|&\^=|&\^|\+=|-=|\*=|/=|%=|&=|\|=|&&|\|\||<-|\+\+|--|==|!=|:=|\.\.\.|[+\-*/%&]|~|\|)", OPERATOR),
        Rule::token(r"(?m)[|^<>=!()\[\]{}.,;:]", PUNCTUATION),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_OTHER),
    ]);
    Table(m)
}

impl Lexer for GoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
