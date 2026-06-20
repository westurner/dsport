#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.actionscript:MxmlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.actionscript:MxmlLexer:mxml

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mxml
pub struct MxmlLexer;

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
            Rule::token(r"(?ms)[^<&]+", TEXT),
            Rule::token(r"(?ms)&\S*?;", NAME_ENTITY),
            Rule::bygroups_g(
                r"(?ms)(\<\!\[CDATA\[)(.*?)(\]\]\>)",
                vec![
                    Some(GroupAction::Token(STRING)),
                    Some(GroupAction::UsingLexer {
                        alias: "actionscript3",
                        state: None,
                    }),
                    Some(GroupAction::Token(STRING)),
                ],
            ),
            Rule::token_to(r"(?ms)<!--", COMMENT, NewState::Push(vec![r"comment"])),
            Rule::token(r"(?ms)<\?.*?\?>", COMMENT_PREPROC),
            Rule::token(r"(?ms)<![^>]*>", COMMENT_PREPROC),
            Rule::token_to(r"(?ms)<\s*[\w:.-]+", NAME_TAG, NewState::Push(vec![r"tag"])),
            Rule::token(r"(?ms)<\s*/\s*[\w:.-]+\s*>", NAME_TAG),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?ms)[^-]+", COMMENT),
            Rule::token_to(r"(?ms)-->", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?ms)-", COMMENT),
        ],
    );
    m.insert(
        r"tag",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token_to(
                r"(?ms)[\w.:-]+\s*=",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"attr"]),
            ),
            Rule::token_to(r"(?ms)/?\s*>", NAME_TAG, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"attr",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token_to(r#"(?ms)".*?""#, STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ms)'.*?'", STRING, NewState::Pop(1)),
            Rule::token_to(r"(?ms)[^\s>]+", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for MxmlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
