//! AUTO-GENERATED from `pygments.pygments.lexers.asn1:Asn1Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asn1:Asn1Lexer:asn1

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: asn1
pub struct Asn1Lexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)--.*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\d+\.\d+([eE][-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)&?[a-z][-a-zA-Z0-9]*[a-zA-Z0-9]\b", NAME_VARIABLE),
        Rule::token(r"(?m)(FALSE|M(?:AX|IN(?:(?:US\-INFINITY)?))|NULL|PLUS\-INFINITY|TRUE)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(OBJECT\s+IDENTIFIER|BIT\s+STRING|OCTET\s+STRING|CHARACTER\s+STRING|EMBEDDED\s+PDV)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(B(?:MPString|OOLEAN)|EXTERNAL|G(?:eneral(?:String|izedTime)|raphicString)|I(?:A5String|NTEGER|SO646String)|NumericString|ObjectDescriptor|PrintableString|RE(?:AL|LATIVE\-OID)|T(?:61String|YPE\-IDENTIFIER|eletexString)|U(?:T(?:CTime|F8String)|niversalString)|Vi(?:(?:deotex|sible)String))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)EXPORTS\s+ALL\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)((?:EX|IM)PORTS)\b", TokenType::new(&["Operator", "Namespace"])),
        Rule::token(r"(?m)(SEQUENCE\s+OF|SET\s+OF|INSTANCE\s+OF|WITH\s+SYNTAX)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(A(?:BSENT|LL)|C(?:HOICE|LASS|ONTAINING)|DEFAULT|ENUMERATED|OPTIONAL|P(?:ATTERN|RESENT)|S(?:E(?:QUENCE|T)|IZE)|UNIQUE)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(EXCEPT|(?:INTERSECT|UN)ION)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(A(?:BSTRACT\-SYNTAX|PPLICATION|UTOMATIC)|B(?:EGIN|Y)|CO(?:MPONENT(?:(?:S)?)|NSTRAINED)|DEFINITIONS|E(?:N(?:(?:(?:CODE)?)D)|X(?:PLICIT|TENSIBILITY))|FROM|I(?:MPLI(?:CIT|ED)|NCLUDES)|OF|PRIVATE|TAGS|UNIVERSAL|WITH)", KEYWORD),
        Rule::token(r"(?m)&?[A-Z][-a-zA-Z0-9]*[a-zA-Z0-9]\b", TokenType::new(&["Name", "Type"])),
        Rule::token(r"(?m)(::=|\.\.\.|\.\.|\[\[|\]\]|\||\^|-)", OPERATOR),
        Rule::token(r"(?m)(\.|,|\{|\}|\(|\)|\[|\])", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::bygroups(r"(?m)('[01 ]*')(B)\b", vec![Some(STRING), Some(STRING_AFFIX)]),
        Rule::bygroups(r"(?m)('[0-9A-F ]*')(H)\b", vec![Some(STRING), Some(STRING_AFFIX)]),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)"""#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"]"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for Asn1Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
