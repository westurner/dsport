#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:XtlangLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:XtlangLexer:extempore

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: extempore
pub struct ExtemporeLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"xtlang", vec![
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)(?<=bind-func\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-val\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-type\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-alias\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-poly\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-lib\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-dylib\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-lib-func\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?<=bind-lib-val\s)[\w.!-]+", NAME_FUNCTION),
        Rule::token(r"(?m):[\]{}\[\w<>,*/|!-]+", KEYWORD_TYPE),
        Rule::token(r"(?m)(<[\]{}\[\w<>,*/|!-]+>|\|[\]{}\[\w<>,*/|!-]+\||/[\]{}\[\w<>,*/|!-]+/|[\]{}\[\w<>,*/|!-]+\*)\**", KEYWORD_TYPE),
        Rule::token(r"(?m)(?<=\()(c(?:(?:as|onver)t)|do(?:loop|times)|letz|memzone)", KEYWORD),
        Rule::token(r"(?m)(?<=\()(<(?:[<>])|>>|a(?:cos|fill!|lloc|ng\-names|r(?:ef(?:(?:\-ptr)?)|ray(?:(?:\-(?:fill!|ref(?:(?:\-ptr)?)|set!))?))|s(?:et!|in)|tan)|b(?:itcast|or)|c(?:allback|eiling|l(?:osure\-(?:ref|set!)|run\->)|os|ref|set!)|dtof|exp(?:(?:t)?)|f(?:loor|ree|tod)|h(?:(?:(?:eap\-)?)alloc)|i(?:1to(?:i(?:32|64|8)|[df])|32to(?:i(?:64|[18])|[df])|64to(?:i(?:32|[18])|[df])|8to(?:i(?:1|32|64)|[df])|fret|mpc_null)|l(?:ist|lvm_(?:(?:(?:s)?)printf)|og)|m(?:ake\-(?:array|env(?:(?:\-zone)?)|tuple)|emzone)|n(?:il|ow|ull)|p(?:dref|fill!|o(?:inter\-(?:fill!|ref(?:(?:\-ptr)?)|set!)|p_zone)|r(?:ef(?:(?:\-ptr)?)|intf)|set!|ush_zone)|r(?:et\->|ound)|s(?:alloc|chedule|in|printf|qrt|tack\-alloc)|t(?:an|bind|fill!|oString|r(?:ef(?:(?:\-ptr)?)|uncate)|set!|uple(?:(?:\-(?:fill!|ref(?:(?:\-ptr)?)|set!))?))|v(?:ector\-fill!|fill!|oid|ref|set!)|z(?:(?:(?:one\-)?)alloc)|[&~])", NAME_FUNCTION),
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(#b|#o|#x)[\d.]+", NUMBER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(#t|#f)", NAME_CONSTANT),
        Rule::token(r"(?m)(?<=\()(and|begin|cond|define|else|for\-each|if|l(?:ambda|et)|map|or|set!)", KEYWORD),
        Rule::token(r"(?m)(?<=\()(<=|>=|a(?:bs|cos|ngle|pp(?:end|ly)|s(?:in|s(?:oc|[qv]))|tan)|boolean\?|c(?:a(?:a(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|d(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|llback|r)|d(?:(?:(?:a(?:a(?:[ad])|d(?:[ad])|[ad])|d(?:a(?:[ad])|d(?:[ad])|[ad])|[ad])?)r)|eiling|o(?:(?:(?:n)?)s))|floor|l(?:ength|ist|og)|m(?:ax|ember|in|odulo)|n(?:o(?:[tw])|ull\?)|println|r(?:andom|everse|ound)|s(?:in|qrt|ubstring)|tan|[%*+\-/<=>])", NAME_FUNCTION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
        Rule::token(r"(?m)[\w.!-]+", NAME_VARIABLE),
    ]);
    m.insert(r"common", vec![
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(#b|#o|#x)[\d.]+", NUMBER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(#t|#f)", NAME_CONSTANT),
        Rule::token(r"(?m)(?<=\()(and|begin|cond|define|else|for\-each|if|l(?:ambda|et)|map|or|set!)", KEYWORD),
        Rule::token(r"(?m)(?<=\()(<=|>=|a(?:bs|cos|ngle|pp(?:end|ly)|s(?:in|s(?:oc|[qv]))|tan)|boolean\?|c(?:a(?:a(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|d(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|llback|r)|d(?:(?:(?:a(?:a(?:[ad])|d(?:[ad])|[ad])|d(?:a(?:[ad])|d(?:[ad])|[ad])|[ad])?)r)|eiling|o(?:(?:(?:n)?)s))|floor|l(?:ength|ist|og)|m(?:ax|ember|in|odulo)|n(?:o(?:[tw])|ull\?)|println|r(?:andom|everse|ound)|s(?:in|qrt|ubstring)|tan|[%*+\-/<=>])", NAME_FUNCTION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
    ]);
    m.insert(r"scheme", vec![
        Rule::token(r"(?m)'[\w!$%&*+,/:<=>?@^~|-]+", STRING_SYMBOL),
        Rule::token(r#"(?m)#\\([()/'\"._!§$%& ?=+-]|[a-zA-Z0-9]+)"#, STRING_CHAR),
        Rule::token(r"(?m)('|#|`|,@|,|\.)", OPERATOR),
        Rule::token(r"(?m)(?<=\()(case|d(?:elay|o)|eval|let(?:\*|rec)|qu(?:(?:(?:asiqu)?)ote)|unquote(?:(?:\-splicing)?))", KEYWORD),
        Rule::token(r"(?m)(?<=\()(c(?:all(?:\-with\-(?:current\-continuation|input\-file|output\-file|values)|/cc)|har(?:\-(?:>integer|alphabetic\?|ci(?:(?:(?:[<>])=|[<=>])\?)|downcase|lower\-case\?|numeric\?|ready\?|up(?:case|per\-case\?)|whitespace\?)|(?:(?:(?:[<>])=|[<=>])?)\?)|lose\-(?:(?:in|out)put\-port)|omplex\?|urrent\-(?:(?:in|out)put\-port))|d(?:enominator|isplay|ynamic\-wind)|e(?:of\-object\?|q(?:(?:(?:ual|v)?)\?)|ven\?|x(?:act(?:\->inexact|\?)|p(?:(?:t)?)))|force|gcd|i(?:mag\-part|n(?:exact(?:\->exact|\?)|put\-port\?|te(?:ger(?:\->char|\?)|raction\-environment)))|l(?:cm|ist(?:\-(?:>(?:string|vector)|ref|tail)|\?)|oad)|m(?:a(?:gnitude|ke\-(?:polar|rectangular|string|vector))|em(?:[qv]))|n(?:e(?:gative\?|wline)|u(?:ll\-environment|m(?:ber(?:\->string|\?)|erator)))|o(?:dd\?|pen\-(?:(?:in|out)put\-file)|utput\-port\?)|p(?:air\?|eek\-char|(?:o(?:rt|sitive)|rocedure)\?)|quotient|r(?:ational(?:\?|ize)|e(?:a(?:d(?:(?:\-char)?)|l(?:\-part|\?))|mainder))|s(?:cheme\-report\-environment|et\-c(?:(?:[ad])r!)|tring(?:(?:\-(?:>(?:list|number|symbol)|append|c(?:i(?:(?:(?:[<>])=|[<=>])\?)|opy)|fill!|length|ref|set!)|(?:(?:(?:[<>])=|[<=>])?)\?)?)|ymbol(?:\->string|\?))|tr(?:anscript\-o(?:ff|n)|uncate)|v(?:alues|ector(?:(?:\-(?:>list|fill!|length)|\?)?))|w(?:ith\-(?:(?:input\-from|output\-to)\-file)|rite(?:(?:\-char)?))|zero\?)", NAME_FUNCTION),
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(#b|#o|#x)[\d.]+", NUMBER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(#t|#f)", NAME_CONSTANT),
        Rule::token(r"(?m)(?<=\()(and|begin|cond|define|else|for\-each|if|l(?:ambda|et)|map|or|set!)", KEYWORD),
        Rule::token(r"(?m)(?<=\()(<=|>=|a(?:bs|cos|ngle|pp(?:end|ly)|s(?:in|s(?:oc|[qv]))|tan)|boolean\?|c(?:a(?:a(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|d(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|llback|r)|d(?:(?:(?:a(?:a(?:[ad])|d(?:[ad])|[ad])|d(?:a(?:[ad])|d(?:[ad])|[ad])|[ad])?)r)|eiling|o(?:(?:(?:n)?)s))|floor|l(?:ength|ist|og)|m(?:ax|ember|in|odulo)|n(?:o(?:[tw])|ull\?)|println|r(?:andom|everse|ound)|s(?:in|qrt|ubstring)|tan|[%*+\-/<=>])", NAME_FUNCTION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
        Rule::token(r"(?m)[\w!$%&*+,/:<=>?@^~|-]+", NAME_VARIABLE),
    ]);
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)(?<=\()(bind\-(?:alias|dylib|func|lib(?:(?:\-(?:func|val))?)|poly|type|val))\b", KEYWORD, NewState::Push(vec![r"xtlang"])),
        Rule::token(r"(?m)'[\w!$%&*+,/:<=>?@^~|-]+", STRING_SYMBOL),
        Rule::token(r#"(?m)#\\([()/'\"._!§$%& ?=+-]|[a-zA-Z0-9]+)"#, STRING_CHAR),
        Rule::token(r"(?m)('|#|`|,@|,|\.)", OPERATOR),
        Rule::token(r"(?m)(?<=\()(case|d(?:elay|o)|eval|let(?:\*|rec)|qu(?:(?:(?:asiqu)?)ote)|unquote(?:(?:\-splicing)?))", KEYWORD),
        Rule::token(r"(?m)(?<=\()(c(?:all(?:\-with\-(?:current\-continuation|input\-file|output\-file|values)|/cc)|har(?:\-(?:>integer|alphabetic\?|ci(?:(?:(?:[<>])=|[<=>])\?)|downcase|lower\-case\?|numeric\?|ready\?|up(?:case|per\-case\?)|whitespace\?)|(?:(?:(?:[<>])=|[<=>])?)\?)|lose\-(?:(?:in|out)put\-port)|omplex\?|urrent\-(?:(?:in|out)put\-port))|d(?:enominator|isplay|ynamic\-wind)|e(?:of\-object\?|q(?:(?:(?:ual|v)?)\?)|ven\?|x(?:act(?:\->inexact|\?)|p(?:(?:t)?)))|force|gcd|i(?:mag\-part|n(?:exact(?:\->exact|\?)|put\-port\?|te(?:ger(?:\->char|\?)|raction\-environment)))|l(?:cm|ist(?:\-(?:>(?:string|vector)|ref|tail)|\?)|oad)|m(?:a(?:gnitude|ke\-(?:polar|rectangular|string|vector))|em(?:[qv]))|n(?:e(?:gative\?|wline)|u(?:ll\-environment|m(?:ber(?:\->string|\?)|erator)))|o(?:dd\?|pen\-(?:(?:in|out)put\-file)|utput\-port\?)|p(?:air\?|eek\-char|(?:o(?:rt|sitive)|rocedure)\?)|quotient|r(?:ational(?:\?|ize)|e(?:a(?:d(?:(?:\-char)?)|l(?:\-part|\?))|mainder))|s(?:cheme\-report\-environment|et\-c(?:(?:[ad])r!)|tring(?:(?:\-(?:>(?:list|number|symbol)|append|c(?:i(?:(?:(?:[<>])=|[<=>])\?)|opy)|fill!|length|ref|set!)|(?:(?:(?:[<>])=|[<=>])?)\?)?)|ymbol(?:\->string|\?))|tr(?:anscript\-o(?:ff|n)|uncate)|v(?:alues|ector(?:(?:\-(?:>list|fill!|length)|\?)?))|w(?:ith\-(?:(?:input\-from|output\-to)\-file)|rite(?:(?:\-char)?))|zero\?)", NAME_FUNCTION),
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)(#b|#o|#x)[\d.]+", NUMBER),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)(#t|#f)", NAME_CONSTANT),
        Rule::token(r"(?m)(?<=\()(and|begin|cond|define|else|for\-each|if|l(?:ambda|et)|map|or|set!)", KEYWORD),
        Rule::token(r"(?m)(?<=\()(<=|>=|a(?:bs|cos|ngle|pp(?:end|ly)|s(?:in|s(?:oc|[qv]))|tan)|boolean\?|c(?:a(?:a(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|d(?:(?:(?:a(?:[ad])|d(?:[ad])|[ad])?)r)|llback|r)|d(?:(?:(?:a(?:a(?:[ad])|d(?:[ad])|[ad])|d(?:a(?:[ad])|d(?:[ad])|[ad])|[ad])?)r)|eiling|o(?:(?:(?:n)?)s))|floor|l(?:ength|ist|og)|m(?:ax|ember|in|odulo)|n(?:o(?:[tw])|ull\?)|println|r(?:andom|everse|ound)|s(?:in|qrt|ubstring)|tan|[%*+\-/<=>])", NAME_FUNCTION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
        Rule::token(r"(?m)[\w!$%&*+,/:<=>?@^~|-]+", NAME_VARIABLE),
    ]);
    Table(m)
}

impl Lexer for ExtemporeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
