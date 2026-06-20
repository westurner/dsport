//! AUTO-GENERATED from `pygments.pygments.lexers.tls:TlsLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tls:TlsLexer:tls

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tls
pub struct TlsLexer;

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
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)/[*].*?[*]/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)(case|enum|s(?:(?:ele|tru)ct))\b", KEYWORD),
            Rule::token(r"(?ms)(opaque|uint(?:16|24|32|64|8))\b", KEYWORD_TYPE),
            Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
            Rule::token(r#"(?ms)"(\\.|[^"\\])*""#, STRING),
            Rule::token(r"(?ms)[.]{2}", OPERATOR),
            Rule::token(r"(?ms)[+\-*/&^]", OPERATOR),
            Rule::token(r"(?ms)[|<>=!()\[\]{}.,;:\?]", PUNCTUATION),
            Rule::token(r"(?ms)[^\W\d]\w*", NAME_OTHER),
        ],
    );
    Table(m)
}

impl Lexer for TlsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
