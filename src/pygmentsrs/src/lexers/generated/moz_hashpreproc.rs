//! AUTO-GENERATED from `pygments.pygments.lexers.markup:MozPreprocHashLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.markup:MozPreprocHashLexer:moz_hashpreproc

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mozhashpreproc
pub struct MozHashpreprocLexer;

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
            Rule::token_to(
                r"(?m)^#",
                COMMENT_PREPROC,
                NewState::Push(vec![r"expr", r"exprstart"]),
            ),
            Rule::token(r"(?m).+", OTHER),
        ],
    );
    m.insert(r"exprstart", vec![
        Rule::bygroups_to(r"(?m)(literal)(.*)", vec![Some(COMMENT_PREPROC), Some(TEXT)], NewState::Pop(2)),
        Rule::token_to(r"(?m)(define|e(?:l(?:if(?:(?:(?:(?:n)?)def)?)|se)|ndif|rror|xpand)|filter|i(?:f(?:(?:(?:(?:n)?)def)?)|nclude(?:(?:subst)?))|un(?:def|filter))", COMMENT_PREPROC, NewState::Pop(1)),
    ]);
    m.insert(
        r"expr",
        vec![
            Rule::token(r"(?m)(!(?:(?:=)?)|\&\&|==|\|\|)", OPERATOR),
            Rule::bygroups(r"(?m)(defined)(\()", vec![Some(KEYWORD), Some(PUNCTUATION)]),
            Rule::token(r"(?m)\)", PUNCTUATION),
            Rule::token(
                r"(?m)[0-9]+",
                TokenType::new(&["Literal", "Number", "Decimal"]),
            ),
            Rule::token(r"(?m)__\w+?__", NAME_VARIABLE),
            Rule::token(r"(?m)@\w+?@", NAME_CLASS),
            Rule::token(r"(?m)\w+", NAME),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)\S", PUNCTUATION),
        ],
    );
    Table(m)
}

impl Lexer for MozHashpreprocLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
