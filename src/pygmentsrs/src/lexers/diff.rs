//! `pygments.lexers.diff.DiffLexer` — port via the [`RegexLexer`][crate::lexer::engine]
//! engine.
//!
//! Direct one-state mapping of upstream's `tokens['root']` rule list,
//! all `bygroups` of `(Whitespace, …, Whitespace)` etc.

use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::{
    GENERIC_DELETED, GENERIC_HEADING, GENERIC_INSERTED, GENERIC_STRONG, GENERIC_SUBHEADING, TEXT,
    TokenType, WHITESPACE,
};

pub struct DiffLexer;

struct Table {
    root: Vec<Rule>,
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    Table {
        root: vec![
            Rule::bygroups(
                r"( )(.*)(\n)",
                vec![Some(WHITESPACE), Some(TEXT), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"(!.*|---)(\n)",
                vec![Some(GENERIC_STRONG), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"((?:< |-).*)(\n)",
                vec![Some(GENERIC_DELETED), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"((?:> |\+).*)(\n)",
                vec![Some(GENERIC_INSERTED), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"(@.*|\d(?:,\d+)?(?:a|c|d)\d+(?:,\d+)?)(\n)",
                vec![Some(GENERIC_SUBHEADING), Some(WHITESPACE)],
            ),
            Rule::bygroups(
                r"((?:[Ii]ndex|diff).*)(\n)",
                vec![Some(GENERIC_HEADING), Some(WHITESPACE)],
            ),
            Rule::bygroups(r"(=.*)(\n)", vec![Some(GENERIC_HEADING), Some(WHITESPACE)]),
            Rule::bygroups(r"(.*)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        ],
    }
}

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        match name {
            "root" => Some(&self.root),
            _ => None,
        }
    }
}

impl Lexer for DiffLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(src: &str) -> Vec<(String, String)> {
        DiffLexer
            .get_tokens(src)
            .into_iter()
            .map(|(t, v)| (t.repr(), v))
            .collect()
    }

    #[test]
    fn deletion_and_insertion() {
        let out = lex("-foo\n+bar\n");
        assert!(
            out.iter()
                .any(|(t, v)| t == "Token.Generic.Deleted" && v == "-foo")
        );
        assert!(
            out.iter()
                .any(|(t, v)| t == "Token.Generic.Inserted" && v == "+bar")
        );
    }

    #[test]
    fn hunk_header_subheading() {
        let out = lex("@@ -1,3 +1,3 @@\n");
        assert!(out.iter().any(|(t, _)| t == "Token.Generic.Subheading"));
    }

    #[test]
    fn index_heading() {
        let out = lex("Index: foo\n");
        assert!(
            out.iter()
                .any(|(t, v)| t == "Token.Generic.Heading" && v == "Index: foo")
        );
    }
}
