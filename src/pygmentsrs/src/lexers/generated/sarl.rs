#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:SarlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:SarlLexer:sarl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sarl
pub struct SarlLexer;

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
        Rule::bygroups_g(r"(?ms)^(\s*(?:[a-zA-Z_][\w.\[\]]*\s+)+?)([a-zA-Z_$][\w$]*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::bygroups(r"(?ms)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(as|break|case|catch|default|do|else|extends|extension|finally|fires|for|if|implements|instanceof|new|on|requires|return|super|switch|throw|throws|try|typeof|uses|while|with)\b", KEYWORD),
        Rule::token(r"(?ms)(abstract|def|dispatch|final|native|override|private|protected|public|static|strictfp|synchronized|transient|val|var|volatile)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(boolean|byte|char|double|float|int|long|short|void)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(false|it|null|occurrence|this|true|void)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?ms)(agent|annotation|artifact|behavior|capacity|class|enum|event|interface|skill|space)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups_to(r"(?ms)(import)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)[a-zA-Z_]\w*:", NAME_LABEL),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)[~^*!%&\[\](){}<>\|+=:;,./?-]", OPERATOR),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+L?", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(
        r"class",
        vec![Rule::token_to(
            r"(?ms)[a-zA-Z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"import",
        vec![Rule::token_to(
            r"(?ms)[\w.]+\*?",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for SarlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
