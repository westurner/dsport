#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ruby:FancyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ruby:FancyLexer:fancy

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fancy, fy
pub struct FancyLexer;

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
        r"balanced-regex",
        vec![
            Rule::token_to(
                r"(?m)/(\\\\|\\[^\\]|[^/\\])*/[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)!(\\\\|\\[^\\]|[^!\\])*![egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\\(\\\\|[^\\])*\\[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\{(\\\\|\\[^\\]|[^}\\])*\}[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)<(\\\\|\\[^\\]|[^>\\])*>[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\[(\\\\|\\[^\\]|[^\]\\])*\][egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\((\\\\|\\[^\\]|[^)\\])*\)[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)@(\\\\|\\[^\\]|[^@\\])*@[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)%(\\\\|\\[^\\]|[^%\\])*%[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\$(\\\\|\\[^\\]|[^$\\])*\$[egimosx]*",
                STRING_REGEX,
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)s\{(\\\\|\\[^\\]|[^}\\])*\}\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?m)s<(\\\\|\\[^\\]|[^>\\])*>\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?m)s\[(\\\\|\\[^\\]|[^\]\\])*\]\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?m)s\((\\\\|\\[^\\]|[^)\\])*\)\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token(r"(?m)m?/(\\\\|\\[^\\]|[^///\n])*/[gcimosx]*", STRING_REGEX),
        Rule::token_to(r"(?m)m(?=[/!\\{<\[(@%$])", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token(r"(?m)#(.*?)\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\'([^\'\s\[\](){}]+|\[\])", STRING_SYMBOL),
        Rule::token(r#"(?m)"""(\\\\|\\[^\\]|[^\\])*?""""#, STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(def|class|try|catch|finally|retry|return|return_local|match|case|->|=>)\b", KEYWORD),
        Rule::token(r"(?m)(self|super|nil|false|true)\b", NAME_CONSTANT),
        Rule::token(r"(?m)[(){};,/?|:\\]", PUNCTUATION),
        Rule::token(r"(?m)(Array|Block|Class|Directory|Enumerable|F(?:a(?:lseClass|ncy(?:Enumerable|Spec))|ile)|Hash|Method|N(?:ilClass|umber)|Object|Package|Range|S(?:et|t(?:ack|ring)|ymbol)|T(?:rueClass|uple))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z](\w|[-+?!=*/^><%])*:", NAME_FUNCTION),
        Rule::token(r"(?m)[-+*/~,<>=&!?%^\[\].$]+", OPERATOR),
        Rule::token(r"(?m)[A-Z]\w*", NAME_CONSTANT),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?m)@@[a-zA-Z_]\w*", NAME_VARIABLE_CLASS),
        Rule::token(r"(?m)@@?", OPERATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::bygroups(r"(?m)(0[oO]?[0-7]+(?:_[0-7]+)*)(\s*)([/?])?", vec![Some(NUMBER_OCT), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(0[xX][0-9A-Fa-f]+(?:_[0-9A-Fa-f]+)*)(\s*)([/?])?", vec![Some(NUMBER_HEX), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(0[bB][01]+(?:_[01]+)*)(\s*)([/?])?", vec![Some(NUMBER_BIN), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)([\d]+(?:_\d+)*)(\s*)([/?])?", vec![Some(NUMBER_INTEGER), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)\d+([eE][+-]?[0-9]+)|\d+\.\d+([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    Table(m)
}

impl Lexer for FancyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
