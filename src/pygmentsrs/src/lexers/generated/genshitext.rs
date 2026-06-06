//! AUTO-GENERATED from `pygments.pygments.lexers.templates:GenshiTextLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.templates:GenshiTextLexer:genshitext

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: genshitext
pub struct GenshitextLexer;

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
        Rule::token(r"(?m)[^#$\s]+", OTHER),
        Rule::bygroups(r"(?m)^(\s*)(##.*)$", vec![Some(TEXT), Some(COMMENT)]),
        Rule::bygroups_to(r"(?m)^(\s*)(#)", vec![Some(TEXT), Some(COMMENT_PREPROC)], NewState::Push(vec![r"directive"])),
        Rule::bygroups_g(r"(?m)(?<!\$)(\$\{)(.+?)(\})", vec![Some(GroupAction::Token(COMMENT_PREPROC)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(COMMENT_PREPROC))]),
        Rule::token(r"(?m)(?<!\$)(\$)([a-zA-Z_][\w.]*)", NAME_VARIABLE),
        Rule::token(r"(?m)[#$\s]", OTHER),
    ]);
    m.insert(r"variable", vec![
        Rule::bygroups_g(r"(?m)(?<!\$)(\$\{)(.+?)(\})", vec![Some(GroupAction::Token(COMMENT_PREPROC)), Some(GroupAction::UsingLexer { alias: "python", state: None }), Some(GroupAction::Token(COMMENT_PREPROC))]),
        Rule::token(r"(?m)(?<!\$)(\$)([a-zA-Z_][\w.]*)", NAME_VARIABLE),
    ]);
    m.insert(r"directive", vec![
        Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
        Rule::using_lexer_to(r"(?m)(?:def|for|if)\s+.*", "python", None, NewState::Pop(1)),
        Rule::bygroups_g_to(r"(?m)(choose|when|with)([^\S\n]+)(.*)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::Token(TEXT)), Some(GroupAction::UsingLexer { alias: "python", state: None })], NewState::Pop(1)),
        Rule::token_to(r"(?m)(choose|otherwise)\b", KEYWORD, NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(end\w*)([^\S\n]*)(.*)", vec![Some(KEYWORD), Some(TEXT), Some(COMMENT)], NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for GenshitextLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
