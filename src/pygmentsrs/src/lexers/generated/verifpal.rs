//! AUTO-GENERATED from `pygments.pygments.lexers.verifpal:VerifpalLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.verifpal:VerifpalLexer:verifpal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: verifpal
pub struct VerifpalLexer;

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
        Rule::token(r"(?m)//.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(principal)( +)(\w+)( *)(\[)(.*)$", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(STRING), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(attacker)( *)(\[)( *)(passive|active)( *)(\])( *)$", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)(knows)( +)(private|public)( +)", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(KEYWORD_CONSTANT), Some(WHITESPACE)], NewState::Push(vec![r"shared"])),
        Rule::bygroups_to(r"(?m)(queries)( +)(\[)", vec![Some(NAME_BUILTIN), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"queries"])),
        Rule::bygroups_to(r"(?m)(\w+)( +)(->|→)( *)(\w+)( *)(\:)", vec![Some(STRING), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(STRING), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"shared"])),
        Rule::token_to(r"(?m)((?:generate|leak)s)\b", NAME_BUILTIN, NewState::Push(vec![r"shared"])),
        Rule::token(r"(?m)(p(?:hase|recondition))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[\[\(\)\]\?:=→^,]", PUNCTUATION),
        Rule::token(r"(?m)->", PUNCTUATION),
        Rule::token(r"(?m)(password)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(A(?:EAD_(?:(?:DE|EN)C)|SSERT)|BLIND|CONCAT|DEC|ENC|H(?:ASH|KDF)|MAC|P(?:KE_(?:(?:DE|EN)C)|W_HASH)|RINGSIGN(?:(?:VERIF)?)|S(?:HAMIR_(?:JOIN|SPLIT)|IGN(?:(?:VERIF)?)|PLIT)|UNBLIND|nil|[G_])\b", NAME_FUNCTION),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
    ]);
    m.insert(r"shared", vec![
        Rule::token(r"(?m)[\^\[\],]", PUNCTUATION),
        Rule::token(r"(?m) +", WHITESPACE),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"queries", vec![
        Rule::token(r"(?m)\s+", NAME_VARIABLE),
        Rule::bygroups_to(r"(?m)((?:authentication|confidentiality|equivalence|freshness|unlinkability)\?)( )", vec![Some(KEYWORD_PSEUDO), Some(WHITESPACE)], NewState::Push(vec![r"shared"])),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for VerifpalLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
