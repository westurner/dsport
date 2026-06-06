//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:CapDLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:CapDLLexer:capdl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: capdl
pub struct CapdlLexer;

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
        Rule::bygroups(r"(?m)^(\s*)(#.*)(\n)", vec![Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
        Rule::token(r"(?m)(//|--).*$", COMMENT),
        Rule::token(r"(?m)[<>\[(){},:;=\]]", PUNCTUATION),
        Rule::token(r"(?m)\.\.", PUNCTUATION),
        Rule::token(r"(?m)(ar(?:ch|m11)|c(?:aps|hild_of)|i(?:a32|rq)|(?:map|object)s)\b", KEYWORD),
        Rule::token(r"(?m)(a(?:ep|sid_pool)|cnode|ep|frame|io_(?:device|p(?:orts|t))|notification|p(?:[dt])|tcb|ut|vcpu)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(R(?:W(?:[GX])|[GWX])|W(?:[GX])|a(?:ddr|sid)|badge|cached|dom(?:(?:ainID)?)|elf|fault_ep|guard(?:(?:_size)?)|i(?:nit|p)|level|mas(?:ked|ter_reply)|p(?:addr|orts|rio)|reply|sp|uncached|[GRW])\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+(\.\d+)?(k|M)?", NUMBER),
        Rule::token(r"(?m)(bits)\b", NUMBER),
        Rule::token(r"(?m)(c(?:aller_slot|space)|ipc_buffer_slot|reply_slot|vspace)\b", NUMBER),
        Rule::token(r"(?m)[a-zA-Z_][-@\.\w]*", NAME),
    ]);
    Table(m)
}

impl Lexer for CapdlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
