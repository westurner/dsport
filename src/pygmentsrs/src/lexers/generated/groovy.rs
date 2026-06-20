#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:GroovyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:GroovyLexer:groovy

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: groovy
pub struct GroovyLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token_to(
                r"(?ms)#!(.*?)$",
                COMMENT_PREPROC,
                NewState::Push(vec![r"base"]),
            ),
            Rule::default(NewState::Push(vec![r"base"])),
        ],
    );
    m.insert(r"base", vec![
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::bygroups(r"(?ms)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)(assert|break|case|catch|continue|default|do|else|finally|for|if|goto|instanceof|new|return|switch|this|throw|try|while|in|as)\b", KEYWORD),
        Rule::bygroups_g(r#"(?ms)^(\s*(?:[a-zA-Z_][\w.\[\]]*\s+)+?)([a-zA-Z_]\w*|"(?:\\\\|\\[^\\]|[^"\\])*"|'(?:\\\\|\\[^\\]|[^'\\])*')(\s*)(\()"#, vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(abstract|const|enum|extends|final|implements|native|private|protected|public|static|strictfp|super|synchronized|throws|transient|volatile)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(def|boolean|byte|char|double|float|int|long|short|void)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?ms)(class|interface)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::bygroups_to(r"(?ms)(import)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r#"(?ms)""".*?""""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'''.*?'''", STRING_SINGLE),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?ms)\$/((?!/\$).)*/\$", STRING),
        Rule::token(r"(?ms)/(\\\\|\\[^\\]|[^/\\])*/", STRING),
        Rule::token(r"(?ms)'\\.'|'[^\\]'|'\\u[0-9a-fA-F]{4}'", STRING_CHAR),
        Rule::bygroups(r"(?ms)(\.)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_]\w*:", NAME_LABEL),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)[~^*!%&\[\](){}<>|+=:;,./?-]", OPERATOR),
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

impl Lexer for GroovyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
