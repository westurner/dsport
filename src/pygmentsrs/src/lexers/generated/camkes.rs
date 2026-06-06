//! AUTO-GENERATED from `pygments.pygments.lexers.esoteric:CAmkESLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.esoteric:CAmkESLexer:camkes

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: camkes, idl4
pub struct CamkesLexer;

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
        Rule::bygroups(r"(?m)^(\s*)(#.*)(\n)", vec![Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT),
        Rule::token(r"(?m)//.*$", COMMENT),
        Rule::token(r"(?m)[\[(){},.;\]]", PUNCTUATION),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)(Dataport(?:(?:s)?)|Event(?:(?:s)?)|Interface|Procedure(?:(?:s)?)|a(?:ssembly|ttribute)|co(?:mpo(?:nent|sition)|n(?:figuration|nect(?:ion|or)|sumes|trol))|dataport|e(?:mits|(?:ven|xpor)t)|from|group|ha(?:rdware|s)|interface|maybe|pro(?:cedure|vides)|t(?:emplate|hread(?:(?:s)?)|o)|uses|with)\b", KEYWORD),
        Rule::token(r"(?m)(Buf|bool(?:(?:ean)?)|char(?:(?:acter)?)|double|float|in(?:(?:out|t(?:(?:16_6|32_t|64_t|8_t|eger)?))?)|mutex|out|re(?:al|fin)|s(?:emaphore|igned|tr(?:ing|uct))|u(?:int(?:(?:16|32|64|8|ptr)_t)|nsigned)|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*_(priority|domain|buffer)", KEYWORD_RESERVED),
        Rule::token(r"(?m)(dma_pool|(?:from|to)_access)\b", KEYWORD_RESERVED),
        Rule::bygroups(r#"(?m)(import)(\s+)((?:<[^>]*>|"[^"]*");)"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::bygroups(r#"(?m)(include)(\s+)((?:<[^>]*>|"[^"]*");)"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)]),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)-?[\d]+", NUMBER),
        Rule::token(r"(?m)-?[\d]+\.[\d]+", NUMBER_FLOAT),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token(r"(?m)[Tt]rue|[Ff]alse", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    Table(m)
}

impl Lexer for CamkesLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
