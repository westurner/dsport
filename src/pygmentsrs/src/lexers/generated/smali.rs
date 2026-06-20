//! AUTO-GENERATED from `pygments.pygments.lexers.dalvik:SmaliLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dalvik:SmaliLexer:smali

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: smali
pub struct SmaliLexer;

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
        Rule::token(r"(?m)#.*?\n", COMMENT),
        Rule::token(r"(?m):\w+", NAME_LABEL),
        Rule::bygroups(r"(?m)(\$?\b)([\w$]*)(:)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)<(?:cl)?init>", NAME_FUNCTION),
        Rule::bygroups(r"(?m)(\$?\b)([\w$]*)(\()", vec![Some(PUNCTUATION), Some(NAME_FUNCTION), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(L)((?:[\w$]+/)*)([\w$]+)(;)", vec![Some(KEYWORD_TYPE), Some(TEXT), Some(NAME_CLASS), Some(TEXT)]),
        Rule::bygroups(r"(?m)^([ \t]*)(\.(?:class|super|implements|field|subannotation|annotation|enum|method|registers|locals|array-data|packed-switch|sparse-switch|catchall|catch|line|parameter|local|prologue|epilogue|source))", vec![Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^([ \t]*)(\.end)( )(field|subannotation|annotation|method|array-data|packed-switch|sparse-switch|parameter|local)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^([ \t]*)(\.restart)( )(local)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?m)(public|private|protected|static|final|synchronized|bridge|varargs|native|abstract|strictfp|synthetic|constructor|declared-synchronized|interface|enum|annotation|volatile|transient)", KEYWORD),
        Rule::token(r"(?m)\b[vp]\d+\b", NAME_BUILTIN),
        Rule::bygroups(r"(?m)(\b[a-z][A-Za-z0-9/-]+)(\s+)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r#"(?m)".*""#, STRING),
        Rule::token(r"(?m)0x[0-9A-Fa-f]+t?", NUMBER_HEX),
        Rule::token(r"(?m)[0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+L?", NUMBER_INTEGER),
        Rule::token(r"(?m)->", PUNCTUATION),
        Rule::token(r"(?m)[{},():=.-]", PUNCTUATION),
        Rule::token(r"(?m)[ZBSCIJFDV\[]+", KEYWORD_TYPE),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"comment", vec![Rule::token(r"(?m)#.*?\n", COMMENT)]);
    m.insert(r"label", vec![Rule::token(r"(?m):\w+", NAME_LABEL)]);
    m.insert(
        r"field",
        vec![Rule::bygroups(
            r"(?m)(\$?\b)([\w$]*)(:)",
            vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)],
        )],
    );
    m.insert(
        r"method",
        vec![
            Rule::token(r"(?m)<(?:cl)?init>", NAME_FUNCTION),
            Rule::bygroups(
                r"(?m)(\$?\b)([\w$]*)(\()",
                vec![Some(PUNCTUATION), Some(NAME_FUNCTION), Some(PUNCTUATION)],
            ),
        ],
    );
    m.insert(
        r"class",
        vec![Rule::bygroups(
            r"(?m)(L)((?:[\w$]+/)*)([\w$]+)(;)",
            vec![Some(KEYWORD_TYPE), Some(TEXT), Some(NAME_CLASS), Some(TEXT)],
        )],
    );
    m.insert(r"directive", vec![
        Rule::bygroups(r"(?m)^([ \t]*)(\.(?:class|super|implements|field|subannotation|annotation|enum|method|registers|locals|array-data|packed-switch|sparse-switch|catchall|catch|line|parameter|local|prologue|epilogue|source))", vec![Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^([ \t]*)(\.end)( )(field|subannotation|annotation|method|array-data|packed-switch|sparse-switch|parameter|local)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?m)^([ \t]*)(\.restart)( )(local)", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
    ]);
    m.insert(r"access-modifier", vec![
        Rule::token(r"(?m)(public|private|protected|static|final|synchronized|bridge|varargs|native|abstract|strictfp|synthetic|constructor|declared-synchronized|interface|enum|annotation|volatile|transient)", KEYWORD),
    ]);
    m.insert(
        r"instruction",
        vec![
            Rule::token(r"(?m)\b[vp]\d+\b", NAME_BUILTIN),
            Rule::bygroups(
                r"(?m)(\b[a-z][A-Za-z0-9/-]+)(\s+)",
                vec![Some(TEXT), Some(WHITESPACE)],
            ),
        ],
    );
    m.insert(
        r"literal",
        vec![
            Rule::token(r#"(?m)".*""#, STRING),
            Rule::token(r"(?m)0x[0-9A-Fa-f]+t?", NUMBER_HEX),
            Rule::token(r"(?m)[0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
            Rule::token(r"(?m)[0-9]+L?", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"punctuation",
        vec![
            Rule::token(r"(?m)->", PUNCTUATION),
            Rule::token(r"(?m)[{},():=.-]", PUNCTUATION),
        ],
    );
    m.insert(
        r"type",
        vec![Rule::token(r"(?m)[ZBSCIJFDV\[]+", KEYWORD_TYPE)],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for SmaliLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
