#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.modeling:BugsLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.modeling:BugsLexer:bugs

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bugs, winbugs, openbugs
pub struct BugsLexer;

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
    m.insert(r"comments", vec![Rule::token(r"(?m)#.*$", COMMENT_SINGLE)]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::bygroups(r"(?m)(model)(\s+)(\{)", vec![Some(KEYWORD_NAMESPACE), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(for|in)(?![\w.])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(abs|arccos|arccosh|arcsin|arcsinh|arctan|arctanh|cloglog|cos|cosh|cumulative|cut|density|deviance|equals|expr|gammap|ilogit|icloglog|integral|log|logfact|loggam|logit|max|min|phi|post.p.value|pow|prior.p.value|probit|replicate.post|replicate.prior|round|sin|sinh|solution|sqrt|step|tan|tanh|trunc|inprod|interp.lin|inverse|logdet|mean|eigen.vals|ode|prod|p.valueM|rank|ranked|replicate.postM|sd|sort|sum|D|I|F|T|C|dbern|dbin|dcat|dnegbin|dpois|dhyper|dbeta|dchisqr|ddexp|dexp|dflat|dgamma|dgev|df|dggamma|dgpar|dloglik|dlnorm|dlogis|dnorm|dpar|dt|dunif|dweib|dmulti|ddirch|dmnorm|dmt|dwish)(?=\s*\()", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Za-z][\w.]*", NAME),
        Rule::token(r"(?m)[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?", NUMBER),
        Rule::token(r"(?m)\[|\]|\(|\)|:|,|;", PUNCTUATION),
        Rule::token(r"(?m)<-|~", OPERATOR),
        Rule::token(r"(?m)\+|-|\*|/", OPERATOR),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for BugsLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
