#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:HyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:HyLexer:hylang

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hylang, hy
pub struct HylangLexer;

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
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[ \t\n\r\f\v]+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r#"(?m)'[^ \t\n\r\f\v()\[\]{};\"'`~]+"#, STRING_SYMBOL),
        Rule::token(r"(?m)\\(.|[a-z]+)", STRING_CHAR),
        Rule::bygroups(r#"(?m)^(\s*)([rRuU]{,2}"""(?:.|\n)*?""")"#, vec![Some(TEXT), Some(STRING_DOC)]),
        Rule::bygroups(r"(?m)^(\s*)([rRuU]{,2}'''(?:.|\n)*?''')", vec![Some(TEXT), Some(STRING_DOC)]),
        Rule::token(r#"(?m)::?[^ \t\n\r\f\v()\[\]{};\"'`~]+"#, STRING_SYMBOL),
        Rule::token(r"(?m)~@|[`\'#^~&@]", OPERATOR),
        Rule::token(r"(?m)(a(?:s(?:(?:sert|ync)?)|wait)|break|continue|del|e(?:l(?:if|se)|xcept)|f(?:inally|or)|global|if|lambda|nonlocal|pass|r(?:aise|eturn)|try|w(?:hile|ith)|yield(?:(?:\ from)?))\b", KEYWORD),
        Rule::token(r"(?m)((?:Fals|Non|Tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<!\.)(__import__|a(?:bs|iter|ll|ny)|b(?:in|ool|reakpoint|yte(?:array|s))|c(?:allable|hr|lassmethod|omp(?:ile|lex))|d(?:elattr|i(?:ct|r|vmod))|e(?:numerate|val)|f(?:ilter|(?:loa|orma|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:(?:(?:pu)?)t)|s(?:instance|subclass)|ter)|l(?:en|ist|ocals)|m(?:a(?:[px])|emoryview|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|r(?:int|operty))|r(?:ange|e(?:pr|versed)|ound)|s(?:et(?:(?:attr)?)|lice|orted|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|vars|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|Ellipsis|NotImplemented|cls)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|B(?:aseException|lockingIOError|rokenPipeError|ufferError|ytesWarning)|C(?:(?:hildProcess|onnection(?:(?:Aborted|Re(?:fused|set))?))Error)|DeprecationWarning|E(?:OFError|n(?:codingWarning|vironmentError)|xception)|F(?:ile(?:(?:Exists|NotFound)Error)|loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|(?:n(?:de(?:ntation|x)|terrupted)|sADirectory)Error)|Key(?:Error|boardInterrupt)|LookupError|M(?:(?:emory|oduleNotFound)Error)|N(?:(?:ame|ot(?:ADirectory|Implemented))Error)|O(?:(?:S|verflow)Error)|P(?:e(?:ndingDeprecationWarning|rmissionError)|rocessLookupError)|R(?:e(?:cursionError|ferenceError|sourceWarning)|untime(?:Error|Warning))|S(?:top(?:(?:(?:Async)?)Iteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|imeout|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|V(?:(?:MS|alue)Error)|W(?:arning|indowsError)|ZeroDivisionError)\b", NAME_EXCEPTION),
        Rule::token(r"(?m)(\->(?:(?:>)?)|<<=|>>=|assoc|c(?:ar|dr|ond)|do|eval\-(?:(?:and|when)\-compile)|f(?:irst|or(?:(?:each)?))|get|i(?:mport|s\-not|[ns])|kwapply|l(?:et|ist_comp)|not\-in|progn|qu(?:(?:(?:asiqu)?)ote)|rest|slice|un(?:less|quote(?:(?:\-splice)?))|w(?:h(?:en|ile)|ith\-decorator)|[,|~]) ", KEYWORD),
        Rule::token(r"(?m)(def(?:(?:class|macro|(?:(?:u)?)n)?)|fn|lambda|setv) ", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(cycle|d(?:ec|istinct|rop)|even\?|filter|i(?:n(?:c|stance\?)|tera(?:ble\?|t(?:e|or\?)))|n(?:eg\?|one\?|th|umeric\?)|odd\?|pos\?|re(?:move|peat(?:(?:edly)?))|take(?:(?:_(?:nth|while))?)|zero\?) ", NAME_BUILTIN),
        Rule::token(r#"(?m)(?<=\()[^ \t\n\r\f\v()\[\]{};\"'`~]+"#, NAME_FUNCTION),
        Rule::token(r#"(?m)[^ \t\n\r\f\v()\[\]{};\"'`~]+"#, NAME_VARIABLE),
        Rule::token(r"(?m)(\[|\])", PUNCTUATION),
        Rule::token(r"(?m)(\{|\})", PUNCTUATION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
    ]);
    m.insert(r"py-keywords", vec![
        Rule::token(r"(?m)(a(?:s(?:(?:sert|ync)?)|wait)|break|continue|del|e(?:l(?:if|se)|xcept)|f(?:inally|or)|global|if|lambda|nonlocal|pass|r(?:aise|eturn)|try|w(?:hile|ith)|yield(?:(?:\ from)?))\b", KEYWORD),
        Rule::token(r"(?m)((?:Fals|Non|Tru)e)\b", KEYWORD_CONSTANT),
    ]);
    m.insert(r"py-builtins", vec![
        Rule::token(r"(?m)(?<!\.)(__import__|a(?:bs|iter|ll|ny)|b(?:in|ool|reakpoint|yte(?:array|s))|c(?:allable|hr|lassmethod|omp(?:ile|lex))|d(?:elattr|i(?:ct|r|vmod))|e(?:numerate|val)|f(?:ilter|(?:loa|orma|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:(?:(?:pu)?)t)|s(?:instance|subclass)|ter)|l(?:en|ist|ocals)|m(?:a(?:[px])|emoryview|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|r(?:int|operty))|r(?:ange|e(?:pr|versed)|ound)|s(?:et(?:(?:attr)?)|lice|orted|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|vars|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|Ellipsis|NotImplemented|cls)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|B(?:aseException|lockingIOError|rokenPipeError|ufferError|ytesWarning)|C(?:(?:hildProcess|onnection(?:(?:Aborted|Re(?:fused|set))?))Error)|DeprecationWarning|E(?:OFError|n(?:codingWarning|vironmentError)|xception)|F(?:ile(?:(?:Exists|NotFound)Error)|loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|(?:n(?:de(?:ntation|x)|terrupted)|sADirectory)Error)|Key(?:Error|boardInterrupt)|LookupError|M(?:(?:emory|oduleNotFound)Error)|N(?:(?:ame|ot(?:ADirectory|Implemented))Error)|O(?:(?:S|verflow)Error)|P(?:e(?:ndingDeprecationWarning|rmissionError)|rocessLookupError)|R(?:e(?:cursionError|ferenceError|sourceWarning)|untime(?:Error|Warning))|S(?:top(?:(?:(?:Async)?)Iteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|imeout|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|V(?:(?:MS|alue)Error)|W(?:arning|indowsError)|ZeroDivisionError)\b", NAME_EXCEPTION),
    ]);
    Table(m)
}

impl Lexer for HylangLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
