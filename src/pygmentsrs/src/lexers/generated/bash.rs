#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.shell:BashLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.shell:BashLexer:bash

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bash, sh, ksh, zsh, shell, openrc
pub struct BashLexer;

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
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&", PUNCTUATION),
        Rule::token(r"(?m)\|", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token(r"(?m)<", TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    m.insert(r"basic", vec![
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
    ]);
    m.insert(
        r"data",
        vec![
            Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
            Rule::token(r"(?m);", PUNCTUATION),
            Rule::token(r"(?m)&", PUNCTUATION),
            Rule::token(r"(?m)\|", PUNCTUATION),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\d+\b", NUMBER),
            Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
            Rule::token(r"(?m)<", TEXT),
        ],
    );
    m.insert(
        r"interp",
        vec![
            Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
            Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
            Rule::token_to(
                r"(?m)\$\{#?",
                STRING_INTERPOL,
                NewState::Push(vec![r"curly"]),
            ),
            Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
            Rule::token(r"(?m)\$", TEXT),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)(?s)(\\\\|\\[0-7]+|\\.|[^"\\$])+"#, STRING_DOUBLE),
            Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
            Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
            Rule::token_to(
                r"(?m)\$\{#?",
                STRING_INTERPOL,
                NewState::Push(vec![r"curly"]),
            ),
            Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
            Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
            Rule::token(r"(?m)\$", TEXT),
        ],
    );
    m.insert(r"curly", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m):-", KEYWORD),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
        Rule::token(r#"(?m)[^}:"\'`$\\]+"#, PUNCTUATION),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&", PUNCTUATION),
        Rule::token(r"(?m)\|", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token(r"(?m)<", TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    m.insert(r"paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&", PUNCTUATION),
        Rule::token(r"(?m)\|", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token(r"(?m)<", TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    m.insert(r"math", vec![
        Rule::token_to(r"(?m)\)\)", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)\*\*|\|\||<<|>>|[-+*/%^|&<>]", OPERATOR),
        Rule::token(r"(?m)\d+#[\da-zA-Z]+", NUMBER),
        Rule::token(r"(?m)\d+#(?! )", NUMBER),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER),
        Rule::token(r"(?m)\d+", NUMBER),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&", PUNCTUATION),
        Rule::token(r"(?m)\|", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token(r"(?m)<", TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    m.insert(r"backticks", vec![
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Pop(1)),
        Rule::bygroups(r"(?m)\b(if|fi|else|while|in|do|done|for|then|return|function|case|select|break|continue|until|esac|elif)(\s*)\b", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)\b(alias|bg|bind|builtin|caller|cd|command|compgen|complete|declare|dirs|disown|echo|enable|eval|exec|exit|export|false|fc|fg|getopts|hash|help|history|jobs|kill|let|local|logout|popd|printf|pushd|pwd|read|readonly|set|shift|shopt|source|suspend|test|time|times|trap|true|type|typeset|ulimit|umask|unalias|unset|wait)(?=[\s)`])", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(\+?=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]", OPERATOR),
        Rule::token(r"(?m)<<<", OPERATOR),
        Rule::token(r"(?m)<<-?\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m)&&|\|\|", OPERATOR),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)\$?"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(?s)\$'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)(?s)'.*?'", STRING_SINGLE),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)&", PUNCTUATION),
        Rule::token(r"(?m)\|", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\d+\b", NUMBER),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\<&|;]+"#, TEXT),
        Rule::token(r"(?m)<", TEXT),
        Rule::token_to(r"(?m)\$\(\(", KEYWORD, NewState::Push(vec![r"math"])),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)\$(?:\d+|[#$?!_*@-])", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    Table(m)
}

impl Lexer for BashLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
