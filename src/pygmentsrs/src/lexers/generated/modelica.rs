#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.modeling:ModelicaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.modeling:ModelicaLexer:modelica

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: modelica
pub struct ModelicaLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?ms)[\s\ufeff]+", TEXT),
            Rule::token(r"(?ms)//[^\n]*\n?", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?ms)[\s\ufeff]+", TEXT),
        Rule::token(r"(?ms)//[^\n]*\n?", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?ms)[()\[\]{},;]+", PUNCTUATION),
        Rule::token(r"(?ms)\.?[*^/+-]|\.|<>|[<>:=]=?", OPERATOR),
        Rule::token(r"(?ms)\d+(\.?\d*[eE][-+]?\d+|\.\d*)", NUMBER_FLOAT),
        Rule::token(r"(?ms)\d+", NUMBER_INTEGER),
        Rule::token(r"(?ms)(abs|acos|actualStream|array|asin|assert|AssertionLevel|atan|atan2|backSample|Boolean|cardinality|cat|ceil|change|Clock|Connections|cos|cosh|cross|delay|diagonal|div|edge|exp|ExternalObject|fill|floor|getInstanceName|hold|homotopy|identity|inStream|integer|Integer|interval|inverse|isPresent|linspace|log|log10|matrix|max|min|mod|ndims|noClock|noEvent|ones|outerProduct|pre|previous|product|Real|reinit|rem|rooted|sample|scalar|semiLinear|shiftSample|sign|sin|sinh|size|skew|smooth|spatialDistribution|sqrt|StateSelect|String|subSample|sum|superSample|symmetric|tan|tanh|terminal|terminate|time|transpose|vector|zeros)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(algorithm|annotation|break|connect|constant|constrainedby|der|discrete|each|else|elseif|elsewhen|encapsulated|enumeration|equation|exit|expandable|extends|external|firstTick|final|flow|for|if|import|impure|in|initial|inner|input|interval|loop|nondiscrete|outer|output|parameter|partial|protected|public|pure|redeclare|replaceable|return|stream|then|when|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?ms)(and|not|or)\b", OPERATOR_WORD),
        Rule::token_to(r"(?ms)(block|class|connector|end|function|model|operator|package|record|type)\b", KEYWORD_RESERVED, NewState::Push(vec![r"class"])),
        Rule::token(r"(?ms)(false|true)\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?ms)within\b", KEYWORD_RESERVED, NewState::Push(vec![r"package-prefix"])),
        Rule::token(r"(?ms)(?:'(?:[^\\']|\\.)+'|[a-zA-Z_]\w*)", NAME),
    ]);
    m.insert(
        r"class",
        vec![
            Rule::token(r"(?ms)[\s\ufeff]+", TEXT),
            Rule::token(r"(?ms)//[^\n]*\n?", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?ms)(function|record)\b", KEYWORD_RESERVED),
            Rule::token_to(
                r"(?ms)(if|for|when|while)\b",
                KEYWORD_RESERVED,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?ms)(?:'(?:[^\\']|\\.)+'|[a-zA-Z_]\w*)",
                NAME_CLASS,
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"package-prefix",
        vec![
            Rule::token(r"(?ms)[\s\ufeff]+", TEXT),
            Rule::token(r"(?ms)//[^\n]*\n?", COMMENT_SINGLE),
            Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token_to(
                r"(?ms)(?:'(?:[^\\']|\\.)+'|[a-zA-Z_]\w*)",
                NAME_NAMESPACE,
                NewState::Pop(1),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?ms)\\[\'"?\\abfnrtv]"#, STRING_ESCAPE),
            Rule::using_lexer(
                r#"(?ms)(?i)<\s*html\s*>([^\\"]|\\.)+?(<\s*/\s*html\s*>|(?="))"#,
                "html",
                None,
            ),
            Rule::token(r#"(?ms)<|\\?[^"\\<]+"#, STRING_DOUBLE),
        ],
    );
    Table(m)
}

impl Lexer for ModelicaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
