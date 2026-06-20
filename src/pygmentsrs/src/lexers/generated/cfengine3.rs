#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:Cfengine3Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:Cfengine3Lexer:cfengine3

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cfengine3, cf3
pub struct Cfengine3Lexer;

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
            Rule::token(r"(?m)#.*?\n", COMMENT),
            Rule::bygroups(
                r"(?m)(body)(\s+)(\S+)(\s+)(control)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                ],
            ),
            Rule::bygroups_to(
                r"(?m)(body|bundle)(\s+)(\S+)(\s+)(\w+)(\()",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"arglist"]),
            ),
            Rule::bygroups(
                r"(?m)(body|bundle)(\s+)(\S+)(\s+)(\w+)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_FUNCTION),
                ],
            ),
            Rule::bygroups(
                r#"(?m)(")([^"]+)(")(\s+)(string|slist|int|real)(\s*)(=>)(\s*)"#,
                vec![
                    Some(PUNCTUATION),
                    Some(NAME_VARIABLE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(KEYWORD_TYPE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\S+)(\s*)(=>)(\s*)",
                vec![
                    Some(KEYWORD_RESERVED),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(TEXT),
                ],
            ),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::bygroups(
                r"(?m)(\w+)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)([\w.!&|()]+)(::)",
                vec![Some(NAME_CLASS), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)(\w+)(:)",
                vec![Some(KEYWORD_DECLARATION), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)@[{(][^)}]+[})]", NAME_VARIABLE),
            Rule::token(r"(?m)[(){},;]", PUNCTUATION),
            Rule::token(r"(?m)=>", OPERATOR),
            Rule::token(r"(?m)->", OPERATOR),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
            Rule::token(r"(?m)\w+", NAME_FUNCTION),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(
                r"(?m)\$[{(]",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol"]),
            ),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?m)\n", STRING),
            Rule::token(r"(?m).", STRING),
        ],
    );
    m.insert(
        r"interpol",
        vec![
            Rule::token_to(r"(?m)\$[{(]", STRING_INTERPOL, NewState::PushSame),
            Rule::token_to(r"(?m)[})]", STRING_INTERPOL, NewState::Pop(1)),
            Rule::token(r"(?m)[^${()}]+", STRING_INTERPOL),
        ],
    );
    m.insert(
        r"arglist",
        vec![
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\w+", NAME_VARIABLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for Cfengine3Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
