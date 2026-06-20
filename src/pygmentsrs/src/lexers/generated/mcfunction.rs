#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.minecraft:MCFunctionLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.minecraft:MCFunctionLexer:mcfunction

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mcfunction, mcf
pub struct McfunctionLexer;

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
            Rule::bygroups(
                r"(?m)^(\s*)([a-z_]+)",
                vec![Some(WHITESPACE), Some(NAME_BUILTIN)],
            ),
            Rule::token(r"(?m)(?<=run)\s+[a-z_]+", NAME_BUILTIN),
            Rule::token(r"(?m)\b[0-9a-fA-F]+(?:-[0-9a-fA-F]+){4}\b", NAME_VARIABLE),
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Za-z_][\w.#%$]+", KEYWORD_CONSTANT),
            Rule::token(r"(?m)[#%$][\w.#%$]+", NAME_VARIABLE_MAGIC),
            Rule::token_to(
                r"(?m)^\s*(#[>!])",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments.block", r"comments.block.emphasized"]),
            ),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token(r"(?m)\.\.", LITERAL),
            Rule::token(r"(?m)(true|false)", KEYWORD_PSEUDO),
            Rule::token(r"(?m)[A-Za-z_]+", NAME_VARIABLE_CLASS),
            Rule::token(
                r"(?m)[0-7]b",
                TokenType::new(&["Literal", "Number", "Byte"]),
            ),
            Rule::token(r"(?m)[+-]?\d*\.?\d+([eE]?[+-]?\d+)?[df]?\b", NUMBER_FLOAT),
            Rule::token(r"(?m)[+-]?\d+\b", NUMBER_INTEGER),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"literals.string-double"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literals.string-single"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"property.curly", r"property.key"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"property.square", r"property.key"]),
            ),
            Rule::token(r"(?m)[\-~%^?!+*<>\\/|&=.]", OPERATOR),
            Rule::token(r"(?m)@[a-z]", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"names",
        vec![
            Rule::bygroups(
                r"(?m)^(\s*)([a-z_]+)",
                vec![Some(WHITESPACE), Some(NAME_BUILTIN)],
            ),
            Rule::token(r"(?m)(?<=run)\s+[a-z_]+", NAME_BUILTIN),
            Rule::token(r"(?m)\b[0-9a-fA-F]+(?:-[0-9a-fA-F]+){4}\b", NAME_VARIABLE),
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
            Rule::token(r"(?m)[A-Za-z_][\w.#%$]+", KEYWORD_CONSTANT),
            Rule::token(r"(?m)[#%$][\w.#%$]+", NAME_VARIABLE_MAGIC),
        ],
    );
    m.insert(
        r"resource-name",
        vec![
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token_to(
                r"(?m)^\s*(#[>!])",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments.block", r"comments.block.emphasized"]),
            ),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"literals",
        vec![
            Rule::token(r"(?m)\.\.", LITERAL),
            Rule::token(r"(?m)(true|false)", KEYWORD_PSEUDO),
            Rule::token(r"(?m)[A-Za-z_]+", NAME_VARIABLE_CLASS),
            Rule::token(
                r"(?m)[0-7]b",
                TokenType::new(&["Literal", "Number", "Byte"]),
            ),
            Rule::token(r"(?m)[+-]?\d*\.?\d+([eE]?[+-]?\d+)?[df]?\b", NUMBER_FLOAT),
            Rule::token(r"(?m)[+-]?\d+\b", NUMBER_INTEGER),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"literals.string-double"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literals.string-single"]),
            ),
        ],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", WHITESPACE)]);
    m.insert(
        r"property",
        vec![
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"property.curly", r"property.key"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"property.square", r"property.key"]),
            ),
        ],
    );
    m.insert(
        r"operators",
        vec![Rule::token(r"(?m)[\-~%^?!+*<>\\/|&=.]", OPERATOR)],
    );
    m.insert(
        r"selectors",
        vec![Rule::token(r"(?m)@[a-z]", NAME_VARIABLE)],
    );
    m.insert(
        r"comments.block",
        vec![
            Rule::token_to(
                r"(?m)^\s*#[>!]",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments.block.emphasized"]),
            ),
            Rule::token_to(
                r"(?m)^\s*#",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments.block.normal"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"comments.block.normal",
        vec![
            Rule::token(r"(?m)@\S+", NAME_DECORATOR),
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
            Rule::token(r"(?m)[#%$][\w.#%$]+", NAME_VARIABLE_MAGIC),
            Rule::token(r"(?m)\S+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"comments.block.special",
        vec![
            Rule::token(r"(?m)@\S+", NAME_DECORATOR),
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
            Rule::token(r"(?m)[#%$][\w.#%$]+", NAME_VARIABLE_MAGIC),
        ],
    );
    m.insert(
        r"comments.block.emphasized",
        vec![
            Rule::token(r"(?m)@\S+", NAME_DECORATOR),
            Rule::token(r"(?m)#?[a-z_][a-z_.-]*:[a-z0-9_./-]+", NAME_FUNCTION),
            Rule::token(r"(?m)#?[a-z0-9_\.\-]+\/[a-z0-9_\.\-\/]+", NAME_FUNCTION),
            Rule::token(r"(?m)[#%$][\w.#%$]+", NAME_VARIABLE_MAGIC),
            Rule::token(r"(?m)\S+", STRING_DOC),
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"literals.string-double",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"literals.string-single",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[^\\'\n]+", STRING_SINGLE),
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.curly",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"property.curly", r"property.key"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"property.square", r"property.key"]),
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.square",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"property.curly", r"property.key"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"property.square", r"property.key"]),
            ),
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"property.key",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)#?[a-z_][a-z_\.\-]*\:[a-z0-9_\.\-/]+(?=\s*\=)",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::token_to(
                r"(?m)#?[a-z_][a-z0-9_\.\-/]+",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::token_to(
                r"(?m)[A-Za-z_\-\+]+",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::token_to(
                r"(?m)'",
                NAME_ATTRIBUTE,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::token_to(
                r"(?m)-?\d+",
                NUMBER_INTEGER,
                NewState::Push(vec![r"property.delimiter"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.key.string-double",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"\n]+"#, NAME_ATTRIBUTE),
            Rule::token_to(r#"(?m)""#, NAME_ATTRIBUTE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.key.string-single",
        vec![
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r"(?m)[^\\'\n]+", NAME_ATTRIBUTE),
            Rule::token_to(r"(?m)'", NAME_ATTRIBUTE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.delimiter",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)[:=]!?",
                PUNCTUATION,
                NewState::Push(vec![r"property.value"]),
            ),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"property.value",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)#?[a-z_][a-z_\.\-]*\:[a-z0-9_\.\-/]+", NAME_TAG),
            Rule::token(r"(?m)#?[a-z_][a-z0-9_\.\-/]+", NAME_TAG),
            Rule::token(r"(?m)\.\.", LITERAL),
            Rule::token(r"(?m)(true|false)", KEYWORD_PSEUDO),
            Rule::token(r"(?m)[A-Za-z_]+", NAME_VARIABLE_CLASS),
            Rule::token(
                r"(?m)[0-7]b",
                TokenType::new(&["Literal", "Number", "Byte"]),
            ),
            Rule::token(r"(?m)[+-]?\d*\.?\d+([eE]?[+-]?\d+)?[df]?\b", NUMBER_FLOAT),
            Rule::token(r"(?m)[+-]?\d+\b", NUMBER_INTEGER),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"literals.string-double"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"literals.string-single"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"property.curly", r"property.key"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                PUNCTUATION,
                NewState::Push(vec![r"property.square", r"property.key"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for McfunctionLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
