//! AUTO-GENERATED from `pygments.pygments.lexers.installers:DebianControlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.installers:DebianControlLexer:debcontrol

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: debcontrol, control
pub struct DebcontrolLexer;

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
        Rule::token_to(r"(?m)^(Description)", KEYWORD, NewState::Push(vec![r"description"])),
        Rule::bygroups_to(r"(?m)^(Maintainer|Uploaders|Changed-By)(:)(\s*)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"maintainer"])),
        Rule::bygroups_to(r"(?m)^((?:Build-|Pre-)?Depends(?:-Indep|-Arch)?)(:)(\s*)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"package_list"])),
        Rule::bygroups_to(r"(?m)^(Recommends|Suggests|Enhances|Breaks|Replaces|Provides|Conflicts)(:)(\s*)", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"package_list"])),
        Rule::bygroups(r"(?m)^((?:Python-)?Version)(:)(\s*)(\S+)$", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NUMBER)]),
        Rule::bygroups(r"(?m)^((?:Installed-)?Size)(:)(\s*)(\S+)$", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NUMBER)]),
        Rule::bygroups(r"(?m)^(MD5Sum|SHA1|SHA256)(:)(\s*)(\S+)$", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(NUMBER)]),
        Rule::bygroups(r"(?m)^([a-zA-Z\-0-9\.]*?)(:)(\s*)(.*?)$", vec![Some(KEYWORD), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING)]),
    ]);
    m.insert(r"maintainer", vec![
        Rule::token_to(r"(?m)<[^>]+>$", GENERIC_STRONG, NewState::Pop(1)),
        Rule::token(r"(?m)<[^>]+>", GENERIC_STRONG),
        Rule::token(r"(?m),\n?", WHITESPACE),
        Rule::token_to(r"(?m)[^,<]+$", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)[^,<]+", TEXT),
    ]);
    m.insert(r"description", vec![
        Rule::bygroups(r"(?m)(.*)(Homepage)(: )(\S+)", vec![Some(TEXT), Some(STRING), Some(NAME), Some(NAME_CLASS)]),
        Rule::token(r"(?m):.*\n", GENERIC_STRONG),
        Rule::token(r"(?m) .*\n", TEXT),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"package_list", vec![
        Rule::bygroups(r"(?m)(\$)(\{)(\w+)(\s*)(:)(\s*)(\w+)(\})", vec![Some(OPERATOR), Some(PUNCTUATION), Some(NAME_ENTITY), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"package_list_vers"])),
        Rule::token(r"(?m)\|", OPERATOR),
        Rule::token(r"(?m)\n\s", WHITESPACE),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)[,\s]", TEXT),
        Rule::token(r"(?m)[+.a-zA-Z0-9-]+", NAME_FUNCTION),
        Rule::bygroups(r"(?m)(\[)(!?)(.*?)(\])", vec![Some(PUNCTUATION), Some(OPERATOR), Some(NAME_ENTITY), Some(PUNCTUATION)]),
    ]);
    m.insert(r"package_list_vers", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r"(?m)([><=]+)(\s*)([^)]+)", vec![Some(OPERATOR), Some(WHITESPACE), Some(NUMBER)]),
    ]);
    Table(m)
}

impl Lexer for DebcontrolLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
