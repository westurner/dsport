//! AUTO-GENERATED from `pygments.pygments.lexers.json5:Json5Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.json5:Json5Lexer:json5

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: json5
pub struct Json5Lexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"_comments", vec![
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)[+-]?0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-.]?[0-9]+[.]?[0-9]?([eE][-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token(r"(?m)(\+Infinity|\-Infinity|Infinity|NaN|false|null|true)\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    m.insert(r"singlestring", vec![
        Rule::token(r"(?m)[^'\\]+", STRING),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r"(?m)\\", PUNCTUATION),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
    ]);
    m.insert(r"doublestring", vec![
        Rule::token(r#"(?m)[^"\\]+"#, STRING),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token(r"(?m)\\", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
    ]);
    m.insert(r"array", vec![
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)[+-]?0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-.]?[0-9]+[.]?[0-9]?([eE][-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token(r"(?m)(\+Infinity|\-Infinity|Infinity|NaN|false|null|true)\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    m.insert(r"object", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\b([^:]+)", NAME_VARIABLE, NewState::Push(vec![r"object_value"])),
        Rule::token_to(r#"(?m)""#, NAME_VARIABLE, NewState::Push(vec![r"double_field_name"])),
        Rule::token_to(r"(?m)'", NAME_VARIABLE, NewState::Push(vec![r"single_field_name"])),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
    ]);
    m.insert(r"double_field_name", vec![
        Rule::token_to(r#"(?m)([^"\\]|\\.)*""#, NAME_VARIABLE, NewState::Push(vec![r"#pop", r"object_value"])),
    ]);
    m.insert(r"single_field_name", vec![
        Rule::token_to(r"(?m)([^'\\]|\\.)*'", NAME_VARIABLE, NewState::Push(vec![r"#pop", r"object_value"])),
    ]);
    m.insert(r"object_value", vec![
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)[+-]?0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[+-.]?[0-9]+[.]?[0-9]?([eE][-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token(r"(?m)(\+Infinity|\-Infinity|Infinity|NaN|false|null|true)\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m):", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for Json5Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
