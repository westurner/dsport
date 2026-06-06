//! AUTO-GENERATED from `pygments.pygments.lexers.business:GoodDataCLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.business:GoodDataCLLexer:gooddata_cl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gooddata-cl
pub struct GooddataClLexer;

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
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
        Rule::token(r"(?im)[a-z]\w*", NAME_FUNCTION),
        Rule::token_to(r"(?im)\(", PUNCTUATION, NewState::Push(vec![r"args-list"])),
        Rule::token(r"(?im);", PUNCTUATION),
        Rule::token(r"(?im)\s+", TEXT),
    ]);
    m.insert(r"args-list", vec![
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?im),", PUNCTUATION),
        Rule::token(r"(?im)[a-z]\w*", NAME_VARIABLE),
        Rule::token(r"(?im)=", OPERATOR),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string-literal"])),
        Rule::token(r"(?im)[0-9]+(?:\.[0-9]+)?(?:e[+-]?[0-9]{1,3})?", NUMBER),
        Rule::token(r"(?im)\s", WHITESPACE),
    ]);
    m.insert(r"string-literal", vec![
        Rule::token(r#"(?im)\\[tnrfbae"\\]"#, STRING_ESCAPE),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?im)[^\\"]+"#, STRING),
    ]);
    Table(m)
}

impl Lexer for GooddataClLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
