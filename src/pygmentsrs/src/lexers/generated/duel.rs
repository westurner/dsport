//! AUTO-GENERATED from `pygments.pygments.lexers.webmisc:DuelLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.webmisc:DuelLexer:duel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: duel, jbst, jsonml+bst
pub struct DuelLexer;

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
            Rule::bygroups_g(
                r"(?ms)(<%[@=#!:]?)(.*?)(%>)",
                vec![
                    Some(GroupAction::Token(NAME_TAG)),
                    Some(GroupAction::UsingLexer {
                        alias: "javascript",
                        state: None,
                    }),
                    Some(GroupAction::Token(NAME_TAG)),
                ],
            ),
            Rule::bygroups(
                r"(?ms)(<%\$)(.*?)(:)(.*?)(%>)",
                vec![
                    Some(NAME_TAG),
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                    Some(STRING),
                    Some(NAME_TAG),
                ],
            ),
            Rule::bygroups(
                r"(?ms)(<%--)(.*?)(--%>)",
                vec![Some(NAME_TAG), Some(COMMENT_MULTILINE), Some(NAME_TAG)],
            ),
            Rule::bygroups_g(
                r"(?ms)(<script.*?>)(.*?)(</script>)",
                vec![
                    Some(GroupAction::UsingLexer {
                        alias: "html",
                        state: None,
                    }),
                    Some(GroupAction::UsingLexer {
                        alias: "javascript",
                        state: None,
                    }),
                    Some(GroupAction::UsingLexer {
                        alias: "html",
                        state: None,
                    }),
                ],
            ),
            Rule::using_lexer(r"(?ms)(.+?)(?=<)", "html", None),
            Rule::using_lexer(r"(?ms).+", "html", None),
        ],
    );
    Table(m)
}

impl Lexer for DuelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
