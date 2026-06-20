//! AUTO-GENERATED from `pygments.pygments.lexers.business:MaqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.business:MaqlLexer:maql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: maql
pub struct MaqlLexer;

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
        Rule::token(r"(?im)IDENTIFIER\b", NAME_BUILTIN),
        Rule::token(r"(?im)\{[^}]+\}", NAME_VARIABLE),
        Rule::token(r"(?im)[0-9]+(?:\.[0-9]+)?(?:e[+-]?[0-9]{1,3})?", NUMBER),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string-literal"])),
        Rule::token(r"(?im)\<\>|\!\=", OPERATOR),
        Rule::token(r"(?im)\=|\>\=|\>|\<\=|\<", OPERATOR),
        Rule::token(r"(?im)\:\=", OPERATOR),
        Rule::token(r"(?im)\[[^\]]+\]", NAME_VARIABLE_CLASS),
        Rule::token(r"(?im)(A(?:DD|L(?:IAS|L|TER)|ND|S(?:(?:C)?)|TTRIBUTE)|B(?:ETWEEN|IGINT|OTTOM|Y)|C(?:O(?:LUMN(?:(?:S)?)|UNT)|REATE)|D(?:AT(?:A(?:SET|TYPE)|E)|E(?:CIMAL|F(?:AULT|INE)|SC(?:(?:RIPTION)?))|IMENSION(?:(?:S)?)|OUBLE|ROP)|EXCEPT|F(?:A(?:CT|LSE)|ILTER|O(?:(?:(?:LDE)?)R)|ROM|ULLSET)|HYPERLINK|IN(?:(?:CLUDE|T)?)|KEY(?:(?:S)?)|L(?:ABEL(?:(?:S)?)|I(?:KE|MIT))|M(?:ATCH|ETRIC|ODIFY)|NOT|O(?:(?:RD|TH)ER|[NR])|P(?:ARENT|F|RIMARY)|R(?:EPORT|OW(?:(?:S)?))|S(?:ELECT|YNCHRONIZE)|T(?:ABLE|EMPLATE|ITLE|OP|(?:RU|YP)E)|V(?:ARCHAR|ISUAL)|W(?:HE(?:N|RE)|ITH(?:(?:OUT)?)))\b", KEYWORD),
        Rule::token(r"(?im)[a-z]\w*\b", NAME_FUNCTION),
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
        Rule::token(r"(?im)[,;()]", PUNCTUATION),
        Rule::token(r"(?im)\s+", WHITESPACE),
    ]);
    m.insert(
        r"string-literal",
        vec![
            Rule::token(r#"(?im)\\[tnrfbae"\\]"#, STRING_ESCAPE),
            Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?im)[^\\"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for MaqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
