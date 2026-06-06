//! AUTO-GENERATED from `pygments.pygments.lexers.whiley:WhileyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.whiley:WhileyLexer:whiley

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: whiley
pub struct WhileyLexer;

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
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?s)/\*\*.*?\*/", STRING_DOC),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(a(?:ll|ss(?:ert|ume))|break|c(?:a(?:se|tch)|ontinue)|d(?:e(?:bug|fault)|o)|e(?:lse|nsures)|f(?:ail|inite|or)|i(?:[fns])|n(?:ew|o)|re(?:quires|turn)|s(?:kip|ome|witch)|t(?:hrow|otal|ry)|wh(?:(?:er|il)e))\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(export|function|method|native|p(?:r(?:ivate|otected)|ublic))\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?m)(constant|type)(\s+)([a-zA-Z_]\w*)(\s+)(is)\b", vec![Some(KEYWORD_DECLARATION), Some(TEXT), Some(NAME), Some(TEXT), Some(KEYWORD_RESERVED)]),
        Rule::token(r"(?m)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(bool|byte|int|real|any|void)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?m)(import)(\s+)(\*)([^\S\n]+)(from)\b", vec![Some(KEYWORD_NAMESPACE), Some(TEXT), Some(PUNCTUATION), Some(TEXT), Some(KEYWORD_NAMESPACE)]),
        Rule::bygroups(r"(?m)(import)(\s+)([a-zA-Z_]\w*)([^\S\n]+)(from)\b", vec![Some(KEYWORD_NAMESPACE), Some(TEXT), Some(NAME), Some(TEXT), Some(KEYWORD_NAMESPACE)]),
        Rule::token(r"(?m)(package|import)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(i(?:16|32|64|8)|nat|toString|u(?:16|32|64|8|int))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[01]+b", NUMBER_BIN),
        Rule::token(r"(?m)[0-9]+\.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+\.(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'[^\\]'", STRING_CHAR),
        Rule::bygroups(r#"(?m)(')(\\['"\\btnfr])(')"#, vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(STRING_CHAR)]),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[{}()\[\],.;]", PUNCTUATION),
        Rule::token(r"(?m)[+\-*/%&|<>^!~@=:?\u2200\u2203\u2205\u2282\u2286\u2283\u2287\u222A\u2229\u2264\u2265\u2208\u2227\u2228]", OPERATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\[btnfr]", STRING_ESCAPE),
        Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
        Rule::token(r"(?m)\\.", STRING),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
    ]);
    Table(m)
}

impl Lexer for WhileyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
