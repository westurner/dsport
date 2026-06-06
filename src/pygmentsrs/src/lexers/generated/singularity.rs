//! AUTO-GENERATED from `pygments.pygments.lexers.configs:SingularityLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:SingularityLexer:singularity

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: singularity
pub struct SingularityLexer;

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
        Rule::bygroups_to(r"(?ims)^(%(?:pre|post|setup|environment|help|labels|test|runscript|files|startscript))(\s*)", vec![Some(GENERIC_HEADING), Some(WHITESPACE)], NewState::Push(vec![r"script"])),
        Rule::bygroups_to(r"(?ims)^(%app(?:install|help|run|labels|env|test|files))(\s*)", vec![Some(GENERIC_HEADING), Some(WHITESPACE)], NewState::Push(vec![r"script"])),
        Rule::bygroups(r"(?ims)^(\s*)(bootstrap|from|osversion|mirrorurl|include|registry|namespace|includecmd)(:)", vec![Some(WHITESPACE), Some(KEYWORD), Some(TEXT)]),
        Rule::token(r"(?ims)\s*#.*?\n", COMMENT),
        Rule::token(r"(?ims)\b(([0-9]+\.?[0-9]*)|(\.[0-9]+))\b", NUMBER),
        Rule::token(r"(?ims)[ \t]+", WHITESPACE),
        Rule::token(r"(?ims)(?!^\s*%).", TEXT),
    ]);
    m.insert(r"script", vec![
        Rule::using_lexer_to(r"(?ims)(.+?(?=^\s*%))|(.*)", "bash", None, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for SingularityLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
