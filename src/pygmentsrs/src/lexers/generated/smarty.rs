#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.templates:SmartyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:SmartyLexer:smarty

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: smarty
pub struct SmartyLexer;

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
            Rule::token(r"(?ms)[^{]+", OTHER),
            Rule::bygroups(
                r"(?ms)(\{)(\*.*?\*)(\})",
                vec![Some(COMMENT_PREPROC), Some(COMMENT), Some(COMMENT_PREPROC)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\{php\})(.*?)(\{/php\})",
                vec![
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                    Some(GroupAction::UsingLexer {
                        alias: "php",
                        state: None,
                    }),
                    Some(GroupAction::Token(COMMENT_PREPROC)),
                ],
            ),
            Rule::bygroups_to(
                r"(?ms)(\{)(/?[a-zA-Z_]\w*)(\s*)",
                vec![Some(COMMENT_PREPROC), Some(NAME_FUNCTION), Some(TEXT)],
                NewState::Push(vec![r"smarty"]),
            ),
            Rule::token_to(r"(?ms)\{", COMMENT_PREPROC, NewState::Push(vec![r"smarty"])),
        ],
    );
    m.insert(
        r"smarty",
        vec![
            Rule::token(r"(?ms)\s+", TEXT),
            Rule::token_to(r"(?ms)\{", COMMENT_PREPROC, NewState::PushSame),
            Rule::token_to(r"(?ms)\}", COMMENT_PREPROC, NewState::Pop(1)),
            Rule::token(r"(?ms)#[a-zA-Z_]\w*#", NAME_VARIABLE),
            Rule::token(r"(?ms)\$[a-zA-Z_]\w*(\.\w+)*", NAME_VARIABLE),
            Rule::token(r"(?ms)[~!%^&*()+=|\[\]:;,.<>/?@-]", OPERATOR),
            Rule::token(r"(?ms)(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::token(
                r"(?ms)[0-9](\.[0-9]*)?(eE[+-][0-9])?[flFLdD]?|0[xX][0-9a-fA-F]+[Ll]?",
                NUMBER,
            ),
            Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
        ],
    );
    Table(m)
}

impl Lexer for SmartyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
