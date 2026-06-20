#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.shell:ExeclineLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.shell:ExeclineLexer:execline

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: execline
pub struct ExeclineLexer;

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
        Rule::token(r"(?m)\b(background|backtick|cd|define|dollarat|elgetopt|elgetpositionals|elglob|emptyenv|envfile|exec|execlineb|exit|export|fdblock|fdclose|fdmove|fdreserve|fdswap|forbacktickx|foreground|forstdin|forx|getcwd|getpid|heredoc|homeof|if|ifelse|ifte|ifthenelse|importas|loopwhilex|multidefine|multisubstitute|pipeline|piperw|posix-cd|redirfd|runblock|shift|trap|tryexec|umask|unexport|wait|withstdinas)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[{}]", OPERATOR),
        Rule::token(r#"(?m)(?s)"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^\s{}$"\\]+"#, TEXT),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[\w@#]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    m.insert(r"basic", vec![
        Rule::token(r"(?m)\b(background|backtick|cd|define|dollarat|elgetopt|elgetpositionals|elglob|emptyenv|envfile|exec|execlineb|exit|export|fdblock|fdclose|fdmove|fdreserve|fdswap|forbacktickx|foreground|forstdin|forx|getcwd|getpid|heredoc|homeof|if|ifelse|ifte|ifthenelse|importas|loopwhilex|multidefine|multisubstitute|pipeline|piperw|posix-cd|redirfd|runblock|shift|trap|tryexec|umask|unexport|wait|withstdinas)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[{}]", OPERATOR),
    ]);
    m.insert(
        r"data",
        vec![
            Rule::token(r#"(?m)(?s)"(\\.|[^"\\$])*""#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)[^\s{}$"\\]+"#, TEXT),
        ],
    );
    m.insert(
        r"interp",
        vec![
            Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
            Rule::token(r"(?m)\$[\w@#]+", NAME_VARIABLE),
            Rule::token(r"(?m)\$", TEXT),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)(?s)(\\\\|\\.|[^"\\$])+"#, STRING_DOUBLE),
            Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
            Rule::token(r"(?m)\$[\w@#]+", NAME_VARIABLE),
            Rule::token(r"(?m)\$", TEXT),
        ],
    );
    m.insert(r"curly", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)[\w#@]+", NAME_VARIABLE),
        Rule::token(r"(?m)\b(background|backtick|cd|define|dollarat|elgetopt|elgetpositionals|elglob|emptyenv|envfile|exec|execlineb|exit|export|fdblock|fdclose|fdmove|fdreserve|fdswap|forbacktickx|foreground|forstdin|forx|getcwd|getpid|heredoc|homeof|if|ifelse|ifte|ifthenelse|importas|loopwhilex|multidefine|multisubstitute|pipeline|piperw|posix-cd|redirfd|runblock|shift|trap|tryexec|umask|unexport|wait|withstdinas)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\A#!.+\n", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[{}]", OPERATOR),
        Rule::token(r#"(?m)(?s)"(\\.|[^"\\$])*""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^\s{}$"\\]+"#, TEXT),
        Rule::token_to(r"(?m)\$\{", STRING_INTERPOL, NewState::Push(vec![r"curly"])),
        Rule::token(r"(?m)\$[\w@#]+", NAME_VARIABLE),
        Rule::token(r"(?m)\$", TEXT),
    ]);
    Table(m)
}

impl Lexer for ExeclineLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
