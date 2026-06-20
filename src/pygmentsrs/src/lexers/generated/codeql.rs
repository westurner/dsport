//! AUTO-GENERATED from `pygments.pygments.lexers.codeql:CodeQLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.codeql:CodeQLLexer:codeql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: codeql, ql
pub struct CodeqlLexer;

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
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token(r"(?m)(a(?:bstract|nd|s)|c(?:ached|lass)|e(?:lse|x(?:ists|te(?:nds|rnal)))|f(?:inal|orall|rom)|i(?:mp(?:lements|ort)|nstanceof|[fn])|library|module|not|o(?:r|verride)|pr(?:(?:edic(?:(?:)?)|iv)ate)|query|select|then|where)\b", TokenType::new(&["Keyword", "Builtin"])),
        Rule::token(r"(?m)\b([hist])\b\??:?", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(boolean|date|float|int|string)\b", KEYWORD_TYPE),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)[0-9]+\.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)<=|>=|<|>|=|!=|\+|-|\*|/", OPERATOR),
        Rule::token(r"(?m)[.,;:\[\]{}()]+", PUNCTUATION),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[A-Z][a-zA-Z0-9_]*", NAME_CLASS),
        Rule::token(r"(?m)[a-z][a-zA-Z0-9_]*", NAME_VARIABLE),
    ]);
    m.insert(
        r"multiline-comments",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for CodeqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
