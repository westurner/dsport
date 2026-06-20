#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:GosuLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:GosuLexer:gosu

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gosu
pub struct GosuLexer;

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
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(in|as|typeof|statictypeof|typeis|typeas|if|else|foreach|for|index|while|do|continue|break|return|try|catch|finally|this|throw|new|switch|case|default|eval|super|outer|classpath|using)\b", KEYWORD),
        Rule::token(r"(?ms)(var|delegate|construct|function|private|internal|protected|public|abstract|override|final|static|extends|transient|implements|represents|readonly)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?ms)(property)(\s+)(get|set)?", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(KEYWORD_DECLARATION)]),
        Rule::token(r"(?ms)(boolean|byte|char|double|float|int|long|short|void|block)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?ms)(class|interface|enhancement|enum)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?ms)(uses)(\s+)([\w.]+\*?)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?ms)(\??[.#])([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?ms)(:)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)and|or|not|[\\~^*!%&\[\](){}<>|+=:;,./?-]", OPERATOR),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(
        r"templateText",
        vec![
            Rule::token(r"(?ms)(\\<)|(\\\$)", STRING),
            Rule::bygroups_to(
                r"(?ms)(<%@\s+)(extends|params)",
                vec![Some(OPERATOR), Some(NAME_DECORATOR)],
                NewState::Push(vec![r"stringTemplate"]),
            ),
            Rule::token(r"(?ms)<%!--.*?--%>", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?ms)(<%)|(<%=)",
                OPERATOR,
                NewState::Push(vec![r"stringTemplate"]),
            ),
            Rule::token_to(
                r"(?ms)\$\{",
                OPERATOR,
                NewState::Push(vec![r"stringTemplateShorthand"]),
            ),
            Rule::token(r"(?ms).", STRING),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?ms)(\\<)|(\\\$)", STRING),
            Rule::bygroups_to(
                r"(?ms)(<%@\s+)(extends|params)",
                vec![Some(OPERATOR), Some(NAME_DECORATOR)],
                NewState::Push(vec![r"stringTemplate"]),
            ),
            Rule::token(r"(?ms)<%!--.*?--%>", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?ms)(<%)|(<%=)",
                OPERATOR,
                NewState::Push(vec![r"stringTemplate"]),
            ),
            Rule::token_to(
                r"(?ms)\$\{",
                OPERATOR,
                NewState::Push(vec![r"stringTemplateShorthand"]),
            ),
            Rule::token(r"(?ms).", STRING),
        ],
    );
    m.insert(r"stringTemplate", vec![
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?ms)%>", OPERATOR, NewState::Pop(1)),
        Rule::bygroups_g(r"(?ms)^(\s*(?:[a-zA-Z_][\w.\[\]]*\s+)+?)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(in|as|typeof|statictypeof|typeis|typeas|if|else|foreach|for|index|while|do|continue|break|return|try|catch|finally|this|throw|new|switch|case|default|eval|super|outer|classpath|using)\b", KEYWORD),
        Rule::token(r"(?ms)(var|delegate|construct|function|private|internal|protected|public|abstract|override|final|static|extends|transient|implements|represents|readonly)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?ms)(property)(\s+)(get|set)?", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(KEYWORD_DECLARATION)]),
        Rule::token(r"(?ms)(boolean|byte|char|double|float|int|long|short|void|block)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?ms)(class|interface|enhancement|enum)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?ms)(uses)(\s+)([\w.]+\*?)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?ms)(\??[.#])([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?ms)(:)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)and|or|not|[\\~^*!%&\[\](){}<>|+=:;,./?-]", OPERATOR),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(r"stringTemplateShorthand", vec![
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?ms)\{", OPERATOR, NewState::Push(vec![r"stringTemplateShorthand"])),
        Rule::token_to(r"(?ms)\}", OPERATOR, NewState::Pop(1)),
        Rule::bygroups_g(r"(?ms)^(\s*(?:[a-zA-Z_][\w.\[\]]*\s+)+?)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(in|as|typeof|statictypeof|typeis|typeas|if|else|foreach|for|index|while|do|continue|break|return|try|catch|finally|this|throw|new|switch|case|default|eval|super|outer|classpath|using)\b", KEYWORD),
        Rule::token(r"(?ms)(var|delegate|construct|function|private|internal|protected|public|abstract|override|final|static|extends|transient|implements|represents|readonly)\b", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?ms)(property)(\s+)(get|set)?", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(KEYWORD_DECLARATION)]),
        Rule::token(r"(?ms)(boolean|byte|char|double|float|int|long|short|void|block)\b", KEYWORD_TYPE),
        Rule::bygroups(r"(?ms)(package)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?ms)(class|interface|enhancement|enum)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?ms)(uses)(\s+)([\w.]+\*?)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?ms)(\??[.#])([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::bygroups(r"(?ms)(:)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)and|or|not|[\\~^*!%&\[\](){}<>|+=:;,./?-]", OPERATOR),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for GosuLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
