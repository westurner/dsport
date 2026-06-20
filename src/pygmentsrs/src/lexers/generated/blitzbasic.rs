#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.basic:BlitzBasicLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:BlitzBasicLexer:blitzbasic

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: blitzbasic, b3d, bplus
pub struct BlitzbasicLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im);.*?\n", COMMENT_SINGLE),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?im)\.[0-9]+(?!\.)", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)\$[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)\%[10]+", NUMBER_BIN),
        Rule::token(r"(?im)\b(A(?:bs|fter|nd)|Before|F(?:(?:irs|loa)t)|Handle|Int|Last|Mod|Not|Or|S(?:ar|gn|h(?:[lr])|tr))\b", OPERATOR),
        Rule::token(r"(?im)([+\-*/~=<>^])", OPERATOR),
        Rule::token(r"(?im)[(),:\[\]\\]", PUNCTUATION),
        Rule::token(r"(?im)\.([ \t]*)([a-z]\w*)", NAME_LABEL),
        Rule::bygroups(r"(?im)\b(New)\b([ \t]+)([a-z]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)\b(Gosub|Goto)\b([ \t]+)([a-z]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::bygroups(r"(?im)\b(Object)\b([ \t]*)([.])([ \t]*)([a-z]\w*)\b", vec![Some(OPERATOR), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)\b([a-z]\w*)(?:([ \t]*)(@{1,2}|[#$%])|([ \t]*)([.])([ \t]*)(?:([a-z]\w*)))?\b([ \t]*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?im)\b(Function)\b([ \t]+)([a-z]\w*)(?:([ \t]*)(@{1,2}|[#$%])|([ \t]*)([.])([ \t]*)(?:([a-z]\w*)))?", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)\b(Type)([ \t]+)([a-z]\w*)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?im)\b(Pi|True|False|Null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?im)\b(Local|Global|Const|Field|Dim)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?im)\b(Asc|C(?:ase|hr)|D(?:ata|e(?:fault|lete))|E(?:ach|lse(?:(?:If)?)|nd(?:(?:If)?)|xit)|F(?:or(?:(?:ever)?)|unction)|Go(?:sub|to)|I(?:f|n(?:clude|sert))|Len|Ne(?:w|xt)|Re(?:ad|peat|store|turn)|S(?:elect|tep)|T(?:hen|o|ype)|Until|W(?:end|hile))\b", KEYWORD_RESERVED),
        Rule::bygroups(r"(?im)([a-z]\w*)(?:([ \t]*)(@{1,2}|[#$%])|([ \t]*)([.])([ \t]*)(?:([a-z]\w*)))?", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?im)"""#, STRING_DOUBLE),
            Rule::token_to(r#"(?im)"C?"#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?im)[^"\n]+"#, STRING_DOUBLE),
        ],
    );
    Table(m)
}

impl Lexer for BlitzbasicLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
