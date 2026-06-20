#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.shell:FishShellLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.shell:FishShellLexer:fish

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fish, fishshell
pub struct FishLexer;

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
        Rule::bygroups(r"(?m)\b(begin|end|if|else|while|break|for|in|return|function|block|case|continue|switch|not|and|or|set|echo|exit|pwd|true|false|cd|count|test)(\s*)\b", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::token(r"(?m)\b(alias|bg|bind|breakpoint|builtin|command|commandline|complete|contains|dirh|dirs|emit|eval|exec|fg|fish|fish_config|fish_indent|fish_pager|fish_prompt|fish_right_prompt|fish_update_completions|fishd|funced|funcsave|functions|help|history|isatty|jobs|math|mimedb|nextd|open|popd|prevd|psub|pushd|random|read|set_color|source|status|trap|type|ulimit|umask|vared|fc|getopts|hash|kill|printf|time|wait)\s*\b(?!\.)", NAME_BUILTIN),
        Rule::token(r"(?m)#.*\n", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]()=]", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r#"(?m)(?s)\$?"(\\\\|\\[0-7]+|\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&|\||\^|<|>", OPERATOR),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    m.insert(r"basic", vec![
        Rule::bygroups(r"(?m)\b(begin|end|if|else|while|break|for|in|return|function|block|case|continue|switch|not|and|or|set|echo|exit|pwd|true|false|cd|count|test)(\s*)\b", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::token(r"(?m)\b(alias|bg|bind|breakpoint|builtin|command|commandline|complete|contains|dirh|dirs|emit|eval|exec|fg|fish|fish_config|fish_indent|fish_pager|fish_prompt|fish_right_prompt|fish_update_completions|fishd|funced|funcsave|functions|help|history|isatty|jobs|math|mimedb|nextd|open|popd|prevd|psub|pushd|random|read|set_color|source|status|trap|type|ulimit|umask|vared|fc|getopts|hash|kill|printf|time|wait)\s*\b(?!\.)", NAME_BUILTIN),
        Rule::token(r"(?m)#.*\n", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]()=]", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
    ]);
    m.insert(
        r"data",
        vec![
            Rule::token(
                r#"(?m)(?s)\$?"(\\\\|\\[0-7]+|\\.|[^"\\$])*""#,
                STRING_DOUBLE,
            ),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)&|\||\^|<|>", OPERATOR),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
            Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        ],
    );
    m.insert(
        r"interp",
        vec![
            Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
            Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
            Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)(?s)(\\\\|\\[0-7]+|\\.|[^"\\$])+"#, STRING_DOUBLE),
            Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
            Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
            Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
        ],
    );
    m.insert(r"paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
        Rule::bygroups(r"(?m)\b(begin|end|if|else|while|break|for|in|return|function|block|case|continue|switch|not|and|or|set|echo|exit|pwd|true|false|cd|count|test)(\s*)\b", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::token(r"(?m)\b(alias|bg|bind|breakpoint|builtin|command|commandline|complete|contains|dirh|dirs|emit|eval|exec|fg|fish|fish_config|fish_indent|fish_pager|fish_prompt|fish_right_prompt|fish_update_completions|fishd|funced|funcsave|functions|help|history|isatty|jobs|math|mimedb|nextd|open|popd|prevd|psub|pushd|random|read|set_color|source|status|trap|type|ulimit|umask|vared|fc|getopts|hash|kill|printf|time|wait)\s*\b(?!\.)", NAME_BUILTIN),
        Rule::token(r"(?m)#.*\n", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]()=]", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r#"(?m)(?s)\$?"(\\\\|\\[0-7]+|\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&|\||\^|<|>", OPERATOR),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    m.insert(r"math", vec![
        Rule::token_to(r"(?m)\)\)", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)[-+*/%^|&]|\*\*|\|\|", OPERATOR),
        Rule::token(r"(?m)\d+#\d+", NUMBER),
        Rule::token(r"(?m)\d+#(?! )", NUMBER),
        Rule::token(r"(?m)\d+", NUMBER),
        Rule::bygroups(r"(?m)\b(begin|end|if|else|while|break|for|in|return|function|block|case|continue|switch|not|and|or|set|echo|exit|pwd|true|false|cd|count|test)(\s*)\b", vec![Some(KEYWORD), Some(TEXT)]),
        Rule::token(r"(?m)\b(alias|bg|bind|breakpoint|builtin|command|commandline|complete|contains|dirh|dirs|emit|eval|exec|fg|fish|fish_config|fish_indent|fish_pager|fish_prompt|fish_right_prompt|fish_update_completions|fishd|funced|funcsave|functions|help|history|isatty|jobs|math|mimedb|nextd|open|popd|prevd|psub|pushd|random|read|set_color|source|status|trap|type|ulimit|umask|vared|fc|getopts|hash|kill|printf|time|wait)\s*\b(?!\.)", NAME_BUILTIN),
        Rule::token(r"(?m)#.*\n", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]()=]", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r#"(?m)(?s)\$?"(\\\\|\\[0-7]+|\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&|\||\^|<|>", OPERATOR),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    Table(m)
}

impl Lexer for FishLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
