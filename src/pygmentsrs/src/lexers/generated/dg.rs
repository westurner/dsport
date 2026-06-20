#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.python:DgLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.python:DgLexer:dg

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dg
pub struct DgLexer;

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
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)(?i)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)(?i)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)(?i)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?m)(?i)[+-]?[0-9]+\.[0-9]+(e[+-]?[0-9]+)?j?", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)[+-]?[0-9]+e[+-]?\d+j?", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)[+-]?[0-9]+j?", NUMBER_INTEGER),
        Rule::token_to(r"(?m)(?i)(br|r?b?)'''", STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r#"(?m)(?i)(br|r?b?)""""#, STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r"(?m)(?i)(br|r?b?)'", STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token_to(r#"(?m)(?i)(br|r?b?)""#, STRING, NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)`\w+'*`", OPERATOR),
        Rule::token(r"(?m)\b(and|in|is|or|where)\b", OPERATOR_WORD),
        Rule::token(r"(?m)[!$%&*+\-./:<-@\\^|~;,]+", OPERATOR),
        Rule::token(r"(?m)(?<!\.)(b(?:ool|yte(?:array|s))|c(?:lassmethod|omplex)|dict(?:(?:')?)|f(?:(?:loa|rozense)t)|int|list(?:(?:')?)|memoryview|object|property|range|s(?:et(?:(?:')?)|lice|t(?:aticmethod|r)|uper)|t(?:uple(?:(?:')?)|ype))(?![\'\w])", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(__import__|a(?:bs|ll|ny)|bin(?:(?:d)?)|c(?:hr|mp|omp(?:ile|lex))|d(?:elattr|i(?:r|vmod)|rop(?:(?:while)?))|e(?:numerate|val|xhaust)|f(?:ilter|lip|o(?:ldl1\?|rmat)|st)|g(?:etattr|lobals)|h(?:as(?:attr|h)|e(?:ad|x))|i(?:d|n(?:(?:i|pu)t)|s(?:instance|subclass)|ter(?:(?:ate)?))|l(?:ast|en|ocals)|m(?:a(?:[px])|in)|next|o(?:ct|pen|rd)|p(?:ow|rint)|r(?:e(?:pr|versed)|ound)|s(?:canl1\?|etattr|nd|orted|um)|ta(?:il|ke(?:(?:while)?))|vars|zip)(?![\'\w])", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|Ellipsis|NotImplemented|None|True|False)(?!['\w])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)[A-Z]\w*(Error|Exception|Warning)'*(?!['\w])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?<!\.)(Exception|GeneratorExit|KeyboardInterrupt|StopIteration|SystemExit)(?!['\w])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?<![\w.])(except|finally|for|if|import|not|otherwise|raise|subclass|while|with|yield)(?!['\w])", KEYWORD_RESERVED),
        Rule::token(r"(?m)[A-Z_]+'*(?!['\w])", NAME),
        Rule::token(r"(?m)[A-Z]\w+'*(?!['\w])", KEYWORD_TYPE),
        Rule::token(r"(?m)\w+'*", NAME),
        Rule::token(r"(?m)[()]", PUNCTUATION),
        Rule::token(r"(?m).", ERROR),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(
        r"tsqs",
        vec![Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1))],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
            Rule::token(r#"(?m)[\'"\\]"#, STRING),
            Rule::token(r"(?m)%", STRING),
            Rule::token(r"(?m)\n", STRING),
        ],
    );
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(
        r"tdqs",
        vec![Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1))],
    );
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(
        r"sqs",
        vec![Rule::token_to(r"(?m)'", STRING, NewState::Pop(1))],
    );
    m.insert(r"_tmp_2", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(
        r"dqs",
        vec![Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1))],
    );
    m.insert(r"_tmp_3", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    Table(m)
}

impl Lexer for DgLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
