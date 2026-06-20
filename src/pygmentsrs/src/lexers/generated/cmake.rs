//! AUTO-GENERATED from `pygments.pygments.lexers.make:CMakeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.make:CMakeLexer:cmake

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cmake
pub struct CmakeLexer;

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
        Rule::bygroups_to(r"(?m)\b(\w+)([ \t]*)(\()", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"args"])),
        Rule::token(r"(?m)\b(WIN32|UNIX|APPLE|CYGWIN|BORLAND|MINGW|MSVC|MSVC_IDE|MSVC60|MSVC70|MSVC71|MSVC80|MSVC90)\b", KEYWORD),
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)#\[(?P<level>=*)\[[\w\W]*?\](?P=level)\]", COMMENT),
        Rule::token(r"(?m)#.*\n", COMMENT),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)\b(WIN32|UNIX|APPLE|CYGWIN|BORLAND|MINGW|MSVC|MSVC_IDE|MSVC60|MSVC70|MSVC71|MSVC80|MSVC90)\b", KEYWORD),
    ]);
    m.insert(
        r"ws",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)#\[(?P<level>=*)\[[\w\W]*?\](?P=level)\]", COMMENT),
            Rule::token(r"(?m)#.*\n", COMMENT),
        ],
    );
    m.insert(r"args", vec![
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups(r"(?m)(\$\{)(.+?)(\})", vec![Some(OPERATOR), Some(NAME_VARIABLE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(\$ENV\{)(.+?)(\})", vec![Some(OPERATOR), Some(NAME_VARIABLE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(\$<)(.+?)(>)", vec![Some(OPERATOR), Some(NAME_VARIABLE), Some(OPERATOR)]),
        Rule::token(r#"(?m)(?s)".*?""#, STRING_DOUBLE),
        Rule::token(r"(?m)\\\S+", STRING),
        Rule::token(r"(?m)\[(?P<level>=*)\[[\w\W]*?\](?P=level)\]", TokenType::new(&["Literal", "String", "Multiline"])),
        Rule::token(r##"(?m)[^)$"# \t\n]+"##, STRING),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\b(WIN32|UNIX|APPLE|CYGWIN|BORLAND|MINGW|MSVC|MSVC_IDE|MSVC60|MSVC70|MSVC71|MSVC80|MSVC90)\b", KEYWORD),
        Rule::token(r"(?m)[ \t]+", WHITESPACE),
        Rule::token(r"(?m)#\[(?P<level>=*)\[[\w\W]*?\](?P=level)\]", COMMENT),
        Rule::token(r"(?m)#.*\n", COMMENT),
    ]);
    m.insert(r"string", vec![]);
    Table(m)
}

impl Lexer for CmakeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
