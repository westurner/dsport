//! AUTO-GENERATED from `pygments.pygments.lexers.configs:NestedTextLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:NestedTextLexer:nestedtext

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nestedtext, nt
pub struct NestedtextLexer;

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
            Rule::bygroups(r"(?m)^([ ]*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT)]),
            Rule::bygroups_to(
                r"(?m)^([ ]*)(\{)",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"inline_dict"]),
            ),
            Rule::bygroups_to(
                r"(?m)^([ ]*)(\[)",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"inline_list"]),
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(>)$",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(>)( )(.*?)([ \t]*)$",
                vec![
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(-)$",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(-)( )(.*?)([ \t]*)$",
                vec![
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(:)$",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)(:)( )([^\n]*?)([ \t]*)$",
                vec![
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NAME_TAG),
                    Some(WHITESPACE),
                ],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)([^\{\[\s].*?)(:)$",
                vec![Some(WHITESPACE), Some(NAME_TAG), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)^([ ]*)([^\{\[\s].*?)(:)( )(.*?)([ \t]*)$",
                vec![
                    Some(WHITESPACE),
                    Some(NAME_TAG),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
            ),
        ],
    );
    m.insert(
        r"inline_list",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)[^\{\}\[\],\s]+", TEXT),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"inline_dict"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"inline_list"])),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", ERROR, NewState::Pop(1)),
        ],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?m)[ \t]+", WHITESPACE)]);
    m.insert(
        r"inline_value",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"inline_dict"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"inline_list"])),
        ],
    );
    m.insert(
        r"inline_dict",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)[^\{\}\[\],:\s]+", NAME_TAG),
            Rule::token_to(
                r"(?m):",
                PUNCTUATION,
                NewState::Push(vec![r"inline_dict_value"]),
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", ERROR, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"inline_dict_value",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token(r"(?m)[^\{\}\[\],:\s]+", TEXT),
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"inline_dict"])),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"inline_list"])),
            Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
        ],
    );
    Table(m)
}

impl Lexer for NestedtextLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
