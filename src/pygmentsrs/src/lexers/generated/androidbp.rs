//! AUTO-GENERATED from `pygments.pygments.lexers.soong:SoongLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.soong:SoongLexer:androidbp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: androidbp, bp, soong
pub struct AndroidbpLexer;

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
            Rule::bygroups_to(
                r"(?m)(\w*)(\s*)(\+?=)(\s*)",
                vec![
                    Some(NAME_VARIABLE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                ],
                NewState::Push(vec![r"assign-rhs"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\w*)(\s*)(\{)",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"in-rule"]),
            ),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"assign-rhs",
        vec![
            Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"in-map"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"in-list"])),
            Rule::token(r"(?m)\w+", NAME),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"expr",
        vec![
            Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"in-map"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"in-list"])),
            Rule::token(r"(?m)\w+", NAME),
        ],
    );
    m.insert(
        r"in-list",
        vec![
            Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"in-map"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"in-list"])),
            Rule::token(r"(?m)\w+", NAME),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"in-map",
        vec![
            Rule::bygroups(
                r"(?m)(\w+)(:)(\s*)",
                vec![Some(NAME), Some(PUNCTUATION), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"in-map"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"in-list"])),
            Rule::token(r"(?m)\w+", NAME),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"in-rule",
        vec![
            Rule::bygroups(
                r"(?m)(\w+)(:)(\s*)",
                vec![Some(NAME), Some(PUNCTUATION), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)(true|false)\b", NAME_BUILTIN),
            Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r#"(?m)".*?""#, STRING),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"in-map"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"in-list"])),
            Rule::token(r"(?m)\w+", NAME),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for AndroidbpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
