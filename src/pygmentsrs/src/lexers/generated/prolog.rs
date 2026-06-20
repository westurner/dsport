#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.prolog:PrologLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.prolog:PrologLexer:prolog

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: prolog
pub struct PrologLexer;

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
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"nested-comment"])),
        Rule::token(r"(?m)%.*", COMMENT_SINGLE),
        Rule::token(r"(?m)0\'.", STRING_CHAR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d\d?\'[a-zA-Z0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[\[\](){}|.,;!]", PUNCTUATION),
        Rule::token(r"(?m):-|-->", PUNCTUATION),
        Rule::token(r#"(?m)"(?:\\x[0-9a-fA-F]+\\|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\[0-7]+\\|\\["\\abcefnrstv]|[^\\"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(?:''|[^'])*'", TokenType::new(&["Literal", "String", "Atom"])),
        Rule::token(r"(?m)is\b", OPERATOR),
        Rule::token(r"(?m)(<|>|=<|>=|==|=:=|=|/|//|\*|\+|-)(?=\s|[a-zA-Z0-9\[])", OPERATOR),
        Rule::token(r"(?m)(mod|div|not)\b", OPERATOR),
        Rule::token(r"(?m)_", KEYWORD),
        Rule::bygroups(r"(?m)([a-z]+)(:)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)([a-z\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef][\w$\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef]*)(\s*)(:-|-->)", vec![Some(NAME_FUNCTION), Some(TEXT), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)([a-z\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef][\w$\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef]*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-z\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef][\w$\u00c0-\u1fff\u3040-\ud7ff\ue000-\uffef]*", TokenType::new(&["Literal", "String", "Atom"])),
        Rule::token(r"(?m)[#&*+\-./:<=>?@\\^~\u00a1-\u00bf\u2010-\u303f]+", TokenType::new(&["Literal", "String", "Atom"])),
        Rule::token(r"(?m)[A-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\s+|[\u2000-\u200f\ufff0-\ufffe\uffef]", TEXT),
    ]);
    m.insert(
        r"nested-comment",
        vec![
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for PrologLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
