#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.markup:MoinWikiLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:MoinWikiLexer:trac_wiki

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: trac-wiki, moin
pub struct TracWikiLexer;

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
            Rule::token(r"(?im)^#.*$", COMMENT),
            Rule::bygroups(r"(?im)(!)(\S+)", vec![Some(KEYWORD), Some(TEXT)]),
            Rule::bygroups_g(
                r"(?im)^(=+)([^=]+)(=+)(\s*#.+)?$",
                vec![
                    Some(GroupAction::Token(GENERIC_HEADING)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(GENERIC_HEADING)),
                    Some(GroupAction::Token(STRING)),
                ],
            ),
            Rule::bygroups_to(
                r"(?im)(\{\{\{)(\n#!.+)?",
                vec![Some(NAME_BUILTIN), Some(NAME_NAMESPACE)],
                NewState::Push(vec![r"codeblock"]),
            ),
            Rule::token(r"(?im)(\'\'\'?|\|\||`|__|~~|\^|,,|::)", COMMENT),
            Rule::bygroups(
                r"(?im)^( +)([.*-])( )",
                vec![Some(TEXT), Some(NAME_BUILTIN), Some(TEXT)],
            ),
            Rule::bygroups(
                r"(?im)^( +)([a-z]{1,5}\.)( )",
                vec![Some(TEXT), Some(NAME_BUILTIN), Some(TEXT)],
            ),
            Rule::token(r"(?im)\[\[\w+.*?\]\]", KEYWORD),
            Rule::bygroups(
                r"(?im)(\[[^\s\]]+)(\s+[^\]]+?)?(\])",
                vec![Some(KEYWORD), Some(STRING), Some(KEYWORD)],
            ),
            Rule::token(r"(?im)^----+$", KEYWORD),
            Rule::token(r"(?im)[^\n\'\[{!_~^,|]+", TEXT),
            Rule::token(r"(?im)\n", TEXT),
            Rule::token(r"(?im).", TEXT),
        ],
    );
    m.insert(
        r"codeblock",
        vec![
            Rule::token_to(r"(?im)\}\}\}", NAME_BUILTIN, NewState::Pop(1)),
            Rule::token_to(r"(?im)\{\{\{", TEXT, NewState::PushSame),
            Rule::token(r"(?im)[^{}]+", COMMENT_PREPROC),
            Rule::token(r"(?im).", COMMENT_PREPROC),
        ],
    );
    Table(m)
}

impl Lexer for TracWikiLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
