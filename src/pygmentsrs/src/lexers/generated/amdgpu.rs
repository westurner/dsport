//! AUTO-GENERATED from `pygments.pygments.lexers.amdgpu:AMDGPULexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.amdgpu:AMDGPULexer:amdgpu

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: amdgpu
pub struct AmdgpuLexer;

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
        Rule::token(r"(?im)[\r\n]+", TEXT),
        Rule::token(r"(?im)(([a-z_0-9])*:([a-z_0-9])*)", NAME_ATTRIBUTE),
        Rule::token(r"(?im)(\[|\]|\(|\)|,|\:|\&)", TEXT),
        Rule::token(r"(?im)([;#]|//).*?\n", COMMENT_SINGLE),
        Rule::token(r"(?im)((s_)?(scratch|ds|buffer|flat|image)_[a-z0-9_]+)", KEYWORD_RESERVED),
        Rule::token(r"(?im)(_lo|_hi)", NAME_VARIABLE),
        Rule::token(r"(?im)(vmcnt|lgkmcnt|expcnt)", NAME_ATTRIBUTE),
        Rule::token(r"(?im)(attr[0-9].[a-z])", NAME_ATTRIBUTE),
        Rule::token(r"(?im)(dlc|format|glc|idxen|l(?:ds|it)|o(?:ff(?:(?:en|set)?)|p)|s(?:lc|offset|rsrc)|tfe|unorm|v(?:addr|data))\b", NAME_ATTRIBUTE),
        Rule::token(r"(?im)(label_[a-z0-9]+)", KEYWORD),
        Rule::token(r"(?im)(_L[0-9]*)", NAME_VARIABLE),
        Rule::token(r"(?im)(s|v)_[a-z0-9_]+", KEYWORD),
        Rule::token(r"(?im)(v[0-9.]+|vcc|exec|v)", NAME_VARIABLE),
        Rule::token(r"(?im)s[0-9.]+|s", NAME_VARIABLE),
        Rule::token(r"(?im)[0-9]+\.[^0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)(0[xX][a-z0-9]+)|([0-9]+)", NUMBER_INTEGER),
    ]);
    Table(m)
}

impl Lexer for AmdgpuLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
