#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.smalltalk:NewspeakLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.smalltalk:NewspeakLexer:newspeak

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: newspeak
pub struct NewspeakLexer;

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
            Rule::token(r"(?m)\b(Newsqueak2)\b", KEYWORD_DECLARATION),
            Rule::token(r"(?m)'[^']*'", STRING),
            Rule::bygroups(
                r"(?m)\b(class)(\s+)(\w+)(\s*)",
                vec![
                    Some(KEYWORD_DECLARATION),
                    Some(TEXT),
                    Some(NAME_CLASS),
                    Some(TEXT),
                ],
            ),
            Rule::token(
                r"(?m)\b(mixin|self|super|private|public|protected|nil|true|false)\b",
                KEYWORD,
            ),
            Rule::bygroups(
                r"(?m)(\w+\:)(\s*)([a-zA-Z_]\w+)",
                vec![Some(NAME_FUNCTION), Some(TEXT), Some(NAME_VARIABLE)],
            ),
            Rule::bygroups(
                r"(?m)(\w+)(\s*)(=)",
                vec![Some(NAME_ATTRIBUTE), Some(TEXT), Some(OPERATOR)],
            ),
            Rule::token(r"(?m)<\w+>", COMMENT_SPECIAL),
            Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m):\w+", NAME_VARIABLE),
            Rule::bygroups(r"(?m)(\w+)(::)", vec![Some(NAME_VARIABLE), Some(OPERATOR)]),
            Rule::token(r"(?m)\w+:", NAME_FUNCTION),
            Rule::token(r"(?m)\w+", NAME_VARIABLE),
            Rule::token(r"(?m)\(|\)", PUNCTUATION),
            Rule::token(r"(?m)\[|\]", PUNCTUATION),
            Rule::token(r"(?m)\{|\}", PUNCTUATION),
            Rule::token(r"(?m)(\^|\+|\/|~|\*|<|>|=|@|%|\||&|\?|!|,|-|:)", OPERATOR),
            Rule::token(r"(?m)\.|;", PUNCTUATION),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"[^"]*""#, COMMENT),
            Rule::token(r"(?m)\$.", STRING),
            Rule::token(r"(?m)'[^']*'", STRING),
            Rule::token(r"(?m)#'[^']*'", STRING_SYMBOL),
            Rule::token(r"(?m)#\w+:?", STRING_SYMBOL),
            Rule::token(r"(?m)#(\+|\/|~|\*|<|>|=|@|%|\||&|\?|!|,|-)+", STRING_SYMBOL),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"[^"]*""#, COMMENT),
        ],
    );
    m.insert(
        r"expressionstat",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m):\w+", NAME_VARIABLE),
            Rule::bygroups(r"(?m)(\w+)(::)", vec![Some(NAME_VARIABLE), Some(OPERATOR)]),
            Rule::token(r"(?m)\w+:", NAME_FUNCTION),
            Rule::token(r"(?m)\w+", NAME_VARIABLE),
            Rule::token(r"(?m)\(|\)", PUNCTUATION),
            Rule::token(r"(?m)\[|\]", PUNCTUATION),
            Rule::token(r"(?m)\{|\}", PUNCTUATION),
            Rule::token(r"(?m)(\^|\+|\/|~|\*|<|>|=|@|%|\||&|\?|!|,|-|:)", OPERATOR),
            Rule::token(r"(?m)\.|;", PUNCTUATION),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"[^"]*""#, COMMENT),
            Rule::token(r"(?m)\$.", STRING),
            Rule::token(r"(?m)'[^']*'", STRING),
            Rule::token(r"(?m)#'[^']*'", STRING_SYMBOL),
            Rule::token(r"(?m)#\w+:?", STRING_SYMBOL),
            Rule::token(r"(?m)#(\+|\/|~|\*|<|>|=|@|%|\||&|\?|!|,|-)+", STRING_SYMBOL),
        ],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)"[^"]*""#, COMMENT),
        ],
    );
    m.insert(
        r"literals",
        vec![
            Rule::token(r"(?m)\$.", STRING),
            Rule::token(r"(?m)'[^']*'", STRING),
            Rule::token(r"(?m)#'[^']*'", STRING_SYMBOL),
            Rule::token(r"(?m)#\w+:?", STRING_SYMBOL),
            Rule::token(r"(?m)#(\+|\/|~|\*|<|>|=|@|%|\||&|\?|!|,|-)+", STRING_SYMBOL),
        ],
    );
    Table(m)
}

impl Lexer for NewspeakLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
