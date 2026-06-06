//! AUTO-GENERATED from `pygments.pygments.lexers.pawn:SourcePawnLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.pawn:SourcePawnLexer:sp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sp
pub struct SpLexer;

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
        Rule::token_to(r"(?m)^#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)^\s*(?:/[*].*?[*]/\s*)*#if\s+0", COMMENT_PREPROC, NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)^\s*(?:/[*].*?[*]/\s*)*#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)/(\\\n)?/(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
        Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)L?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[LlUu]*", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[LlUu]*", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[LlUu]*", NUMBER_OCT),
        Rule::token(r"(?m)\d+[LlUu]*", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.;]", PUNCTUATION),
        Rule::token(r"(?m)(case|const|continue|native|default|else|enum|for|if|new|operator|public|return|sizeof|static|decl|struct|switch)\b", KEYWORD),
        Rule::token(r"(?m)(bool|Float)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(r"macro", vec![
        Rule::token(r"(?m)[^/\n]+", COMMENT_PREPROC),
        Rule::token(r"(?m)/\*(.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)//.*?\n", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)/", COMMENT_PREPROC),
        Rule::token(r"(?m)(?<=\\)\n", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\n", COMMENT_PREPROC, NewState::Pop(1)),
    ]);
    m.insert(r"if0", vec![
        Rule::token_to(r"(?m)^\s*#if.*?(?<!\\)\n", COMMENT_PREPROC, NewState::PushSame),
        Rule::token_to(r"(?m)^\s*#endif.*?(?<!\\)\n", COMMENT_PREPROC, NewState::Pop(1)),
        Rule::token(r"(?m).*?\n", COMMENT),
    ]);
    Table(m)
}

impl Lexer for SpLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
