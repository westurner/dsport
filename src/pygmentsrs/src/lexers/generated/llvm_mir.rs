#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.asm:LlvmMirLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:LlvmMirLexer:llvm_mir

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: llvm-mir
pub struct LlvmMirLexer;

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
            Rule::token(r"(?m)#.*", COMMENT),
            Rule::token_to(r"(?m)--- \|$", KEYWORD, NewState::Push(vec![r"llvm_ir"])),
            Rule::token_to(r"(?m)---", KEYWORD, NewState::Push(vec![r"llvm_mir"])),
            Rule::token(r"(?m)[^-#]+|.", TEXT),
        ],
    );
    m.insert(
        r"llvm_ir",
        vec![
            Rule::token_to(r"(?m)(\.\.\.|(?=---))", KEYWORD, NewState::Pop(1)),
            Rule::bygroups_g(
                r"(?m)((?:.|\n)+?)(?=(\.\.\.|---))",
                vec![Some(GroupAction::UsingLexer {
                    alias: "llvm",
                    state: None,
                })],
            ),
        ],
    );
    m.insert(
        r"llvm_mir",
        vec![
            Rule::token(r"(?m)#.*", COMMENT),
            Rule::token_to(r"(?m)(\.\.\.|(?=---))", KEYWORD, NewState::Pop(1)),
            Rule::token_to(r"(?m)name:", KEYWORD, NewState::Push(vec![r"name"])),
            Rule::token_to(
                r"(?m)(alignment):",
                KEYWORD,
                NewState::Push(vec![r"number"]),
            ),
            Rule::token_to(
                r"(?m)(exposesReturnsTwice|legalized|regBankSelected|selected|tracksRegLiveness):",
                KEYWORD,
                NewState::Push(vec![r"boolean"]),
            ),
            Rule::token(
                r"(?m)(f(?:ixedStack|rameInfo)|liveins|machineFunctionInfo|registers|stack):",
                KEYWORD,
            ),
            Rule::token_to(
                r"(?m)body: *\|",
                KEYWORD,
                NewState::Push(vec![r"llvm_mir_body"]),
            ),
            Rule::token(r"(?m).+", TEXT),
            Rule::token(r"(?m)\n", WHITESPACE),
        ],
    );
    m.insert(
        r"name",
        vec![
            Rule::token(r"(?m)[^\n]+", NAME),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"boolean",
        vec![
            Rule::token(r"(?m) *(true|false)", NAME_BUILTIN),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"number",
        vec![
            Rule::token(r"(?m) *[0-9]+", NUMBER),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"llvm_mir_body",
        vec![
            Rule::token_to(r"(?m)(\.\.\.|(?=---))", KEYWORD, NewState::Pop(2)),
            Rule::bygroups_g(
                r"(?m)((?:.|\n)+?)(?=\.\.\.|---)",
                vec![Some(GroupAction::UsingLexer {
                    alias: "llvm-mir-body",
                    state: None,
                })],
            ),
            Rule::bygroups_g(
                r"(?m)(?!\.\.\.|---)((?:.|\n)+)",
                vec![Some(GroupAction::UsingLexer {
                    alias: "llvm-mir-body",
                    state: None,
                })],
            ),
        ],
    );
    Table(m)
}

impl Lexer for LlvmMirLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
