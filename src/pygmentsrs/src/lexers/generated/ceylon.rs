#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:CeylonLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:CeylonLexer:ceylon

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ceylon
pub struct CeylonLexer;

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
        Rule::bygroups_g(r"(?ms)^(\s*(?:[a-zA-Z_][\w.\[\]]*\s+)+?)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::bygroups(r"(?ms)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token_to(r"(?ms)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)(shared|abstract|formal|default|actual|variable|deprecated|small|late|literal|doc|by|see|throws|optional|license|tagged|final|native|annotation|sealed)\b", NAME_DECORATOR),
        Rule::token(r"(?ms)(break|case|catch|continue|else|finally|for|in|if|return|switch|this|throw|try|while|is|exists|dynamic|nonempty|then|outer|assert|let)\b", KEYWORD),
        Rule::token(r"(?ms)(abstracts|extends|satisfies|super|given|of|out|assign)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(function|value|void|new)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(assembly|module|package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?ms)(class|interface|object|alias)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups_to(r"(?ms)(import)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?ms)'\\.'|'[^\\]'|'\\\{#[0-9a-fA-F]{4}\}'", STRING_CHAR),
        Rule::bygroups(r"(?ms)(\.)([a-z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_]\w*:", NAME_LABEL),
        Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?ms)[~^*!%&\[\](){}<>|+=:;,./?-]", OPERATOR),
        Rule::token(r"(?ms)\d{1,3}(_\d{3})+\.\d{1,3}(_\d{3})+[kMGTPmunpf]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)\d{1,3}(_\d{3})+\.[0-9]+([eE][+-]?[0-9]+)?[kMGTPmunpf]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9][0-9]*\.\d{1,3}(_\d{3})+[kMGTPmunpf]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][+-]?[0-9]+)?[kMGTPmunpf]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)#([0-9a-fA-F]{4})(_[0-9a-fA-F]{4})+", NUMBER_HEX),
        Rule::token(r"(?ms)#[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)\$([01]{4})(_[01]{4})+", NUMBER_BIN),
        Rule::token(r"(?ms)\$[01]+", NUMBER_BIN),
        Rule::token(r"(?ms)\d{1,3}(_\d{3})+[kMGTP]?", NUMBER_INTEGER),
        Rule::token(r"(?ms)[0-9]+[kMGTP]?", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(
        r"class",
        vec![Rule::token_to(
            r"(?ms)[A-Za-z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"import",
        vec![Rule::token_to(
            r"(?ms)[a-z][\w.]*",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?ms)[^*/]", COMMENT_MULTILINE),
            Rule::token_to(r"(?ms)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?ms)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?ms)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for CeylonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
