#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.textedit:VimLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.textedit:VimLexer:vim

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vim
pub struct VimLexer;

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
        Rule::bygroups_g(r"(?m)^([ \t:]*)(py(?:t(?:h(?:o(?:n)?)?)?)?)([ \t]*)(<<)([ \t]*)(.*)((?:\n|.)*)(\6)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(OPERATOR)), Some(GroupAction::Token(TEXT)), Some(GroupAction::Token(TEXT)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(TEXT))]),
        Rule::bygroups_g(r"(?m)^([ \t:]*)(py(?:t(?:h(?:o(?:n)?)?)?)?)([ \t])(.*)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(KEYWORD)), Some(GroupAction::Token(TEXT)), Some(GroupAction::UsingLexer { alias: "python", state: None })]),
        Rule::token(r#"(?m)^\s*".*"#, COMMENT),
        Rule::token(r"(?m)[ \t]+", TEXT),
        Rule::token(r"(?m)/[^/\\\n]*(?:\\[\s\S][^/\\\n]*)*/", STRING_REGEX),
        Rule::token(r#"(?m)"[^"\\\n]*(?:\\[\s\S][^"\\\n]*)*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'[^\n']*(?:''[^\n']*)*'", STRING_SINGLE),
        Rule::token(r#"(?m)(?<=\s)"[^\-:.%#=*].*"#, COMMENT),
        Rule::token(r"(?m)-?\d+", NUMBER),
        Rule::token(r"(?m)#[0-9a-f]{6}", NUMBER_HEX),
        Rule::token(r"(?m)^:", PUNCTUATION),
        Rule::token(r"(?m)[()<>+=!|,~-]", PUNCTUATION),
        Rule::token(r"(?m)\b(let|if|else|endif|elseif|fun|function|endfunction)\b", KEYWORD),
        Rule::token(r"(?m)\b(NONE|bold|italic|underline|dark|light)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\b\w+\b", NAME_OTHER),
        Rule::token(r"(?m).", TEXT),
    ]);
    Table(m)
}

impl Lexer for VimLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
