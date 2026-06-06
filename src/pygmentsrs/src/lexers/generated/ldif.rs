//! AUTO-GENERATED from `pygments.pygments.lexers.ldap:LdifLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ldap:LdifLexer:ldif

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ldif
pub struct LdifLexer;

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
        Rule::token(r"(?m)\s*\n", WHITESPACE),
        Rule::bygroups(r"(?m)(-)(\n)", vec![Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(#.*)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(version)(:)([ \t]*)(.*)([ \t]*\n)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)(control)(:)([ \t]*)([\.0-9]+)([ \t]+)((?:true|false)?)([ \t]*)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_OTHER), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"after-control"])),
        Rule::bygroups(r"(?m)(deleteoldrdn)(:)([ \n]*)([0-1]+)([ \t]*\n)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NUMBER), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(add|delete|replace)(::?)(\s*)(.*)([ \t]*\n)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(changetype)(:)([ \t]*)([a-z]*)([ \t]*\n)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)(dn|newrdn)(::)", vec![Some(KEYWORD), Some(PUNCTUATION)], NewState::Push(vec![r"base64-dn"])),
        Rule::bygroups_to(r"(?m)(dn|newrdn)(:)", vec![Some(KEYWORD), Some(PUNCTUATION)], NewState::Push(vec![r"dn"])),
        Rule::bygroups(r"(?m)(objectclass)(:)([ \t]*)([^ \t\n]*)([ \t]*\n)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)([a-zA-Z]*|[0-9][0-9\.]*[0-9])(;)", vec![Some(NAME_ATTRIBUTE), Some(PUNCTUATION)], NewState::Push(vec![r"property"])),
        Rule::bygroups_to(r"(?m)([a-zA-Z]*|[0-9][0-9\.]*[0-9])(:<)", vec![Some(NAME_ATTRIBUTE), Some(PUNCTUATION)], NewState::Push(vec![r"url"])),
        Rule::bygroups_to(r"(?m)([a-zA-Z]*|[0-9][0-9\.]*[0-9])(::?)", vec![Some(NAME_ATTRIBUTE), Some(PUNCTUATION)], NewState::Push(vec![r"value"])),
    ]);
    m.insert(r"after-control", vec![
        Rule::token_to(r"(?m):<", PUNCTUATION, NewState::Push(vec![r"#pop", r"url"])),
        Rule::token_to(r"(?m)::?", PUNCTUATION, NewState::Push(vec![r"#pop", r"value"])),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"property", vec![
        Rule::bygroups(r"(?m)([-a-zA-Z0-9]*)(;)", vec![Some(NAME_PROPERTY), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)([-a-zA-Z0-9]*)(:<)", vec![Some(NAME_PROPERTY), Some(PUNCTUATION)], NewState::Push(vec![r"#pop", r"url"])),
        Rule::bygroups_to(r"(?m)([-a-zA-Z0-9]*)(::?)", vec![Some(NAME_PROPERTY), Some(PUNCTUATION)], NewState::Push(vec![r"#pop", r"value"])),
    ]);
    m.insert(r"value", vec![
        Rule::bygroups(r"(?m)(\s*)([^\n]+\S)(\n )", vec![Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)(\s*)([^\n]+\S)(\n)", vec![Some(WHITESPACE), Some(STRING), Some(WHITESPACE)], NewState::Pop(1)),
    ]);
    m.insert(r"url", vec![
        Rule::bygroups(r"(?m)([ \t]*)(\S*)([ \t]*\n )", vec![Some(WHITESPACE), Some(COMMENT_PREPROCFILE), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)([ \t]*)(\S*)([ \t]*\n)", vec![Some(WHITESPACE), Some(COMMENT_PREPROCFILE), Some(WHITESPACE)], NewState::Pop(1)),
    ]);
    m.insert(r"dn", vec![
        Rule::bygroups_to(r"(?m)([ \t]*)([-a-zA-Z0-9\.]+)(=)", vec![Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(OPERATOR)], NewState::Push(vec![r"#pop", r"dn-value"])),
    ]);
    m.insert(r"dn-value", vec![
        Rule::token(r"(?m)\\[^\n]", ESCAPE),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Push(vec![r"#pop", r"dn"])),
        Rule::token_to(r"(?m)\+", OPERATOR, NewState::Push(vec![r"#pop", r"dn"])),
        Rule::token(r"(?m)[^,\+\n]+", STRING),
        Rule::token(r"(?m)\n ", WHITESPACE),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"base64-dn", vec![
        Rule::bygroups(r"(?m)([ \t]*)([^ \t\n][^ \t\n]*[^\n])([ \t]*\n )", vec![Some(WHITESPACE), Some(NAME), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)([ \t]*)([^ \t\n][^ \t\n]*[^\n])([ \t]*\n)", vec![Some(WHITESPACE), Some(NAME), Some(WHITESPACE)], NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for LdifLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
