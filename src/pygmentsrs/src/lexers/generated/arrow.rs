//! AUTO-GENERATED from `pygments.pygments.lexers.arrow:ArrowLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.arrow:ArrowLexer:arrow

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: arrow
pub struct ArrowLexer;

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
        Rule::token(r"(?m)^[|\s]+", PUNCTUATION),
        Rule::bygroups_to(r"(?m)(function)(\n+)(/-->)(\s*)\b(int|bool|char)((?:\[\])*)(?=\s+)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)(\()", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"fparams"])),
        Rule::token(r"(?m)/-->$|\\-->$|/--<|\\--<|\^", PUNCTUATION),
        Rule::bygroups(r"(?m)\b(int|bool|char)((?:\[\])*)(?=\s+)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)", vec![Some(KEYWORD_TYPE), Some(PUNCTUATION), Some(TEXT), Some(NAME_VARIABLE)]),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)require|main", KEYWORD_RESERVED),
        Rule::token_to(r"(?m)print", KEYWORD_RESERVED, NewState::Push(vec![r"print"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
        Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
        Rule::bygroups(r"(?m)(input)(\s+)(int|char\[\])", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)]),
        Rule::bygroups_to(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"fargs"])),
        Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"blocks", vec![
        Rule::bygroups_to(r"(?m)(function)(\n+)(/-->)(\s*)\b(int|bool|char)((?:\[\])*)(?=\s+)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)(\()", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION), Some(PUNCTUATION)], NewState::Push(vec![r"fparams"])),
        Rule::token(r"(?m)/-->$|\\-->$|/--<|\\--<|\^", PUNCTUATION),
    ]);
    m.insert(
        r"statements",
        vec![
            Rule::bygroups(
                r"(?m)\b(int|bool|char)((?:\[\])*)(?=\s+)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)",
                vec![
                    Some(KEYWORD_TYPE),
                    Some(PUNCTUATION),
                    Some(TEXT),
                    Some(NAME_VARIABLE),
                ],
            ),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token(r"(?m)=", OPERATOR),
            Rule::token(r"(?m)require|main", KEYWORD_RESERVED),
            Rule::token_to(
                r"(?m)print",
                KEYWORD_RESERVED,
                NewState::Push(vec![r"print"]),
            ),
        ],
    );
    m.insert(
        r"expressions",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
            Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
            Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
            Rule::bygroups(
                r"(?m)(input)(\s+)(int|char\[\])",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
                NewState::Push(vec![r"fargs"]),
            ),
            Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"print",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
            Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
            Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
            Rule::bygroups(
                r"(?m)(input)(\s+)(int|char\[\])",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
                NewState::Push(vec![r"fargs"]),
            ),
            Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fparams",
        vec![
            Rule::bygroups(
                r"(?m)\b(int|bool|char)((?:\[\])*)(?=\s+)(\s+)([a-zA-Z_][a-zA-Z0-9_]*)",
                vec![
                    Some(KEYWORD_TYPE),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(NAME_VARIABLE),
                ],
            ),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"escape",
        vec![Rule::token(
            r#"(?m)\\(["\\/abfnrtv]|[0-9]{1,3}|x[0-9a-fA-F]{2}|u[0-9a-fA-F]{4})"#,
            STRING_ESCAPE,
        )],
    );
    m.insert(
        r"char",
        vec![
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\(["\\/abfnrtv]|[0-9]{1,3}|x[0-9a-fA-F]{2}|u[0-9a-fA-F]{4})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)[^'\\]", STRING_CHAR),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\(["\\/abfnrtv]|[0-9]{1,3}|x[0-9a-fA-F]{2}|u[0-9a-fA-F]{4})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"array",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
            Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
            Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
            Rule::bygroups(
                r"(?m)(input)(\s+)(int|char\[\])",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
                NewState::Push(vec![r"fargs"]),
            ),
            Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"fargs",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
            Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
            Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
            Rule::bygroups(
                r"(?m)(input)(\s+)(int|char\[\])",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
                NewState::Push(vec![r"fargs"]),
            ),
            Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"index",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
            Rule::token(r"(?m)true|false", KEYWORD_CONSTANT),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"char"])),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"array"])),
            Rule::token(r"(?m)==|!=|<|>|\+|-|\*|/|%", OPERATOR),
            Rule::token(r"(?m)and|or|not|length", OPERATOR_WORD),
            Rule::bygroups(
                r"(?m)(input)(\s+)(int|char\[\])",
                vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_TYPE)],
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)(\()",
                vec![Some(NAME_FUNCTION), Some(PUNCTUATION)],
                NewState::Push(vec![r"fargs"]),
            ),
            Rule::token(r"(?m)([a-zA-Z_][a-zA-Z0-9_]*)", NAME_VARIABLE),
            Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"index"])),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expressions"])),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for ArrowLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
