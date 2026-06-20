#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.elm:ElmLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.elm:ElmLexer:elm

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: elm
pub struct ElmLexer;

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
        Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)--.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublequote"])),
        Rule::bygroups_to(r"(?m)^(\s*)(module)(\s*)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"imports"])),
        Rule::bygroups_to(r"(?m)^(\s*)(import)(\s*)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"imports"])),
        Rule::token_to(r"(?m)\[glsl\|.*", NAME_ENTITY, NewState::Push(vec![r"shader"])),
        Rule::token(r"(?m)(a(?:(?:(?:lia)?)s)|case|else|i(?:mport|[fn])|let|module|of|port|t(?:hen|ype)|where)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)[A-Z][a-zA-Z0-9_]*", KEYWORD_TYPE),
        Rule::token(r"(?m)^main ", KEYWORD_RESERVED),
        Rule::token(r"(?m)\((\&\&|\+\+|\->|\.\.|/(?:[/=])|::|<(?:[\-<=|~])|==|>(?:[=>])|\|(?:[>|])|[%'*+\-./:<=>\\\^`|~])\)", NAME_FUNCTION),
        Rule::token(r"(?m)(\&\&|\+\+|\->|\.\.|/(?:[/=])|::|<(?:[\-<=|~])|==|>(?:[=>])|\|(?:[>|])|[%'*+\-./:<=>\\\^`|~])", NAME_FUNCTION),
        Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[a-z_][a-zA-Z0-9_\']*", NAME_VARIABLE),
        Rule::token(r"(?m)[,()\[\]{}]", PUNCTUATION),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)_?\d+\.(?=\d+)", NUMBER_FLOAT),
            Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)-(?!\})", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?m)\{-",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r"(?m)[^-}]", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)-\}", COMMENT_MULTILINE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"doublequote",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r#"(?m)\\[nrfvb\\"]"#, STRING_ESCAPE),
            Rule::token(r#"(?m)[^"]"#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"imports",
        vec![Rule::token_to(
            r"(?m)\w+(\.\w+)*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"shader",
        vec![
            Rule::token(r"(?m)\|(?!\])", NAME_ENTITY),
            Rule::token_to(r"(?m)\|\]", NAME_ENTITY, NewState::Pop(1)),
            Rule::bygroups(r"(?m)(.*)(\n)", vec![Some(NAME_ENTITY), Some(WHITESPACE)]),
        ],
    );
    Table(m)
}

impl Lexer for ElmLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
