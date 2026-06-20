//! AUTO-GENERATED from `pygments.pygments.lexers.dylan:DylanLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dylan:DylanLexer:dylan

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dylan
pub struct DylanLexer;

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
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)//.*?\n", COMMENT_SINGLE),
            Rule::bygroups(
                r"(?im)([a-z0-9-]+)(:)([ \t]*)(.*(?:\n[ \t].+)*)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::default(NewState::Push(vec![r"code"])),
        ],
    );
    m.insert(r"code", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::bygroups(r"(?im)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)'(\\.|\\[0-7]{1,3}|\\x[a-f0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?im)#b[01]+", NUMBER_BIN),
        Rule::token(r"(?im)#o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?im)[-+]?(\d*\.\d+(e[-+]?\d+)?|\d+(\.\d*)?e[-+]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?im)[-+]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)#x[0-9a-f]+", NUMBER_HEX),
        Rule::bygroups(r"(?im)(\?\\?[\w!&*<>|^$%@\-+~?/=]+)(:)(token|name|variable|expression|body|case-body|\*)", vec![Some(NAME_TAG), Some(OPERATOR), Some(NAME_BUILTIN)]),
        Rule::bygroups(r"(?im)(\?)(:)(token|name|variable|expression|body|case-body|\*)", vec![Some(NAME_TAG), Some(OPERATOR), Some(NAME_BUILTIN)]),
        Rule::token(r"(?im)\?\\?[\w!&*<>|^$%@\-+~?/=]+", NAME_TAG),
        Rule::token(r"(?im)(=>|::|#\(|#\[|##|\?\?|\?=|\?|[(){}\[\],.;])", PUNCTUATION),
        Rule::token(r"(?im):=", OPERATOR),
        Rule::token(r"(?im)#[tf]", LITERAL),
        Rule::token_to(r#"(?im)#""#, STRING_SYMBOL, NewState::Push(vec![r"keyword"])),
        Rule::token(r"(?im)#[a-z0-9-]+", KEYWORD),
        Rule::token(r"(?im)\\?[\w!&*<>|^$%@\-+~?/=]+:", KEYWORD),
        Rule::token(r"(?im)<\\?[\w!&*<>|^$%@\-+~?/=]+>", NAME_CLASS),
        Rule::token(r"(?im)\*\\?[\w!&*<>|^$%@\-+~?/=]+\*", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)\$\\?[\w!&*<>|^$%@\-+~?/=]+", NAME_CONSTANT),
        Rule::token(r"(?im)\\?[\w!&*<>|^$%@\-+~?/=]+", NAME),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?im)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?im)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"keyword",
        vec![
            Rule::token_to(r#"(?im)""#, STRING_SYMBOL, NewState::Pop(1)),
            Rule::token(r#"(?im)[^\\"]+"#, STRING_SYMBOL),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?im)\\([\\abfnrtv"\']|x[a-f0-9]{2,4}|[0-7]{1,3})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?im)[^\\"\n]+"#, STRING),
            Rule::token(r"(?im)\\\n", STRING),
            Rule::token(r"(?im)\\", STRING),
        ],
    );
    Table(m)
}

impl Lexer for DylanLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
