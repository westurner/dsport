//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:LuauLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:LuauLexer:luau

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: luau
pub struct LuauLexer;

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
            Rule::token_to(r"(?m)#!.*", COMMENT_HASHBANG, NewState::Push(vec![r"base"])),
            Rule::default(NewState::Push(vec![r"base"])),
        ],
    );
    m.insert(
        r"ws",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(r"base", vec![
        Rule::token(r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"closing_brace_base", r"expression"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"closing_parenthesis_base", r"expression"])),
        Rule::token_to(r"(?m)::?", PUNCTUATION, NewState::Push(vec![r"type_end", r"type_start"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"string_interpolated"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
        Rule::token_to(r"(?m)type\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED, NewState::Push(vec![r"type_declaration"])),
        Rule::token(r"(?m)export\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED),
        Rule::token_to(r"(?m)(?:\.\.|//|[+\-*\/%^<>=])=?", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)~=", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(and|not|or)\b", OPERATOR_WORD, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(elseif|for|i(?:[fn])|re(?:peat|turn)|until|while)\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)local\b", KEYWORD_DECLARATION, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)function\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression", r"func_name"])),
        Rule::token(r"(?m)[\])};]+", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|do|e(?:lse(?:(?:if)?)|nd)|for|i(?:[fn])|re(?:peat|turn)|then|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)0[xX][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][\d_]*", NUMBER_BIN),
        Rule::token(r"(?m)\.?\d[\d_]*(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(false|nil|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\[(=*)\[[.\n]*?\]\1\]", STRING),
        Rule::bygroups(r#"(?m)(\.)([a-zA-Z_]\w*)(?=%s*[({"\'])"#, vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r#"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*(?=\s*[({"\'])"#, NAME_OTHER),
        Rule::token(r"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*", NAME),
        Rule::token_to(r"(?m)[\[.,]", PUNCTUATION, NewState::Push(vec![r"expression"])),
    ]);
    m.insert(r"expression_static", vec![
        Rule::token(r"(?m)(break|continue|do|e(?:lse(?:(?:if)?)|nd)|for|i(?:[fn])|re(?:peat|turn)|then|until|while)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"expression", vec![
        Rule::token(r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)if\b", KEYWORD_RESERVED, NewState::Push(vec![r"ternary", r"expression"])),
        Rule::token(r"(?m)local\b", KEYWORD_DECLARATION),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"#pop", r"closing_brace_base", r"expression"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"#pop", r"closing_parenthesis_base", r"expression"])),
        Rule::token_to(r"(?m)::?", PUNCTUATION, NewState::Push(vec![r"#pop", r"type_end", r"type_start"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"#pop", r"string_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"#pop", r"string_double"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"#pop", r"string_interpolated"])),
        Rule::token_to(r"(?m)\.\.\.", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)function\b", KEYWORD_RESERVED, NewState::Push(vec![r"func_name"])),
        Rule::token(r"(?m)(break|continue|do|e(?:lse(?:(?:if)?)|nd)|for|i(?:[fn])|re(?:peat|turn)|then|until|while)\b", KEYWORD_RESERVED),
        Rule::token_to(r"(?m)0[xX][\da-fA-F_]*", NUMBER_HEX, NewState::Pop(1)),
        Rule::token_to(r"(?m)0[bB][\d_]*", NUMBER_BIN, NewState::Pop(1)),
        Rule::token_to(r"(?m)\.?\d[\d_]*(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?", NUMBER_FLOAT, NewState::Pop(1)),
        Rule::token_to(r"(?m)(false|nil|true)\b", KEYWORD_CONSTANT, NewState::Pop(1)),
        Rule::token_to(r"(?m)\[(=*)\[[.\n]*?\]\1\]", STRING, NewState::Pop(1)),
        Rule::bygroups_to(r#"(?m)(\.)([a-zA-Z_]\w*)(?=%s*[({"\'])"#, vec![Some(PUNCTUATION), Some(NAME_FUNCTION)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)], NewState::Pop(1)),
        Rule::token_to(r#"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*(?=\s*[({"\'])"#, NAME_OTHER, NewState::Pop(1)),
        Rule::token_to(r"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*", NAME, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"ternary",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)else\b", KEYWORD_RESERVED, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)(elseif|then)\b",
                TokenType::new(&["Operator", "Reserved"]),
                NewState::Push(vec![r"expression"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"closing_brace_pop",
        vec![Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1))],
    );
    m.insert(
        r"closing_parenthesis_pop",
        vec![Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1))],
    );
    m.insert(
        r"closing_gt_pop",
        vec![Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1))],
    );
    m.insert(r"closing_parenthesis_base", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"closing_brace_base", r"expression"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"closing_parenthesis_base", r"expression"])),
        Rule::token_to(r"(?m)::?", PUNCTUATION, NewState::Push(vec![r"type_end", r"type_start"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"string_interpolated"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
        Rule::token_to(r"(?m)type\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED, NewState::Push(vec![r"type_declaration"])),
        Rule::token(r"(?m)export\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED),
        Rule::token_to(r"(?m)(?:\.\.|//|[+\-*\/%^<>=])=?", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)~=", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(and|not|or)\b", OPERATOR_WORD, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(elseif|for|i(?:[fn])|re(?:peat|turn)|until|while)\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)local\b", KEYWORD_DECLARATION, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)function\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression", r"func_name"])),
        Rule::token(r"(?m)[\])};]+", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|do|e(?:lse(?:(?:if)?)|nd)|for|i(?:[fn])|re(?:peat|turn)|then|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)0[xX][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][\d_]*", NUMBER_BIN),
        Rule::token(r"(?m)\.?\d[\d_]*(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(false|nil|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\[(=*)\[[.\n]*?\]\1\]", STRING),
        Rule::bygroups(r#"(?m)(\.)([a-zA-Z_]\w*)(?=%s*[({"\'])"#, vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r#"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*(?=\s*[({"\'])"#, NAME_OTHER),
        Rule::token(r"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*", NAME),
        Rule::token_to(r"(?m)[\[.,]", PUNCTUATION, NewState::Push(vec![r"expression"])),
    ]);
    m.insert(
        r"closing_parenthesis_type",
        vec![
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"closing_parenthesis_type"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"closing_brace_type"]),
            ),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token(r"(?m)[|&\.,\[\]:=]+", PUNCTUATION),
            Rule::token(r"(?m)->", PUNCTUATION),
            Rule::token_to(
                r"(?m)typeof\(",
                NAME_BUILTIN,
                NewState::Push(vec![r"closing_parenthesis_base", r"expression"]),
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
        ],
    );
    m.insert(
        r"type",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"closing_parenthesis_type"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"closing_brace_type"]),
            ),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token(r"(?m)[|&\.,\[\]:=]+", PUNCTUATION),
            Rule::token(r"(?m)->", PUNCTUATION),
            Rule::token_to(
                r"(?m)typeof\(",
                NAME_BUILTIN,
                NewState::Push(vec![r"closing_parenthesis_base", r"expression"]),
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
        ],
    );
    m.insert(r"closing_brace_base", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"closing_brace_base", r"expression"])),
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"closing_parenthesis_base", r"expression"])),
        Rule::token_to(r"(?m)::?", PUNCTUATION, NewState::Push(vec![r"type_end", r"type_start"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"string_interpolated"])),
        Rule::token(r"(?m)\.\.\.", PUNCTUATION),
        Rule::token_to(r"(?m)type\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED, NewState::Push(vec![r"type_declaration"])),
        Rule::token(r"(?m)export\b(?=(?:(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])|(?:--.*$)|\s+)+[a-zA-Z_])", KEYWORD_RESERVED),
        Rule::token_to(r"(?m)(?:\.\.|//|[+\-*\/%^<>=])=?", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)~=", OPERATOR, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(and|not|or)\b", OPERATOR_WORD, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)(elseif|for|i(?:[fn])|re(?:peat|turn)|until|while)\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)local\b", KEYWORD_DECLARATION, NewState::Push(vec![r"expression"])),
        Rule::token_to(r"(?m)function\b", KEYWORD_RESERVED, NewState::Push(vec![r"expression", r"func_name"])),
        Rule::token(r"(?m)[\])};]+", PUNCTUATION),
        Rule::token(r"(?m)(break|continue|do|e(?:lse(?:(?:if)?)|nd)|for|i(?:[fn])|re(?:peat|turn)|then|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)0[xX][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][\d_]*", NUMBER_BIN),
        Rule::token(r"(?m)\.?\d[\d_]*(?:\.[\d_]*)?(?:[eE][+-]?[\d_]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(false|nil|true)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\[(=*)\[[.\n]*?\]\1\]", STRING),
        Rule::bygroups(r#"(?m)(\.)([a-zA-Z_]\w*)(?=%s*[({"\'])"#, vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(\.)([a-zA-Z_]\w*)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r#"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*(?=\s*[({"\'])"#, NAME_OTHER),
        Rule::token(r"(?m)[a-zA-Z_]\w*(?:\.[a-zA-Z_]\w*)*", NAME),
        Rule::token_to(r"(?m)[\[.,]", PUNCTUATION, NewState::Push(vec![r"expression"])),
    ]);
    m.insert(
        r"closing_brace_type",
        vec![
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"closing_parenthesis_type"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"closing_brace_type"]),
            ),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token(r"(?m)[|&\.,\[\]:=]+", PUNCTUATION),
            Rule::token(r"(?m)->", PUNCTUATION),
            Rule::token_to(
                r"(?m)typeof\(",
                NAME_BUILTIN,
                NewState::Push(vec![r"closing_parenthesis_base", r"expression"]),
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
        ],
    );
    m.insert(
        r"closing_gt_type",
        vec![
            Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"closing_parenthesis_type"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"closing_brace_type"]),
            ),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string_single"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string_double"]),
            ),
            Rule::token(r"(?m)[|&\.,\[\]:=]+", PUNCTUATION),
            Rule::token(r"(?m)->", PUNCTUATION),
            Rule::token_to(
                r"(?m)typeof\(",
                NAME_BUILTIN,
                NewState::Push(vec![r"closing_parenthesis_base", r"expression"]),
            ),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
        ],
    );
    m.insert(r"string_escape", vec![
        Rule::token(r"(?m)\\z\s*", STRING_ESCAPE),
        Rule::token(r#"(?m)\\(?:[abfnrtvz\\"\'`\{\n])|[\r\n]{1,2}|x[\da-fA-F]{2}|\d{1,3}|u\{\}[\da-fA-F]*\}"#, STRING_ESCAPE),
    ]);
    m.insert(r"string_single", vec![
        Rule::token(r"(?m)\\z\s*", STRING_ESCAPE),
        Rule::token(r#"(?m)\\(?:[abfnrtvz\\"\'`\{\n])|[\r\n]{1,2}|x[\da-fA-F]{2}|\d{1,3}|u\{\}[\da-fA-F]*\}"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\']+", STRING_SINGLE),
    ]);
    m.insert(r"string_double", vec![
        Rule::token(r"(?m)\\z\s*", STRING_ESCAPE),
        Rule::token(r#"(?m)\\(?:[abfnrtvz\\"\'`\{\n])|[\r\n]{1,2}|x[\da-fA-F]{2}|\d{1,3}|u\{\}[\da-fA-F]*\}"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"string_interpolated", vec![
        Rule::token(r"(?m)\\z\s*", STRING_ESCAPE),
        Rule::token(r#"(?m)\\(?:[abfnrtvz\\"\'`\{\n])|[\r\n]{1,2}|x[\da-fA-F]{2}|\d{1,3}|u\{\}[\da-fA-F]*\}"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"closing_brace_base", r"expression"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\`\{]+", STRING_BACKTICK),
    ]);
    m.insert(
        r"func_name",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[.:]", PUNCTUATION),
            Rule::token(r"(?m)[a-zA-Z_]\w*(?=\s*[.:])", NAME_CLASS),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type_start",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"closing_parenthesis_type"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"closing_brace_type"]),
            ),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"string_single"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"#pop", r"string_double"]),
            ),
            Rule::token_to(
                r"(?m)typeof\(",
                NAME_BUILTIN,
                NewState::Push(vec![r"#pop", r"closing_parenthesis_base", r"expression"]),
            ),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type_end",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m)[|&\.]",
                PUNCTUATION,
                NewState::Push(vec![r"type_start"]),
            ),
            Rule::token_to(r"(?m)->", PUNCTUATION, NewState::Push(vec![r"type_start"])),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type_declaration",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_CLASS),
            Rule::token_to(
                r"(?m)<",
                PUNCTUATION,
                NewState::Push(vec![r"closing_gt_type"]),
            ),
            Rule::token_to(
                r"(?m)=",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"type_end", r"type_start"]),
            ),
        ],
    );
    Table(m)
}

impl Lexer for LuauLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
