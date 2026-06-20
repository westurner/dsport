#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.modeling:JagsLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.modeling:JagsLexer:jags

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jags
pub struct JagsLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"whitespace", vec![Rule::token(r"(?m)\s+", TEXT)]);
    m.insert(r"names", vec![Rule::token(r"(?m)[a-zA-Z][\w.]*\b", NAME)]);
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)(?s)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::bygroups(r"(?m)(model|data)(\s+)(\{)", vec![Some(KEYWORD_NAMESPACE), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)var(?![\w.])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(for|in)(?![\w.])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(abs|arccos|arccosh|arcsin|arcsinh|arctan|arctanh|cos|cosh|cloglog|equals|exp|icloglog|ifelse|ilogit|log|logfact|loggam|logit|phi|pow|probit|round|sin|sinh|sqrt|step|tan|tanh|trunc|inprod|interp.lin|logdet|max|mean|min|prod|sum|sd|inverse|rank|sort|t|acos|acosh|asin|asinh|atan|T|I|[dpq]bern|[dpq]beta|[dpq]dchiqsqr|[dpq]ddexp|[dpq]dexp|[dpq]df|[dpq]gamma|[dpq]gen.gamma|[dpq]logis|[dpq]lnorm|[dpq]negbin|[dpq]nchisqr|[dpq]norm|[dpq]par|[dpq]pois|[dpq]weib|dt|dunif|dbetabin|dbern|dbin|dcat|dhyper|ddirch|dmnorm|dwish|dmt|dmulti|dbinom|dchisq|dnbinom|dweibull|ddirich)(?=\s*\()", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z][\w.]*\b", NAME),
        Rule::token(r"(?m)[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?", NUMBER),
        Rule::token(r"(?m)\[|\]|\(|\)|:|,|;", PUNCTUATION),
        Rule::token(r"(?m)<-|~", OPERATOR),
        Rule::token(r"(?m)\+|-|\*|\/|\|\|[&]{2}|[<>=]=?|\^|%.*?%", OPERATOR),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for JagsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
