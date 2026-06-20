#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:NewLispLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:NewLispLexer:newlisp

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: newlisp
pub struct NewlispLexer;

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
        Rule::token(r"(?im)#!(.*?)$", COMMENT_PREPROC),
        Rule::token(r"(?im);.*$", COMMENT_SINGLE),
        Rule::token(r"(?im)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r#"(?im)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token_to(r"(?im)\{", STRING, NewState::Push(vec![r"bracestring"])),
        Rule::token_to(r"(?im)\[text\]*", STRING, NewState::Push(vec![r"tagstring"])),
        Rule::token(r"(?im)('|:)", OPERATOR),
        Rule::token(r"(?im)(!=|\$(?:1(?:[012345])|args|i(?:dx|t)|main\-args|[0123456789])|\+\+|\-\-|<(?:[<=])|>(?:[=>])|Class|MAIN|NaN\?|Tree|a(?:b(?:ort|s)|cos(?:(?:h)?)|dd(?:(?:ress)?)|mb|nd|pp(?:end(?:(?:\-file)?)|ly)|r(?:gs|ray(?:(?:\-list|\?)?))|s(?:in(?:(?:h)?)|soc)|t(?:an(?:(?:[2h])?)|om\?))|b(?:a(?:se64\-(?:(?:de|en)c)|yes\-(?:query|train))|e(?:gin|ta(?:(?:i)?))|i(?:n(?:d|omial)|ts))|c(?:a(?:llback|se|tch)|eil|h(?:a(?:(?:(?:nge\-di)?)r)|op)|l(?:ean|ose)|o(?:mmand\-event|n(?:stant|text(?:(?:\?)?)|[ds])|py(?:(?:\-file)?)|s(?:(?:h)?)|unt)|pymem|r(?:c32|it\-(?:chi2|z))|urr(?:ent\-line|y))|d(?:ate(?:(?:\-(?:list|(?:pars|valu)e))?)|e(?:bug|f(?:\-new|ault|ine(?:(?:\-macro)?))|lete(?:(?:\-(?:file|url))?)|stroy|vice|[ct])|i(?:fference|rectory(?:(?:\?)?)|v)|o(?:\-(?:until|while)|args|list|string|t(?:imes|ree))|u(?:(?:(?:m)?)p))|e(?:mpty\?|n(?:crypt|ds\-with|v)|r(?:f|ror\-event)|val(?:(?:\-string)?)|x(?:ec|i(?:sts|t)|p(?:(?:and|lode)?)|tend))|f(?:actor|ft|i(?:l(?:e(?:\-info|\?)|ter)|nd(?:(?:\-all)?)|rst)|l(?:at|o(?:at(?:(?:\?)?)|or)|t)|or(?:(?:\-all|k|mat)?)|[nv])|g(?:amma(?:i|ln)|cd|et\-(?:char|float|int|long|string|url)|lobal(?:(?:\?)?))|i(?:f(?:(?:(?:\-no|f)t)?)|mport|n(?:dex|f\?|te(?:ger(?:(?:\?)?)|rsect)|vert|[ct])|rr)|join|l(?:a(?:mbda(?:(?:\-macro|\?)?)|st(?:(?:\-error)?))|e(?:gal\?|ngth|t(?:(?:ex|n)?))|ist(?:(?:\?)?)|o(?:ad|cal|g|okup|wer\-case))|m(?:a(?:cro\?|in\-args|ke\-dir|tch|[ptx])|ember|in|od(?:(?:ule)?)|ul(?:(?:tiply)?))|n(?:e(?:t\-(?:accept|c(?:lose|onnect)|e(?:rror|val)|i(?:nterface|pv)|l(?:isten|o(?:cal|okup))|p(?:acket|ee(?:[kr])|ing)|receive(?:(?:\-(?:from|udp))?)|se(?:lect|nd(?:(?:\-(?:to|udp))?)|rvice|ssions))|w)|il(?:(?:\?)?)|o(?:rmal|[tw])|p(?:er|v)|th|u(?:(?:ll|mber)\?))|o(?:pen|r|stype)|p(?:a(?:ck|rse(?:(?:\-date)?))|eek|ipe|mt|o(?:p\-assoc|st\-url|[pw])|r(?:e(?:fix|tty\-print)|i(?:mitive\?|nt(?:(?:ln)?))|o(?:b\-(?:chi2|z)|cess|mpt\-event|tected\?))|u(?:sh|t\-url)|v)|quote(?:(?:\?)?)|r(?:and(?:(?:om(?:(?:ize)?))?)|e(?:a(?:d(?:(?:\-(?:char|expr|file|key|line|utf8)|er\-event)?)|l\-path)|ceive|f(?:(?:\-all)?)|gex(?:(?:\-comp)?)|move\-dir|name\-file|place|s(?:(?:(?:e)?)t)|verse)|o(?:tate|und))|s(?:ave|e(?:arch|e(?:[dk])|l(?:ect|f)|maphore|nd|quence|ries|t(?:(?:\-(?:locale|ref(?:(?:\-all)?))|[fq])?))|gn|hare|i(?:gnal|lent|n(?:(?:h)?))|l(?:eep|ice)|o(?:rt|urce)|pawn|qrt|t(?:arts\-with|ring(?:(?:\?)?))|ub|wap|y(?:m(?:(?:bol(?:[?s]))?)|nc|s\-(?:error|info)))|t(?:an(?:(?:h)?)|erm|hrow(?:(?:\-error)?)|i(?:me(?:(?:\-of\-day|r)?)|tle\-case)|r(?:a(?:ce(?:(?:\-highlight)?)|nspose)|im|ue(?:(?:\?)?)))|u(?:n(?:i(?:code|fy|que)|less|pack|til)|pper\-case|tf8(?:(?:len)?)|uid)|w(?:ait\-pid|h(?:en|ile)|rite(?:(?:\-(?:char|(?:fil|lin)e))?))|x(?:fer\-event|ml\-(?:error|parse|type\-tags))|zero\?|[!$%&*+\-/:<=>?@\^|~])\b", KEYWORD),
        Rule::token(r"(?im)(?<=\()([\w!$%&*+.,/<=>?@^~|-])+|(\[.*?\])+", NAME_VARIABLE),
        Rule::token(r"(?im)([\w!$%&*+.,/<=>?@^~|-])+|(\[.*?\])+", STRING_SYMBOL),
        Rule::token(r"(?im)(\(|\))", PUNCTUATION),
    ]);
    m.insert(
        r"bracestring",
        vec![
            Rule::token_to(r"(?im)\{", STRING, NewState::PushSame),
            Rule::token_to(r"(?im)\}", STRING, NewState::Pop(1)),
            Rule::token(r"(?im)[^{}]+", STRING),
        ],
    );
    m.insert(
        r"tagstring",
        vec![Rule::token_to(
            r"(?im)(?s)(.*?)(\[/text\])",
            STRING,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for NewlispLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
