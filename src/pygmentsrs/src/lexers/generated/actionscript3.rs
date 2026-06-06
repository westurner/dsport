//! AUTO-GENERATED from `pygments.pygments.lexers.actionscript:ActionScript3Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.actionscript:ActionScript3Lexer:actionscript3

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: actionscript3, as3
pub struct Actionscript3Lexer;

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
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups_to(r"(?ms)(function\s+)([$a-zA-Z_]\w*)(\s*)(\()", vec![Some(KEYWORD_DECLARATION), Some(NAME_FUNCTION), Some(TEXT), Some(OPERATOR)], NewState::Push(vec![r"funcparams"])),
        Rule::bygroups(r"(?ms)(var|const)(\s+)([$a-zA-Z_]\w*)(\s*)(:)(\s*)([$a-zA-Z_]\w*(?:\.<\w+>)?)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?ms)(import|package)(\s+)((?:[$a-zA-Z_]\w*|\.)+)(\s*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE)]),
        Rule::bygroups(r"(?ms)(new)(\s+)([$a-zA-Z_]\w*(?:\.<\w+>)?)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)/(\\\\|\\[^\\]|[^\\\n])*/[gisx]*", STRING_REGEX),
        Rule::bygroups(r"(?ms)(\.)([$a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)(case|default|for|each|in|while|do|break|return|continue|if|else|throw|try|catch|with|new|typeof|arguments|instanceof|this|switch|import|include|as|is)\b", KEYWORD),
        Rule::token(r"(?ms)(class|public|final|internal|native|override|private|protected|static|import|extends|implements|interface|intrinsic|return|super|dynamic|function|const|get|namespace|package|set)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity|-Infinity|undefined|void)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(decodeURI|decodeURIComponent|encodeURI|escape|eval|isFinite|isNaN|isXMLName|clearInterval|fscommand|getTimer|getURL|getVersion|isFinite|parseFloat|parseInt|setInterval|trace|updateAfterEvent|unescape)\b", NAME_FUNCTION),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)[~^*!%&<>|+=:;,/?\\{}\[\]().-]+", OPERATOR),
    ]);
    m.insert(r"funcparams", vec![
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups_to(r"(?ms)(\s*)(\.\.\.)?([$a-zA-Z_]\w*)(\s*)(:)(\s*)([$a-zA-Z_]\w*(?:\.<\w+>)?|\*)(\s*)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(NAME), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE)], NewState::Push(vec![r"defval"])),
        Rule::token_to(r"(?ms)\)", OPERATOR, NewState::Push(vec![r"type"])),
    ]);
    m.insert(r"type", vec![
        Rule::bygroups_to(r"(?ms)(\s*)(:)(\s*)([$a-zA-Z_]\w*(?:\.<\w+>)?|\*)", vec![Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_TYPE)], NewState::Pop(2)),
        Rule::token_to(r"(?ms)\s+", TEXT, NewState::Pop(2)),
        Rule::default(NewState::Pop(2)),
    ]);
    m.insert(r"defval", vec![
        Rule::bygroups_g_to(r"(?ms)(=)(\s*)([^(),]+)(\s*)(,?)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))], NewState::Pop(1)),
        Rule::token_to(r"(?ms),", OPERATOR, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for Actionscript3Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
