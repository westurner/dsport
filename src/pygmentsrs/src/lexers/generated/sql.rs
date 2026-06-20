#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sql:SqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:SqlLexer:sql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sql
pub struct SqlLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)--.*\n?", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token(r"(?im)(\ TEMP|A(?:B(?:ORT|S(?:(?:OLUTE)?))|CCESS|D(?:MIN|[AD])|FTER|GGREGATE|L(?:IAS|L(?:(?:OCATE)?)|TER)|N(?:ALY(?:(?:[SZ])E)|[DY])|RE|S(?:C|ENSITIVE|S(?:ERTION|IGNMENT)|YMMETRIC)|TOMIC|UTHORIZATION|VG|[ST])|B(?:ACKWARD|E(?:FORE|(?:GI|TWEE)N)|IT(?:VAR|_LENGTH)|OTH|READTH|Y)|C(?:A(?:CHE|LL(?:(?:ED)?)|RDINALITY|S(?:CADE(?:(?:D)?)|[ET])|TALOG(?:(?:_NAME)?))|H(?:A(?:IN|R(?:ACTER(?:ISTICS|_(?:LENGTH|SET_(?:CATALOG|NAME|SCHEMA)))|_LENGTH))|ECK(?:(?:ED|POINT)?))|L(?:ASS(?:(?:_ORIGIN)?)|O(?:B|SE)|USTER)|O(?:ALESCE|BOL|L(?:LAT(?:E|ION(?:(?:_(?:CATALOG|NAME|SCHEMA))?))|UMN(?:(?:_NAME)?))|M(?:M(?:AND_FUNCTION(?:(?:_CODE)?)|ENT|IT(?:(?:TED)?))|PLETION)|N(?:DITION_NUMBER|NECT(?:(?:ION(?:(?:_NAME)?))?)|STR(?:AINT(?:(?:S|_(?:CATALOG|NAME|SCHEMA))?)|UCTOR)|T(?:AINS|INUE)|VER(?:SION|T))|PY|RRESPONDING|UNT)|R(?:EATE(?:(?:DB|USER)?)|OSS)|U(?:BE|R(?:RENT(?:(?:_(?:DATE|PATH|ROLE|TIME(?:(?:STAMP)?)|USER))?)|SOR(?:(?:_NAME)?)))|YCLE)|D(?:A(?:T(?:A(?:(?:BASE)?)|ETIME_INTERVAL_(?:CODE|PRECISION))|Y)|E(?:ALLOCATE|CLARE|F(?:AULT(?:(?:S)?)|ERR(?:ABLE|ED)|INE(?:[DR]))|L(?:ETE|IMITER(?:(?:S)?))|REF|S(?:C(?:(?:RI(?:BE|PTOR))?)|TR(?:OY|UCTOR))|TERMINISTIC)|I(?:AGNOSTICS|CTIONARY|S(?:CONNECT|PATCH|TINCT))|O(?:(?:MAIN)?)|ROP|YNAMIC(?:(?:_FUNCTION(?:(?:_CODE)?))?))|E(?:ACH|LS(?:E|IF)|N(?:C(?:ODING|RYPTED)|D(?:(?:\-EXEC)?))|QUALS|SCAPE|VERY|X(?:C(?:EPT(?:(?:ION)?)|LU(?:DING|SIVE))|EC(?:(?:UTE)?)|IST(?:ING|S)|PLAIN|T(?:ERNAL|RACT)))|F(?:ALSE|ETCH|I(?:NAL|RST)|O(?:R(?:(?:CE|EIGN|TRAN|WARD)?)|UND)|R(?:EE(?:(?:ZE)?)|OM)|U(?:LL|NCTION))|G(?:E(?:NERA(?:L|TED)|T)|LOBAL|O(?:(?:TO)?)|R(?:ANT(?:(?:ED)?)|OUP(?:(?:ING)?)))|H(?:A(?:NDLER|VING)|IERARCHY|O(?:LD|ST))|I(?:DENTITY|GNORE|LIKE|M(?:M(?:EDIATE(?:(?:LY)?)|UTABLE)|PL(?:EMENTATION|ICIT))|N(?:C(?:LUDING|REMENT)|D(?:EX|ITCATOR)|FIX|HERITS|ITIAL(?:IZE|LY)|NER|OUT|PUT|S(?:E(?:NSITIVE|RT)|T(?:ANTIABLE|EAD))|T(?:ERSECT|O)|VOKER)|S(?:NULL|OLATION)|TERATE|[FNS])|JOIN|KEY(?:(?:_(?:MEMBER|TYPE))?)|L(?:A(?:N(?:COMPILER|GUAGE)|RGE|ST|TERAL)|E(?:ADING|FT|NGTH|SS|VEL)|I(?:KE|MIT|STEN)|O(?:AD|C(?:A(?:L(?:(?:TIME(?:(?:STAMP)?))?)|T(?:ION|OR))|K)|WER))|M(?:A(?:TCH|XVALUE|[PX])|E(?:SSAGE_(?:LENGTH|OCTET_LENGTH|TEXT)|THOD)|IN(?:(?:(?:UT|VALU)E)?)|O(?:D(?:(?:E|IF(?:IES|Y))?)|NTH|(?:[RV])E)|UMPS)|N(?:A(?:MES|T(?:(?:ION|UR)AL))|C(?:HAR|LOB)|E(?:W|XT)|O(?:(?:CREATE(?:DB|USER)|NE|T(?:(?:HING|IFY|NULL)?))?)|ULL(?:(?:ABLE|IF)?))|O(?:BJECT|CTET_LENGTH|FF(?:(?:SET)?)|IDS|LD|NLY|P(?:E(?:N|RAT(?:ION|OR))|TION(?:(?:S)?))|RD(?:ER|INALITY)|UT(?:(?:ER|PUT)?)|VER(?:LA(?:PS|Y)|RIDING)|WNER|[FNR])|P(?:A(?:D|R(?:AMETER(?:(?:S|_(?:MODE|NAME|ORDINAL_POSITION|SPECIFIC_(?:CATALOG|NAME|SCHEMA)))?)|TIAL)|SCAL)|E(?:NDANT|RIOD)|L(?:ACING|I)|OS(?:ITION|TFIX)|R(?:E(?:C(?:EEDS|ISION)|FIX|ORDER|(?:PAR|SERV)E)|I(?:MARY|OR|VILEGES)|OCEDUR(?:AL|E))|UBLIC)|R(?:E(?:AD(?:(?:S)?)|C(?:HECK|URSIVE)|F(?:(?:ERENC(?:ES|ING))?)|INDEX|LATIVE|NAME|P(?:(?:EATABL|LAC)E)|S(?:(?:E|T(?:AR|RIC)|UL)T)|TURN(?:(?:ED_(?:LENGTH|OCTET_LENGTH|SQLSTATE)|S)?)|VOKE)|IGHT|O(?:L(?:E|L(?:BACK|UP))|UTINE(?:(?:_(?:CATALOG|NAME|SCHEMA))?)|W(?:(?:S|_COUNT)?))|ULE)|S(?:AVE_POINT|C(?:ALE|HEMA(?:(?:_NAME)?)|OPE|ROLL)|E(?:ARCH|C(?:OND|URITY)|L(?:ECT|F)|NSITIVE|R(?:(?:IALIZABL|VER_NAM)E)|SSION(?:(?:_USER)?)|T(?:(?:OF|S)?))|H(?:ARE|OW)|I(?:M(?:ILAR|PLE)|ZE)|O(?:(?:M|URC)E)|P(?:ACE|ECIFIC(?:(?:(?:TYP|_NAM)E)?))|QL(?:(?:CODE|E(?:RROR|XCEPTION)|STATE|WARNINIG)?)|T(?:A(?:BLE|RT|T(?:E(?:(?:MENT)?)|I(?:C|STICS)))|D(?:IN|OUT)|ORAGE|R(?:ICT|UCTURE)|YPE)|U(?:B(?:CLASS_ORIGIN|LIST|STRING)|CCEEDS|M)|Y(?:MMETRIC|S(?:ID|TEM(?:(?:_USER)?))))|T(?:ABLE(?:(?:_NAME)?)|E(?:MP(?:LATE|ORARY)|RMINATE)|H(?:(?:[AE])N)|IME(?:(?:STAMP|ZONE_(?:HOUR|MINUTE))?)|O(?:(?:AST)?)|R(?:A(?:ILING|NS(?:ACTION(?:(?:S_(?:COMMITTED|ROLLED_BACK)|_ACTIVE)?)|FORM(?:(?:S)?)|LAT(?:E|ION)))|EAT|I(?:GGER(?:(?:_(?:CATALOG|NAME|SCHEMA))?)|M)|U(?:E|NCATE|STED))|YPE)|U(?:N(?:COMMITTED|DER|ENCRYPTED|I(?:ON|QUE)|KNOWN|LISTEN|N(?:AMED|EST)|TIL)|P(?:DATE|PER)|S(?:AGE|ER(?:(?:_DEFINED_TYPE_(?:CATALOG|NAME|SCHEMA))?)|ING))|V(?:A(?:CUUM|L(?:ID(?:(?:ATOR)?)|UES)|RIABLE)|ER(?:BOSE|SION(?:(?:ING|S)?))|IEW|OLATILE)|W(?:HE(?:N(?:(?:EVER)?)|RE)|ITH(?:(?:OUT)?)|ORK|RITE)|YEAR|ZONE|[CG])\b", KEYWORD),
        Rule::token(r"(?im)(ARRAY|B(?:I(?:GINT|NARY|T)|LOB|OOLEAN)|CHAR(?:(?:ACTER)?)|D(?:ATE|EC(?:(?:IMAL)?))|FLOAT|INT(?:(?:8|E(?:GER|RVAL))?)|NUM(?:BER|ERIC)|REAL|S(?:ERIAL(?:(?:8)?)|MALLINT)|TEXT|VAR(?:CHAR|YING))\b", NAME_BUILTIN),
        Rule::token(r"(?im)[+*/<>=~!@#%^&|`?-]", OPERATOR),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r#"(?im)"(""|[^"])*""#, STRING_SYMBOL),
        Rule::token(r"(?im)[a-z_][\w$]*", NAME),
        Rule::token(r"(?im)[;:()\[\],.]", PUNCTUATION),
    ]);
    m.insert(
        r"multiline-comments",
        vec![
            Rule::token_to(
                r"(?im)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"multiline-comments"]),
            ),
            Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?im)[^/*]+", COMMENT_MULTILINE),
            Rule::token(r"(?im)[/*]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for SqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
