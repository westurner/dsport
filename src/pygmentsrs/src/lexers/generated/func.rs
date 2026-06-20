#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.func:FuncLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.func:FuncLexer:func

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: func, fc
pub struct FuncLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(?<=\s)(!=|%=|\&=|\*=|\+=|\-=|/(?:[%=])|<(?:<=|=>|[<=])|==|>(?:>=|[=>])|\^(?:%=|/=|>>(?:(?:=)?)|[%/=])|\|=|\~(?:/=|>>(?:(?:=)?)|[%/])|[%&*+\-/:<=>?\^|~])(?=\s)", OPERATOR),
        Rule::token(r"(?m)\b(asm|do|else(?:(?:if(?:(?:not)?))?)|forall|i(?:f(?:(?:not)?)|mpure|nline(?:(?:_ref)?))|method_id|re(?:peat|turn)|until|while)\b", KEYWORD),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r#"(?m)\"([^\n\"]+)\"[Hhcusa]?"#, STRING),
        Rule::token_to(r"(?m)#include|#pragma", KEYWORD, NewState::Push(vec![r"directive"])),
        Rule::token(r"(?m)\b(-?(?!_)([\d_]+|0x[\d_a-fA-F]+)|0b[1_0]+)(?<!_)(?=[\s\)\],;])", NUMBER),
        Rule::token(r"(?m);;([^\n]*)", TokenType::new(&["Comment", "Singleline"])),
        Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\b(_|builder|c(?:ell|ont)|int|slice|tuple|var)(?=[\s\(\),\[\]])", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(const|global)\b", KEYWORD_CONSTANT),
        Rule::token(r#"(?m)(?!")(`([^`]+)`|((?=_)_|(?=\{)\{|(?=\})\}|(?![_`{}]))([^;,\[\]\(\)\s~.]+))(?=[\(])"#, NAME_FUNCTION),
        Rule::token(r#"(?m)(?!")(`([^`]+)`|((?=_)_|(?=\{)\{|(?=\})\}|(?![_`{}]))([^;,\[\]\(\)\s~.]+))"#, NAME_VARIABLE),
        Rule::token(r"(?m)[.;(),\[\]~{}]", PUNCTUATION),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(?<=\s)(!=|%=|\&=|\*=|\+=|\-=|/(?:[%=])|<(?:<=|=>|[<=])|==|>(?:>=|[=>])|\^(?:%=|/=|>>(?:(?:=)?)|[%/=])|\|=|\~(?:/=|>>(?:(?:=)?)|[%/])|[%&*+\-/:<=>?\^|~])(?=\s)", OPERATOR),
        Rule::token(r"(?m)\b(asm|do|else(?:(?:if(?:(?:not)?))?)|forall|i(?:f(?:(?:not)?)|mpure|nline(?:(?:_ref)?))|method_id|re(?:peat|turn)|until|while)\b", KEYWORD),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
    ]);
    m.insert(
        r"strings",
        vec![Rule::token(r#"(?m)\"([^\n\"]+)\"[Hhcusa]?"#, STRING)],
    );
    m.insert(
        r"directives",
        vec![Rule::token_to(
            r"(?m)#include|#pragma",
            KEYWORD,
            NewState::Push(vec![r"directive"]),
        )],
    );
    m.insert(
        r"numeric",
        vec![Rule::token(
            r"(?m)\b(-?(?!_)([\d_]+|0x[\d_a-fA-F]+)|0b[1_0]+)(?<!_)(?=[\s\)\],;])",
            NUMBER,
        )],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(
                r"(?m);;([^\n]*)",
                TokenType::new(&["Comment", "Singleline"]),
            ),
            Rule::token_to(
                r"(?m)\{-",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
        ],
    );
    m.insert(
        r"storage",
        vec![
            Rule::token(
                r"(?m)\b(_|builder|c(?:ell|ont)|int|slice|tuple|var)(?=[\s\(\),\[\]])",
                KEYWORD_TYPE,
            ),
            Rule::token(r"(?m)\b(const|global)\b", KEYWORD_CONSTANT),
        ],
    );
    m.insert(r"functions", vec![
        Rule::token(r#"(?m)(?!")(`([^`]+)`|((?=_)_|(?=\{)\{|(?=\})\}|(?![_`{}]))([^;,\[\]\(\)\s~.]+))(?=[\(])"#, NAME_FUNCTION),
    ]);
    m.insert(
        r"variables",
        vec![Rule::token(
            r#"(?m)(?!")(`([^`]+)`|((?=_)_|(?=\{)\{|(?=\})\}|(?![_`{}]))([^;,\[\]\(\)\s~.]+))"#,
            NAME_VARIABLE,
        )],
    );
    m.insert(
        r"directive",
        vec![
            Rule::token(r#"(?m)\"([^\n\"]+)\"[Hhcusa]?"#, STRING),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)version|not-version", KEYWORD),
            Rule::token(r"(?m)(>=|<=|=|>|<|\^)?([0-9]+)(.[0-9]+)?(.[0-9]+)?", NUMBER),
            Rule::token_to(r"(?m);", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^-}{]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)-\}", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[-}{]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for FuncLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
