//! AUTO-GENERATED from `pygments.pygments.lexers.csound:CsoundDocumentLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.csound:CsoundDocumentLexer:csound_document

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: csound-document, csound-csd
pub struct CsoundDocumentLexer;

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
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:;|//).*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[^/;<]+|/(?!/)", TEXT),
        Rule::token_to(r"(?m)<\s*CsInstruments", NAME_TAG, NewState::Push(vec![r"orchestra", r"tag"])),
        Rule::token_to(r"(?m)<\s*CsScore", NAME_TAG, NewState::Push(vec![r"score", r"tag"])),
        Rule::token_to(r"(?m)<\s*[Hh][Tt][Mm][Ll]", NAME_TAG, NewState::Push(vec![r"HTML", r"tag"])),
        Rule::token_to(r"(?m)<\s*[\w:.-]+", NAME_TAG, NewState::Push(vec![r"tag"])),
        Rule::token(r"(?m)<\s*/\s*[\w:.-]+\s*>", NAME_TAG),
    ]);
    m.insert(r"orchestra", vec![
        Rule::token_to(r"(?m)<\s*/\s*CsInstruments\s*>", NAME_TAG, NewState::Pop(1)),
        Rule::using_lexer(r"(?m)(.|\n)+?(?=<\s*/\s*CsInstruments\s*>)", "csound", None),
    ]);
    m.insert(r"score", vec![
        Rule::token_to(r"(?m)<\s*/\s*CsScore\s*>", NAME_TAG, NewState::Pop(1)),
        Rule::using_lexer(r"(?m)(.|\n)+?(?=<\s*/\s*CsScore\s*>)", "csound-score", None),
    ]);
    m.insert(r"HTML", vec![
        Rule::token_to(r"(?m)<\s*/\s*[Hh][Tt][Mm][Ll]\s*>", NAME_TAG, NewState::Pop(1)),
        Rule::using_lexer(r"(?m)(.|\n)+?(?=<\s*/\s*[Hh][Tt][Mm][Ll]\s*>)", "html", None),
    ]);
    m.insert(r"tag", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)[\w.:-]+\s*=", NAME_ATTRIBUTE, NewState::Push(vec![r"attr"])),
        Rule::token_to(r"(?m)/?\s*>", NAME_TAG, NewState::Pop(1)),
    ]);
    m.insert(r"attr", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)".*?""#, STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)'.*?'", STRING, NewState::Pop(1)),
        Rule::token_to(r"(?m)[^\s>]+", STRING, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for CsoundDocumentLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
