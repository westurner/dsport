#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:GoloLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:GoloLexer:golo

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: golo
pub struct GoloLexer;

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
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)(\^|\.\.\.|:|\?:|->|==|!=|=|\+|\*|%|/|<=|<|>=|>|=|\.)", OPERATOR),
        Rule::token(r"(?m)(?<=[^-])(-)(?=[^-])", OPERATOR),
        Rule::token(r"(?m)(?<=[^`])(is|isnt|and|or|not|oftype|in|orIfNull)\b", OPERATOR_WORD),
        Rule::token(r"(?m)[\]{}|(),\[]", PUNCTUATION),
        Rule::bygroups_to(r"(?m)(module|import)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"modname"])),
        Rule::bygroups(r"(?m)\b([a-zA-Z_][\w$.]*)(::)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\b([a-zA-Z_][\w$]*(?:\.[a-zA-Z_][\w$]*)+)\b", NAME_NAMESPACE),
        Rule::bygroups_to(r"(?m)(let|var)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"varname"])),
        Rule::bygroups_to(r"(?m)(struct)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"structname"])),
        Rule::bygroups_to(r"(?m)(function)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::token(r"(?m)(null|true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(augment|pimp|if|else|case|match|return|case|when|then|otherwise|while|for|foreach|try|catch|finally|throw|local|continue|break)\b", KEYWORD),
        Rule::bygroups(r"(?m)(map|array|list|set|vector|tuple)(\[)", vec![Some(NAME_BUILTIN), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(print|println|readln|raise|fun|asInterfaceInstance)\b", NAME_BUILTIN),
        Rule::bygroups(r"(?m)(`?[a-zA-Z_][\w$]*)(\()", vec![Some(NAME_FUNCTION), Some(PUNCTUATION)]),
        Rule::token(r"(?m)-?[\d_]*\.[\d_]*([eE][+-]?\d[\d_]*)?F?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)-?\d[\d_]*L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)-?\d[\d_]*", NUMBER_INTEGER),
        Rule::token(r"(?m)`?[a-zA-Z_][\w$]*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_][\w$.]*", NAME_DECORATOR),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token(r"(?m)----((.|\n)*?)----", STRING_DOC),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(
        r"triplestring",
        vec![
            Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
            Rule::token(r#"(?m)[\'"\\]"#, STRING),
            Rule::token(r"(?m)\n", STRING),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
            Rule::token(r#"(?m)[\'"\\]"#, STRING),
        ],
    );
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(
        r"doublestring",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
            Rule::token(r#"(?m)[\'"\\]"#, STRING),
        ],
    );
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(
        r"singlestring",
        vec![
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
            Rule::token(r#"(?m)[\'"\\]"#, STRING),
        ],
    );
    m.insert(r"_tmp_2", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\\'"\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
    ]);
    m.insert(
        r"funcname",
        vec![Rule::token_to(
            r"(?m)`?[a-zA-Z_][\w$]*",
            NAME_FUNCTION,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"modname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_][\w$.]*\*?",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"structname",
        vec![Rule::token_to(
            r"(?m)`?[\w.]+\*?",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"varname",
        vec![Rule::token_to(
            r"(?m)`?[a-zA-Z_][\w$]*",
            NAME_VARIABLE,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?m)[#=,./%+\-?]", OPERATOR),
            Rule::token(r"(?m)(eq|gt|lt|gte|lte|neq|matches)\b", OPERATOR),
            Rule::token(r"(?m)(==|<=|<|>=|>|!=)", OPERATOR),
        ],
    );
    Table(m)
}

impl Lexer for GoloLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
