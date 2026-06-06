//! AUTO-GENERATED from `pygments.pygments.lexers.asm:Ca65Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:Ca65Lexer:ca65

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ca65
pub struct Ca65Lexer;

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
        Rule::token(r"(?im);.*", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)[a-z_.@$][\w.@$]*:", NAME_LABEL),
        Rule::token(r"(?im)((ld|st)[axy]|(in|de)[cxy]|asl|lsr|ro[lr]|adc|sbc|cmp|cp[xy]|cl[cvdi]|se[cdi]|jmp|jsr|bne|beq|bpl|bmi|bvc|bvs|bcc|bcs|p[lh][ap]|rt[is]|brk|nop|ta[xy]|t[xy]a|txs|tsx|and|ora|eor|bit)\b", KEYWORD),
        Rule::token(r"(?im)\.\w+", KEYWORD_PSEUDO),
        Rule::token(r"(?im)[-+~*/^&|!<>=]", OPERATOR),
        Rule::token(r#"(?im)"[^"\n]*."#, STRING),
        Rule::token(r"(?im)'[^'\n]*.", STRING_CHAR),
        Rule::token(r"(?im)\$[0-9a-f]+|[0-9a-f]+h\b", NUMBER_HEX),
        Rule::token(r"(?im)\d+", NUMBER_INTEGER),
        Rule::token(r"(?im)%[01]+", NUMBER_BIN),
        Rule::token(r"(?im)[#,.:()=\[\]]", PUNCTUATION),
        Rule::token(r"(?im)[a-z_.@$][\w.@$]*", NAME),
    ]);
    Table(m)
}

impl Lexer for Ca65Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
