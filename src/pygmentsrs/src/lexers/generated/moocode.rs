#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:MOOCodeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:MOOCodeLexer:moocode

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: moocode, moo
pub struct MoocodeLexer;

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
        Rule::token(r"(?m)(0|[1-9][0-9_]*)", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(E_PERM|E_DIV)", NAME_EXCEPTION),
        Rule::token(r"(?m)((#[-0-9]+)|(\$\w+))", NAME_ENTITY),
        Rule::token(r"(?m)\b(if|else|elseif|endif|for|endfor|fork|endfork|while|endwhile|break|continue|return|try|except|endtry|finally|in)\b", KEYWORD),
        Rule::token(r"(?m)(random|length)", NAME_BUILTIN),
        Rule::token(r"(?m)(player|caller|this|args)", NAME_VARIABLE_INSTANCE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)([!;=,{}&|:.\[\]@()<>?]+)", OPERATOR),
        Rule::bygroups(r"(?m)(\w+)(\()", vec![Some(NAME_FUNCTION), Some(OPERATOR)]),
        Rule::token(r"(?m)(\w+)", TEXT),
    ]);
    Table(m)
}

impl Lexer for MoocodeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
