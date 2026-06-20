//! AUTO-GENERATED from `pygments.pygments.lexers.graphql:GraphQLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphql:GraphQLLexer:graphql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: graphql
pub struct GraphqlLexer;

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
        r"ignored_tokens",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"value",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)-?\d+(?![.eE])", NUMBER_INTEGER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)-?\d+(\.\d+)?([eE][+-]?\d+)?",
                NUMBER_FLOAT,
                NewState::Pop(1),
            ),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"#pop", r"string"])),
            Rule::token_to(r"(?m)(false|null|true)\b", NAME_BUILTIN, NewState::Pop(1)),
            Rule::token_to(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CONSTANT, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"list_value"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"object_value"]),
            ),
        ],
    );
    m.insert(
        r"list_value",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)]", PUNCTUATION, NewState::Pop(1)),
            Rule::default(NewState::Push(vec![r"value"])),
        ],
    );
    m.insert(
        r"object_value",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"value"])),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)\\(["\\/bfnrt]|u[a-fA-F0-9]{4})"#, STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)(mutation|query|subscription)\b", KEYWORD, NewState::Push(vec![r"operation"])),
        Rule::token(r"(?m)(ARGUMENT_DEFINITION|ENUM(?:(?:_VALUE)?)|F(?:IELD(?:(?:_DEFINITION)?)|RAGMENT_(?:DEFINITION|SPREAD))|IN(?:LINE_FRAGMENT|PUT_(?:FIELD_DEFINITION|OBJECT)|TERFACE)|MUTATION|OBJECT|QUERY|S(?:C(?:ALAR|HEMA)|UBSCRIPTION)|UNION|directive|e(?:num|xtend)|i(?:mplements|n(?:put|terface))|sc(?:alar|hema)|type|union)\b", KEYWORD),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"selection_set"])),
        Rule::token_to(r"(?m)fragment\b", KEYWORD, NewState::Push(vec![r"fragment_definition"])),
    ]);
    m.insert(
        r"operation",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"variable_definition"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"selection_set"]),
            ),
        ],
    );
    m.insert(
        r"variable_definition",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)[\]!]", PUNCTUATION),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"type"])),
            Rule::token_to(r"(?m)=", PUNCTUATION, NewState::Push(vec![r"value"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\[", PUNCTUATION),
            Rule::token_to(
                r"(?m)(Boolean|Float|I(?:D|nt)|String)\b",
                NAME_BUILTIN,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"selection_set",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::bygroups(
                r"(?m)([a-zA-Z_]\w*)(\s*)(:)",
                vec![Some(NAME_LABEL), Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
            Rule::bygroups_to(
                r"(?m)(\.\.\.)(\s+)(on)\b",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD)],
                NewState::Push(vec![r"inline_fragment"]),
            ),
            Rule::token_to(
                r"(?m)\.\.\.",
                PUNCTUATION,
                NewState::Push(vec![r"fragment_spread"]),
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"arguments"])),
            Rule::token_to(
                r"(?m)@[a-zA-Z_]\w*",
                NAME_DECORATOR,
                NewState::Push(vec![r"directive"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"selection_set"]),
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"directive",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"arguments"]),
            ),
        ],
    );
    m.insert(
        r"arguments",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"value"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fragment_definition",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[\]!]", PUNCTUATION),
            Rule::token_to(r"(?m)on\b", KEYWORD, NewState::Push(vec![r"type"])),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION),
            Rule::token_to(
                r"(?m)@[a-zA-Z_]\w*",
                NAME_DECORATOR,
                NewState::Push(vec![r"directive"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"selection_set"]),
            ),
        ],
    );
    m.insert(
        r"fragment_spread",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(
                r"(?m)@[a-zA-Z_]\w*",
                NAME_DECORATOR,
                NewState::Push(vec![r"directive"]),
            ),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"inline_fragment",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#.*$", COMMENT),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
            Rule::token_to(
                r"(?m)@[a-zA-Z_]\w*",
                NAME_DECORATOR,
                NewState::Push(vec![r"directive"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"selection_set"]),
            ),
        ],
    );
    Table(m)
}

impl Lexer for GraphqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
