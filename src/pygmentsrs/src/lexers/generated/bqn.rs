//! AUTO-GENERATED from `pygments.pygments.lexers.bqn:BQNLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.bqn:BQNLexer:bqn

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bqn
pub struct BqnLexer;

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
        r"root",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token(r"(?m)\'((\'\')|[^\'])*\'", STRING_SINGLE),
            Rule::token(r#"(?m)"(("")|[^"])*""#, STRING_DOUBLE),
            Rule::token(r"(?m)@", STRING_SYMBOL),
            Rule::token(r"(?m)[\.⋄,\[\]⟨⟩‿]", PUNCTUATION),
            Rule::token(r"(?m)[\(\)]", STRING_REGEX),
            Rule::token(
                r"(?m)¯?[0-9](([0-9]|_)*\.?([0-9]|_)+|([0-9]|_)*)([Ee][¯]?([0-9]|_)+)?|¯|∞|π|·",
                NUMBER,
            ),
            Rule::token(r"(?m)[a-z]((?=[^𝕎𝕏𝔽𝔾𝕊𝕨𝕩𝕗𝕘𝕤𝕣])\w)*", NAME_VARIABLE),
            Rule::token(r"(?m)[∘○⊸⟜⌾⊘◶⎉⚇⍟⎊]", NAME_PROPERTY),
            Rule::token(r"(?m)_(𝕣|[a-zA-Z0-9]+)_", NAME_PROPERTY),
            Rule::token(r"(?m)[˙˜˘¨⌜⁼´˝`𝕣]", NAME_ATTRIBUTE),
            Rule::token(r"(?m)_(𝕣|[a-zA-Z0-9]+)", NAME_ATTRIBUTE),
            Rule::token(
                r"(?m)[+\-×÷\⋆√⌊⌈∧∨¬|≤<>≥=≠≡≢⊣⊢⥊∾≍⋈↑↓↕«»⌽⍉/⍋⍒⊏⊑⊐⊒∊⍷⊔!𝕎𝕏𝔽𝔾𝕊]",
                OPERATOR,
            ),
            Rule::token(
                r"(?m)[A-Z]((?=[^𝕎𝕏𝔽𝔾𝕊𝕨𝕩𝕗𝕘𝕤𝕣])\w)*|•((?=[^𝕎𝕏𝔽𝔾𝕊𝕨𝕩𝕗𝕘𝕤𝕣])\w)+",
                OPERATOR,
            ),
            Rule::token(r"(?m)˙", NAME_CONSTANT),
            Rule::token(r"(?m)[←↩⇐]", KEYWORD_DECLARATION),
            Rule::token(r"(?m)[{}]", KEYWORD_TYPE),
            Rule::token(r"(?m)[;:?𝕨𝕩𝕗𝕘𝕤]", NAME_ENTITY),
        ],
    );
    Table(m)
}

impl Lexer for BqnLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
