//! AUTO-GENERATED from `pygments.pygments.lexers.asm:ObjdumpLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:ObjdumpLexer:objdump

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: objdump
pub struct ObjdumpLexer;

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
            Rule::bygroups(
                r"(?m)(.*?)(:)( +file format )(.*?)$",
                vec![
                    Some(NAME_LABEL),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(STRING),
                ],
            ),
            Rule::bygroups(
                r"(?m)(Disassembly of section )(.*?)(:)$",
                vec![Some(TEXT), Some(NAME_LABEL), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)([0-9A-Za-z]+)( )(<)(.*?)([-+])(0[xX][A-Za-z0-9]+)(>:)$",
                vec![
                    Some(NUMBER_HEX),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                    Some(NUMBER_HEX),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups(
                r"(?m)([0-9A-Za-z]+)( )(<)(.*?)(>:)$",
                vec![
                    Some(NUMBER_HEX),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)( *)([0-9A-Za-z]+:)(\t)((?:[0-9A-Za-z][0-9A-Za-z] )+)( *	)([a-zA-Z].*?)$",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(NAME_LABEL)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(NUMBER_HEX)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "gas",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups_g(
                r"(?m)( *)([0-9A-Za-z]+:)( *\t)([a-zA-Z].*?)$",
                vec![
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::Token(NAME_LABEL)),
                    Some(GroupAction::Token(WHITESPACE)),
                    Some(GroupAction::UsingLexer {
                        alias: "gas",
                        state: None,
                    }),
                ],
            ),
            Rule::bygroups(
                r"(?m)( *)([0-9A-Za-z]+:)(\t)((?:[0-9A-Za-z][0-9A-Za-z] )+)( *)(.*?)$",
                vec![
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(NUMBER_HEX),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::bygroups(
                r"(?m)( *)([0-9A-Za-z]+:)(\t)((?:[0-9A-Za-z][0-9A-Za-z] )+)$",
                vec![
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(NUMBER_HEX),
                ],
            ),
            Rule::token(r"(?m)\t\.\.\.$", TEXT),
            Rule::bygroups(
                r"(?m)(\t\t\t)([0-9A-Za-z]+:)( )([^\t]+)(\t)(.*?)([-+])(0x[0-9A-Za-z]+)$",
                vec![
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(NAME_PROPERTY),
                    Some(WHITESPACE),
                    Some(NAME_CONSTANT),
                    Some(PUNCTUATION),
                    Some(NUMBER_HEX),
                ],
            ),
            Rule::bygroups(
                r"(?m)(\t\t\t)([0-9A-Za-z]+:)( )([^\t]+)(\t)(.*?)$",
                vec![
                    Some(WHITESPACE),
                    Some(NAME_LABEL),
                    Some(WHITESPACE),
                    Some(NAME_PROPERTY),
                    Some(WHITESPACE),
                    Some(NAME_CONSTANT),
                ],
            ),
            Rule::token(r"(?m)[^\n]+\n", OTHER),
        ],
    );
    Table(m)
}

impl Lexer for ObjdumpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
