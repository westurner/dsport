//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:JavaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:JavaLexer:java

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: java
pub struct JavaLexer;

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
        Rule::bygroups_g_to(r"(?ms)(^\s*)((?:(?:public|private|protected|static|strictfp)(?:\s+))*)(record)\b", vec![Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD_DECLARATION))], NewState::Push(vec![r"class"])),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::bygroups(r"(?ms)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)(assert|break|case|catch|continue|default|do|else|finally|for|if|goto|instanceof|new|return|switch|this|throw|try|while)\b", KEYWORD),
        Rule::bygroups_g(r"(?ms)((?:(?:[^\W\d]|\$)[\w.\[\]$<>?]*\s+)+?)((?:[^\W\d]|\$)[\w$]*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)@[^\W\d][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(abstract|const|enum|exports|extends|final|implements|native|non-sealed|open|opens|permits|private|protected|provides|public|requires|sealed|static|strictfp|super|synchronized|throws|to|transient|transitive|uses|volatile|with|yield)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(boolean|byte|char|double|float|int|long|short|void)\b", KEYWORD_TYPE),
        Rule::bygroups_to(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?ms)(class|interface)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"class"])),
        Rule::token_to(r"(?ms)(module)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"module"])),
        Rule::bygroups_to(r"(?ms)(var)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"var"])),
        Rule::bygroups_to(r"(?ms)(import(?:\s+(?:static|module))?)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token_to(r#"(?ms)"""\n"#, STRING, NewState::Push(vec![r"multiline_string"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?ms)'\\.'|'[^\\]'|'\\u[0-9a-fA-F]{4}'", STRING_CHAR),
        Rule::bygroups(r"(?ms)(\.)((?:[^\W\d]|\$)[\w$]*)", vec![Some(PUNCTUATION), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?ms)^(\s*)(default)(:)", vec![Some(WHITESPACE), Some(KEYWORD), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?ms)^(\s*)((?:[^\W\d]|\$)[\w$]*)(:)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?ms)([^\W\d]|\$)[\w$]*", NAME),
        Rule::token(r"(?ms)([0-9][0-9_]*\.([0-9][0-9_]*)?|\.[0-9][0-9_]*)([eE][+\-]?[0-9][0-9_]*)?[fFdD]?|[0-9][eE][+\-]?[0-9][0-9_]*[fFdD]?|[0-9]([eE][+\-]?[0-9][0-9_]*)?[fFdD]|0[xX]([0-9a-fA-F][0-9a-fA-F_]*\.?|([0-9a-fA-F][0-9a-fA-F_]*)?\.[0-9a-fA-F][0-9a-fA-F_]*)[pP][+\-]?[0-9][0-9_]*[fFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0[xX][0-9a-fA-F][0-9a-fA-F_]*[lL]?", NUMBER_HEX),
        Rule::token(r"(?ms)0[bB][01][01_]*[lL]?", NUMBER_BIN),
        Rule::token(r"(?ms)0[0-7_]+[lL]?", NUMBER_OCT),
        Rule::token(r"(?ms)0|[1-9][0-9_]*[lL]?", NUMBER_INTEGER),
        Rule::token(r"(?ms)[~^*!%&\[\]<>|+=/?-]", OPERATOR),
        Rule::token(r"(?ms)[{}();:.,]", PUNCTUATION),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(
        r"class",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(r"(?ms)([^\W\d]|\$)[\w$]*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"module",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(r"(?ms)([^\W\d]|\$)[\w$]*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"var",
        vec![Rule::token_to(
            r"(?ms)([^\W\d]|\$)[\w$]*",
            NAME,
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
    m.insert(
        r"multiline_string",
        vec![
            Rule::token_to(r#"(?ms)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)""#, STRING),
            Rule::token(r#"(?ms)[^\\"]+"#, STRING),
            Rule::token(r"(?ms)\\\\", STRING),
            Rule::token(r#"(?ms)\\""#, STRING),
            Rule::token(r"(?ms)\\", STRING),
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?ms)[^\\"]+"#, STRING),
            Rule::token(r"(?ms)\\\\", STRING),
            Rule::token(r#"(?ms)\\""#, STRING),
            Rule::token(r"(?ms)\\", STRING),
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for JavaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
