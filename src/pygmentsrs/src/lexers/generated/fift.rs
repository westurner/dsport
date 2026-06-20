//! AUTO-GENERATED from `pygments.pygments.lexers.fift:FiftLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.fift:FiftLexer:fift

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fift, fif
pub struct FiftLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//.*", TokenType::new(&["Comment", "Singleline"])),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token_to(r#"(?m)[\.+]?\""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
            Rule::token(
                r#"(?m)-?[0-9]+("/"-?[0-9]+)?"#,
                TokenType::new(&["Literal", "Number", "Decimal"]),
            ),
            Rule::token(r"(?m)b\{[01]+\}", LITERAL),
            Rule::token(r"(?m)x\{[0-9a-fA-F_]+\}", LITERAL),
            Rule::token(r"(?m)B\{[0-9a-fA-F_]+\}", LITERAL),
            Rule::token(r"(?m)\S+", NAME),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*", TokenType::new(&["Comment", "Singleline"])),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)\""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\"\r\n\\]+"#, STRING),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for FiftLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
