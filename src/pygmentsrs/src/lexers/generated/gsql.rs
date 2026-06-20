#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.gsql:GSQLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.gsql:GSQLLexer:gsql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gsql
pub struct GsqlLexer;

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
        Rule::token(r"(?im)\#.*", COMMENT_SINGLE),
        Rule::token(r"(?im)/\*(.|\n)*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?im)(?<!\.)(A(?:CCUM|N(?:[DY])|PI|S(?:(?:C)?)|VG)|B(?:A(?:G|TCH)|ETWEEN|O(?:OL|TH)|REAK|Y)|C(?:A(?:SE|TCH)|O(?:ALESCE|MPRESS|NTINUE|UNT)|REATE)|D(?:ATETIME(?:(?:_(?:ADD|SUB))?)|E(?:LETE|SC)|ISTRIBUTED|O(?:(?:UBLE)?))|E(?:DGE|LSE|ND|SCAPE|XCEPTION)|F(?:ALSE|IL(?:E|TER)|LOAT|OR(?:(?:EACH)?)|ROM)|G(?:R(?:APH|OUP)|SQL_(?:INT_M(?:AX|IN)|UINT_MAX))|HAVING|I(?:N(?:SERT|T(?:(?:ER(?:PRET|SECT|VAL)|O)?))|SEMPTY|[FNS])|JSON(?:ARRAY|OBJECT)|L(?:ASTHOP|EADING|I(?:KE|(?:MI|S)T)|O(?:AD_ACCUM|G))|M(?:A(?:TCH|[PX])|IN(?:(?:US)?))|N(?:O(?:[TW])|ULL)|O(?:FFSET|R(?:(?:DER)?))|P(?:ATH|ER|INNED|OST(?:(?:[\-_])ACCUM)|RI(?:MARY_ID|NT))|QUERY|R(?:A(?:(?:IS|NG)E)|E(?:PLACE|SET_COLLECTION_ACCUM|TURN(?:(?:S)?))|UN)|S(?:AMPLE|E(?:LECT(?:(?:_VERTEX)?)|T)|RC|T(?:ATIC|RING)|UM|YNTAX)|T(?:A(?:(?:GSTG|RGE)T)|HEN|O(?:(?:_(?:CSV|DATETIME))?)|R(?:AILING|IM|UE|Y)|UPLE|YPEDEF)|U(?:INT|NION|PDATE)|V(?:ALUES|ERTEX)|W(?:H(?:E(?:N|RE)|ILE)|ITH))\b", KEYWORD),
        Rule::token(r"(?im)(accum|having|limit|order|postAccum|(?:sampl|wher)e)", NAME_BUILTIN),
        Rule::token(r"(?im)((?:MapA|(?:a(?:nd|rray|vg)|b(?:ag|itwise(?:and|or))|groupby|heap|list|m(?:ax|in)|or|s(?:et|um))a)ccum)", NAME_BUILTIN),
        Rule::bygroups_g(r"(?im)(-\s?)(\(.*\:\w?\))(\s?-)", vec![Some(GroupAction::Token(OPERATOR)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(OPERATOR))]),
        Rule::token(r"(?im)->|<-", OPERATOR),
        Rule::token(r"(?im)[.*{}\[\]\<\>\_]", PUNCTUATION),
        Rule::token(r#"(?im)"([^"\\]|\\.)*""#, STRING),
        Rule::token(r"(?im)@{1,2}\w+", NAME_VARIABLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)[a-z]\w*", NAME),
        Rule::token(r"(?im)(\d+\.\d+|\d+)", NUMBER),
        Rule::token(r"(?im)\$|[^0-9|\/|\-](\-\=|\+\=|\*\=|\\\=|\=|\=\=|\=\=\=|\+|\-|\*|\\|\+\=|\>|\<)[^\>|\/]", OPERATOR),
        Rule::token(r"(?im)(\||\(|\)|\,|\;|\=|\-|\+|\*|\/|\>|\<|\:)", OPERATOR),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?im)\#.*", COMMENT_SINGLE),
            Rule::token(r"(?im)/\*(.|\n)*?\*/", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)(?<!\.)(A(?:CCUM|N(?:[DY])|PI|S(?:(?:C)?)|VG)|B(?:A(?:G|TCH)|ETWEEN|O(?:OL|TH)|REAK|Y)|C(?:A(?:SE|TCH)|O(?:ALESCE|MPRESS|NTINUE|UNT)|REATE)|D(?:ATETIME(?:(?:_(?:ADD|SUB))?)|E(?:LETE|SC)|ISTRIBUTED|O(?:(?:UBLE)?))|E(?:DGE|LSE|ND|SCAPE|XCEPTION)|F(?:ALSE|IL(?:E|TER)|LOAT|OR(?:(?:EACH)?)|ROM)|G(?:R(?:APH|OUP)|SQL_(?:INT_M(?:AX|IN)|UINT_MAX))|HAVING|I(?:N(?:SERT|T(?:(?:ER(?:PRET|SECT|VAL)|O)?))|SEMPTY|[FNS])|JSON(?:ARRAY|OBJECT)|L(?:ASTHOP|EADING|I(?:KE|(?:MI|S)T)|O(?:AD_ACCUM|G))|M(?:A(?:TCH|[PX])|IN(?:(?:US)?))|N(?:O(?:[TW])|ULL)|O(?:FFSET|R(?:(?:DER)?))|P(?:ATH|ER|INNED|OST(?:(?:[\-_])ACCUM)|RI(?:MARY_ID|NT))|QUERY|R(?:A(?:(?:IS|NG)E)|E(?:PLACE|SET_COLLECTION_ACCUM|TURN(?:(?:S)?))|UN)|S(?:AMPLE|E(?:LECT(?:(?:_VERTEX)?)|T)|RC|T(?:ATIC|RING)|UM|YNTAX)|T(?:A(?:(?:GSTG|RGE)T)|HEN|O(?:(?:_(?:CSV|DATETIME))?)|R(?:AILING|IM|UE|Y)|UPLE|YPEDEF)|U(?:INT|NION|PDATE)|V(?:ALUES|ERTEX)|W(?:H(?:E(?:N|RE)|ILE)|ITH))\b", KEYWORD),
    ]);
    m.insert(
        r"clauses",
        vec![Rule::token(
            r"(?im)(accum|having|limit|order|postAccum|(?:sampl|wher)e)",
            NAME_BUILTIN,
        )],
    );
    m.insert(r"accums", vec![
        Rule::token(r"(?im)((?:MapA|(?:a(?:nd|rray|vg)|b(?:ag|itwise(?:and|or))|groupby|heap|list|m(?:ax|in)|or|s(?:et|um))a)ccum)", NAME_BUILTIN),
    ]);
    m.insert(
        r"relations",
        vec![
            Rule::bygroups_g(
                r"(?im)(-\s?)(\(.*\:\w?\))(\s?-)",
                vec![
                    Some(GroupAction::Token(OPERATOR)),
                    Some(GroupAction::UsingThis { state: None }),
                    Some(GroupAction::Token(OPERATOR)),
                ],
            ),
            Rule::token(r"(?im)->|<-", OPERATOR),
            Rule::token(r"(?im)[.*{}\[\]\<\>\_]", PUNCTUATION),
        ],
    );
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?im)"([^"\\]|\\.)*""#, STRING),
            Rule::token(r"(?im)@{1,2}\w+", NAME_VARIABLE),
        ],
    );
    m.insert(r"whitespace", vec![Rule::token(r"(?im)\s+", WHITESPACE)]);
    m.insert(
        r"barewords",
        vec![
            Rule::token(r"(?im)[a-z]\w*", NAME),
            Rule::token(r"(?im)(\d+\.\d+|\d+)", NUMBER),
        ],
    );
    m.insert(r"operators", vec![
        Rule::token(r"(?im)\$|[^0-9|\/|\-](\-\=|\+\=|\*\=|\\\=|\=|\=\=|\=\=\=|\+|\-|\*|\\|\+\=|\>|\<)[^\>|\/]", OPERATOR),
        Rule::token(r"(?im)(\||\(|\)|\,|\;|\=|\-|\+|\*|\/|\>|\<|\:)", OPERATOR),
    ]);
    Table(m)
}

impl Lexer for GsqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
