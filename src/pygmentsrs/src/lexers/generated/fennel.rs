#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:FennelLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:FennelLexer:fennel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fennel, fnl
pub struct FennelLexer;

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
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m),+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(true|false|nil)", NAME_CONSTANT),
        Rule::token(r"(?m):[a-zA-Z_!$%&*+/:<=>?^~|-][\w!$%&*+/:<=>?^~|\.-]*", STRING_SYMBOL),
        Rule::token(r"(?m)(\-(?:(?:(?:\?>|[>?])?)>)|\.\.|//|<=|>=|\?\.|a(?:ccumulate|nd|ssert\-repl)|b(?:and|not|(?:(?:x)?)or)|c(?:ase(?:(?:\-try)?)|o(?:(?:llec|mmen)t))|do(?:(?:to)?)|e(?:ach|val\-compiler)|f(?:accumulate|collect|or)|hashfn|i(?:collect|f|mport\-macros|nclude)|l(?:e(?:ngth|t)|shift|ua)|ma(?:crodebug|tch(?:(?:\-try)?))|not(?:(?:=)?)|or|p(?:artial|ick\-(?:(?:arg|value)s))|quote|r(?:equire\-macros|shift)|set(?:(?:\-forcibly!)?)|t(?:ail!|set)|unquote|values|w(?:h(?:en|ile)|ith\-open)|\~=|[#%*+\-./:<=>\^]) ", KEYWORD),
        Rule::token(r"(?m)(fn|global|l(?:ambda|ocal)|macro(?:(?:s)?)|var|λ) ", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(_(?:G|VERSION)|a(?:rg|ssert)|bit32|co(?:(?:llectgarbag|routin)e)|d(?:ebug|ofile)|error|get(?:fenv|metatable)|i(?:o|pairs)|load(?:(?:file|string)?)|math|next|os|p(?:a(?:ckage|irs)|call|rint)|r(?:aw(?:equal|get|len|set)|equire)|s(?:e(?:lect|t(?:fenv|metatable))|tring)|t(?:able|o(?:number|string)|ype)|unpack|xpcall) ", NAME_BUILTIN),
        Rule::token(r"(?m)\.\.\.", NAME_VARIABLE),
        Rule::token(r"(?m)[a-zA-Z_!$%&*+/:<=>?^~|-][\w!$%&*+/:<=>?^~|\.-]*", NAME_VARIABLE),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
        Rule::token(r"(?m)(\[|\])", PUNCTUATION),
        Rule::token(r"(?m)(\{|\})", PUNCTUATION),
        Rule::token(r"(?m)#", PUNCTUATION),
        Rule::token(r"(?m)`", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for FennelLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
