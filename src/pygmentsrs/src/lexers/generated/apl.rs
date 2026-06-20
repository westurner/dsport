#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.apl:APLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.apl:APLLexer:apl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: apl
pub struct AplLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[⍝#].*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\'((\'\')|[^\'])*\'", STRING_SINGLE),
        Rule::token(r#"(?m)"(("")|[^"])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)[⋄◇()]", PUNCTUATION),
        Rule::token(r"(?m)[\[\];]", STRING_REGEX),
        Rule::token(r"(?m)⎕[A-Za-zΔ∆⍙][A-Za-zΔ∆⍙_¯0-9]*", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Za-zΔ∆⍙_][A-Za-zΔ∆⍙_¯0-9]*", NAME_VARIABLE),
        Rule::token(r"(?m)¯?(0[Xx][0-9A-Fa-f]+|[0-9]*\.?[0-9]+([Ee][+¯]?[0-9]+)?|¯|∞)([Jj]¯?(0[Xx][0-9A-Fa-f]+|[0-9]*\.?[0-9]+([Ee][+¯]?[0-9]+)?|¯|∞))?", NUMBER),
        Rule::token(r"(?m)[\.\\\/⌿⍀¨⍣⍨⍠⍤∘⌸&⌶@⌺⍥⍛⍢]", NAME_ATTRIBUTE),
        Rule::token(r"(?m)[+\-×÷⌈⌊∣|⍳?*⍟○!⌹<≤=>≥≠≡≢∊⍷∪∩~∨∧⍱⍲⍴,⍪⌽⊖⍉↑↓⊂⊃⌷⍋⍒⊤⊥⍕⍎⊣⊢⍁⍂≈⌸⍯↗⊆⊇⍸√⌾…⍮]", OPERATOR),
        Rule::token(r"(?m)⍬", NAME_CONSTANT),
        Rule::token(r"(?m)[⎕⍞]", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)[←→]", KEYWORD_DECLARATION),
        Rule::token(r"(?m)[⍺⍵⍶⍹∇:]", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)[{}]", KEYWORD_TYPE),
    ]);
    Table(m)
}

impl Lexer for AplLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
