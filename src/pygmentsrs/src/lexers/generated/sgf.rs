#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sgf:SmartGameFormatLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sgf:SmartGameFormatLexer:sgf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sgf
pub struct SgfLexer;

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
        Rule::token(r"(?m)[():;]+", PUNCTUATION),
        Rule::token(r"(?m)(A[BW]|AE|AN|AP|AR|AS|[BW]L|BM|[BW]R|[BW]S|[BW]T|CA|CH|CP|CR|DD|DM|DO|DT|EL|EV|EX|FF|FG|G[BW]|GC|GM|GN|HA|HO|ID|IP|IT|IY|KM|KO|LB|LN|LT|L|MA|MN|M|N|OB|OM|ON|OP|OT|OV|P[BW]|PC|PL|PM|RE|RG|RO|RU|SO|SC|SE|SI|SL|SO|SQ|ST|SU|SZ|T[BW]|TC|TE|TM|TR|UC|US|VW|V|[BW]|C)", NAME_BUILTIN),
        Rule::bygroups(r"(?m)(\[)([0-9.]+)(\])", vec![Some(PUNCTUATION), Some(NUMBER), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)([0-9]{4}-[0-9]{2}-[0-9]{2})(\])", vec![Some(PUNCTUATION), Some(LITERAL_DATE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)([a-z]{2})(\])", vec![Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)([a-z]{2})(:)([a-z]{2})(\])", vec![Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)([\w\s#()+,\-.:?]+)(\])", vec![Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(\[)(\s.*)(\])", vec![Some(PUNCTUATION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for SgfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
