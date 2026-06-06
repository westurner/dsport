//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:VGLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:VGLLexer:vgl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vgl
pub struct VglLexer;

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
        Rule::token(r"(?ims)\{[^}]*\}", COMMENT_MULTILINE),
        Rule::token(r"(?ims)declare", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)(if|then|else|endif|while|do|endwhile|and|or|prompt|object|create|on|line|with|global|routine|value|endroutine|constant|global|set|join|library|compile_option|file|exists|create|copy|delete|enable|windows|name|notprotected)(?! *[=<>.,()])", KEYWORD),
        Rule::token(r"(?ims)(true|false|null|empty|error|locked)", KEYWORD_CONSTANT),
        Rule::token(r"(?ims)[~^*#!%&\[\]()<>|+=:;,./?-]", OPERATOR),
        Rule::token(r#"(?ims)"[^"]*""#, STRING),
        Rule::bygroups(r"(?ims)(\.)([a-z_$][\w$]*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ims)[0-9][0-9]*(\.[0-9]+(e[+\-]?[0-9]+)?)?", NUMBER),
        Rule::token(r"(?ims)[a-z_$][\w$]*", NAME),
        Rule::token(r"(?ims)[\r\n]+", WHITESPACE),
        Rule::token(r"(?ims)\s+", WHITESPACE),
    ]);
    Table(m)
}

impl Lexer for VglLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
