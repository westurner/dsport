//! AUTO-GENERATED from `pygments.pygments.lexers.typoscript:TypoScriptHtmlDataLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.typoscript:TypoScriptHtmlDataLexer:typoscripthtmldata

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: typoscripthtmldata
pub struct TyposcripthtmldataLexer;

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
        Rule::token(r"(?m)(INCLUDE_TYPOSCRIPT)", NAME_CLASS),
        Rule::token(r#"(?m)(EXT|FILE|LLL):[^}\n"]*"#, STRING),
        Rule::bygroups(r"(?m)(.*)(###\w+###)(.*)", vec![Some(STRING), Some(NAME_CONSTANT), Some(STRING)]),
        Rule::bygroups(r"(?m)(\{)(\$)((?:[\w\-]+\.)*)([\w\-]+)(\})", vec![Some(STRING_SYMBOL), Some(OPERATOR), Some(NAME_CONSTANT), Some(NAME_CONSTANT), Some(STRING_SYMBOL)]),
        Rule::bygroups(r"(?m)(.*)(\{)([\w\-]+)(\s*:\s*)([\w\-]+)(\})(.*)", vec![Some(STRING), Some(STRING_SYMBOL), Some(NAME_CONSTANT), Some(OPERATOR), Some(NAME_CONSTANT), Some(STRING_SYMBOL), Some(STRING)]),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)[<>,:=.*%+|]", STRING),
        Rule::token(r#"(?m)[\w"\-!/&;(){}#]+"#, STRING),
    ]);
    Table(m)
}

impl Lexer for TyposcripthtmldataLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
