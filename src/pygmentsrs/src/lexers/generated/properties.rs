//! AUTO-GENERATED from `pygments.pygments.lexers.configs:PropertiesLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:PropertiesLexer:properties

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: properties, jproperties
pub struct PropertiesLexer;

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
            Rule::token(r"(?m)[!#].*|/{2}.*", COMMENT_SINGLE),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)^[^\S\n]+", WHITESPACE),
            Rule::default(NewState::Push(vec![r"key"])),
        ],
    );
    m.insert(
        r"key",
        vec![
            Rule::token(r"(?m)[^\\:=\s]+", NAME_ATTRIBUTE),
            Rule::bygroups(
                r"(?m)(\\\n)([^\S\n]*)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\\(.|\n)", STRING_ESCAPE),
            Rule::bygroups_to(
                r"(?m)([^\S\n]*)([:=])([^\S\n]*)",
                vec![Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)],
                NewState::Push(vec![r"#pop", r"value"]),
            ),
            Rule::token_to(
                r"(?m)[^\S\n]+",
                WHITESPACE,
                NewState::Push(vec![r"#pop", r"value"]),
            ),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"escapes",
        vec![
            Rule::bygroups(
                r"(?m)(\\\n)([^\S\n]*)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\\(.|\n)", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"value",
        vec![
            Rule::token(r"(?m)[^\\\n]+", STRING),
            Rule::bygroups(
                r"(?m)(\\\n)([^\S\n]*)",
                vec![Some(STRING_ESCAPE), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\\(.|\n)", STRING_ESCAPE),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for PropertiesLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
