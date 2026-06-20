#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sql:GoogleSqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:GoogleSqlLexer:googlesql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: googlesql, zetasql
pub struct GooglesqlLexer;

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
        Rule::token(r"(?im)(?:#|--\s+).*", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comment"])),
        Rule::token(r"(?im)x'([0-9a-f]{2})+'", NUMBER_HEX),
        Rule::token(r"(?im)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)b'[01]+'", NUMBER_BIN),
        Rule::token(r"(?im)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]*\.[0-9]+(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+(?=[^0-9a-z$_\u0080-\uffff])", NUMBER_INTEGER),
        Rule::token(r#"(?im)\{\s*d\s*(?P<quote>['\"])\s*\d{2}(\d{2})?.?\d{2}.?\d{2}\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token(r#"(?im)\{\s*t\s*(?P<quote>['\"])\s*(?:\d+\s+)?\d{1,2}.?\d{1,2}.?\d{1,2}(\.\d*)?\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token(r#"(?im)\{\s*ts\s*(?P<quote>['\"])\s*\d{2}(?:\d{2})?.?\d{2}.?\d{2}\s+\d{1,2}.?\d{1,2}.?\d{1,2}(\.\d*)?\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token_to(r"(?im)'", STRING_SINGLE, NewState::Push(vec![r"single-quoted-string"])),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"double-quoted-string"])),
        Rule::token(r"(?im)@@(?:global\.|persist\.|persist_only\.|session\.)?[a-z_]+", NAME_VARIABLE),
        Rule::token(r"(?im)@[a-z0-9_$.]+", NAME_VARIABLE),
        Rule::token_to(r"(?im)@'", NAME_VARIABLE, NewState::Push(vec![r"single-quoted-variable"])),
        Rule::token_to(r#"(?im)@""#, NAME_VARIABLE, NewState::Push(vec![r"double-quoted-variable"])),
        Rule::token_to(r"(?im)@`", NAME_VARIABLE, NewState::Push(vec![r"backtick-quoted-variable"])),
        Rule::token(r"(?im)\?", NAME_VARIABLE),
        Rule::token(r"(?im)\b(set)(?!\s*\()", KEYWORD),
        Rule::bygroups(r"(?im)\b(character)(\s+)(set)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)\b(FALSE|NULL|TRUE|UNKNOWN)\b", NAME_CONSTANT),
        Rule::token(r"(?im)\b(ARRAY|B(?:IGNUMERIC|OOL|YTES)|D(?:(?:AT(?:(?:ETIM)?)|OUBL)E)|E(?:NUM|XTENDED)|FLOAT|G(?:EOGRAPHY|RAPH_(?:ELEMENT|PATH))|INT(?:32|64|ERVAL)|JSON|M(?:AP|EASURE)|NUMERIC|PROTO|RANGE|STR(?:ING|UCT)|T(?:IME(?:(?:STAMP(?:(?:_PICOS)?))?)|OKENLIST)|U(?:INT(?:32|64)|UID))\b", KEYWORD_TYPE),
        Rule::token(r"(?im)\b(A(?:BORT|C(?:CESS|TION|YCLIC)|DD|FTER|GGREGATE|L(?:L|TER|WAYS)|N(?:ALYZE|[DY])|PPROX|RE|S(?:C(?:(?:ENDING)?)|SERT(?:(?:_ROWS_MODIFIED)?))|[ST])|B(?:ATCH|E(?:(?:GI|TWEE)N)|IGDECIMAL|REAK|Y)|C(?:A(?:LL|S(?:CADE|[ET]))|HECK|L(?:AMPED|ONE|USTER)|O(?:L(?:LATE|UMN(?:(?:S)?))|MMIT|N(?:FLICT|NECTION|ST(?:(?:A|RAI)NT)|T(?:AINS|INUE))|PY|RRESPONDING)|R(?:EATE|OSS)|U(?:BE|RRENT)|YCLE)|D(?:A(?:TA(?:(?:BASE)?)|Y(?:(?:OF(?:WEEK|YEAR))?))|E(?:C(?:IMAL|LARE)|F(?:AULT|INE(?:(?:R)?))|LET(?:E|ION)|PTH|S(?:C(?:(?:ENDING|RI(?:BE|PTOR))?)|TINATION)|TERMINISTIC)|ISTINCT|O|ROP)|E(?:DGE|LSE(?:(?:IF)?)|N(?:(?:(?:FORCE)?)D)|RROR|SCAPE|X(?:C(?:EPT(?:(?:ION)?)|LUDE)|ECUTE|ISTS|P(?:LAIN|ORT)|T(?:E(?:ND|RNAL)|RACT)))|F(?:ALSE|ETCH|I(?:ELD|L(?:ES|L|TER)|RST)|O(?:LLOWING|R(?:(?:EIGN|K|MAT)?))|R(?:IDAY|OM)|U(?:LL|NCTION))|G(?:ENERATED|R(?:A(?:NT|PH(?:(?:_TABLE)?))|OUP(?:(?:ING|(?:(?:_ROW)?)S)?)))|H(?:A(?:S(?:(?:H)?)|VING)|IDDEN|OUR)|I(?:DENTITY|GNORE|M(?:M(?:(?:EDIAT|UTABL)E)|PORT)|N(?:C(?:LUDE|REMENT)|DEX|NER|OUT|PUT|SERT|T(?:ER(?:LEAVE|SECT)|O)|VOKER)|SO(?:LATION|WEEK\ |YEAR)|TERATE|[FNS])|JOIN|KEY|L(?:A(?:BEL(?:(?:ED)?)|NGUAGE|ST|TERAL)|E(?:AVE|FT|T|VEL)|I(?:KE|MIT)|O(?:AD|G|O(?:(?:(?:KU)?)P)))|M(?:A(?:CRO|T(?:CH(?:(?:ED|_RECOGNIZE)?)|ERIALIZED)|X(?:(?:VALUE)?))|E(?:ASURES|RGE|SSAGE|TADATA)|I(?:CROSECOND|LLISECOND|N(?:(?:(?:UT|VALU)E)?))|O(?:D(?:EL|ULE)|N(?:DAY|TH)))|N(?:A(?:ME|NOSECOND|TURAL)|E(?:W|XT)|O(?:(?:DE|T(?:(?:HING)?))?)|ULL(?:(?:S|_FILTERED)?))|O(?:FFSET|N(?:EOF_CASE|LY)|PTION(?:AL|S)|RDER|UT(?:(?:ER|PUT)?)|VER(?:(?:WRITE)?)|[FNR])|P(?:A(?:R(?:ENT|TITION(?:(?:S)?))|ST|T(?:H(?:(?:S)?)|TERN))|ERCENT|IVOT|OLIC(?:IES|Y)|R(?:ECEDING|I(?:MARY|V(?:ATE|ILEGE(?:(?:S)?)))|O(?:CEDURE|JECT|PERT(?:IES|Y)))|UBLIC)|QUA(?:LIFY|RTER)|R(?:A(?:ISE|W)|E(?:AD|CURSIVE|FERENCES|MO(?:(?:[TV])E)|NAME|P(?:EAT(?:(?:ABLE)?)|L(?:ACE(?:(?:_FIELDS)?)|ICA)|ORT)|S(?:PECT|TRICT(?:(?:ION)?))|TURN(?:(?:S)?)|VOKE)|IGHT|O(?:LL(?:BACK|UP)|W(?:(?:S)?))|UN)|S(?:A(?:FE_CAST|TURDAY)|CHEMA|E(?:ARCH|C(?:OND\ |URITY)|LECT|QUENCE|T(?:(?:S)?))|HO(?:RTEST|W)|IMPLE|KIP|NAPSHOT|O(?:(?:M|URC)E)|QL|T(?:A(?:BLE|RT|TIC_DESCRIBE)|OR(?:ED|ING)|RICT)|UNDAY|YSTEM(?:(?:_TIME)?))|T(?:A(?:BLE(?:(?:S(?:(?:AMPLE)?))?)|RGET)|EMP(?:(?:ORARY)?)|H(?:EN|URSDAY)|O|R(?:A(?:IL|NS(?:ACTION|FORM))|EAT|U(?:(?:(?:NCAT)?)E))|UESDAY|YPE)|U(?:N(?:BOUNDED|DROP|I(?:ON|QUE)|KNOWN|NEST|PIVOT|TIL)|PDATE|SING)|V(?:ALUE(?:(?:S)?)|ECTOR|IEW(?:(?:S)?)|OLATILE)|W(?:ALK|E(?:DNESDAY|EK|IGHT)|H(?:E(?:N|RE)|ILE)|I(?:NDOW|TH(?:(?:IN)?))|RITE)|YEAR|ZONE)\b", KEYWORD),
        Rule::bygroups(r"(?im)\b(A(?:BS|COS(?:(?:H)?)|EAD\.(?:DECRYPT_(?:BYTES|STRING)|EN(?:CRYPT|VELOPE_(?:DECRYPT_(?:BYTES|STRING)|ENCRYPT)))|LL_DIFFERENT|N(?:ON_(?:AVG|COUNT(?:(?:)?)|PERCENTILE_CONT|QUANTILES|S(?:TDDEV_POP|UM)|VAR_POP)|Y_VALUE)|PPROX_(?:CO(?:SINE_DISTANCE|UNT_DISTINCT)|DOT_PRODUCT|EUCLIDEAN_DISTANCE|QUANTILES|TOP_(?:COUNT|SUM))|RRAY(?:\[(?:(?:(?:SAFE_)?)KEY\(\)\])|_(?:A(?:(?:[GV])G)|CONCAT(?:(?:_AGG)?)|FI(?:LTER|ND(?:(?:_ALL)?)|RST(?:(?:_N)?))|I(?:NCLUDES(?:(?:_A(?:LL|NY))?)|S_DISTINCT)|L(?:AST(?:(?:_N)?)|ENGTH)|M(?:AX|IN)|OFFSET(?:(?:S)?)|RE(?:MOVE_(?:(?:FIR|LA)ST_N)|VERSE)|S(?:LICE|UM)|T(?:O_STRING|RANSFORM)|ZIP))|S(?:CII|IN(?:(?:H)?))|TAN(?:(?:[2H])?)|VG)|B(?:IT_(?:AND|COUNT|(?:(?:X)?)OR)|OOL(?:(?:_ARRAY)?)|YTE_LENGTH)|C(?:AS(?:[ET])|BRT|EIL(?:(?:ING)?)|H(?:AR(?:(?:(?:ACTER)?)_LENGTH)|R)|O(?:ALESCE|DE_POINTS_TO_(?:BYTES|STRING)|LLATE|NCAT|RR|S(?:H|INE_DISTANCE)|TH|UNT(?:(?:\(\*\)|IF)?)|VAR_(?:(?:PO|SAM)P)|[ST])|SC(?:(?:H)?)|U(?:ME_DIST|RRENT_(?:DATE(?:(?:TIME)?)|TIME(?:(?:STAMP)?))))|D(?:3A_COUNT\.(?:EXTRACT|INIT|MERGE(?:(?:_PARTIAL)?)|TO_HLL)|ATE(?:(?:TIME(?:(?:_(?:ADD|BUCKET|DIFF|SUB|TRUNC))?)|_(?:ADD|BUCKET|DIFF|FROM_UNIX_DATE|SUB|TRUNC))?)|E(?:NSE_RANK|STINATION_NODE_ID|TERMINISTIC_(?:DECRYPT_(?:BYTES|STRING)|ENCRYPT))|IV|OT_PRODUCT)|E(?:D(?:GES|IT_DISTANCE)|LEMENT(?:WISE_(?:AVG|SUM)|_(?:DEFINITION_NAME|ID))|N(?:DS_WITH|UM_VALUE_DESCRIPTOR_PROTO)|RROR|UCLIDEAN_DISTANCE|X(?:P|TRACT(?:(?:_FOR_DP_APPROX_COUNT_DISTINCT)?)))|F(?:ARM_FINGERPRINT|I(?:LTER_FIELDS|RST_VALUE)|L(?:ATTEN|O(?:AT(?:32(?:(?:_ARRAY)?)|64(?:(?:_ARRAY)?))|OR))|ORMAT(?:(?:_(?:DATE(?:(?:TIME)?)|TIME(?:(?:STAMP)?)))?)|ROM_(?:BASE(?:32|64)|HEX))|G(?:ENERATE_(?:ARRAY|DATE_ARRAY|RANGE_ARRAY|TIMESTAMP_ARRAY|UUID)|R(?:EATEST|OUPING))|HLL_COUNT\.(?:EXTRACT|INIT|MERGE(?:(?:_PARTIAL)?))|I(?:EEE_DIVIDE|F(?:(?:ERROR|NULL)?)|N(?:\ UNNEST|IT(?:CAP|_FOR_DP_APPROX_COUNT_DISTINCT)|STR|T64(?:(?:_ARRAY)?))|S(?:\ (?:D(?:ESTINATION\ OF|ISTINCT\ FROM)|NOT\ DISTINCT\ FROM|SOURCE\ OF)|ERROR|_(?:ACYCLIC|INF|NAN|SIMPLE|TRAIL)))|J(?:SON_(?:ARRAY(?:(?:_(?:APPEND|INSERT))?)|CONTAINS|EXTRACT(?:(?:_(?:ARRAY|S(?:CALAR|TRING_ARRAY)))?)|KEYS|OBJECT|QUERY(?:(?:_ARRAY)?)|REMOVE|S(?:ET|TRIP_NULLS)|TYPE|VALUE(?:(?:_ARRAY)?))|USTIFY_(?:DAYS|HOURS|INTERVAL))|K(?:EYS\.(?:ADD_KEY_FROM_RAW_BYTES|KEYSET_(?:CHAIN|FROM_JSON|LENGTH|TO_JSON)|(?:NEW(?:(?:_WRAPPED)?)|R(?:EWRAP|OTATE(?:(?:_WRAPPED)?)))_KEYSET)|LL_QUANTILES\.(?:EXTRACT_(?:(?:FLOA|IN|POINT_(?:FLOA|IN))T64)|INIT_(?:(?:FLOA|IN)T64)|MERGE_(?:FLOAT64|INT64|P(?:ARTIAL|OINT_(?:(?:FLOA|IN)T64)))))|L(?:1_NORM|2_NORM|A(?:BELS|G|ST_(?:DAY|VALUE)|X_(?:BOOL(?:(?:_ARRAY)?)|FLOAT(?:32(?:(?:_ARRAY)?)|64(?:(?:_ARRAY)?))|INT64(?:(?:_ARRAY)?)|STRING(?:(?:_ARRAY)?)))|E(?:A(?:D|ST)|FT|NGTH)|IKE\ A(?:LL(?:(?:\ UNNEST)?)|NY(?:(?:\ UNNEST)?))|N|O(?:G(?:(?:10|ICAL_(?:AND|OR))?)|WER)|PAD|TRIM)|M(?:A(?:KE_INTERVAL|NHATTAN_DISTANCE|P_(?:C(?:(?:ARDINALIT|ONTAINS_KE)Y)|DELETE|E(?:MPTY|NTRIES_(?:(?:(?:UN)?)SORTED))|F(?:ILTER|ROM_ARRAY)|GET|INSERT(?:(?:_OR_REPLACE)?)|KEYS_(?:(?:(?:UN)?)SORTED)|REPLACE|VALUES_(?:SORTED(?:(?:_BY_KEY)?)|UNSORTED))|X)|D5|ERGE_PARTIAL_FOR_DP_APPROX_COUNT_DISTINCT|IN|OD)|N(?:E(?:T\.(?:HOST|IP(?:V4_(?:(?:FROM|TO)_INT64)|_(?:FROM_STRING|NET_MASK|T(?:O_STRING|RUNC)))|PUBLIC_SUFFIX|REG_DOMAIN|SAFE_IP_FROM_STRING)|W_UUID)|O(?:DES|RMALIZE(?:(?:_AND_CASEFOLD)?)|T\ LIKE\ A(?:LL(?:(?:\ UNNEST)?)|NY(?:(?:\ UNNEST)?)))|T(?:(?:H_VALU|IL)E)|ULLIF(?:(?:ERROR|ZERO)?))|O(?:CTET_LENGTH|FFSET|RDINAL)|P(?:A(?:RSE_(?:BIGNUMERIC|DATE(?:(?:TIME)?)|JSON|NUMERIC|TIME(?:(?:STAMP)?))|TH(?:(?:_(?:FIRST|L(?:AST|ENGTH)))?))|ERCENT(?:ILE_(?:CONT|DISC)|_RANK)|I(?:(?:VOT|_(?:(?:(?:BIG)?)NUMERIC))?)|OW(?:(?:ER)?)|RO(?:PERTY_(?:(?:EXIST|NAME)S)|TO_M(?:AP_CONTAINS_KEY|ODIFY_MAP)))|R(?:AN(?:GE(?:(?:_(?:BUCKET|CONTAINS|END|I(?:NTERSECT|S_(?:(?:END|START)_UNBOUNDED))|OVERLAPS|START))?)|[DK])|E(?:GEXP_(?:CONTAINS|EXTRACT(?:(?:_ALL)?)|INSTR|REPLACE|SUBSTR)|P(?:EAT|LACE)|VERSE)|IGHT|O(?:UND|W_NUMBER)|PAD|TRIM)|S(?:2_C(?:ELLIDFROMPOINT|OVERINGCELLIDS)|A(?:FE_(?:ADD|CONVERT_BYTES_TO_STRING|DIVIDE|MULTIPLY|NEGATE|O(?:FFSET|RDINAL)|SUBTRACT|TO_JSON)|ME)|E(?:C(?:(?:H)?)|SSION_USER)|HA(?:1|256|512)|I(?:GN|N(?:(?:H)?))|OU(?:NDEX|RCE_NODE_ID)|PLIT(?:(?:_SUBSTR)?)|QRT|T(?:ARTS_WITH|DDEV(?:(?:_(?:(?:PO|SAM)P))?)|R(?:ING(?:(?:_A(?:GG|RRAY))?)|POS)|_(?:A(?:NGLE|REA|S(?:BINARY|GEOJSON|KML|TEXT)|ZIMUTH)|B(?:OUND(?:ARY|INGBOX)|UFFER(?:(?:WITHTOLERANCE)?))|C(?:ENTROID(?:(?:_AGG)?)|L(?:OSESTPOINT|USTERDBSCAN)|O(?:N(?:TAINS|VEXHULL)|VER(?:EDBY|S)))|D(?:I(?:FFERENCE|MENSION|S(?:JOINT|TANCE))|UMP(?:(?:POINTS)?)|WITHIN)|E(?:NDPOINT|QUALS|XTE(?:NT|RIORRING))|GEO(?:G(?:FROM(?:(?:GEOJSON|KML|TEXT|WKB)?)|POINT(?:(?:FROMGEOHASH)?))|HASH|METRYTYPE)|HAUSDORFFD(?:ISTANCE|WITHIN)|I(?:NTER(?:IORRINGS|SECT(?:ION|S(?:(?:BOX)?)))|S(?:C(?:LOSED|OLLECTION)|EMPTY|RING))|L(?:ENGTH|INE(?:INTERPOLATEPOINT|LOCATEPOINT|SUBSTRING))|MA(?:KE(?:LINE|POLYGON(?:(?:ORIENTED)?))|XDISTANCE)|N(?:(?:EAREST_NEIGHBOR|POINT|UM(?:GEOMETRIE|POINT))S)|P(?:ERIMETER|OINTN)|S(?:IMPLIFY|NAPTOGRID|TARTPOINT)|TOUCHES|UN(?:ARYUNION|ION(?:(?:_AGG)?))|WITHIN|[XY]))|U(?:BSTR(?:(?:ING)?)|M))|T(?:AN(?:(?:H)?)|IME(?:(?:STAMP(?:(?:_(?:ADD|BUCKET|DIFF|FROM_UNIX_(?:(?:MI(?:CRO|LLI)|SECOND)S)|MI(?:(?:CRO|LLI)S)|S(?:ECONDS|UB)|TRUNC))?)|_(?:ADD|DIFF|SUB|TRUNC))?)|O_(?:BASE(?:32|64)|CODE_POINTS|HEX|JSON(?:(?:_STRING)?))|R(?:ANSLATE|IM|UNC)|YPEOF)|U(?:N(?:I(?:CODE|X_(?:DATE|(?:MI(?:CRO|LLI)|SECOND)S))|(?:NES|PIVO)T)|PPER)|VAR(?:IANCE|_(?:(?:PO|SAM)P))|ZEROIFNULL)\b(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?im)\b(!=|<(?:[<=])|>(?:[=>])|\|\||[&*+\-/<=>\^|~])\b", OPERATOR),
        Rule::token(r"(?im)[0-9a-z$_-￿]+", NAME),
        Rule::token_to(r"(?im)`", TokenType::new(&["Name", "Quoted"]), NewState::Push(vec![r"schema-object-name"])),
        Rule::token(r"(?im)[(),.;]", PUNCTUATION),
    ]);
    m.insert(
        r"multiline-comment",
        vec![
            Rule::token(r"(?im)[^*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?im)\*", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"single-quoted-string",
        vec![
            Rule::token(r"(?im)[^'\\]+", STRING_SINGLE),
            Rule::token(r"(?im)''", STRING_ESCAPE),
            Rule::token(r#"(?im)\\[0'"bnrtZ\\%_]"#, STRING_ESCAPE),
            Rule::token_to(r"(?im)'", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"double-quoted-string",
        vec![
            Rule::token(r#"(?im)[^"\\]+"#, STRING_DOUBLE),
            Rule::token(r#"(?im)"""#, STRING_ESCAPE),
            Rule::token(r#"(?im)\\[0'"bnrtZ\\%_]"#, STRING_ESCAPE),
            Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"single-quoted-variable",
        vec![
            Rule::token(r"(?im)[^']+", NAME_VARIABLE),
            Rule::token(r"(?im)''", NAME_VARIABLE),
            Rule::token_to(r"(?im)'", NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"double-quoted-variable",
        vec![
            Rule::token(r#"(?im)[^"]+"#, NAME_VARIABLE),
            Rule::token(r#"(?im)"""#, NAME_VARIABLE),
            Rule::token_to(r#"(?im)""#, NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"backtick-quoted-variable",
        vec![
            Rule::token(r"(?im)[^`]+", NAME_VARIABLE),
            Rule::token(r"(?im)``", NAME_VARIABLE),
            Rule::token_to(r"(?im)`", NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"schema-object-name",
        vec![
            Rule::token(r"(?im)[^`]+", TokenType::new(&["Name", "Quoted"])),
            Rule::token(r"(?im)``", TokenType::new(&["Name", "Quoted", "Escape"])),
            Rule::token_to(
                r"(?im)`",
                TokenType::new(&["Name", "Quoted"]),
                NewState::Pop(1),
            ),
        ],
    );
    Table(m)
}

impl Lexer for GooglesqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
