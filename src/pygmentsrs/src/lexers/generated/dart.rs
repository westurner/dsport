//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:DartLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:DartLexer:dart

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dart
pub struct DartLexer;

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
        Rule::token(r#"(?ms)r"""([\w\W]*?)""""#, STRING_DOUBLE),
        Rule::token(r"(?ms)r'''([\w\W]*?)'''", STRING_SINGLE),
        Rule::token(r#"(?ms)r"(.*?)""#, STRING_DOUBLE),
        Rule::token(r"(?ms)r'(.*?)'", STRING_SINGLE),
        Rule::token_to(r#"(?ms)""""#, STRING_DOUBLE, NewState::Push(vec![r"string_double_multiline"])),
        Rule::token_to(r"(?ms)'''", STRING_SINGLE, NewState::Push(vec![r"string_single_multiline"])),
        Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?ms)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::token(r"(?ms)#!(.*?)$", COMMENT_PREPROC),
        Rule::token_to(r"(?ms)\b(import|export)\b", KEYWORD, NewState::Push(vec![r"import_decl"])),
        Rule::token(r"(?ms)\b(library|source|part of|part)\b", KEYWORD),
        Rule::token(r"(?ms)[^\S\n]+", WHITESPACE),
        Rule::bygroups(r"(?ms)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::bygroups_to(r"(?ms)\b(class|extension|mixin)\b(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"class"])),
        Rule::token(r"(?ms)\b(as|assert|break|case|catch|const|continue|default|do|else|finally|for|if|in|is|new|rethrow|return|super|switch|this|throw|try|while)\b", KEYWORD),
        Rule::token(r"(?ms)\b(abstract|async|await|const|covariant|extends|external|factory|final|get|implements|late|native|on|operator|required|set|static|sync|typedef|var|with|yield)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)\b(bool|double|dynamic|int|num|Function|Never|Null|Object|String|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?ms)\b(false|null|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)[~!%^&*+=|?:<>/-]|as\b", OPERATOR),
        Rule::token(r"(?ms)@[a-zA-Z_$]\w*", NAME_DECORATOR),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*:", NAME_LABEL),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)[(){}\[\],.;]", PUNCTUATION),
        Rule::token(r"(?ms)0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)\d+(\.\d*)?([eE][+-]?\d+)?", NUMBER),
        Rule::token(r"(?ms)\.\d+([eE][+-]?\d+)?", NUMBER),
        Rule::token(r"(?ms)\n", WHITESPACE),
    ]);
    m.insert(
        r"string_literal",
        vec![
            Rule::token(r#"(?ms)r"""([\w\W]*?)""""#, STRING_DOUBLE),
            Rule::token(r"(?ms)r'''([\w\W]*?)'''", STRING_SINGLE),
            Rule::token(r#"(?ms)r"(.*?)""#, STRING_DOUBLE),
            Rule::token(r"(?ms)r'(.*?)'", STRING_SINGLE),
            Rule::token_to(
                r#"(?ms)""""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double_multiline"]),
            ),
            Rule::token_to(
                r"(?ms)'''",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single_multiline"]),
            ),
            Rule::token_to(
                r#"(?ms)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token_to(
                r"(?ms)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
        ],
    );
    m.insert(
        r"class",
        vec![Rule::token_to(
            r"(?ms)[a-zA-Z_$]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"import_decl",
        vec![
            Rule::token(r#"(?ms)r"""([\w\W]*?)""""#, STRING_DOUBLE),
            Rule::token(r"(?ms)r'''([\w\W]*?)'''", STRING_SINGLE),
            Rule::token(r#"(?ms)r"(.*?)""#, STRING_DOUBLE),
            Rule::token(r"(?ms)r'(.*?)'", STRING_SINGLE),
            Rule::token_to(
                r#"(?ms)""""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double_multiline"]),
            ),
            Rule::token_to(
                r"(?ms)'''",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single_multiline"]),
            ),
            Rule::token_to(
                r#"(?ms)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token_to(
                r"(?ms)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)\b(as|deferred|show|hide)\b", KEYWORD),
            Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
            Rule::token(r"(?ms)\,", PUNCTUATION),
            Rule::token_to(r"(?ms)\;", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string_common",
        vec![
            Rule::token(
                r#"(?ms)\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}|u\{[0-9A-Fa-f]*\}|[a-z'\"$\\])"#,
                STRING_ESCAPE,
            ),
            Rule::bygroups(
                r"(?ms)(\$)([a-zA-Z_]\w*)",
                vec![Some(STRING_INTERPOL), Some(NAME)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(STRING_INTERPOL)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(STRING_INTERPOL)),
                ],
            ),
        ],
    );
    m.insert(
        r"string_double",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?ms)[^"$\\\n]+"#, STRING_DOUBLE),
            Rule::token(
                r#"(?ms)\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}|u\{[0-9A-Fa-f]*\}|[a-z'\"$\\])"#,
                STRING_ESCAPE,
            ),
            Rule::bygroups(
                r"(?ms)(\$)([a-zA-Z_]\w*)",
                vec![Some(STRING_INTERPOL), Some(NAME)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(STRING_INTERPOL)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(STRING_INTERPOL)),
                ],
            ),
            Rule::token(r"(?ms)\$+", STRING_DOUBLE),
        ],
    );
    m.insert(
        r"string_double_multiline",
        vec![
            Rule::token_to(r#"(?ms)""""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?ms)[^"$\\]+"#, STRING_DOUBLE),
            Rule::token(
                r#"(?ms)\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}|u\{[0-9A-Fa-f]*\}|[a-z'\"$\\])"#,
                STRING_ESCAPE,
            ),
            Rule::bygroups(
                r"(?ms)(\$)([a-zA-Z_]\w*)",
                vec![Some(STRING_INTERPOL), Some(NAME)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(STRING_INTERPOL)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(STRING_INTERPOL)),
                ],
            ),
            Rule::token(r#"(?ms)(\$|\")+"#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"string_single",
        vec![
            Rule::token_to(r"(?ms)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ms)[^'$\\\n]+", STRING_SINGLE),
            Rule::token(
                r#"(?ms)\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}|u\{[0-9A-Fa-f]*\}|[a-z'\"$\\])"#,
                STRING_ESCAPE,
            ),
            Rule::bygroups(
                r"(?ms)(\$)([a-zA-Z_]\w*)",
                vec![Some(STRING_INTERPOL), Some(NAME)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(STRING_INTERPOL)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(STRING_INTERPOL)),
                ],
            ),
            Rule::token(r"(?ms)\$+", STRING_SINGLE),
        ],
    );
    m.insert(
        r"string_single_multiline",
        vec![
            Rule::token_to(r"(?ms)'''", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?ms)[^\'$\\]+", STRING_SINGLE),
            Rule::token(
                r#"(?ms)\\(x[0-9A-Fa-f]{2}|u[0-9A-Fa-f]{4}|u\{[0-9A-Fa-f]*\}|[a-z'\"$\\])"#,
                STRING_ESCAPE,
            ),
            Rule::bygroups(
                r"(?ms)(\$)([a-zA-Z_]\w*)",
                vec![Some(STRING_INTERPOL), Some(NAME)],
            ),
            Rule::bygroups_g(
                r"(?ms)(\$\{)(.*?)(\})",
                vec![
                    Some(GroupAction::Token(STRING_INTERPOL)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(STRING_INTERPOL)),
                ],
            ),
            Rule::token(r"(?ms)(\$|\')+", STRING_SINGLE),
        ],
    );
    Table(m)
}

impl Lexer for DartLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
