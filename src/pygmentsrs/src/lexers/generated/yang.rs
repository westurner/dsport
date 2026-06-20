#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.yang:YangLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.yang:YangLexer:yang

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: yang
pub struct YangLexer;

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
        r"comments",
        vec![
            Rule::token(r"(?m)[^*/]", COMMENT),
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[{};]+", PUNCTUATION),
        Rule::token(r"(?m)(?<![\-\w])(and|or|not|\+|\.)(?![\-\w])", OPERATOR),
        Rule::token(r#"(?m)"(?:\\"|[^"])*?""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(?:\\'|[^'])*?'", STRING_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"comments"])),
        Rule::token(r"(?m)//.*?$", COMMENT),
        Rule::bygroups(r"(?m)(?:^|(?<=[\s{};]))([\w.-]+)(:)([\w.-]+)(?=[\s{};])", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r"(?m)([0-9]{4}\-[0-9]{2}\-[0-9]{2})(?=[\s{};])", NAME_LABEL),
        Rule::token(r"(?m)([0-9]+\.[0-9]+)(?=[\s{};])", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]+)(?=[\s{};])", NUMBER_INTEGER),
        Rule::token(r"(?m)((?:(?:sub)?)module)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(belongs\-to|namespace|prefix|yang\-version)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(contact|description|organization|re(?:ference|vision))(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(i(?:mport|nclude)|revision\-date)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(a(?:ction|(?:rgu|ug)ment)|deviation|extension|feature|grouping|i(?:dentity|f\-feature|nput)|notification|output|rpc|typedef)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(any(?:data|xml)|c(?:ase|hoice|on(?:fig|tainer))|deviate|l(?:eaf(?:(?:\-list)?)|ist)|must|presence|refine|uses|when)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(b(?:ase|it)|default|e(?:num|rror\-(?:app\-tag|message))|fraction\-digits|length|m(?:ax\-elements|in\-elements|odifier)|ordered\-by|p(?:at(?:h|tern)|osition)|r(?:(?:ang|equire\-instanc)e)|status|type|units|value|yin\-element)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(key|mandatory|unique)(?=[^\w\-:])", KEYWORD),
        Rule::token(r"(?m)(b(?:i(?:nary|ts)|oolean)|decimal64|e(?:mpty|numeration)|i(?:dentityref|n(?:stance\-identifier|t(?:16|32|64|8)))|leafref|string|u(?:int(?:16|32|64|8)|nion))(?=[^\w\-:])", NAME_CLASS),
        Rule::token(r"(?m)(add|current|de(?:lete|precated)|false|invert\-match|m(?:ax|in)|not\-supported|obsolete|replace|true|u(?:nbounded|ser))(?=[^\w\-:])", NAME_CLASS),
        Rule::token(r#"(?m)[^;{}\s\'"]+"#, NAME_VARIABLE),
    ]);
    Table(m)
}

impl Lexer for YangLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
