//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:AlloyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:AlloyLexer:alloy

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: alloy
pub struct AlloyLexer;

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
        r"sig",
        vec![
            Rule::token_to(r"(?ms)(extends)\b", KEYWORD, NewState::Pop(1)),
            Rule::token(r#"(?ms)[a-zA-Z_][\w]*"*"#, NAME),
            Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
            Rule::token(r"(?ms),", PUNCTUATION),
            Rule::token_to(r"(?ms)\{", OPERATOR, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"module",
        vec![
            Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
            Rule::token_to(r#"(?ms)[a-zA-Z_][\w]*"*"#, NAME, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fun",
        vec![
            Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
            Rule::token_to(r"(?ms)\{", OPERATOR, NewState::Pop(1)),
            Rule::token_to(r#"(?ms)[a-zA-Z_][\w]*"*"#, NAME, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fact",
        vec![
            Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
            Rule::token_to(r"(?ms)\{", OPERATOR, NewState::Pop(1)),
            Rule::token_to(r#"(?ms)[a-zA-Z_][\w]*"*"#, NAME, NewState::Pop(1)),
            Rule::token_to(
                r#"(?ms)"\b(\\\\|\\[^\\]|[^"\\])*""#,
                STRING,
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?ms)--.*?$", COMMENT_SINGLE),
            Rule::token(r"(?ms)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
            Rule::bygroups_to(
                r"(?ms)(module|open)(\s+)",
                vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)],
                NewState::Push(vec![r"module"]),
            ),
            Rule::bygroups_to(
                r"(?ms)(sig|enum)(\s+)",
                vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)],
                NewState::Push(vec![r"sig"]),
            ),
            Rule::token(r"(?ms)(iden|univ|none)\b", KEYWORD_CONSTANT),
            Rule::token(r"(?ms)(int|Int)\b", KEYWORD_TYPE),
            Rule::token(
                r"(?ms)(var|this|abstract|extends|set|seq|one|lone|let)\b",
                KEYWORD,
            ),
            Rule::token(r"(?ms)(all|some|no|sum|disj|when|else)\b", KEYWORD),
            Rule::token(
                r"(?ms)(run|check|for|but|exactly|expect|as|steps)\b",
                KEYWORD,
            ),
            Rule::token(r"(?ms)(always|after|eventually|until|release)\b", KEYWORD),
            Rule::token(
                r"(?ms)(historically|before|once|since|triggered)\b",
                KEYWORD,
            ),
            Rule::token(r"(?ms)(and|or|implies|iff|in)\b", OPERATOR_WORD),
            Rule::bygroups_to(
                r"(?ms)(fun|pred|assert)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"fun"]),
            ),
            Rule::bygroups_to(
                r"(?ms)(fact)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"fact"]),
            ),
            Rule::token(r"(?ms)!|#|&&|\+\+|<<|>>|>=|<=>|<=|\.\.|\.|->", OPERATOR),
            Rule::token(r"(?ms)[-+/*%=<>&!^|~{}\[\]().\';]", OPERATOR),
            Rule::token(r#"(?ms)[a-zA-Z_][\w]*"*"#, NAME),
            Rule::token(r"(?ms)[:,]", PUNCTUATION),
            Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
            Rule::token(r#"(?ms)"\b(\\\\|\\[^\\]|[^"\\])*""#, STRING),
            Rule::token(r"(?ms)\n", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for AlloyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
