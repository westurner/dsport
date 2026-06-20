//! AUTO-GENERATED from `pygments.pygments.lexers.floscript:FloScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.floscript:FloScriptLexer:floscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: floscript, flo
pub struct FloscriptLexer;

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
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(to|by|with|from|per|for|cum|qua|via|as|at|in|of|on|re|is|if|be|into|and|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|[-~+/*%=<>&^|.]", OPERATOR),
        Rule::token(r"(?m)(load|init|server|logger|log|loggee|first|over|under|next|done|timeout|repeat|native|benter|enter|recur|exit|precur|renter|rexit|print|put|inc|copy|set|aux|rear|raze|go|let|do|bid|ready|start|stop|run|abort|use|flo|give|take)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(frame|framer|house)\b", KEYWORD),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)@[\w.]+", NAME_DECORATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        Rule::token(r"(?m)#.+$", COMMENT_SINGLE),
    ]);
    m.insert(
        r"name",
        vec![
            Rule::token(r"(?m)@[\w.]+", NAME_DECORATOR),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
            Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
            Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(
                r"(?m)\d+L",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^"]+"#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for FloscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
