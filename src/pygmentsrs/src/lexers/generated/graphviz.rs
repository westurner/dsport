//! AUTO-GENERATED from `pygments.pygments.lexers.graphviz:GraphvizLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphviz:GraphvizLexer:graphviz

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: graphviz, dot
pub struct GraphvizLexer;

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
            Rule::token(r"(?m)(#|//).*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(
                r"(?m)(?i)(node|edge|graph|digraph|subgraph|strict)\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)--|->", OPERATOR),
            Rule::token(r"(?m)[{}\[\]:;,]", PUNCTUATION),
            Rule::bygroups_to(
                r"(?m)(\b\D\w*)(\s*)(=)(\s*)",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                ],
                NewState::Push(vec![r"attr_id"]),
            ),
            Rule::token(r"(?m)\b(n|ne|e|se|s|sw|w|nw|c|_)\b", NAME_BUILTIN),
            Rule::token(r"(?m)\b\D\w*", NAME_TAG),
            Rule::token(r"(?m)[-]?((\.[0-9]+)|([0-9]+(\.[0-9]*)?))", NUMBER),
            Rule::token(r#"(?m)"(\\"|[^"])*?""#, NAME_TAG),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"xml"])),
        ],
    );
    m.insert(
        r"attr_id",
        vec![
            Rule::token_to(r"(?m)\b\D\w*", STRING, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)[-]?((\.[0-9]+)|([0-9]+(\.[0-9]*)?))",
                NUMBER,
                NewState::Pop(1),
            ),
            Rule::token_to(r#"(?m)"(\\"|[^"])*?""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"#pop", r"xml"])),
        ],
    );
    m.insert(
        r"xml",
        vec![
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[^<>\s]", NAME_TAG),
        ],
    );
    Table(m)
}

impl Lexer for GraphvizLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
