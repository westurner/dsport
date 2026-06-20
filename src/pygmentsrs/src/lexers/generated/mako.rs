//! AUTO-GENERATED from `pygments.pygments.lexers.templates:MakoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:MakoLexer:mako

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mako
pub struct MakoLexer;

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
            Rule::bygroups(
                r"(?m)(\s*)(%)(\s*end(?:\w+))(\n|\Z)",
                vec![
                    Some(WHITESPACE),
                    Some(COMMENT_PREPROC),
                    Some(KEYWORD),
                    Some(OTHER),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(\s*)(%)([^\n]*)(\n|\Z)",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "python",
                        state: None,
                    }),
                    Some(GroupAction::Token(OTHER)),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\s*)(##[^\n]*)(\n|\Z)",
                vec![Some(WHITESPACE), Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)(?s)<%doc>.*?</%doc>", COMMENT_MULTILINE),
            Rule::bygroups_to(
                r"(?m)(<%)([\w.:]+)",
                vec![Some(COMMENT_PREPROC), Some(NAME_BUILTIN)],
                NewState::Push(vec![r"tag"]),
            ),
            Rule::bygroups(
                r"(?m)(</%)([\w.:]+)(>)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(NAME_BUILTIN),
                    Some(COMMENT_PREPROC),
                ],
            ),
            Rule::token_to(
                r"(?m)<%(?=([\w.:]+))",
                COMMENT_PREPROC,
                NewState::Push(vec![r"ondeftags"]),
            ),
            Rule::bygroups_g(
                r"(?m)(?s)(<%(?:!?))(.*?)(%>)",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "python",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "python",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?sx)
                (.+?)                # anything, followed by:
                (?:
                 (?<=\n)(?=%|\#\#) | # an eval or comment line
                 (?=\#\*) |          # multiline comment
                 (?=</?%) |          # a python block
                                     # call start or end
                 (?=\$\{) |          # a substitution
                 (?<=\n)(?=\s*%) |
                                     # - don't consume
                 (\\\n) |            # an escaped newline
                 \Z                  # end of string
                )
            ",
                vec![Some(OTHER), Some(OPERATOR)],
            ),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    m.insert(
        r"ondeftags",
        vec![
            Rule::token(r"(?m)<%", COMMENT_PREPROC),
            Rule::token(r"(?m)(?<=<%)(include|inherit|namespace|page)", NAME_BUILTIN),
            Rule::bygroups(
                r#"(?m)((?:\w+)\s*=)(\s*)(".*?")"#,
                vec![Some(NAME_ATTRIBUTE), Some(TEXT), Some(STRING)],
            ),
            Rule::token_to(r"(?m)/?\s*>", COMMENT_PREPROC, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::bygroups(
                r#"(?m)((?:\w+)\s*=)(\s*)(".*?")"#,
                vec![Some(NAME_ATTRIBUTE), Some(TEXT), Some(STRING)],
            ),
            Rule::token_to(r"(?m)/?\s*>", COMMENT_PREPROC, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    m.insert(
        r"attr",
        vec![
            Rule::token_to(r#"(?m)".*?""#, STRING, NewState::Pop(1)),
            Rule::token_to(r"(?m)'.*?'", STRING, NewState::Pop(1)),
            Rule::token_to(r"(?m)[^\s>]+", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for MakoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
