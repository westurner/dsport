#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sieve:SieveLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sieve:SieveLexer:sieve

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sieve
pub struct SieveLexer;

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
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)[();,{}\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(?i)require", KEYWORD_NAMESPACE),
        Rule::bygroups(r"(?m)(?i)(:)(addresses|all|contains|content|create|copy|comparator|count|days|detail|domain|fcc|flags|from|handle|importance|is|localpart|length|lowerfirst|lower|matches|message|mime|options|over|percent|quotewildcard|raw|regex|specialuse|subject|text|under|upperfirst|upper|value)", vec![Some(NAME_TAG), Some(NAME_TAG)]),
        Rule::token(r"(?m)(?i)(address|addflag|allof|anyof|body|discard|elsif|else|envelope|ereject|exists|false|fileinto|if|hasflag|header|keep|notify_method_capability|notify|not|redirect|reject|removeflag|setflag|size|spamtest|stop|string|true|vacation|virustest)", NAME_BUILTIN),
        Rule::token(r"(?m)(?i)set", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?m)([0-9.]+)([kmgKMG])?", vec![Some(NUMBER), Some(NUMBER)]),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)/\*.*\*/", COMMENT_MULTILINE),
        Rule::token(r#"(?m)"[^"]*?""#, STRING),
        Rule::token_to(r"(?m)text:", NAME_TAG, NewState::Push(vec![r"text"])),
    ]);
    m.insert(
        r"text",
        vec![
            Rule::token(r"(?m)[^.].*?\n", STRING),
            Rule::token_to(r"(?m)^\.", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for SieveLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
