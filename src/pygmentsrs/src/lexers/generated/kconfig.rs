//! AUTO-GENERATED from `pygments.pygments.lexers.configs:KconfigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:KconfigLexer:kconfig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: kconfig, menuconfig, linux-config, kernel-config
pub struct KconfigLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)(c(?:hoice|o(?:mment|nfig))|de(?:fault|pends\ on)|end(?:choice|if|menu)|if|m(?:ainmenu|enu(?:(?:config)?))|option|prompt|range|s(?:elect|ource)|visible\ if)\b", KEYWORD),
        Rule::token_to(r"(?m)(---help---|help)[\t ]*\n", KEYWORD, NewState::Push(vec![r"help"])),
        Rule::token(r"(?m)(bool|tristate|string|hex|int|defconfig_list|modules|env)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[!=&|]", OPERATOR),
        Rule::token(r"(?m)[()]", PUNCTUATION),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r#"(?m)"(""|[^"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)\S+", TEXT),
    ]);
    m.insert(
        r"help",
        vec![
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){7}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent7"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){6}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent6"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){5}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent5"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){4}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent4"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){3}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent3"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}){2}.*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent2"]),
            ),
            Rule::token_to(
                r"(?m)(?:\t| {1,7}\t| {8}).*\n",
                STRING_DOC,
                NewState::Push(vec![r"indent1"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"indent7",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){7}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent6",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){6}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent5",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){5}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent4",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){4}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent3",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){3}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent2",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}){2}.*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"indent1",
        vec![
            Rule::token(r"(?m)(?:\t| {1,7}\t| {8}).*\n", STRING_DOC),
            Rule::token(r"(?m)\s*\n", TEXT),
            Rule::default(NewState::Pop(2)),
        ],
    );
    Table(m)
}

impl Lexer for KconfigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
