#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:TOMLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:TOMLLexer:toml

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: toml
pub struct TomlLexer;

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
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*", COMMENT_SINGLE),
            Rule::token(r"(?m)[A-Za-z0-9_-]+", NAME),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"basic-string"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literal-string"]),
            ),
            Rule::token(r"(?m)\.", PUNCTUATION),
            Rule::bygroups_to(
                r"(?m)(=)(\s*)",
                vec![Some(OPERATOR), Some(WHITESPACE)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::token_to(r"(?m)\[\[?", KEYWORD, NewState::Push(vec![r"table-key"])),
        ],
    );
    m.insert(
        r"key",
        vec![
            Rule::token(r"(?m)[A-Za-z0-9_-]+", NAME),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"basic-string"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literal-string"]),
            ),
            Rule::token(r"(?m)\.", PUNCTUATION),
        ],
    );
    m.insert(
        r"table-key",
        vec![
            Rule::token(r"(?m)[A-Za-z0-9_-]+", KEYWORD),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"basic-string"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literal-string"]),
            ),
            Rule::token(r"(?m)\.", KEYWORD),
            Rule::token_to(r"(?m)\]\]?", KEYWORD, NewState::Pop(1)),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
        ],
    );
    m.insert(
        r"value",
        vec![
            Rule::token_to(
                r"(?m)(?x)
                  \d\d\d\d-\d\d-\d\d # date, e.g., 1988-10-27
                (
                  [Tt ] \d\d:\d\d(:\d\d(\.\d+)?)? # optional time
                  (
                    [Zz]|[+-]\d\d:\d\d # optional time offset
                  )?
                )?
              ",
                LITERAL_DATE,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)\d\d:\d\d(:\d\d(\.\d+)?)?",
                LITERAL_DATE,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)[+-]?\d[0-9_]*[eE][+-]?\d[0-9_]*",
                NUMBER_FLOAT,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)[+-]?\d[0-9_]*\.\d[0-9_]*([eE][+-]?\d[0-9_]*)?",
                NUMBER_FLOAT,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)[+-]?(inf|nan)", NUMBER_FLOAT, NewState::Pop(1)),
            Rule::token_to(r"(?m)-?0b[01_]+", NUMBER_BIN, NewState::Pop(1)),
            Rule::token_to(r"(?m)-?0o[0-7_]+", NUMBER_OCT, NewState::Pop(1)),
            Rule::token_to(r"(?m)-?0x[0-9a-fA-F_]+", NUMBER_HEX, NewState::Pop(1)),
            Rule::token_to(r"(?m)[+-]?[0-9_]+", NUMBER_INTEGER, NewState::Pop(1)),
            Rule::token_to(
                r#"(?m)""""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"#pop", r"multiline-basic-string"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"#pop", r"basic-string"]),
            ),
            Rule::token_to(
                r"(?m)'''",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"multiline-literal-string"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"literal-string"]),
            ),
            Rule::token_to(r"(?m)true|false", KEYWORD_CONSTANT, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"array"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"inline-table"]),
            ),
        ],
    );
    m.insert(
        r"array",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*", COMMENT_SINGLE),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::default(NewState::Push(vec![r"value"])),
        ],
    );
    m.insert(
        r"inline-table",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*", COMMENT_SINGLE),
            Rule::token(r"(?m)[A-Za-z0-9_-]+", NAME),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"basic-string"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literal-string"]),
            ),
            Rule::token(r"(?m)\.", PUNCTUATION),
            Rule::bygroups_to(
                r"(?m)(=)(\s*)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"basic-string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(
                r"(?m)\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}",
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"escapes",
        vec![
            Rule::token(
                r"(?m)\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}",
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"literal-string",
        vec![Rule::token_to(r"(?m).*?'", STRING_SINGLE, NewState::Pop(1))],
    );
    m.insert(
        r"multiline-basic-string",
        vec![
            Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
            Rule::token(
                r"(?m)\\x[0-9a-fA-F]{2}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}",
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)""#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"multiline-literal-string",
        vec![
            Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)[^']+", STRING_SINGLE),
            Rule::token(r"(?m)'", STRING_SINGLE),
        ],
    );
    Table(m)
}

impl Lexer for TomlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
