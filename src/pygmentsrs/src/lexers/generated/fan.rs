#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.fantom:FantomLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.fantom:FantomLexer:fan

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fan
pub struct FanLexer;

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
        r"comments",
        vec![
            Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"literals",
        vec![
            Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
            Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
            Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
            Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
            Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
            Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
            Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
            Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
            Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::bygroups(
                r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)",
                vec![
                    Some(NAME_NAMESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_CLASS),
                    Some(PUNCTUATION),
                    Some(STRING),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?",
                vec![
                    Some(NAME_NAMESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_CLASS),
                    Some(PUNCTUATION),
                    Some(NAME_FUNCTION),
                ],
            ),
            Rule::token(r"(?m)\[,\]", LITERAL),
            Rule::bygroups_g(
                r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inType"]),
                    }),
                    Some(GroupAction::Token(LITERAL)),
                ],
            ),
            Rule::token(r"(?m)\[:\]", LITERAL),
            Rule::bygroups_g(
                r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inType"]),
                    }),
                    Some(GroupAction::Token(LITERAL)),
                ],
            ),
        ],
    );
    m.insert(
        r"insideStr",
        vec![
            Rule::token(r"(?m)\\\\", STRING_ESCAPE),
            Rule::token(r#"(?m)\\""#, STRING_ESCAPE),
            Rule::token(r"(?m)\\`", STRING_ESCAPE),
            Rule::token(r"(?m)\$\w+", STRING_INTERPOL),
            Rule::token(r"(?m)\$\{.*?\}", STRING_INTERPOL),
            Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m).", STRING),
        ],
    );
    m.insert(
        r"insideUri",
        vec![
            Rule::token(r"(?m)\\\\", STRING_ESCAPE),
            Rule::token(r#"(?m)\\""#, STRING_ESCAPE),
            Rule::token(r"(?m)\\`", STRING_ESCAPE),
            Rule::token(r"(?m)\$\w+", STRING_INTERPOL),
            Rule::token(r"(?m)\$\{.*?\}", STRING_INTERPOL),
            Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m).", STRING_BACKTICK),
        ],
    );
    m.insert(
        r"protectionKeywords",
        vec![Rule::token(
            r"(?m)\b(public|protected|private|internal)\b",
            KEYWORD,
        )],
    );
    m.insert(
        r"typeKeywords",
        vec![Rule::token(
            r"(?m)\b(abstract|final|const|native|facet|enum)\b",
            KEYWORD,
        )],
    );
    m.insert(
        r"methodKeywords",
        vec![Rule::token(
            r"(?m)\b(abstract|native|once|override|static|virtual|final)\b",
            KEYWORD,
        )],
    );
    m.insert(
        r"fieldKeywords",
        vec![Rule::token(
            r"(?m)\b(abstract|const|final|native|override|static|virtual|readonly)\b",
            KEYWORD,
        )],
    );
    m.insert(r"otherKeywords", vec![
        Rule::token(r"(?m)\b(as|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|else|f(?:inally|or)|get|i(?:snot|[fs])|return|s(?:et|witch)|t(?:hrow|ry)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(it|this|super)\b", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(
        r"operators",
        vec![Rule::token(
            r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]",
            OPERATOR,
        )],
    );
    m.insert(
        r"inType",
        vec![
            Rule::token(r"(?m)[\[\]|\->:?]", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(public|protected|private|internal)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|final|const|native|facet|enum)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|native|once|override|static|virtual|final)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|const|final|native|override|static|virtual|readonly)\b", KEYWORD),
        Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
        Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
        Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
        Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\[,\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\[:\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\b(as|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|else|f(?:inally|or)|get|i(?:snot|[fs])|return|s(?:et|witch)|t(?:hrow|ry)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(it|this|super)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]", OPERATOR),
        Rule::token_to(r"(?m)using\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"using"])),
        Rule::token_to(r"(?m)@\w+", NAME_DECORATOR, NewState::Push(vec![r"facet"])),
        Rule::bygroups_to(r"(?m)(class|mixin)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"inheritance"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups_to(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)(\s*)(\()", vec![Some(OPERATOR), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideParen"])),
        Rule::bygroups(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(new)(\s+)(make\w*)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g_to(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(,)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\->)(\s*)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE))]),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"insideParen"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"insideBrace"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"insideParen", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(public|protected|private|internal)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|final|const|native|facet|enum)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|native|once|override|static|virtual|final)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|const|final|native|override|static|virtual|readonly)\b", KEYWORD),
        Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
        Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
        Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
        Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\[,\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\[:\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\b(as|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|else|f(?:inally|or)|get|i(?:snot|[fs])|return|s(?:et|witch)|t(?:hrow|ry)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(it|this|super)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]", OPERATOR),
        Rule::token_to(r"(?m)using\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"using"])),
        Rule::token_to(r"(?m)@\w+", NAME_DECORATOR, NewState::Push(vec![r"facet"])),
        Rule::bygroups_to(r"(?m)(class|mixin)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"inheritance"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups_to(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)(\s*)(\()", vec![Some(OPERATOR), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideParen"])),
        Rule::bygroups(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(new)(\s+)(make\w*)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g_to(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(,)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\->)(\s*)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE))]),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"insideParen"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"insideBrace"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"insideMethodDeclArgs", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::bygroups_g_to(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\))", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))], NewState::Pop(1)),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(public|protected|private|internal)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|final|const|native|facet|enum)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|native|once|override|static|virtual|final)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|const|final|native|override|static|virtual|readonly)\b", KEYWORD),
        Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
        Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
        Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
        Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\[,\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\[:\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\b(as|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|else|f(?:inally|or)|get|i(?:snot|[fs])|return|s(?:et|witch)|t(?:hrow|ry)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(it|this|super)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]", OPERATOR),
        Rule::token_to(r"(?m)using\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"using"])),
        Rule::token_to(r"(?m)@\w+", NAME_DECORATOR, NewState::Push(vec![r"facet"])),
        Rule::bygroups_to(r"(?m)(class|mixin)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"inheritance"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups_to(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)(\s*)(\()", vec![Some(OPERATOR), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideParen"])),
        Rule::bygroups(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(new)(\s+)(make\w*)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g_to(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(,)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\->)(\s*)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE))]),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"insideParen"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"insideBrace"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(r"insideBrace", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(public|protected|private|internal)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|final|const|native|facet|enum)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|native|once|override|static|virtual|final)\b", KEYWORD),
        Rule::token(r"(?m)\b(abstract|const|final|native|override|static|virtual|readonly)\b", KEYWORD),
        Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
        Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
        Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
        Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
        Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?", vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\[,\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\[:\]", LITERAL),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(LITERAL))]),
        Rule::token(r"(?m)\b(as|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|else|f(?:inally|or)|get|i(?:snot|[fs])|return|s(?:et|witch)|t(?:hrow|ry)|while)\b", KEYWORD),
        Rule::token(r"(?m)\b(it|this|super)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]", OPERATOR),
        Rule::token_to(r"(?m)using\b", KEYWORD_NAMESPACE, NewState::Push(vec![r"using"])),
        Rule::token_to(r"(?m)@\w+", NAME_DECORATOR, NewState::Push(vec![r"facet"])),
        Rule::bygroups_to(r"(?m)(class|mixin)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"inheritance"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(OPERATOR))]),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(:=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups_to(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)(\s*)(\()", vec![Some(OPERATOR), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideParen"])),
        Rule::bygroups(r"(?m)(\.|(?:\->))([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(new)(\s+)(make\w*)(\s*)(\()", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g_to(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"insideMethodDeclArgs"])),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(,)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\->)(\s*)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\s+)([a-zA-Z_]\w*)(\s*)(\|)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE)), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)([ \t]+)([a-zA-Z_]\w*)", vec![Some(GroupAction::UsingThis { state: Some(vec!["root", "inType"]) }), Some(GroupAction::Token(WHITESPACE)), Some(GroupAction::Token(NAME_VARIABLE))]),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"insideParen"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"insideBrace"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m).", TEXT),
    ]);
    m.insert(
        r"inheritance",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m):|,", PUNCTUATION),
            Rule::bygroups(
                r"(?m)(?:(\w+)(::))?(\w+)",
                vec![Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(NAME_CLASS)],
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"using",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups(
                r"(?m)(\[)(\w+)(\])",
                vec![Some(PUNCTUATION), Some(COMMENT_SPECIAL), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r#"(?m)(\")?([\w.]+)(\")?"#,
                vec![Some(PUNCTUATION), Some(NAME_NAMESPACE), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)::", PUNCTUATION, NewState::Push(vec![r"usingClass"])),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"usingClass",
        vec![
            Rule::token(r"(?m)[ \t]+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(as)(\s+)(\w+)",
                vec![
                    Some(KEYWORD_DECLARATION),
                    Some(WHITESPACE),
                    Some(NAME_CLASS),
                ],
                NewState::Pop(2),
            ),
            Rule::token(r"(?m)[\w$]+", NAME_CLASS),
            Rule::default(NewState::Pop(2)),
        ],
    );
    m.insert(
        r"facet",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"facetFields"])),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"facetFields",
        vec![
            Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token(r"(?m)\*\*.*?$", COMMENT_SPECIAL),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
            Rule::token(r"(?m)\b-?[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
            Rule::token(r"(?m)\b-?[\d_]*\.[\d_]+(ns|ms|sec|min|hr|day)", NUMBER),
            Rule::token(r"(?m)\b-?(\d+)?\.\d+(f|F|d|D)?", NUMBER_FLOAT),
            Rule::token(r"(?m)\b-?0x[0-9a-fA-F_]+", NUMBER_HEX),
            Rule::token(r"(?m)\b-?[\d_]+", NUMBER_INTEGER),
            Rule::token(r"(?m)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
            Rule::token_to(r#"(?m)""#, PUNCTUATION, NewState::Push(vec![r"insideStr"])),
            Rule::token_to(r"(?m)`", PUNCTUATION, NewState::Push(vec![r"insideUri"])),
            Rule::token(r"(?m)\b(true|false|null)\b", KEYWORD_CONSTANT),
            Rule::bygroups(
                r"(?m)(?:(\w+)(::))?(\w+)(<\|)(.*?)(\|>)",
                vec![
                    Some(NAME_NAMESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_CLASS),
                    Some(PUNCTUATION),
                    Some(STRING),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups(
                r"(?m)(?:(\w+)(::))?(\w+)?(#)(\w+)?",
                vec![
                    Some(NAME_NAMESPACE),
                    Some(PUNCTUATION),
                    Some(NAME_CLASS),
                    Some(PUNCTUATION),
                    Some(NAME_FUNCTION),
                ],
            ),
            Rule::token(r"(?m)\[,\]", LITERAL),
            Rule::bygroups_g(
                r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[,\])",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inType"]),
                    }),
                    Some(GroupAction::Token(LITERAL)),
                ],
            ),
            Rule::token(r"(?m)\[:\]", LITERAL),
            Rule::bygroups_g(
                r"(?m)((?:\[|[a-zA-Z_]|\|)[:\w\[\]|\->?]*?)(\[:\])",
                vec![
                    Some(GroupAction::UsingThis {
                        state: Some(vec!["root", "inType"]),
                    }),
                    Some(GroupAction::Token(LITERAL)),
                ],
            ),
            Rule::token(
                r"(?m)\+\+|\-\-|\+|\-|\*|/|\|\||&&|<=>|<=|<|>=|>|=|!|\[|\]",
                OPERATOR,
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(
                r"(?m)(\s*)(\w+)(\s*)(=)",
                vec![
                    Some(WHITESPACE),
                    Some(NAME),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                ],
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for FanLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
