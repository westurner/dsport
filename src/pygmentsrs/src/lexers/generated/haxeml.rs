//! AUTO-GENERATED from `pygments.pygments.lexers.haxe:HxmlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.haxe:HxmlLexer:haxeml

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: haxeml, hxml
pub struct HaxemlLexer;

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
        Rule::bygroups(r"(?m)(--)(next)", vec![Some(PUNCTUATION), Some(GENERIC_HEADING)]),
        Rule::bygroups(r"(?m)(-)(prompt|debug|v)", vec![Some(PUNCTUATION), Some(TokenType::new(&["Keyword", "Keyword"]))]),
        Rule::bygroups(r"(?m)(--)(neko-source|flash-strict|flash-use-stage|no-opt|no-traces|no-inline|times|no-output)", vec![Some(PUNCTUATION), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)(-)(cpp|js|neko|x|as3|swf9?|swf-lib|php|xml|main|lib|D|resource|cp|cmd)( +)(.+)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE), Some(STRING)]),
        Rule::bygroups(r"(?m)(-)(swf-version)( +)(\d+)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?m)(-)(swf-header)( +)(\d+)(:)(\d+)(:)(\d+)(:)([A-Fa-f0-9]{6})", vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(NUMBER_INTEGER), Some(PUNCTUATION), Some(NUMBER_HEX)]),
        Rule::bygroups(r"(?m)(--)(js-namespace|php-front|php-lib|remap|gen-hx-classes)( +)(.+)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE), Some(STRING)]),
        Rule::token(r"(?m)#.*", COMMENT_SINGLE),
    ]);
    Table(m)
}

impl Lexer for HaxemlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
