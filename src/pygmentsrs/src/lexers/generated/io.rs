//! AUTO-GENERATED from `pygments.pygments.lexers.iolang:IoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.iolang:IoLexer:io

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: io
pub struct IoLexer;

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
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
            Rule::token(r"(?m)#(.*?)$", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?m)/\+",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"nestedcomment"]),
            ),
            Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
            Rule::token(
                r"(?m)::=|:=|=|\(|\)|;|,|\*|-|\+|>|<|@|!|/|\||\^|\.|%|&|\[|\]|\{|\}",
                OPERATOR,
            ),
            Rule::token(
                r"(?m)(clone|do|doFile|doString|method|for|if|else|elseif|then)\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)(nil|false|true)\b", NAME_CONSTANT),
            Rule::token(
                r"(?m)(Object|list|List|Map|args|Sequence|Coroutine|File)\b",
                NAME_BUILTIN,
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
            Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"nestedcomment",
        vec![
            Rule::token(r"(?m)[^+/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\+", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\+/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[+/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for IoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
