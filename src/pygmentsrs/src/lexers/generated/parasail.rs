//! AUTO-GENERATED from `pygments.pygments.lexers.parasail:ParaSailLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.parasail:ParaSailLexer:parasail

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: parasail
pub struct ParasailLexer;

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
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\b(and|or|xor)=", OPERATOR_WORD),
        Rule::token(r"(?m)\b(and(\s+then)?|or(\s+else)?|xor|rem|mod|(is|not)\s+null)\b", OPERATOR_WORD),
        Rule::token(r"(?m)\b(abs|abstract|all|block|class|concurrent|const|continue|each|end|exit|extends|exports|forward|func|global|implements|import|in|interface|is|lambda|locked|new|not|null|of|op|optional|private|queued|ref|return|reverse|separate|some|type|until|var|with|if|then|else|elsif|case|for|while|loop)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(abstract\s+)?(interface|class|op|func|type)", KEYWORD_DECLARATION),
        Rule::token(r#"(?m)"[^"]*""#, STRING),
        Rule::token(r#"(?m)\\[\'ntrf"0]"#, STRING_ESCAPE),
        Rule::token(r"(?m)#[a-zA-Z]\w*", LITERAL),
        Rule::token(r"(?m)\d[0-9_]*#[0-9a-fA-F][0-9a-fA-F_]*#", NUMBER_HEX),
        Rule::token(r"(?m)0[xX][0-9a-fA-F][0-9a-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token(r"(?m)\d[0-9_]*\.\d[0-9_]*[eE][+-]\d[0-9_]*", NUMBER_FLOAT),
        Rule::token(r"(?m)\d[0-9_]*\.\d[0-9_]*", NUMBER_FLOAT),
        Rule::token(r"(?m)\d[0-9_]*", NUMBER_INTEGER),
        Rule::token(r"(?m)'[^']'", STRING_CHAR),
        Rule::token(r"(?m)[a-zA-Z]\w*", NAME),
        Rule::token(r"(?m)(<==|==>|<=>|\*\*=|<\|=|<<=|>>=|==|!=|=\?|<=|>=|\*\*|<<|>>|=>|:=|\+=|-=|\*=|\|=|\||/=|\+|-|\*|/|\.\.|<\.\.|\.\.<|<\.\.<)", OPERATOR),
        Rule::token(r"(?m)(<|>|\[|\]|\(|\)|\||:|;|,|.|\{|\}|->)", PUNCTUATION),
        Rule::token(r"(?m)\n+", TEXT),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)\d[0-9_]*#[0-9a-fA-F][0-9a-fA-F_]*#", NUMBER_HEX),
        Rule::token(r"(?m)0[xX][0-9a-fA-F][0-9a-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token(r"(?m)\d[0-9_]*\.\d[0-9_]*[eE][+-]\d[0-9_]*", NUMBER_FLOAT),
        Rule::token(r"(?m)\d[0-9_]*\.\d[0-9_]*", NUMBER_FLOAT),
        Rule::token(r"(?m)\d[0-9_]*", NUMBER_INTEGER),
    ]);
    Table(m)
}

impl Lexer for ParasailLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
