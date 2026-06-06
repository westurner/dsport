//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:CrmshLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:CrmshLexer:crmsh

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: crmsh, pcmk
pub struct CrmshLexer;

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
        Rule::bygroups(r"(?m)^(#.*)(\n)?", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::bygroups(r#"(?m)([\w#$-]+)(=)("(?:""|[^"])*"|\S+)"#, vec![Some(NAME_ATTRIBUTE), Some(PUNCTUATION), Some(STRING)]),
        Rule::bygroups(r"(?m)(node)(\s+)([\w#$-]+)(:)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(PUNCTUATION)]),
        Rule::token(r"(?m)([+-]?([0-9]+|inf)):", NUMBER),
        Rule::token(r"(?m)(acl_(?:group|target)|c(?:lone|olocation)|fencing_topology|group|location|ms|node|o(?:p_defaults|rder)|pr(?:imitive|operty)|r(?:ole|sc_(?:defaults|t(?:emplate|icket)))|tag|user)(?![\w#$-])", KEYWORD),
        Rule::token(r"(?m)(attributes|meta|op(?:(?:erations)?)|params|rule|utilization)(?![\w#$-])", KEYWORD),
        Rule::token(r"(?m)(deny|read|write)(?![\w#$-])", KEYWORD),
        Rule::token(r"(?m)(?:(?:string|version|number):)?((?:lt|gt|lte|gte|eq|ne))(?![\w#$-])", OPERATOR_WORD),
        Rule::token(r"(?m)(and|or)(?![\w#$-])", OPERATOR_WORD),
        Rule::token(r"(?m)((?:(?:not_)?)defined)(?![\w#$-])", OPERATOR_WORD),
        Rule::token(r"(?m)(date|in(?:(?:_range)?)|spec)(?![\w#$-])", OPERATOR_WORD),
        Rule::token(r"(?m)#[a-z]+(?![\w#$-])", NAME_BUILTIN),
        Rule::bygroups(r#"(?m)((?:tag|ref|reference|attribute|type|xpath))(:)("(?:""|[^"])*"|\S+)"#, vec![Some(KEYWORD), Some(PUNCTUATION), Some(NAME)]),
        Rule::bygroups(r"(?m)([\w#$-]+)(?:(:)((?:Master|Started|Slave|Stopped|start|promote|demote|stop)))?(?![\w#$-])", vec![Some(NAME), Some(PUNCTUATION), Some(OPERATOR_WORD)]),
        Rule::token(r"(?m)(\\(?=\n)|[\[\](){}/:@])", PUNCTUATION),
        Rule::token(r"(?m)\s+|\n", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for CrmshLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
