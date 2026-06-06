//! AUTO-GENERATED from `pygments.pygments.lexers.snobol:SnobolLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.snobol:SnobolLexer:snobol

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: snobol
pub struct SnobolLexer;

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
        Rule::token(r"(?m)\*.*\n", COMMENT),
        Rule::token_to(r"(?m)[+.] ", PUNCTUATION, NewState::Push(vec![r"statement"])),
        Rule::token(r"(?m)-.*\n", COMMENT),
        Rule::token_to(r"(?m)END\s*\n", NAME_LABEL, NewState::Push(vec![r"heredoc"])),
        Rule::token_to(r"(?m)[A-Za-z$][\w$]*", NAME_LABEL, NewState::Push(vec![r"statement"])),
        Rule::token_to(r"(?m)\s+", TEXT, NewState::Push(vec![r"statement"])),
    ]);
    m.insert(r"statement", vec![
        Rule::token_to(r"(?m)\s*\n", TEXT, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)(?<=[^\w.])(LT|LE|EQ|NE|GE|GT|INTEGER|IDENT|DIFFER|LGT|SIZE|REPLACE|TRIM|DUPL|REMDR|DATE|TIME|EVAL|APPLY|OPSYN|LOAD|UNLOAD|LEN|SPAN|BREAK|ANY|NOTANY|TAB|RTAB|REM|POS|RPOS|FAIL|FENCE|ABORT|ARB|ARBNO|BAL|SUCCEED|INPUT|OUTPUT|TERMINAL)(?=[^\w.])", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Za-z][\w.]*", NAME),
        Rule::token(r"(?m)\*\*|[?$.!%*/#+\-@|&\\=]", OPERATOR),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token(r"(?m)'[^']*'", STRING),
        Rule::token(r"(?m)[0-9]+(?=[^.EeDd])", NUMBER_INTEGER),
        Rule::token(r"(?m)[0-9]+(\.[0-9]*)?([EDed][-+]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"goto"])),
        Rule::token(r"(?m)[()<>,;]", PUNCTUATION),
    ]);
    m.insert(r"goto", vec![
        Rule::token_to(r"(?m)\s*\n", TEXT, NewState::Pop(2)),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)F|S", KEYWORD),
        Rule::bygroups(r"(?m)(\()([A-Za-z][\w.]*)(\))", vec![Some(PUNCTUATION), Some(NAME_LABEL), Some(PUNCTUATION)]),
    ]);
    m.insert(r"heredoc", vec![
        Rule::token(r"(?m).*\n", STRING_HEREDOC),
    ]);
    Table(m)
}

impl Lexer for SnobolLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
