#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.smithy:SmithyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.smithy:SmithyLexer:smithy

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: smithy
pub struct SmithyLexer;

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
        Rule::token(r"(?m)///.*$", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*$", COMMENT),
        Rule::token(r"(?m)@[0-9a-zA-Z\.#-]*", NAME_DECORATOR),
        Rule::token(r"(?m)(=)", NAME_DECORATOR),
        Rule::bygroups(r"(?m)^(\$version)(:)(.+)", vec![Some(KEYWORD_DECLARATION), Some(NAME_DECORATOR), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)^(namespace)(\s+[A-Za-z0-9_\.#$-]+)\b", vec![Some(KEYWORD_DECLARATION), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)^(b(?:ig(?:Decimal|Integer)|lob|oolean|yte)|do(?:cument|uble)|float|integer|long|s(?:hort|tring)|timestamp|use)(\s+[A-Za-z0-9_\.#$-]+)\b", vec![Some(KEYWORD_DECLARATION), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)^(apply|list|map|operation|resource|s(?:e(?:rvice|t)|tructure)|trait|union)(\s+[A-Za-z0-9_\.#$-]+)", vec![Some(KEYWORD_DECLARATION), Some(NAME_CLASS)]),
        Rule::bygroups(r#"(?m)^(metadata)(\s+)((?:\S+)|(?:\"[^"]+\"))(\s*)(=)"#, vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_DECORATOR)]),
        Rule::token(r"(?m)(true|false|null)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)", NUMBER),
        Rule::token(r"(?m)[A-Za-z0-9_\.#$-]+:", NAME_LABEL),
        Rule::token(r"(?m)[A-Za-z0-9_\.#$-]+", NAME_VARIABLE_CLASS),
        Rule::token_to(r"(?m)\[", TEXT, NewState::PushSame),
        Rule::token_to(r"(?m)\]", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?m)\(", TEXT, NewState::PushSame),
        Rule::token_to(r"(?m)\)", TEXT, NewState::Pop(1)),
        Rule::token_to(r"(?m)\{", TEXT, NewState::PushSame),
        Rule::token_to(r"(?m)\}", TEXT, NewState::Pop(1)),
        Rule::token(r#"(?m)"{3}(\\\\|\n|\\")*"{3}"#, STRING_DOC),
        Rule::token(r#"(?m)"(\\\\|\n|\\"|[^"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(\\\\|\n|\\'|[^'])*'", STRING_SINGLE),
        Rule::token(r"(?m)[:,]+", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for SmithyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
