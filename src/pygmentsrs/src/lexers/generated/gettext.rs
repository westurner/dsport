#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.textfmts:GettextLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textfmts:GettextLexer:gettext

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pot, po
pub struct GettextLexer;

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
            Rule::token(r"(?m)^#,\s.*?$", KEYWORD_TYPE),
            Rule::token(r"(?m)^#:\s.*?$", KEYWORD_DECLARATION),
            Rule::token(r"(?m)^(#|#\.\s|#\|\s|#~\s|#\s).*$", COMMENT_SINGLE),
            Rule::bygroups(
                r#"(?m)^(")([A-Za-z-]+:)(.*")$"#,
                vec![Some(STRING), Some(NAME_PROPERTY), Some(STRING)],
            ),
            Rule::token(r#"(?m)^".*"$"#, STRING),
            Rule::bygroups(
                r#"(?m)^(msgid|msgid_plural|msgstr|msgctxt)(\s+)(".*")$"#,
                vec![Some(NAME_VARIABLE), Some(TEXT), Some(STRING)],
            ),
            Rule::bygroups(
                r#"(?m)^(msgstr\[)(\d)(\])(\s+)(".*")$"#,
                vec![
                    Some(NAME_VARIABLE),
                    Some(NUMBER_INTEGER),
                    Some(NAME_VARIABLE),
                    Some(TEXT),
                    Some(STRING),
                ],
            ),
        ],
    );
    Table(m)
}

impl Lexer for GettextLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
