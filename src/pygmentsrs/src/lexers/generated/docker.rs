#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:DockerLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:DockerLexer:docker

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: docker, dockerfile
pub struct DockerLexer;

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
            Rule::token(r"(?im)#.*", COMMENT),
            Rule::bygroups(
                r"(?im)(FROM)([ \t]*)(\S*)([ \t]*)(?:(AS)([ \t]*)(\S*))?",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(STRING),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::bygroups_g(
                r"(?im)(ONBUILD)(\s+)((?:\s*\\?\s*))",
                vec![
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "bash",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?im)(HEALTHCHECK)(\s+)(((?:\s*\\?\s*)--\w+=\w+(?:\s*\\?\s*))*)",
                vec![
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "bash",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?im)(VOLUME|ENTRYPOINT|CMD|SHELL)(\s+)((?:\s*\\?\s*))(\[.*?\])",
                vec![
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "bash",
                        state: None,
                    }),
                    Some(GroupAction::UsingLexer {
                        alias: "json",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?im)(LABEL|ENV|ARG)(\s+)(((?:\s*\\?\s*)\w+=\w+(?:\s*\\?\s*))*)",
                vec![
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "bash",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?im)((?:MAINTAINER|EXPOSE|WORKDIR|USER|STOPSIGNAL)|VOLUME)\b(\s+)(.*)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(STRING)],
            ),
            Rule::bygroups(
                r"(?im)((?:RUN|CMD|ENTRYPOINT|ENV|ARG|LABEL|ADD|COPY))(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
            ),
            Rule::using_lexer(r"(?im)(.*\\\n)*.+", "bash", None),
        ],
    );
    Table(m)
}

impl Lexer for DockerLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
