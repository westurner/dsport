//! AUTO-GENERATED from `pygments.pygments.lexers.asm:GasLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:GasLexer:gas

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gas, asm
pub struct GasLexer;

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
        Rule::token(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*][\w\W]*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+):", NAME_LABEL),
        Rule::token_to(r"(?m)\.(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)", NAME_ATTRIBUTE, NewState::Push(vec![r"directive-args"])),
        Rule::token(r"(?m)lock|rep(n?z)?|data\d+", NAME_ATTRIBUTE),
        Rule::token_to(r"(?m)(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)", NAME_FUNCTION, NewState::Push(vec![r"instruction-args"])),
        Rule::token(r"(?m)[\r\n]+", TEXT),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*][\w\W]*?[*]/", COMMENT_MULTILINE),
    ]);
    m.insert(r"directive-args", vec![
        Rule::token(r"(?m)(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)", NAME_CONSTANT),
        Rule::token(r#"(?m)"(\\"|[^"])*""#, STRING),
        Rule::token(r"(?m)@(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:0[xX][a-fA-F0-9]+|#?-?\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)%(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)\b", NAME_VARIABLE),
        Rule::token_to(r"(?m)[\r\n]+", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)/[*].*?[*]/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/[*].*?\n[\w\W]*?[*]/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[-*,.()\[\]!:{}]+", PUNCTUATION),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*][\w\W]*?[*]/", COMMENT_MULTILINE),
    ]);
    m.insert(r"punctuation", vec![
        Rule::token(r"(?m)[-*,.()\[\]!:{}]+", PUNCTUATION),
    ]);
    m.insert(r"instruction-args", vec![
        Rule::bygroups(r"(?m)([a-z0-9]+)( )(<)((?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+))(>)", vec![Some(NUMBER_HEX), Some(TEXT), Some(PUNCTUATION), Some(NAME_CONSTANT), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)([a-z0-9]+)( )(<)((?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+))([-+])((?:0[xX][a-fA-F0-9]+|#?-?\d+))(>)", vec![Some(NUMBER_HEX), Some(TEXT), Some(PUNCTUATION), Some(NAME_CONSTANT), Some(PUNCTUATION), Some(NUMBER_INTEGER), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)", NAME_CONSTANT),
        Rule::token(r"(?m)(?:0[xX][a-fA-F0-9]+|#?-?\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)%(?:[a-zA-Z$_][\w$.@-]*|\.[\w$.@-]+)\b", NAME_VARIABLE),
        Rule::token(r"(?m)$(?:0[xX][a-fA-F0-9]+|#?-?\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)$'(.|\\')'", STRING_CHAR),
        Rule::token_to(r"(?m)[\r\n]+", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)/[*].*?[*]/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/[*].*?\n[\w\W]*?[*]/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[-*,.()\[\]!:{}]+", PUNCTUATION),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)([;#]|//).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/[*][\w\W]*?[*]/", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for GasLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
