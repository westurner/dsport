//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:RslLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:RslLexer:rsl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rsl
pub struct RslLexer;

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
        Rule::token(r"(?ms)\b(\-(?:(?:inf(?:lis|se)|lis|se)t)|Bool|Char|Int|Nat|Real|Text|Unit|a(?:bs|l(?:l|ways)|ny|s|xiom)|c(?:a(?:rd|se)|ha(?:nnel|os)|lass)|d(?:evt_relation|om)|e(?:l(?:ems|if|se)|nd|x(?:ists|tend))|f(?:alse|or)|h(?:d|ide)|i(?:n(?:ds|itialise|t(?:(?:er)?))|sin|[fns])|l(?:e(?:[nt])|ocal|tl_assertion)|o(?:bject|f|ut)|p(?:ost|re)|r(?:ea(?:[dl])|ng)|s(?:cheme|(?:ki|to|wa)p)|t(?:est_case|he(?:n|ory)|l|r(?:ansition_system|ue)|ype)|u(?:n(?:ion|til)|se)|va(?:(?:lu|riabl)e)|w(?:hile|ith|rite)|\~isin)\b", KEYWORD),
        Rule::token(r"(?ms)(variable|value)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)--.*?\n", COMMENT),
        Rule::token(r"(?ms)<:.*?:>", COMMENT),
        Rule::token(r"(?ms)\{!.*?!\}", COMMENT),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT),
        Rule::bygroups(r"(?ms)^([ \t]*)([\w]+)([ \t]*)(:[^:])", vec![Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?ms)(^[ \t]*)([\w]+)([ \t]*)(\([\w\s,]*\))([ \t]*)(is|as)", vec![Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?ms)\b[A-Z]\w*\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r#"(?ms)".*""#, STRING),
        Rule::token(r"(?ms)\'.\'", STRING_CHAR),
        Rule::token(r"(?ms)(><|->|-m->|/\\|<=|<<=|<\.|\|\||\|\^\||-~->|-~m->|\\/|>=|>>|\.>|\+\+|-\\|<->|=>|:-|~=|\*\*|<<|>>=|\+>|!!|\|=\||#)", OPERATOR),
        Rule::token(r"(?ms)[0-9]+\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms).", TEXT),
    ]);
    Table(m)
}

impl Lexer for RslLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
