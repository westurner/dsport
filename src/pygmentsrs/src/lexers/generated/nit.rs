//! AUTO-GENERATED from `pygments.pygments.lexers.nit:NitLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.nit:NitLexer:nit

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nit
pub struct NitLexer;

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
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)(__debug__|a(?:b(?:(?:or|strac)t)|nd|s(?:(?:sert)?))|break|c(?:lass|ontinue)|do|e(?:lse|n(?:d|um)|xtern)|f(?:alse|or|un)|i(?:mp(?:lies|ort)|n(?:it|t(?:er(?:face|n)|rude))|s(?:a|set)|[fns])|l(?:abel|oop)|module|n(?:ew|ot|ull(?:(?:able)?))|o(?:nce|r)|p(?:ackage|r(?:ivate|otected)|ublic)|re(?:adable|def|turn)|s(?:elf|uper)|t(?:hen|(?:ru|yp)e)|universal|var|w(?:(?:hi|ritab)le))(?=[\r\n\t( ])", KEYWORD),
        Rule::token(r"(?m)[A-Z]\w*", NAME_CLASS),
        Rule::token(r#"(?m)"""(([^\'\\]|\\.)|\\r|\\n)*((\{\{?)?(""?\{\{?)*""""*)"#, STRING),
        Rule::token(r"(?m)\'\'\'(((\\.|[^\'\\])|\\r|\\n)|\'((\\.|[^\'\\])|\\r|\\n)|\'\'((\\.|[^\'\\])|\\r|\\n))*\'\'\'", STRING),
        Rule::token(r#"(?m)"""(([^\'\\]|\\.)|\\r|\\n)*((""?)?(\{\{?""?)*\{\{\{\{*)"#, STRING),
        Rule::token(r#"(?m)\}\}\}(((\\.|[^\'\\])|\\r|\\n))*(""?)?(\{\{?""?)*\{\{\{\{*"#, STRING),
        Rule::token(r#"(?m)\}\}\}(((\\.|[^\'\\])|\\r|\\n))*(\{\{?)?(""?\{\{?)*""""*"#, STRING),
        Rule::token(r#"(?m)"(\\.|([^"}{\\]))*""#, STRING),
        Rule::token(r#"(?m)"(\\.|([^"}{\\]))*\{"#, STRING),
        Rule::token(r#"(?m)\}(\\.|([^"}{\\]))*\{"#, STRING),
        Rule::token(r#"(?m)\}(\\.|([^"}{\\]))*""#, STRING),
        Rule::token(r"(?m)(\'[^\'\\]\')|(\'\\.\')", STRING_CHAR),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]*.[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0(x|X)[0-9A-Fa-f]+", NUMBER_HEX),
        Rule::token(r"(?m)[a-z]\w*", NAME),
        Rule::token(r"(?m)_\w+", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?m)==|!=|<==>|>=|>>|>|<=|<<|<|\+|-|=|/|\*|%|\+=|-=|!|@", OPERATOR),
        Rule::token(r"(?m)\(|\)|\[|\]|,|\.\.\.|\.\.|\.|::|:", PUNCTUATION),
        Rule::token(r"(?m)`\{[^`]*`\}", TEXT),
        Rule::token(r"(?m)[\r\n\t ]+", TEXT),
    ]);
    Table(m)
}

impl Lexer for NitLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
