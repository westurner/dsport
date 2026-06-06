//! AUTO-GENERATED from `pygments.pygments.lexers.templates:EvoqueLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:EvoqueLexer:evoque

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: evoque
pub struct EvoqueLexer;

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
        Rule::token(r"(?ms)[^#$]+", OTHER),
        Rule::token_to(r"(?ms)#\[", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?ms)\$\$", OTHER),
        Rule::token(r"(?ms)\$\w+:[^$\n]*\$", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(\$)(begin|end)(\{(%)?)(.*?)((?(4)%)\})", vec![Some(PUNCTUATION), Some(NAME_BUILTIN), Some(PUNCTUATION), None, Some(STRING), Some(PUNCTUATION)]),
        Rule::bygroups_g(r#"(?ms)(\$)(evoque|overlay)(\{(%)?)(\s*[#\w\-"\'.]+)?(.*?)((?(4)%)\})"#, vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(NAME_BUILTIN)), Some(GroupAction::Token(PUNCTUATION)), None, Some(GroupAction::Token(STRING)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups_g(r"(?ms)(\$)(\w+)(\{(%)?)(.*?)((?(4)%)\})", vec![Some(GroupAction::Token(PUNCTUATION)), Some(GroupAction::Token(NAME_BUILTIN)), Some(GroupAction::Token(PUNCTUATION)), None, Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::bygroups(r"(?ms)(\$)(else|rof|fi)", vec![Some(PUNCTUATION), Some(NAME_BUILTIN)]),
        Rule::bygroups_g(r"(?ms)(\$\{(%)?)(.*?)((!)(.*?))?((?(2)%)\})", vec![Some(GroupAction::Token(PUNCTUATION)), None, Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(NAME_BUILTIN)), None, None, Some(GroupAction::Token(PUNCTUATION))]),
        Rule::token(r"(?ms)#", OTHER),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?ms)[^\]#]", COMMENT_MULTILINE),
        Rule::token_to(r"(?ms)#\[", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?ms)\]#", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?ms)[\]#]", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for EvoqueLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
