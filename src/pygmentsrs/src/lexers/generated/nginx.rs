//! AUTO-GENERATED from `pygments.pygments.lexers.configs:NginxConfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:NginxConfLexer:nginx

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nginx
pub struct NginxLexer;

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
                r"(?m)(include)(\s+)([^\s;]+)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME)],
            ),
            Rule::token_to(r"(?m)[^\s;#]+", KEYWORD, NewState::Push(vec![r"stmt"])),
            Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)on|off", NAME_CONSTANT),
            Rule::token(r"(?m)\$[^\s;#()]+", NAME_VARIABLE),
            Rule::bygroups(
                r"(?m)([a-z0-9.-]+)(:)([0-9]+)",
                vec![Some(NAME), Some(PUNCTUATION), Some(NUMBER_INTEGER)],
            ),
            Rule::token(r"(?m)[a-z-]+/[a-z-+]+", STRING),
            Rule::token(r"(?m)[0-9]+[km]?\b", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(~)(\s*)([^\s{]+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX)],
            ),
            Rule::token(r"(?m)[:=~]", PUNCTUATION),
            Rule::token(r"(?m)[^\s;#{}$]+", STRING),
            Rule::token(r"(?m)/[^\s;#]*", NAME),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[$;]", TEXT),
        ],
    );
    m.insert(
        r"base",
        vec![
            Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)on|off", NAME_CONSTANT),
            Rule::token(r"(?m)\$[^\s;#()]+", NAME_VARIABLE),
            Rule::bygroups(
                r"(?m)([a-z0-9.-]+)(:)([0-9]+)",
                vec![Some(NAME), Some(PUNCTUATION), Some(NUMBER_INTEGER)],
            ),
            Rule::token(r"(?m)[a-z-]+/[a-z-+]+", STRING),
            Rule::token(r"(?m)[0-9]+[km]?\b", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(~)(\s*)([^\s{]+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX)],
            ),
            Rule::token(r"(?m)[:=~]", PUNCTUATION),
            Rule::token(r"(?m)[^\s;#{}$]+", STRING),
            Rule::token(r"(?m)/[^\s;#]*", NAME),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[$;]", TEXT),
        ],
    );
    m.insert(
        r"block",
        vec![
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
            Rule::token_to(
                r"(?m)[^\s;#]+",
                KEYWORD_NAMESPACE,
                NewState::Push(vec![r"stmt"]),
            ),
            Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)on|off", NAME_CONSTANT),
            Rule::token(r"(?m)\$[^\s;#()]+", NAME_VARIABLE),
            Rule::bygroups(
                r"(?m)([a-z0-9.-]+)(:)([0-9]+)",
                vec![Some(NAME), Some(PUNCTUATION), Some(NUMBER_INTEGER)],
            ),
            Rule::token(r"(?m)[a-z-]+/[a-z-+]+", STRING),
            Rule::token(r"(?m)[0-9]+[km]?\b", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(~)(\s*)([^\s{]+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX)],
            ),
            Rule::token(r"(?m)[:=~]", PUNCTUATION),
            Rule::token(r"(?m)[^\s;#{}$]+", STRING),
            Rule::token(r"(?m)/[^\s;#]*", NAME),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[$;]", TEXT),
        ],
    );
    m.insert(
        r"stmt",
        vec![
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block"])),
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)#.*\n", COMMENT_SINGLE),
            Rule::token(r"(?m)on|off", NAME_CONSTANT),
            Rule::token(r"(?m)\$[^\s;#()]+", NAME_VARIABLE),
            Rule::bygroups(
                r"(?m)([a-z0-9.-]+)(:)([0-9]+)",
                vec![Some(NAME), Some(PUNCTUATION), Some(NUMBER_INTEGER)],
            ),
            Rule::token(r"(?m)[a-z-]+/[a-z-+]+", STRING),
            Rule::token(r"(?m)[0-9]+[km]?\b", NUMBER_INTEGER),
            Rule::bygroups(
                r"(?m)(~)(\s*)([^\s{]+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE), Some(STRING_REGEX)],
            ),
            Rule::token(r"(?m)[:=~]", PUNCTUATION),
            Rule::token(r"(?m)[^\s;#{}$]+", STRING),
            Rule::token(r"(?m)/[^\s;#]*", NAME),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[$;]", TEXT),
        ],
    );
    Table(m)
}

impl Lexer for NginxLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
