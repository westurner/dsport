#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.scdoc:ScdocLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scdoc:ScdocLexer:scdoc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: scdoc, scd
pub struct ScdocLexer;

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
            Rule::bygroups(r"(?m)^(;.+\n)", vec![Some(COMMENT)]),
            Rule::bygroups(
                r"(?m)^(#)([^#].+\n)",
                vec![Some(GENERIC_HEADING), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)^(#{2})(.+\n)",
                vec![Some(GENERIC_SUBHEADING), Some(TEXT)],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)([*-])(\s)(.+\n)",
                vec![
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)^(\s*)(\.+\.)( .+\n)",
                vec![
                    Some(GroupAction::Token(TEXT)),
                    Some(GroupAction::Token(KEYWORD)),
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inline"]),
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?m)^(\s*>\s)(.+\n)",
                vec![Some(KEYWORD), Some(GENERIC_EMPH)],
            ),
            Rule::bygroups(
                r"(?m)^(```\n)([\w\W]*?)(^```$)",
                vec![Some(STRING), Some(TEXT), Some(STRING)],
            ),
            Rule::token(r"(?m)\\.", TEXT),
            Rule::bygroups(
                r"(?m)(\s)(_[^_]+_)(\W|\n)",
                vec![Some(TEXT), Some(GENERIC_EMPH), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(\*[^*]+\*)(\W|\n)",
                vec![Some(TEXT), Some(GENERIC_STRONG), Some(TEXT)],
            ),
            Rule::token(r"(?m)`[^`]+`", STRING_BACKTICK),
            Rule::token(r"(?m)[^\\\s]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"inline",
        vec![
            Rule::token(r"(?m)\\.", TEXT),
            Rule::bygroups(
                r"(?m)(\s)(_[^_]+_)(\W|\n)",
                vec![Some(TEXT), Some(GENERIC_EMPH), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?m)(\s)(\*[^*]+\*)(\W|\n)",
                vec![Some(TEXT), Some(GENERIC_STRONG), Some(TEXT)],
            ),
            Rule::token(r"(?m)`[^`]+`", STRING_BACKTICK),
            Rule::token(r"(?m)[^\\\s]+", TEXT),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for ScdocLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
