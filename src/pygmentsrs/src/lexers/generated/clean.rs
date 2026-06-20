//! AUTO-GENERATED from `pygments.pygments.lexers.clean:CleanLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.clean:CleanLexer:clean

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: clean
pub struct CleanLexer;

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
        Rule::token(r"(?m)//.*\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments.in"])),
        Rule::token_to(r"(?m)/\*\*", COMMENT_SPECIAL, NewState::Push(vec![r"comments.in"])),
        Rule::token(r"(?m)\b(c(?:ase|call|lass|ode(?:(?:\ inline)?))|derive|export|foreign|generic|i(?:n(?:fix(?:(?:[lr])?)|stance)|[fn])|let|o(?:f|therwise)|s(?:(?:pecia|tdcal)l)|w(?:here|ith))\b", KEYWORD),
        Rule::token(r"(?m)\b(definition|implementation|system)\b", KEYWORD_NAMESPACE),
        Rule::token_to(r"(?m)\bmodule\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"module.name"])),
        Rule::bygroups_to(r"(?m)\b(import)\b(\s*)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"import.module"])),
        Rule::bygroups_to(r"(?m)\b(from)\b(\s*)\b([a-zA-Z_][a-zA-Z0-9_.`]+)\b(\s*)\b(import)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"import.what"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\'([^\'\\]|\\(x[\da-fA-F]+|\d+|.))\'", TokenType::new(&["Literal", "Char"])),
        Rule::token(r"(?m)[+~-]?0[0-7]+\b", NUMBER_OCT),
        Rule::token(r"(?m)[+~-]?\d+\.\d+(E[+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+~-]?\d+\b", NUMBER_INTEGER),
        Rule::token(r"(?m)[+~-]?0x[\da-fA-F]+\b", NUMBER_HEX),
        Rule::token(r"(?m)True|False", LITERAL),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"literals.stringd"])),
        Rule::token(r"(?m)[-~@#$%\^?!+*<>\\/|&=:.]+", OPERATOR),
        Rule::token(r"(?m)\b_+\b", OPERATOR),
        Rule::token(r"(?m)[,;(){}\[\]]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\')([\w`.]+)(\')", vec![Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-z`][\w`]*", NAME),
        Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
        Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
        Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
    ]);
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*\n", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments.in"]),
            ),
            Rule::token_to(
                r"(?m)/\*\*",
                COMMENT_SPECIAL,
                NewState::Push(vec![r"comments.in"]),
            ),
        ],
    );
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)\b(c(?:ase|call|lass|ode(?:(?:\ inline)?))|derive|export|foreign|generic|i(?:n(?:fix(?:(?:[lr])?)|stance)|[fn])|let|o(?:f|therwise)|s(?:(?:pecia|tdcal)l)|w(?:here|ith))\b", KEYWORD),
    ]);
    m.insert(
        r"module",
        vec![
            Rule::token(
                r"(?m)\b(definition|implementation|system)\b",
                KEYWORD_NAMESPACE,
            ),
            Rule::token_to(
                r"(?m)\bmodule\b",
                KEYWORD_NAMESPACE,
                NewState::Push(vec![r"module.name"]),
            ),
        ],
    );
    m.insert(
        r"import",
        vec![
            Rule::bygroups_to(
                r"(?m)\b(import)\b(\s*)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"import.module"]),
            ),
            Rule::bygroups_to(
                r"(?m)\b(from)\b(\s*)\b([a-zA-Z_][a-zA-Z0-9_.`]+)\b(\s*)\b(import)\b",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_CLASS),
                    Some(WHITESPACE),
                    Some(KEYWORD),
                ],
                NewState::Push(vec![r"import.what"]),
            ),
        ],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", WHITESPACE)]);
    m.insert(
        r"literals",
        vec![
            Rule::token(
                r"(?m)\'([^\'\\]|\\(x[\da-fA-F]+|\d+|.))\'",
                TokenType::new(&["Literal", "Char"]),
            ),
            Rule::token(r"(?m)[+~-]?0[0-7]+\b", NUMBER_OCT),
            Rule::token(r"(?m)[+~-]?\d+\.\d+(E[+-]?\d+)?", NUMBER_FLOAT),
            Rule::token(r"(?m)[+~-]?\d+\b", NUMBER_INTEGER),
            Rule::token(r"(?m)[+~-]?0x[\da-fA-F]+\b", NUMBER_HEX),
            Rule::token(r"(?m)True|False", LITERAL),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"literals.stringd"]),
            ),
        ],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?m)[-~@#$%\^?!+*<>\\/|&=:.]+", OPERATOR),
            Rule::token(r"(?m)\b_+\b", OPERATOR),
        ],
    );
    m.insert(
        r"delimiters",
        vec![
            Rule::token(r"(?m)[,;(){}\[\]]", PUNCTUATION),
            Rule::bygroups(
                r"(?m)(\')([\w`.]+)(\')",
                vec![Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION)],
            ),
        ],
    );
    m.insert(
        r"names",
        vec![
            Rule::token(r"(?m)[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
            Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
        ],
    );
    m.insert(
        r"comments.in",
        vec![
            Rule::token_to(r"(?m)\*\/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token(r"(?m)\*(?!/)", COMMENT_MULTILINE),
            Rule::token(r"(?m)/", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"module.name",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)[a-zA-Z_][a-zA-Z0-9_.`]+",
                NAME_CLASS,
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"import.module",
        vec![
            Rule::bygroups(
                r"(?m)\b(qualified)\b(\s*)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
            ),
            Rule::bygroups_to(
                r"(?m)(\s*)\b(as)\b",
                vec![Some(WHITESPACE), Some(KEYWORD)],
                NewState::Push(vec![r"#pop", r"import.module.as"]),
            ),
            Rule::token(r"(?m)[a-zA-Z_][a-zA-Z0-9_.`]+", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\s*)(,)(\s*)",
                vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"import.module.as",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)[a-z`][\w`]*", NAME_CLASS, NewState::Pop(1)),
            Rule::token_to(r"(?m)[A-Z`][\w`]*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"import.what",
        vec![
            Rule::bygroups_to(
                r"(?m)\b(class)\b(\s+)([a-z`][\w`]*|[A-Z`][\w`]*|[~@#$%\^?!+\-*<>\\/|&=:]+)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)],
                NewState::Push(vec![r"import.what.class"]),
            ),
            Rule::bygroups_to(
                r"(?m)\b(instance)(\s+)([a-z`][\w`]*|[A-Z`][\w`]*|[~@#$%\^?!+\-*<>\\/|&=:]+)(\s+)",
                vec![
                    Some(KEYWORD),
                    Some(WHITESPACE),
                    Some(NAME_CLASS),
                    Some(WHITESPACE),
                ],
                NewState::Push(vec![r"import.what.instance"]),
            ),
            Rule::bygroups_to(
                r"(?m)(::)(\s*)\b([A-Z`][\w`]*)\b",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)],
                NewState::Push(vec![r"import.what.type"]),
            ),
            Rule::bygroups(
                r"(?m)\b(generic)\b(\s+)\b([a-z`][\w`]*|[A-Z`][\w`]*)\b",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME)],
            ),
            Rule::token(r"(?m)[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
            Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
            Rule::bygroups(r"(?m)(,)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)]),
            Rule::token_to(r"(?m)$", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"import.what.class",
        vec![
            Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"import.what.class.members"]),
            ),
            Rule::token_to(r"(?m)$", WHITESPACE, NewState::Pop(2)),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"import.what.class.members",
        vec![
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\.\.", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
            Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
        ],
    );
    m.insert(
        r"import.what.instance",
        vec![
            Rule::token_to(r"(?m)[,)]", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"import.what.instance"]),
            ),
            Rule::token_to(r"(?m)$", WHITESPACE, NewState::Pop(2)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
            Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
        ],
    );
    m.insert(
        r"import.what.type",
        vec![
            Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)[({]",
                PUNCTUATION,
                NewState::Push(vec![r"import.what.type.consesandfields"]),
            ),
            Rule::token_to(r"(?m)$", WHITESPACE, NewState::Pop(2)),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"import.what.type.consesandfields",
        vec![
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\.\.", PUNCTUATION),
            Rule::token_to(r"(?m)[)}]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)_[a-z`][\w`]*", NAME),
            Rule::token(r"(?m)[~@#$%\^?!+\-*<>\\/|&=:]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Z`][\w`]*", NAME_CLASS),
            Rule::token(r"(?m)_[A-Z`][\w`]*", NAME_CLASS),
        ],
    );
    m.insert(
        r"literals.stringd",
        vec![
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_DOUBLE),
            Rule::token_to(r"(?m)[$\n]", ERROR, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for CleanLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
