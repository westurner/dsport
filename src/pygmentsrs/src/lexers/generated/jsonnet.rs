//! AUTO-GENERATED from `pygments.pygments.lexers.jsonnet:JsonnetLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jsonnet:JsonnetLexer:jsonnet

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jsonnet
pub struct JsonnetLexer;

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
        r"_comments",
        vec![
            Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
            Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"singlestring",
        vec![
            Rule::token(r"(?m)[^'\\]", STRING),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"doublestring",
        vec![
            Rule::token(r#"(?m)[^"\\]"#, STRING),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(r"array", vec![
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"local_name",
        vec![
            Rule::token_to(
                r"(?m)[^\W\d]\w*(?=\()",
                NAME_FUNCTION,
                NewState::Push(vec![r"function_params"]),
            ),
            Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)(?==)",
                WHITESPACE,
                NewState::Push(vec![r"#pop", r"local_value"]),
            ),
        ],
    );
    m.insert(r"local_value", vec![
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(r"assert", vec![
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"function_params",
        vec![
            Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)\(", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)=",
                OPERATOR,
                NewState::Push(vec![r"function_param_default"]),
            ),
        ],
    );
    m.insert(r"function_args", vec![
        Rule::token(r"(?m)\(", PUNCTUATION),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"object",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)local\b",
                KEYWORD,
                NewState::Push(vec![r"object_local_name"]),
            ),
            Rule::token_to(
                r"(?m)assert\b",
                KEYWORD,
                NewState::Push(vec![r"object_assert"]),
            ),
            Rule::token_to(
                r"(?m)\[",
                OPERATOR,
                NewState::Push(vec![r"field_name_expr"]),
            ),
            Rule::token_to(
                r"(?m)(?=[^\W\d]\w*)",
                TEXT,
                NewState::Push(vec![r"field_name"]),
            ),
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(
                r#"(?m)""#,
                NAME_VARIABLE,
                NewState::Push(vec![r"double_field_name"]),
            ),
            Rule::token_to(
                r"(?m)'",
                NAME_VARIABLE,
                NewState::Push(vec![r"single_field_name"]),
            ),
            Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
            Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        ],
    );
    m.insert(
        r"field_name",
        vec![
            Rule::token_to(
                r"(?m)[^\W\d]\w*(?=\()",
                NAME_FUNCTION,
                NewState::Push(vec![r"field_separator", r"function_params"]),
            ),
            Rule::token_to(
                r"(?m)[^\W\d]\w*",
                NAME_VARIABLE,
                NewState::Push(vec![r"field_separator"]),
            ),
        ],
    );
    m.insert(
        r"double_field_name",
        vec![Rule::token_to(
            r#"(?m)([^"\\]|\\.)*""#,
            NAME_VARIABLE,
            NewState::Push(vec![r"field_separator"]),
        )],
    );
    m.insert(
        r"single_field_name",
        vec![Rule::token_to(
            r"(?m)([^'\\]|\\.)*'",
            NAME_VARIABLE,
            NewState::Push(vec![r"field_separator"]),
        )],
    );
    m.insert(r"field_name_expr", vec![
        Rule::token_to(r"(?m)\]", OPERATOR, NewState::Push(vec![r"field_separator"])),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(r"function_param_default", vec![
        Rule::token_to(r"(?m)(?=[,\)])", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"field_separator",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\+?::?:?",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"#pop", r"field_value"]),
            ),
            Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
            Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        ],
    );
    m.insert(r"field_value", vec![
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(r"object_assert", vec![
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    m.insert(
        r"object_local_name",
        vec![
            Rule::token_to(
                r"(?m)[^\W\d]\w*",
                NAME_VARIABLE,
                NewState::Push(vec![r"#pop", r"object_local_value"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(r"object_local_value", vec![
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
        Rule::token(r"(?m)(//|#).*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*\*([^/]|/(?!\*))*\*/", STRING_DOC),
        Rule::token(r"(?m)/\*([^/]|/(?!\*))*\*/", COMMENT),
        Rule::token(r"(?m)@'.*'", STRING),
        Rule::token(r#"(?m)@".*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"singlestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublestring"])),
        Rule::token(r"(?m)\|\|\|(.|\n)*\|\|\|", STRING),
        Rule::token(r"(?m)[+-]?[0-9]+(.[0-9])?", NUMBER_FLOAT),
        Rule::token(r"(?m)[!$~+\-&|^=<>*/%]", OPERATOR),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"object"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"array"])),
        Rule::token_to(r"(?m)local\b", KEYWORD, NewState::Push(vec![r"local_name"])),
        Rule::token_to(r"(?m)assert\b", KEYWORD, NewState::Push(vec![r"assert"])),
        Rule::token(r"(?m)(assert|e(?:lse|rror)|f(?:alse|or)|i(?:mport(?:(?:str)?)|[fn])|null|s(?:elf|uper)|t(?:ailstrict|hen|rue))\b", KEYWORD),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)function(?=\()", KEYWORD, NewState::Push(vec![r"function_params"])),
        Rule::token_to(r"(?m)std\.[^\W\d]\w*(?=\()", NAME_BUILTIN, NewState::Push(vec![r"function_args"])),
        Rule::token_to(r"(?m)[^\W\d]\w*(?=\()", NAME_FUNCTION, NewState::Push(vec![r"function_args"])),
        Rule::token(r"(?m)[^\W\d]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)[\.()]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for JsonnetLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
