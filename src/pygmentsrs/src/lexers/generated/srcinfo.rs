//! AUTO-GENERATED from `pygments.pygments.lexers.srcinfo:SrcinfoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.srcinfo:SrcinfoLexer:srcinfo

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: srcinfo
pub struct SrcinfoLexer;

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
        Rule::token(r"(?m)#.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)(arch|backup|changelog|epoch|groups|install|license|noextract|options|pkg(?:base|desc|name|rel|ver)|url|validpgpkeys)", KEYWORD, NewState::Push(vec![r"assignment"])),
        Rule::token_to(r"(?m)(c(?:(?:heckdepend|onflict)s)|depends|m(?:(?:akedepend|d5sum)s)|optdepends|provides|replaces|s(?:ha(?:(?:1|2(?:24|56)|384|512)sums)|ource))_\w+", KEYWORD, NewState::Push(vec![r"assignment"])),
        Rule::token_to(r"(?m)\w+", NAME_VARIABLE, NewState::Push(vec![r"assignment"])),
    ]);
    m.insert(
        r"assignment",
        vec![
            Rule::token(r"(?m) +", WHITESPACE),
            Rule::token_to(r"(?m)=", OPERATOR, NewState::Push(vec![r"value"])),
        ],
    );
    m.insert(
        r"value",
        vec![
            Rule::token(r"(?m) +", WHITESPACE),
            Rule::token_to(r"(?m).*", TEXT, NewState::Pop(2)),
        ],
    );
    Table(m)
}

impl Lexer for SrcinfoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
