#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.templates:MasonLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:MasonLexer:mason

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mason
pub struct MasonLexer;

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
            Rule::bygroups(
                r"(?m)(?s)(<%doc>)(.*?)(</%doc>)",
                vec![Some(NAME_TAG), Some(COMMENT_MULTILINE), Some(NAME_TAG)],
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(<%(?:def|method))(\s*)(.*?)(>)(.*?)(</%\2\s*>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(NAME_FUNCTION)),
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(<%(\w+)(.*?)(>))(.*?)(</%\2\s*>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    None,
                    None,
                    None,
                    Some(GroupAction::UsingLexer {
                        alias: "perl",
                        state: None,
                    }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(<&[^|])(.*?)(,.*?)?(&>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::Token(NAME_FUNCTION)),
                    Some(GroupAction::UsingLexer {
                        alias: "perl",
                        state: None,
                    }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(<&\|)(.*?)(,.*?)?(&>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::Token(NAME_FUNCTION)),
                    Some(GroupAction::UsingLexer {
                        alias: "perl",
                        state: None,
                    }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::token(r"(?m)</&>", NAME_TAG),
            Rule::bygroups_g(
                r"(?m)(?s)(<%!?)(.*?)(%>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::UsingLexer {
                        alias: "perl",
                        state: None,
                    }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::token(r"(?m)(?<=^)#[^\n]*(\n|\Z)", COMMENT),
            Rule::bygroups_g(
                r"(?m)(?<=^)(%)([^\n]*)(\n|\Z)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::UsingLexer {
                        alias: "perl",
                        state: None,
                    }),
                    Some(GroupAction::Token(OTHER)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(?sx)
                 (.+?)               # anything, followed by:
                 (?:
                  (?<=\n)(?=[%#]) |  # an eval or comment line
                  (?=</?[%&]) |      # a substitution or block or
                                     # call start or end
                                     # - don't consume
                  (\\\n) |           # an escaped newline
                  \Z                 # end of string
                 )",
                vec![
                    Some(GroupAction::UsingLexer {
                        alias: "html",
                        state: None,
                    }),
                    Some(GroupAction::Token(OPERATOR)),
                ],
            ),
        ],
    );
    Table(m)
}

impl Lexer for MasonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
