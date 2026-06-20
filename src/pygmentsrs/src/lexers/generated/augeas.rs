//! AUTO-GENERATED from `pygments.pygments.lexers.configs:AugeasLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:AugeasLexer:augeas

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: augeas
pub struct AugeasLexer;

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
        Rule::bygroups(r"(?m)(module)(\s*)([^\s=]+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)(let)(\s*)([^\s=]+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)(del|store|value|counter|seq|key|label|autoload|incl|excl|transform|test|get|put)(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\()([^:]+)(\:)(unit|string|regexp|lens|tree|filter)(\))", vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION), Some(KEYWORD_TYPE), Some(PUNCTUATION)]),
        Rule::token_to(r"(?m)\(\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[*+\-.;=?|]", OPERATOR),
        Rule::token(r"(?m)[()\[\]{}]", OPERATOR),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)\/", STRING_REGEX, NewState::Push(vec![r"regex"])),
        Rule::bygroups(r"(?m)([A-Z]\w*)(\.)(\w+)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r"(?m).", NAME_VARIABLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^"]"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"regex",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[^/]", STRING_REGEX),
            Rule::token_to(r"(?m)\/", STRING_REGEX, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*)]", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)\(\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*\)", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[)*]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for AugeasLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
