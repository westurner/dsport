//! AUTO-GENERATED from `pygments.pygments.lexers.wowtoc:WoWTocLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.wowtoc:WoWTocLexer:wowtoc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: wowtoc
pub struct WowtocLexer;

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
        Rule::bygroups(r"(?m)^(##)( *)((?:[nN][oO][tT][eE][sS]|[tT][iI][tT][lL][eE])-(?:ptBR|zhCN|enCN|frFR|deDE|itIT|esMX|ptPT|koKR|ruRU|esES|zhTW|enTW|enGB|enUS))( *)(:)( *)(.*?)( *)$", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_BUILTIN), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(?i)^(##)( *)(Interface|Title|Notes|RequiredDeps|Dep[^: ]*|OptionalDeps|LoadOnDemand|LoadWith|LoadManagers|SavedVariablesPerCharacter|SavedVariables|DefaultState|Secure|Author|Version)( *)(:)( *)(.*?)( *)$", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_BUILTIN), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(?i)^(##)( *)(X-[^: ]*)( *)(:)( *)(.*?)( *)$", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)^(##)( *)([^: ]*)( *)(:)( *)(.*?)( *)$", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_OTHER), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE)]),
        Rule::token(r"(?m)^#.*$", COMMENT),
        Rule::token(r"(?m)^.+$", NAME),
    ]);
    Table(m)
}

impl Lexer for WowtocLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
