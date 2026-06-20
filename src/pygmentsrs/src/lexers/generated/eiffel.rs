#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.eiffel:EiffelLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.eiffel:EiffelLexer:eiffel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: eiffel
pub struct EiffelLexer;

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
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)--.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)(?i)(true|false|void|current|result|precursor)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?i)(not|xor|implies|or)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)(?i)(and)(?:(\s+)(then))?\b", vec![Some(OPERATOR_WORD), Some(WHITESPACE), Some(OPERATOR_WORD)]),
        Rule::bygroups(r"(?m)(?i)(or)(?:(\s+)(else))?\b", vec![Some(OPERATOR_WORD), Some(WHITESPACE), Some(OPERATOR_WORD)]),
        Rule::token(r"(?m)(?i)\b(a(?:cross|gent|l(?:ias|l)|s(?:(?:sign)?)|tt(?:ached|ribute))|c(?:heck|lass|onvert|reate)|d(?:e(?:bug|ferred|tachable)|o)|e(?:lse(?:(?:if)?)|n(?:d|sure)|x(?:p(?:anded|ort)|ternal))|f(?:eature|ro(?:m|zen))|i(?:f|n(?:(?:heri|spec|varian)t))|l(?:ike|o(?:cal|op))|no(?:(?:[nt])e)|o(?:bsolete|ld|n(?:ce|ly))|re(?:define|name|quire|scue|try)|se(?:lect|parate)|then|un(?:define|til)|variant|when)\b", KEYWORD_RESERVED),
        Rule::token(r#"(?m)"\[([^\]%]|%(.|\n)|\][^"])*?\]""#, STRING),
        Rule::token(r#"(?m)"([^"%\n]|%.)*?""#, STRING),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)0[cC][0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)([0-9]+\.[0-9]*)|([0-9]*\.[0-9]+)", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'([^'%]|%'|%%)'", STRING_CHAR),
        Rule::token(r"(?m)(//|\\\\|>=|<=|:=|/=|~|/~|[\\?!#%&@|+/\-=>*$<^\[\]])", OPERATOR),
        Rule::token(r"(?m)([{}():;,.])", PUNCTUATION),
        Rule::token(r"(?m)([a-z]\w*)|([A-Z][A-Z0-9_]*[a-z]\w*)", NAME),
        Rule::token(r"(?m)([A-Z][A-Z0-9_]*)", NAME_CLASS),
        Rule::token(r"(?m)\n+", WHITESPACE),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
            Rule::token(r"(?m)0[cC][0-7]+", NUMBER_OCT),
            Rule::token(r"(?m)([0-9]+\.[0-9]*)|([0-9]*\.[0-9]+)", NUMBER_FLOAT),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        ],
    );
    Table(m)
}

impl Lexer for EiffelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
