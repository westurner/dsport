#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ezhil:EzhilLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ezhil:EzhilLexer:ezhil

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ezhil
pub struct EzhilLexer;

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
        Rule::token(r"(?m)பதிப்பி|தேர்ந்தெடு|தேர்வு|ஏதேனில்|ஆனால்|இல்லைஆனால்|இல்லை|ஆக|ஒவ்வொன்றாக|இல்|வரை|செய்|முடியேனில்|பின்கொடு|முடி|நிரல்பாகம்|தொடர்|நிறுத்து|நிரல்பாகம்", KEYWORD),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[@+/*,^\-%]|[!<>=]=?|&&?|\|\|?", OPERATOR),
        Rule::token(r"(?m)இல்", OPERATOR_WORD),
        Rule::token(r"(?m)(assert|cos|ex(?:it|p)|hypot|log(?:(?:10)?)|m(?:ax|in)|p(?:i|ow)|s(?:in|qrt)|tan|எடு|கோப்பை_(?:எழுது|திற|மூடு)|சரம்_(?:இடமாற்று|கண்டுபிடி)|தலைகீழ்|ந(?:ீ(?:ட்டிக்க|ளம்)|ுழைக்க)|ப(?:ட்டியல்|ின்இணை)|வ(?:ரிசைப்படுத்து|ை))\b", NAME_BUILTIN),
        Rule::token(r"(?m)(True|False)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)(?:[a-zA-Z_]|[஀-௿])(?:[0-9]|[a-zA-Z_]|[஀-௿])*", NAME),
        Rule::token(r#"(?m)".*?""#, STRING),
        Rule::token(r"(?m)\d+((\.\d*)?[eE][+-]?\d+|\.\d*)", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[(){}\[\]:;.]", PUNCTUATION),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)பதிப்பி|தேர்ந்தெடு|தேர்வு|ஏதேனில்|ஆனால்|இல்லைஆனால்|இல்லை|ஆக|ஒவ்வொன்றாக|இல்|வரை|செய்|முடியேனில்|பின்கொடு|முடி|நிரல்பாகம்|தொடர்|நிறுத்து|நிரல்பாகம்", KEYWORD),
    ]);
    m.insert(
        r"identifier",
        vec![Rule::token(
            r"(?m)(?:[a-zA-Z_]|[஀-௿])(?:[0-9]|[a-zA-Z_]|[஀-௿])*",
            NAME,
        )],
    );
    m.insert(
        r"literal",
        vec![
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token(r"(?m)\d+((\.\d*)?[eE][+-]?\d+|\.\d*)", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    Table(m)
}

impl Lexer for EzhilLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
