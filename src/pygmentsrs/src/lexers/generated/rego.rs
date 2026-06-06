//! AUTO-GENERATED from `pygments.pygments.lexers.rego:RegoLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rego:RegoLexer:rego

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rego
pub struct RegoLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)(as|contains|d(?:ata|efault)|e(?:lse|very)|false|i(?:mport|[fn])|n(?:ot|ull)|package|some|true|with)\b", KEYWORD),
        Rule::token(r"(?m)(data|input)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_][a-zA-Z0-9_]*", NAME),
        Rule::token(r#"(?m)"(\\\\|\\"|[^"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)`[^`]*`", STRING_BACKTICK),
        Rule::token(r"(?m)-?\d+(\.\d+)?", NUMBER),
        Rule::token(r"(?m)(==|!=|<=|>=|:=)", OPERATOR),
        Rule::token(r"(?m)[=<>+\-*/%&|]", OPERATOR),
        Rule::token(r"(?m)[\[\]{}(),.:;]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for RegoLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
