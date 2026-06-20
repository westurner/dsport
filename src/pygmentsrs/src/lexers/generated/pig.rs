#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:PigLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:PigLexer:pig

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: pig
pub struct PigLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)--.*", COMMENT),
        Rule::token(r"(?im)/\*[\w\W]*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?im)\\$", STRING_ESCAPE),
        Rule::token(r"(?im)\\", TEXT),
        Rule::token(r"(?im)\'(?:\\[ntbrf\\\']|\\u[0-9a-f]{4}|[^\'\\\n\r])*\'", STRING),
        Rule::token(r"(?im)(assert|and|any|all|arrange|as|asc|bag|by|cache|CASE|cat|cd|cp|%declare|%default|define|dense|desc|describe|distinct|du|dump|eval|exex|explain|filter|flatten|foreach|full|generate|group|help|if|illustrate|import|inner|input|into|is|join|kill|left|limit|load|ls|map|matches|mkdir|mv|not|null|onschema|or|order|outer|output|parallel|pig|pwd|quit|register|returns|right|rm|rmf|rollup|run|sample|set|ship|split|stderr|stdin|stdout|store|stream|through|union|using|void)\b", KEYWORD),
        Rule::token(r"(?im)(bytearray|BIGINTEGER|BIGDECIMAL|chararray|datetime|double|float|int|long|tuple)\b", KEYWORD_TYPE),
        Rule::token(r"(?im)(AVG|BinStorage|cogroup|CONCAT|copyFromLocal|copyToLocal|COUNT|cross|DIFF|MAX|MIN|PigDump|PigStorage|SIZE|SUM|TextLoader|TOKENIZE)\b", NAME_BUILTIN),
        Rule::token(r"(?im)[;(){}\[\]]", PUNCTUATION),
        Rule::token(r"(?im)[#=,./%+\-?]", OPERATOR),
        Rule::token(r"(?im)(eq|gt|lt|gte|lte|neq|matches)\b", OPERATOR),
        Rule::token(r"(?im)(==|<=|<|>=|>|!=)", OPERATOR),
        Rule::token(r"(?im)[0-9]*\.[0-9]+(e[0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?im)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)[0-9]+L?", NUMBER_INTEGER),
        Rule::token(r"(?im)\n", WHITESPACE),
        Rule::bygroups(r"(?im)([a-z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?im)[()#:]", TEXT),
        Rule::token(r#"(?im)[^(:#\'")\s]+"#, TEXT),
        Rule::token(r"(?im)\S+\s+", TEXT),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)(assert|and|any|all|arrange|as|asc|bag|by|cache|CASE|cat|cd|cp|%declare|%default|define|dense|desc|describe|distinct|du|dump|eval|exex|explain|filter|flatten|foreach|full|generate|group|help|if|illustrate|import|inner|input|into|is|join|kill|left|limit|load|ls|map|matches|mkdir|mv|not|null|onschema|or|order|outer|output|parallel|pig|pwd|quit|register|returns|right|rm|rmf|rollup|run|sample|set|ship|split|stderr|stdin|stdout|store|stream|through|union|using|void)\b", KEYWORD),
    ]);
    m.insert(r"types", vec![
        Rule::token(r"(?im)(bytearray|BIGINTEGER|BIGDECIMAL|chararray|datetime|double|float|int|long|tuple)\b", KEYWORD_TYPE),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?im)(AVG|BinStorage|cogroup|CONCAT|copyFromLocal|copyToLocal|COUNT|cross|DIFF|MAX|MIN|PigDump|PigStorage|SIZE|SUM|TextLoader|TOKENIZE)\b", NAME_BUILTIN),
    ]);
    m.insert(
        r"punct",
        vec![Rule::token(r"(?im)[;(){}\[\]]", PUNCTUATION)],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?im)[#=,./%+\-?]", OPERATOR),
            Rule::token(r"(?im)(eq|gt|lt|gte|lte|neq|matches)\b", OPERATOR),
            Rule::token(r"(?im)(==|<=|<|>=|>|!=)", OPERATOR),
        ],
    );
    Table(m)
}

impl Lexer for PigLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
