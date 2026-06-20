//! AUTO-GENERATED from `pygments.pygments.lexers.capnproto:CapnProtoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.capnproto:CapnProtoLexer:capnp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: capnp
pub struct CapnpLexer;

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
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)@[0-9a-zA-Z]*", NAME_DECORATOR),
        Rule::token_to(r"(?m)=", LITERAL, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m):", NAME_CLASS, NewState::Push(vec![r"type"])),
        Rule::token_to(r"(?m)\$", NAME_ATTRIBUTE, NewState::Push(vec![r"annotation"])),
        Rule::token(r"(?m)(struct|enum|interface|union|import|using|const|annotation|extends|in|of|on|as|with|from|fixed)\b", KEYWORD),
        Rule::token(r"(?m)[\w.]+", NAME),
        Rule::token(r"(?m)[^#@=:$\w\s]+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"type",
        vec![
            Rule::token(r"(?m)[^\]\[=;,(){}$]+", NAME_CLASS),
            Rule::token_to(r"(?m)[\[(]", NAME_CLASS, NewState::Push(vec![r"parentype"])),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"parentype",
        vec![
            Rule::token(r"(?m)[^\]\[;()]+", NAME_CLASS),
            Rule::token_to(r"(?m)[\[(]", NAME_CLASS, NewState::PushSame),
            Rule::token_to(r"(?m)[\])]", NAME_CLASS, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"expression",
        vec![
            Rule::token(r"(?m)[^\]\[;,(){}$]+", LITERAL),
            Rule::token_to(r"(?m)[\[(]", LITERAL, NewState::Push(vec![r"parenexp"])),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"parenexp",
        vec![
            Rule::token(r"(?m)[^\]\[;()]+", LITERAL),
            Rule::token_to(r"(?m)[\[(]", LITERAL, NewState::PushSame),
            Rule::token_to(r"(?m)[\])]", LITERAL, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"annotation",
        vec![
            Rule::token(r"(?m)[^\]\[;,(){}=:]+", NAME_ATTRIBUTE),
            Rule::token_to(
                r"(?m)[\[(]",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"annexp"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"annexp",
        vec![
            Rule::token(r"(?m)[^\]\[;()]+", NAME_ATTRIBUTE),
            Rule::token_to(r"(?m)[\[(]", NAME_ATTRIBUTE, NewState::PushSame),
            Rule::token_to(r"(?m)[\])]", NAME_ATTRIBUTE, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for CapnpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
