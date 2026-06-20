//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:BefungeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:BefungeLexer:befunge

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: befunge
pub struct BefungeLexer;

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
            Rule::token(r"(?m)[0-9a-f]", NUMBER),
            Rule::token(r"(?m)[+*/%!`-]", OPERATOR),
            Rule::token(r"(?m)[<>^v?\[\]rxjk]", NAME_VARIABLE),
            Rule::token(r"(?m)[:\\$.,n]", NAME_BUILTIN),
            Rule::token(r"(?m)[|_mw]", KEYWORD),
            Rule::token(r"(?m)[{}]", NAME_TAG),
            Rule::token(r#"(?m)".*?""#, STRING_DOUBLE),
            Rule::token(r"(?m)\'.", STRING_SINGLE),
            Rule::token(r"(?m)[#;]", COMMENT),
            Rule::token(r"(?m)[pg&~=@iotsy]", KEYWORD),
            Rule::token(r"(?m)[()A-Z]", COMMENT),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for BefungeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
