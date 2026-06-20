#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.templates:CheetahLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:CheetahLexer:cheetah

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cheetah, spitfire
pub struct CheetahLexer;

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
            Rule::bygroups(r"(?m)(##[^\n]*)$", vec![Some(COMMENT)]),
            Rule::token(r"(?m)#[*](.|\n)*?[*]#", COMMENT),
            Rule::token(r"(?m)#end[^#\n]*(?:#|$)", COMMENT_PREPROC),
            Rule::token(r"(?m)#slurp$", COMMENT_PREPROC),
            Rule::bygroups_g(
                r"(?m)(#[a-zA-Z]+)([^#\n]*)(#|$)",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "cheetahpython",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(\$)([a-zA-Z_][\w.]*\w)",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "cheetahpython",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(\$\{!?)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "cheetahpython",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::token(
                r"(?m)(?sx)
                (.+?)               # anything, followed by:
                (?:
                 (?=\#[#a-zA-Z]*) | # an eval comment
                 (?=\$[a-zA-Z_{]) | # a substitution
                 \Z                 # end of string
                )
            ",
                OTHER,
            ),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for CheetahLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
