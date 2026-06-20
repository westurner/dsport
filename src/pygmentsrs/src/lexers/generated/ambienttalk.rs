#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ambient:AmbientTalkLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ambient:AmbientTalkLexer:ambienttalk

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ambienttalk, ambienttalk/2, at
pub struct AmbienttalkLexer;

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
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)(def|deftype|import|alias|exclude)\b", KEYWORD),
        Rule::token(r"(?ms)((?:a(?:ctor|s)|becomes|disco(?:(?:nnect|ver)ed)|e(?:lse|xport)|i(?:[fs])|mirror(?:(?:edBy)?)|object|reconnected|t(?:a(?:ggedAs|kenOffline)|hen)|when(?:(?:ever)?)):)", NAME_BUILTIN),
        Rule::token(r"(?ms)(true|false|nil)\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?ms)(~|lobby|jlobby|/)\.", KEYWORD_CONSTANT, NewState::Push(vec![r"namespace"])),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token_to(r"(?ms)\|", PUNCTUATION, NewState::Push(vec![r"arglist"])),
        Rule::token(r"(?ms)<:|[*^!%&<>+=,./?-]|:=", OPERATOR),
        Rule::token(r"(?ms)`[a-zA-Z_]\w*", STRING_SYMBOL),
        Rule::token(r"(?ms)[a-zA-Z_]\w*:", NAME_FUNCTION),
        Rule::token(r"(?ms)[{}()\[\];`]", PUNCTUATION),
        Rule::token(r"(?ms)(self|super)\b", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?ms)@[a-zA-Z_]\w*", NAME_CLASS),
        Rule::token_to(r"(?ms)@\[", NAME_CLASS, NewState::Push(vec![r"annotations"])),
        Rule::token(r"(?ms)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)\d+", NUMBER_INTEGER),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?ms)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
            Rule::token(r"(?ms)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"namespace",
        vec![
            Rule::token(r"(?ms)[a-zA-Z_]\w*\.", NAME_NAMESPACE),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*:", NAME_FUNCTION, NewState::Pop(1)),
            Rule::token_to(r"(?ms)[a-zA-Z_]\w*(?!\.)", NAME_FUNCTION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"annotations",
        vec![Rule::token_to(
            r"(?ms)(.*?)\]",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"arglist",
        vec![
            Rule::token_to(r"(?ms)\|", PUNCTUATION, NewState::Pop(1)),
            Rule::bygroups(
                r"(?ms)(\s*)(,)(\s*)",
                vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)],
            ),
            Rule::token(r"(?ms)[a-zA-Z_]\w*", NAME_VARIABLE),
        ],
    );
    Table(m)
}

impl Lexer for AmbienttalkLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
