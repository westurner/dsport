#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.business:CobolLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.business:CobolLexer:cobol

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cobol
pub struct CobolLexer;

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
        Rule::token(r"(?im)(^.{6}[*/].*\n|^.{6}|\*>.*\n)", COMMENT),
        Rule::token(r#"(?im)"[^"\n]*("|\n)"#, STRING_DOUBLE),
        Rule::token(r"(?im)'[^'\n]*('|\n)", STRING_SINGLE),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(ALL\s+)?((ZEROES)|(HIGH-VALUE|LOW-VALUE|QUOTE|SPACE|ZERO)(S)?)\s*($|(?=[^\w\-]))", NAME_CONSTANT),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CCEPT|DD|LLOCATE)|C(?:A(?:(?:L|NCE)L)|LOSE|O(?:MPUTE|N(?:FIGURATION|TINUE)))|D(?:ATA|ELETE|I(?:SPLAY|VI(?:DE|SION)))|E(?:LSE|N(?:D(?:(?:\-(?:A(?:CCEPT|DD)|C(?:ALL|OMPUTE)|D(?:ELETE|I(?:SPLAY|VIDE))|EVALUATE|IF|MULTIPLY|OF\-PAGE|PERFORM|RE(?:AD|TURN|WRITE)|S(?:EARCH|T(?:ART|RING)|UBTRACT)|UNSTRING|WRITE))?)|VIRONMENT)|VALUATE|XIT)|F(?:D|ILE(?:(?:\-CONTROL)?)|OREVER|REE)|G(?:ENERATE|O(?:(?:BACK)?))|I(?:\-O\-CONTROL|DENTIFICATION|F|N(?:ITIA(?:(?:LIZ|T)E)|PUT\-OUTPUT|SPECT|VOKE))|L(?:(?:INK|OCAL\-STOR)AGE)|M(?:ERGE|OVE|ULTIPLY)|OPEN|P(?:ERFORM|RO(?:CEDURE|GRAM\-ID))|R(?:AISE|E(?:AD|LEASE|SUME|TURN|WRITE))|S(?:CREEN|D|E(?:ARCH|CTION|T)|ORT|T(?:ART|OP|RING)|U(?:BTRACT|PPRESS))|T(?:ERMINATE|HEN)|U(?:N(?:LOCK|STRING)|SE)|(?:VALIDAT|W(?:ORKING\-STORAG|RIT))E)\s*($|(?=[^\w\-]))", KEYWORD_RESERVED),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CCESS|D(?:DRESS|VANCING)|FTER|L(?:L|PHA(?:BET(?:(?:IC(?:(?:\-(?:(?:LOW|UPP)ER))?))?)|NUMERIC(?:(?:\-EDITED)?))|SO|TER(?:(?:NATEANY)?))|R(?:E(?:(?:A(?:(?:S)?))?)|GUMENT\-(?:NUMBER|VALUE))|S(?:CENDING|SIGN)|UTO(?:(?:\-SKIP|MATIC|TERMINATE)?)|[ST])|B(?:A(?:CKGROUND\-COLOR|SED)|E(?:EP|FORE|LL)|L(?:(?:AN|IN|OC)K)|OTTOM|Y(?:(?:TE\-LENGTH)?))|C(?:HA(?:INING|RACTER(?:(?:S)?))|LASS|O(?:DE(?:(?:\-SET)?)|L(?:(?:LATING|S|UMN(?:(?:S)?))?)|MM(?:A(?:(?:ND\-LINE)?)|IT|ON)|N(?:STANT|T(?:AINS|ENT|ROL(?:(?:S)?))|VERTING)|PY|RR(?:(?:ESPONDING)?)|UNT)|RT|UR(?:RENCY|SOR)|YCLE)|D(?:A(?:TE|Y(?:(?:\-OF\-WEEK)?))|E(?:(?:BUGGING|C(?:IMAL\-POINT|LARATIVES)|FAULT|LIMITE(?:[DR])|PENDING|SCENDING|TAIL)?)|ISK|OWN|UPLICATES|YNAMIC)|E(?:BCDIC|N(?:TRY|VIRONMENT\-(?:(?:NAM|VALU)E))|O(?:[LPS])|R(?:ASE|ROR)|SCAPE|X(?:C(?:EPTION|LUSIVE)|TE(?:ND|RNAL)))|F(?:I(?:L(?:E\-ID|LER)|NAL|RST|XED)|LOAT\-(?:LONG|SHORT)|O(?:OTING|R(?:(?:EGROUND\-COLOR|MAT)?))|ROM|U(?:LL|NCTION(?:(?:\-ID)?)))|G(?:IVING|LOBAL|ROUP)|H(?:EADING|IGHLIGHT)|I(?:\-O|GNOR(?:E|ING)|N(?:D(?:EX(?:(?:ED)?)|ICATE)|ITIAL(?:(?:IZED)?)|PUT|T(?:O|RINSIC)|VALID)|[DNS])|JUST(?:(?:IFIED)?)|KEY|L(?:A(?:BEL|ST)|E(?:ADING|FT|NGTH)|I(?:MIT(?:(?:S)?)|N(?:AGE(?:(?:\-COUNTER)?)|E(?:(?:S)?)))|O(?:C(?:ALE|K)|WLIGHT))|M(?:ANUAL|EMORY|INUS|(?:OD|ULTIPL)E)|N(?:ATI(?:ONAL(?:(?:\-EDITED)?)|VE)|E(?:GATIVE|XT)|O|U(?:LL(?:(?:S)?)|M(?:BER(?:(?:S)?)|ERIC(?:(?:\-EDITED)?))))|O(?:BJECT\-COMPUTER|CCURS|FF|MITTED|NLY|PTIONAL|R(?:DER|GANIZATION)|THER|UTPUT|VER(?:FLOW|LINE)|[FN])|P(?:A(?:CKED\-DECIMAL|DDING|GE|RAGRAPH)|LUS|O(?:INTER|SITI(?:ON|VE))|R(?:E(?:SENT|VIOUS)|INT(?:ER|ING)|O(?:CE(?:DURE(?:\-POINTER|S)|ED)|GRAM(?:(?:\-POINTER)?)|MPT)))|QUOTE(?:(?:S)?)|R(?:ANDOM|D|E(?:C(?:ORD(?:(?:ING|S)?)|URSIVE)|DEFINES|EL|FERENCE|LATIVE|M(?:AINDER|OVAL)|NAMES|P(?:LACING|O(?:RT(?:(?:ING|S)?)|SITORY))|QUIRED|SERVE|TURNING|VERSE\-VIDEO|WIND)|IGHT|O(?:LLBACK|UNDED)|UN)|S(?:AME|CROLL|E(?:CURE|GMENT\-LIMIT|LECT|NTENCE|PARATE|QUEN(?:CE|TIAL))|HARING|I(?:GN(?:(?:ED(?:(?:\-(?:INT|LONG|SHORT))?))?)|ZE)|O(?:RT\-MERGE|URCE(?:(?:\-COMPUTER)?))|PECIAL\-NAMES|TA(?:NDARD(?:(?:\-(?:[12]))?)|TUS)|U(?:BKEY|M)|Y(?:MBOLIC|NC(?:(?:HRONIZED)?)))|T(?:A(?:LLYING|PE)|EST|HR(?:OUGH|U)|IME(?:(?:S)?)|O(?:(?:P)?)|RA(?:ILING|NSFORM)|YPE)|U(?:N(?:DERLINE|IT|SIGNED(?:(?:\-(?:INT|LONG|SHORT))?)|TIL)|P(?:(?:DATE|ON)?)|S(?:AGE|ING))|VA(?:LUE(?:(?:S)?)|RYING)|W(?:AIT|HEN|ITH|ORDS)|YYYY(?:(?:D|MM)DD))\s*($|(?=[^\w\-]))", KEYWORD_PSEUDO),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CTIVE\-CLASS|LIGNED|NYCASE|RITHMETIC|TTRIBUTE)|B(?:\-(?:AND|NOT|(?:(?:X)?)OR)|IT|OOLEAN)|C(?:ENTER|HAIN|LASS(?:\-ID|IFICATION)|O(?:(?:MMUNICA|NDI)TION)|[DFH])|D(?:ATA\-POINTER|ESTINATION|ISABLE)|E(?:GI|MI|N(?:ABLE|D\-RECEIVE|TRY\-CONVENTION)|SI|X(?:CEPTION\-OBJECT|PANDS)|[CO])|F(?:ACTORY|LOAT\-(?:BINARY\-(?:16|34|7)|DECIMAL\-(?:16|34)|EXTENDED)|ORMAT|UNCTION\-POINTER)|G(?:ET|ROUP\-USAGE)|I(?:MPLEMENTS|N(?:FINITY|HERITS|TERFACE(?:(?:\-ID)?)|VOKE))|L(?:C_(?:ALL|C(?:(?:OLLAT|TYP)E)|M(?:ESSAGES|ONETARY)|NUMERIC|TIME)|INE\-COUNTER)|ME(?:SSAGE|THOD(?:(?:\-ID)?))|N(?:ESTED|O(?:NE|RMAL))|O(?:BJECT(?:(?:\-REFERENCE)?)|PTIONS|VERRIDE)|P(?:AGE\-COUNTER|RO(?:PERTY|TOTYPE)|URGE|[FH])|QUEUE|R(?:AIS(?:E|ING)|E(?:CEIVE|LATION|P(?:LACE|RESENTS\-NOT\-A\-NUMBER)|S(?:ET|UME)|TRY)|[FH])|S(?:E(?:CONDS|GMENT|LF|ND)|OURCES|T(?:ATEMENT|EP|RONG)|U(?:B\-QUEUE\-(?:[123])|PER)|Y(?:MBOL|STEM\-DEFAULT))|T(?:ABLE|E(?:RMINAL|XT)|YPEDEF)|U(?:CS\-4|NIVERSAL|SER\-DEFAULT|TF\-(?:16|8))|VAL(?:\-STATUS|ID(?:(?:ATE(?:(?:\-STATUS)?))?)))\s*($|(?=[^\w\-]))", ERROR),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(PIC\s+.+?(?=(\s|\.\s))|PICTURE\s+.+?(?=(\s|\.\s))|(COMPUTATIONAL)(-[1-5X])?|(COMP)(-[1-5X])?|BINARY-C-LONG|BINARY-CHAR|BINARY-DOUBLE|BINARY-LONG|BINARY-SHORT|BINARY)\s*($|(?=[^\w\-]))", KEYWORD_TYPE),
        Rule::token(r"(?im)(\*\*|\*|\+|-|/|<=|>=|<|>|==|/=|=)", OPERATOR),
        Rule::token(r"(?im)([(),;:&%.])", PUNCTUATION),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(ABS|ACOS|ANNUITY|ASIN|ATAN|BYTE-LENGTH|CHAR|COMBINED-DATETIME|CONCATENATE|COS|CURRENT-DATE|DATE-OF-INTEGER|DATE-TO-YYYYMMDD|DAY-OF-INTEGER|DAY-TO-YYYYDDD|EXCEPTION-(?:FILE|LOCATION|STATEMENT|STATUS)|EXP10|EXP|E|FACTORIAL|FRACTION-PART|INTEGER-OF-(?:DATE|DAY|PART)|INTEGER|LENGTH|LOCALE-(?:DATE|TIME(?:-FROM-SECONDS)?)|LOG(?:10)?|LOWER-CASE|MAX|MEAN|MEDIAN|MIDRANGE|MIN|MOD|NUMVAL(?:-C)?|ORD(?:-MAX|-MIN)?|PI|PRESENT-VALUE|RANDOM|RANGE|REM|REVERSE|SECONDS-FROM-FORMATTED-TIME|SECONDS-PAST-MIDNIGHT|SIGN|SIN|SQRT|STANDARD-DEVIATION|STORED-CHAR-LENGTH|SUBSTITUTE(?:-CASE)?|SUM|TAN|TEST-DATE-YYYYMMDD|TEST-DAY-YYYYDDD|TRIM|UPPER-CASE|VARIANCE|WHEN-COMPILED|YEAR-TO-YYYY)\s*($|(?=[^\w\-]))", NAME_FUNCTION),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(true|false)\s*($|(?=[^\w\-]))", NAME_BUILTIN),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(equal|equals|ne|lt|le|gt|ge|greater|less|than|not|and|or)\s*($|(?=[^\w\-]))", OPERATOR_WORD),
        Rule::token(r"(?im)\d+(\s*|\.$|$)", NUMBER_INTEGER),
        Rule::token(r"(?im)[+-]?\d*\.\d+(E[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[+-]?\d+\.\d*(E[-+]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[a-z0-9]([\w\-]*[a-z0-9]+)?", NAME_VARIABLE),
        Rule::token(r"(?im)[ \t]+", WHITESPACE),
    ]);
    m.insert(
        r"comment",
        vec![Rule::token(r"(?im)(^.{6}[*/].*\n|^.{6}|\*>.*\n)", COMMENT)],
    );
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?im)"[^"\n]*("|\n)"#, STRING_DOUBLE),
            Rule::token(r"(?im)'[^'\n]*('|\n)", STRING_SINGLE),
        ],
    );
    m.insert(r"core", vec![
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(ALL\s+)?((ZEROES)|(HIGH-VALUE|LOW-VALUE|QUOTE|SPACE|ZERO)(S)?)\s*($|(?=[^\w\-]))", NAME_CONSTANT),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CCEPT|DD|LLOCATE)|C(?:A(?:(?:L|NCE)L)|LOSE|O(?:MPUTE|N(?:FIGURATION|TINUE)))|D(?:ATA|ELETE|I(?:SPLAY|VI(?:DE|SION)))|E(?:LSE|N(?:D(?:(?:\-(?:A(?:CCEPT|DD)|C(?:ALL|OMPUTE)|D(?:ELETE|I(?:SPLAY|VIDE))|EVALUATE|IF|MULTIPLY|OF\-PAGE|PERFORM|RE(?:AD|TURN|WRITE)|S(?:EARCH|T(?:ART|RING)|UBTRACT)|UNSTRING|WRITE))?)|VIRONMENT)|VALUATE|XIT)|F(?:D|ILE(?:(?:\-CONTROL)?)|OREVER|REE)|G(?:ENERATE|O(?:(?:BACK)?))|I(?:\-O\-CONTROL|DENTIFICATION|F|N(?:ITIA(?:(?:LIZ|T)E)|PUT\-OUTPUT|SPECT|VOKE))|L(?:(?:INK|OCAL\-STOR)AGE)|M(?:ERGE|OVE|ULTIPLY)|OPEN|P(?:ERFORM|RO(?:CEDURE|GRAM\-ID))|R(?:AISE|E(?:AD|LEASE|SUME|TURN|WRITE))|S(?:CREEN|D|E(?:ARCH|CTION|T)|ORT|T(?:ART|OP|RING)|U(?:BTRACT|PPRESS))|T(?:ERMINATE|HEN)|U(?:N(?:LOCK|STRING)|SE)|(?:VALIDAT|W(?:ORKING\-STORAG|RIT))E)\s*($|(?=[^\w\-]))", KEYWORD_RESERVED),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CCESS|D(?:DRESS|VANCING)|FTER|L(?:L|PHA(?:BET(?:(?:IC(?:(?:\-(?:(?:LOW|UPP)ER))?))?)|NUMERIC(?:(?:\-EDITED)?))|SO|TER(?:(?:NATEANY)?))|R(?:E(?:(?:A(?:(?:S)?))?)|GUMENT\-(?:NUMBER|VALUE))|S(?:CENDING|SIGN)|UTO(?:(?:\-SKIP|MATIC|TERMINATE)?)|[ST])|B(?:A(?:CKGROUND\-COLOR|SED)|E(?:EP|FORE|LL)|L(?:(?:AN|IN|OC)K)|OTTOM|Y(?:(?:TE\-LENGTH)?))|C(?:HA(?:INING|RACTER(?:(?:S)?))|LASS|O(?:DE(?:(?:\-SET)?)|L(?:(?:LATING|S|UMN(?:(?:S)?))?)|MM(?:A(?:(?:ND\-LINE)?)|IT|ON)|N(?:STANT|T(?:AINS|ENT|ROL(?:(?:S)?))|VERTING)|PY|RR(?:(?:ESPONDING)?)|UNT)|RT|UR(?:RENCY|SOR)|YCLE)|D(?:A(?:TE|Y(?:(?:\-OF\-WEEK)?))|E(?:(?:BUGGING|C(?:IMAL\-POINT|LARATIVES)|FAULT|LIMITE(?:[DR])|PENDING|SCENDING|TAIL)?)|ISK|OWN|UPLICATES|YNAMIC)|E(?:BCDIC|N(?:TRY|VIRONMENT\-(?:(?:NAM|VALU)E))|O(?:[LPS])|R(?:ASE|ROR)|SCAPE|X(?:C(?:EPTION|LUSIVE)|TE(?:ND|RNAL)))|F(?:I(?:L(?:E\-ID|LER)|NAL|RST|XED)|LOAT\-(?:LONG|SHORT)|O(?:OTING|R(?:(?:EGROUND\-COLOR|MAT)?))|ROM|U(?:LL|NCTION(?:(?:\-ID)?)))|G(?:IVING|LOBAL|ROUP)|H(?:EADING|IGHLIGHT)|I(?:\-O|GNOR(?:E|ING)|N(?:D(?:EX(?:(?:ED)?)|ICATE)|ITIAL(?:(?:IZED)?)|PUT|T(?:O|RINSIC)|VALID)|[DNS])|JUST(?:(?:IFIED)?)|KEY|L(?:A(?:BEL|ST)|E(?:ADING|FT|NGTH)|I(?:MIT(?:(?:S)?)|N(?:AGE(?:(?:\-COUNTER)?)|E(?:(?:S)?)))|O(?:C(?:ALE|K)|WLIGHT))|M(?:ANUAL|EMORY|INUS|(?:OD|ULTIPL)E)|N(?:ATI(?:ONAL(?:(?:\-EDITED)?)|VE)|E(?:GATIVE|XT)|O|U(?:LL(?:(?:S)?)|M(?:BER(?:(?:S)?)|ERIC(?:(?:\-EDITED)?))))|O(?:BJECT\-COMPUTER|CCURS|FF|MITTED|NLY|PTIONAL|R(?:DER|GANIZATION)|THER|UTPUT|VER(?:FLOW|LINE)|[FN])|P(?:A(?:CKED\-DECIMAL|DDING|GE|RAGRAPH)|LUS|O(?:INTER|SITI(?:ON|VE))|R(?:E(?:SENT|VIOUS)|INT(?:ER|ING)|O(?:CE(?:DURE(?:\-POINTER|S)|ED)|GRAM(?:(?:\-POINTER)?)|MPT)))|QUOTE(?:(?:S)?)|R(?:ANDOM|D|E(?:C(?:ORD(?:(?:ING|S)?)|URSIVE)|DEFINES|EL|FERENCE|LATIVE|M(?:AINDER|OVAL)|NAMES|P(?:LACING|O(?:RT(?:(?:ING|S)?)|SITORY))|QUIRED|SERVE|TURNING|VERSE\-VIDEO|WIND)|IGHT|O(?:LLBACK|UNDED)|UN)|S(?:AME|CROLL|E(?:CURE|GMENT\-LIMIT|LECT|NTENCE|PARATE|QUEN(?:CE|TIAL))|HARING|I(?:GN(?:(?:ED(?:(?:\-(?:INT|LONG|SHORT))?))?)|ZE)|O(?:RT\-MERGE|URCE(?:(?:\-COMPUTER)?))|PECIAL\-NAMES|TA(?:NDARD(?:(?:\-(?:[12]))?)|TUS)|U(?:BKEY|M)|Y(?:MBOLIC|NC(?:(?:HRONIZED)?)))|T(?:A(?:LLYING|PE)|EST|HR(?:OUGH|U)|IME(?:(?:S)?)|O(?:(?:P)?)|RA(?:ILING|NSFORM)|YPE)|U(?:N(?:DERLINE|IT|SIGNED(?:(?:\-(?:INT|LONG|SHORT))?)|TIL)|P(?:(?:DATE|ON)?)|S(?:AGE|ING))|VA(?:LUE(?:(?:S)?)|RYING)|W(?:AIT|HEN|ITH|ORDS)|YYYY(?:(?:D|MM)DD))\s*($|(?=[^\w\-]))", KEYWORD_PSEUDO),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(A(?:CTIVE\-CLASS|LIGNED|NYCASE|RITHMETIC|TTRIBUTE)|B(?:\-(?:AND|NOT|(?:(?:X)?)OR)|IT|OOLEAN)|C(?:ENTER|HAIN|LASS(?:\-ID|IFICATION)|O(?:(?:MMUNICA|NDI)TION)|[DFH])|D(?:ATA\-POINTER|ESTINATION|ISABLE)|E(?:GI|MI|N(?:ABLE|D\-RECEIVE|TRY\-CONVENTION)|SI|X(?:CEPTION\-OBJECT|PANDS)|[CO])|F(?:ACTORY|LOAT\-(?:BINARY\-(?:16|34|7)|DECIMAL\-(?:16|34)|EXTENDED)|ORMAT|UNCTION\-POINTER)|G(?:ET|ROUP\-USAGE)|I(?:MPLEMENTS|N(?:FINITY|HERITS|TERFACE(?:(?:\-ID)?)|VOKE))|L(?:C_(?:ALL|C(?:(?:OLLAT|TYP)E)|M(?:ESSAGES|ONETARY)|NUMERIC|TIME)|INE\-COUNTER)|ME(?:SSAGE|THOD(?:(?:\-ID)?))|N(?:ESTED|O(?:NE|RMAL))|O(?:BJECT(?:(?:\-REFERENCE)?)|PTIONS|VERRIDE)|P(?:AGE\-COUNTER|RO(?:PERTY|TOTYPE)|URGE|[FH])|QUEUE|R(?:AIS(?:E|ING)|E(?:CEIVE|LATION|P(?:LACE|RESENTS\-NOT\-A\-NUMBER)|S(?:ET|UME)|TRY)|[FH])|S(?:E(?:CONDS|GMENT|LF|ND)|OURCES|T(?:ATEMENT|EP|RONG)|U(?:B\-QUEUE\-(?:[123])|PER)|Y(?:MBOL|STEM\-DEFAULT))|T(?:ABLE|E(?:RMINAL|XT)|YPEDEF)|U(?:CS\-4|NIVERSAL|SER\-DEFAULT|TF\-(?:16|8))|VAL(?:\-STATUS|ID(?:(?:ATE(?:(?:\-STATUS)?))?)))\s*($|(?=[^\w\-]))", ERROR),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(PIC\s+.+?(?=(\s|\.\s))|PICTURE\s+.+?(?=(\s|\.\s))|(COMPUTATIONAL)(-[1-5X])?|(COMP)(-[1-5X])?|BINARY-C-LONG|BINARY-CHAR|BINARY-DOUBLE|BINARY-LONG|BINARY-SHORT|BINARY)\s*($|(?=[^\w\-]))", KEYWORD_TYPE),
        Rule::token(r"(?im)(\*\*|\*|\+|-|/|<=|>=|<|>|==|/=|=)", OPERATOR),
        Rule::token(r"(?im)([(),;:&%.])", PUNCTUATION),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(ABS|ACOS|ANNUITY|ASIN|ATAN|BYTE-LENGTH|CHAR|COMBINED-DATETIME|CONCATENATE|COS|CURRENT-DATE|DATE-OF-INTEGER|DATE-TO-YYYYMMDD|DAY-OF-INTEGER|DAY-TO-YYYYDDD|EXCEPTION-(?:FILE|LOCATION|STATEMENT|STATUS)|EXP10|EXP|E|FACTORIAL|FRACTION-PART|INTEGER-OF-(?:DATE|DAY|PART)|INTEGER|LENGTH|LOCALE-(?:DATE|TIME(?:-FROM-SECONDS)?)|LOG(?:10)?|LOWER-CASE|MAX|MEAN|MEDIAN|MIDRANGE|MIN|MOD|NUMVAL(?:-C)?|ORD(?:-MAX|-MIN)?|PI|PRESENT-VALUE|RANDOM|RANGE|REM|REVERSE|SECONDS-FROM-FORMATTED-TIME|SECONDS-PAST-MIDNIGHT|SIGN|SIN|SQRT|STANDARD-DEVIATION|STORED-CHAR-LENGTH|SUBSTITUTE(?:-CASE)?|SUM|TAN|TEST-DATE-YYYYMMDD|TEST-DAY-YYYYDDD|TRIM|UPPER-CASE|VARIANCE|WHEN-COMPILED|YEAR-TO-YYYY)\s*($|(?=[^\w\-]))", NAME_FUNCTION),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(true|false)\s*($|(?=[^\w\-]))", NAME_BUILTIN),
        Rule::token(r"(?im)(^|(?<=[^\w\-]))(equal|equals|ne|lt|le|gt|ge|greater|less|than|not|and|or)\s*($|(?=[^\w\-]))", OPERATOR_WORD),
    ]);
    m.insert(
        r"nums",
        vec![
            Rule::token(r"(?im)\d+(\s*|\.$|$)", NUMBER_INTEGER),
            Rule::token(r"(?im)[+-]?\d*\.\d+(E[-+]?\d+)?", NUMBER_FLOAT),
            Rule::token(r"(?im)[+-]?\d+\.\d*(E[-+]?\d+)?", NUMBER_FLOAT),
        ],
    );
    Table(m)
}

impl Lexer for CobolLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
