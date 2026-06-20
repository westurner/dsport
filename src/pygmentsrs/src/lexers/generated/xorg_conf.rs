//! AUTO-GENERATED from `pygments.pygments.lexers.xorg:XorgLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.xorg:XorgLexer:xorg_conf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: xorg.conf
pub struct XorgConfLexer;

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
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::bygroups(
                r#"(?m)((?:Sub)?Section)(\s+)("\w+")"#,
                vec![Some(STRING_ESCAPE), Some(TEXT), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(End(?:Sub)?Section)", STRING_ESCAPE),
            Rule::bygroups(
                r"(?m)(\w+)(\s+)([^\n#]+)",
                vec![Some(NAME_BUILTIN), Some(TEXT), Some(NAME_CONSTANT)],
            ),
        ],
    );
    Table(m)
}

impl Lexer for XorgConfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
