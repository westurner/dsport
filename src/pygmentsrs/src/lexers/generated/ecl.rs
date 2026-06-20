//! AUTO-GENERATED from `pygments.pygments.lexers.ecl:ECLLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ecl:ECLLexer:ecl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ecl
pub struct EclLexer;

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
        Rule::token(r"(?im)\/\/.*", COMMENT_SINGLE),
        Rule::token(r"(?im)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?im)(RECORD|END)\D", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?im)((?:ASCII|BIG_ENDIAN|BOOLEAN|DATA|DECIMAL|EBCDIC|INTEGER|PATTERN|QSTRING|REAL|RECORD|RULE|SET OF|STRING|TOKEN|UDECIMAL|UNICODE|UNSIGNED|VARSTRING|VARUNICODE)\d*)(\s+)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE)]),
        Rule::token(r"(?im)(A(?:PPLY|SSERT)|BUILD(?:(?:INDEX)?)|CHECKPOINT|DEPRECATED|EVALUATE|FAIL(?:(?:(?:COD|MESSAG|UR)E)?)|GLOBAL|INDEPENDENT|KEY(?:DIFF|PATCH)|LOADXML|NOT(?:HOR|IFY)|O(?:NWARNING|UTPUT)|P(?:ARALLEL|ERSIST|RIORITY)|RECOVERY|S(?:EQUENTIAL|OAPCALL|TORED|UCCESS)|W(?:AIT|HEN))\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(A(?:LL|N(?:[DY])|S|TMOST)|BE(?:FORE|GINC\+\+|ST|TWEEN)|C(?:ASE|O(?:NST|UNTER)|SV)|DESCEND|E(?:N(?:CRYPT|D(?:C\+\+|MACRO))|X(?:C(?:EPT|LUSIVE)|P(?:IRE|ORT)|TEND))|F(?:ALSE|EW|IRST|LAT|U(?:LL|NCTION))|GROUP|H(?:EAD(?:ER|ING)|OLE)|I(?:FBLOCK|MPORT|N(?:(?:TERFACE)?))|JOINED|KE(?:EP|YED)|L(?:AST|EFT|IMIT|O(?:AD|CAL(?:(?:E)?)|OKUP))|M(?:A(?:CRO|NY|X(?:COUNT|LENGTH))|IN\ SKEW|ODULE)|N(?:AMED|O(?:CASE|ROOT|S(?:CAN|ORT)|T))|O(?:NLY|PT|UTER|VERWRITE|[FR])|P(?:A(?:CKED|RTITION)|ENALTY|HYSICALLENGTH|IPE)|QUOTE|R(?:E(?:LATIONSHIP|PEAT|TURN)|IGHT)|S(?:CAN|E(?:LF|PARATOR|RVICE)|HARED|K(?:EW|IP)|QL|TORE)|T(?:ERMINATOR|H(?:OR|RESHOLD)|OKEN|R(?:ANSFORM|IM|UE)|YPE)|UN(?:ICODEORDER|SORTED)|V(?:ALIDATE|IRTUAL)|W(?:HOLE|I(?:LD|THIN))|X(?:ML|PATH)|__COMPRESSED__)\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(A(?:BS|COS|LLNODES|S(?:CII|IN|STRING)|TAN(?:(?:2)?)|VE)|C(?:ASE|HOOSE(?:(?:N|SETS)?)|LUSTERSIZE|O(?:MBINE|RRELATION|S(?:(?:H)?)|UNT|VARIANCE)|RON)|D(?:ATASET|E(?:DUP|(?:FIN|NORMALIZ)E)|ISTRIBUT(?:E(?:(?:D)?)|ION))|E(?:BCDIC|NTH|RROR|V(?:ALUATE|ENT(?:(?:EXTRA|NAME)?))|X(?:ISTS|P))|F(?:AIL(?:(?:COD|MESSAG)E)|ETCH|ROMUNICODE)|G(?:ETISVALID|LOBAL|R(?:APH|OUP))|HA(?:SH(?:(?:32|64|CRC|MD5)?)|VING)|I(?:F|N(?:DEX|TFORMAT)|SVALID|TERATE)|JOIN|KEYUNICODE|L(?:ENGTH|I(?:BRARY|MIT)|N|O(?:CAL|G|OP))|M(?:A(?:TCH(?:ED|LENGTH|POSITION|TEXT|UNICODE)|[PX])|ERGE(?:(?:JOIN)?)|IN)|NO(?:LOCAL|NEMPTY|RMALIZE)|P(?:ARSE|IPE|OWER|R(?:ELOAD|O(?:CESS|JECT))|ULL)|R(?:AN(?:DOM|GE|K(?:(?:ED)?))|E(?:ALFORMAT|CORDOF|G(?:EX(?:FIND|REPLACE)|ROUP)|JECTED)|O(?:LLUP|UND(?:(?:UP)?)|W(?:(?:DIFF)?)))|S(?:AMPLE|ET|I(?:N(?:(?:H)?)|ZEOF)|O(?:APCALL|RT(?:(?:ED)?))|QRT|T(?:(?:EPP|OR)ED)|UM)|T(?:A(?:BLE|N(?:(?:H)?))|HISNODE|O(?:PN|UNICODE)|R(?:ANSFER|IM|UNCATE)|YPEOF)|UN(?:GROUP|ICODEORDER)|VARIANCE|W(?:HICH|ORKUNIT)|XML(?:DECODE|ENCODE|TEXT|UNICODE))\b", NAME_FUNCTION),
        Rule::token(r"(?im)^#.*$", COMMENT_PREPROC),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+)e[+-]?\d+[lu]*", NUMBER_FLOAT),
        Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+f)f?", NUMBER_FLOAT),
        Rule::token(r"(?im)0x[0-9a-f]+[lu]*", NUMBER_HEX),
        Rule::token(r"(?im)0[0-7]+[lu]*", NUMBER_OCT),
        Rule::token(r"(?im)\d+[lu]*", NUMBER_INTEGER),
        Rule::token(r"(?im)[~!%^&*+=|?:<>/-]+", OPERATOR),
        Rule::token(r"(?im)[{}()\[\],.;]", PUNCTUATION),
        Rule::token(r"(?im)[a-z_]\w*", NAME),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)\/\/.*", COMMENT_SINGLE),
            Rule::token(r"(?im)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"statements", vec![
        Rule::token(r"(?im)(RECORD|END)\D", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?im)((?:ASCII|BIG_ENDIAN|BOOLEAN|DATA|DECIMAL|EBCDIC|INTEGER|PATTERN|QSTRING|REAL|RECORD|RULE|SET OF|STRING|TOKEN|UDECIMAL|UNICODE|UNSIGNED|VARSTRING|VARUNICODE)\d*)(\s+)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE)]),
        Rule::token(r"(?im)(A(?:PPLY|SSERT)|BUILD(?:(?:INDEX)?)|CHECKPOINT|DEPRECATED|EVALUATE|FAIL(?:(?:(?:COD|MESSAG|UR)E)?)|GLOBAL|INDEPENDENT|KEY(?:DIFF|PATCH)|LOADXML|NOT(?:HOR|IFY)|O(?:NWARNING|UTPUT)|P(?:ARALLEL|ERSIST|RIORITY)|RECOVERY|S(?:EQUENTIAL|OAPCALL|TORED|UCCESS)|W(?:AIT|HEN))\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(A(?:LL|N(?:[DY])|S|TMOST)|BE(?:FORE|GINC\+\+|ST|TWEEN)|C(?:ASE|O(?:NST|UNTER)|SV)|DESCEND|E(?:N(?:CRYPT|D(?:C\+\+|MACRO))|X(?:C(?:EPT|LUSIVE)|P(?:IRE|ORT)|TEND))|F(?:ALSE|EW|IRST|LAT|U(?:LL|NCTION))|GROUP|H(?:EAD(?:ER|ING)|OLE)|I(?:FBLOCK|MPORT|N(?:(?:TERFACE)?))|JOINED|KE(?:EP|YED)|L(?:AST|EFT|IMIT|O(?:AD|CAL(?:(?:E)?)|OKUP))|M(?:A(?:CRO|NY|X(?:COUNT|LENGTH))|IN\ SKEW|ODULE)|N(?:AMED|O(?:CASE|ROOT|S(?:CAN|ORT)|T))|O(?:NLY|PT|UTER|VERWRITE|[FR])|P(?:A(?:CKED|RTITION)|ENALTY|HYSICALLENGTH|IPE)|QUOTE|R(?:E(?:LATIONSHIP|PEAT|TURN)|IGHT)|S(?:CAN|E(?:LF|PARATOR|RVICE)|HARED|K(?:EW|IP)|QL|TORE)|T(?:ERMINATOR|H(?:OR|RESHOLD)|OKEN|R(?:ANSFORM|IM|UE)|YPE)|UN(?:ICODEORDER|SORTED)|V(?:ALIDATE|IRTUAL)|W(?:HOLE|I(?:LD|THIN))|X(?:ML|PATH)|__COMPRESSED__)\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(A(?:BS|COS|LLNODES|S(?:CII|IN|STRING)|TAN(?:(?:2)?)|VE)|C(?:ASE|HOOSE(?:(?:N|SETS)?)|LUSTERSIZE|O(?:MBINE|RRELATION|S(?:(?:H)?)|UNT|VARIANCE)|RON)|D(?:ATASET|E(?:DUP|(?:FIN|NORMALIZ)E)|ISTRIBUT(?:E(?:(?:D)?)|ION))|E(?:BCDIC|NTH|RROR|V(?:ALUATE|ENT(?:(?:EXTRA|NAME)?))|X(?:ISTS|P))|F(?:AIL(?:(?:COD|MESSAG)E)|ETCH|ROMUNICODE)|G(?:ETISVALID|LOBAL|R(?:APH|OUP))|HA(?:SH(?:(?:32|64|CRC|MD5)?)|VING)|I(?:F|N(?:DEX|TFORMAT)|SVALID|TERATE)|JOIN|KEYUNICODE|L(?:ENGTH|I(?:BRARY|MIT)|N|O(?:CAL|G|OP))|M(?:A(?:TCH(?:ED|LENGTH|POSITION|TEXT|UNICODE)|[PX])|ERGE(?:(?:JOIN)?)|IN)|NO(?:LOCAL|NEMPTY|RMALIZE)|P(?:ARSE|IPE|OWER|R(?:ELOAD|O(?:CESS|JECT))|ULL)|R(?:AN(?:DOM|GE|K(?:(?:ED)?))|E(?:ALFORMAT|CORDOF|G(?:EX(?:FIND|REPLACE)|ROUP)|JECTED)|O(?:LLUP|UND(?:(?:UP)?)|W(?:(?:DIFF)?)))|S(?:AMPLE|ET|I(?:N(?:(?:H)?)|ZEOF)|O(?:APCALL|RT(?:(?:ED)?))|QRT|T(?:(?:EPP|OR)ED)|UM)|T(?:A(?:BLE|N(?:(?:H)?))|HISNODE|O(?:PN|UNICODE)|R(?:ANSFER|IM|UNCATE)|YPEOF)|UN(?:GROUP|ICODEORDER)|VARIANCE|W(?:HICH|ORKUNIT)|XML(?:DECODE|ENCODE|TEXT|UNICODE))\b", NAME_FUNCTION),
        Rule::token(r"(?im)^#.*$", COMMENT_PREPROC),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+)e[+-]?\d+[lu]*", NUMBER_FLOAT),
        Rule::token(r"(?im)(\d+\.\d*|\.\d+|\d+f)f?", NUMBER_FLOAT),
        Rule::token(r"(?im)0x[0-9a-f]+[lu]*", NUMBER_HEX),
        Rule::token(r"(?im)0[0-7]+[lu]*", NUMBER_OCT),
        Rule::token(r"(?im)\d+[lu]*", NUMBER_INTEGER),
        Rule::token(r"(?im)[~!%^&*+=|?:<>/-]+", OPERATOR),
        Rule::token(r"(?im)[{}()\[\],.;]", PUNCTUATION),
        Rule::token(r"(?im)[a-z_]\w*", NAME),
    ]);
    m.insert(r"types", vec![
        Rule::token(r"(?im)(RECORD|END)\D", KEYWORD_DECLARATION),
        Rule::bygroups(r"(?im)((?:ASCII|BIG_ENDIAN|BOOLEAN|DATA|DECIMAL|EBCDIC|INTEGER|PATTERN|QSTRING|REAL|RECORD|RULE|SET OF|STRING|TOKEN|UDECIMAL|UNICODE|UNSIGNED|VARSTRING|VARUNICODE)\d*)(\s+)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE)]),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)(A(?:PPLY|SSERT)|BUILD(?:(?:INDEX)?)|CHECKPOINT|DEPRECATED|EVALUATE|FAIL(?:(?:(?:COD|MESSAG|UR)E)?)|GLOBAL|INDEPENDENT|KEY(?:DIFF|PATCH)|LOADXML|NOT(?:HOR|IFY)|O(?:NWARNING|UTPUT)|P(?:ARALLEL|ERSIST|RIORITY)|RECOVERY|S(?:EQUENTIAL|OAPCALL|TORED|UCCESS)|W(?:AIT|HEN))\b", KEYWORD_RESERVED),
        Rule::token(r"(?im)(A(?:LL|N(?:[DY])|S|TMOST)|BE(?:FORE|GINC\+\+|ST|TWEEN)|C(?:ASE|O(?:NST|UNTER)|SV)|DESCEND|E(?:N(?:CRYPT|D(?:C\+\+|MACRO))|X(?:C(?:EPT|LUSIVE)|P(?:IRE|ORT)|TEND))|F(?:ALSE|EW|IRST|LAT|U(?:LL|NCTION))|GROUP|H(?:EAD(?:ER|ING)|OLE)|I(?:FBLOCK|MPORT|N(?:(?:TERFACE)?))|JOINED|KE(?:EP|YED)|L(?:AST|EFT|IMIT|O(?:AD|CAL(?:(?:E)?)|OKUP))|M(?:A(?:CRO|NY|X(?:COUNT|LENGTH))|IN\ SKEW|ODULE)|N(?:AMED|O(?:CASE|ROOT|S(?:CAN|ORT)|T))|O(?:NLY|PT|UTER|VERWRITE|[FR])|P(?:A(?:CKED|RTITION)|ENALTY|HYSICALLENGTH|IPE)|QUOTE|R(?:E(?:LATIONSHIP|PEAT|TURN)|IGHT)|S(?:CAN|E(?:LF|PARATOR|RVICE)|HARED|K(?:EW|IP)|QL|TORE)|T(?:ERMINATOR|H(?:OR|RESHOLD)|OKEN|R(?:ANSFORM|IM|UE)|YPE)|UN(?:ICODEORDER|SORTED)|V(?:ALIDATE|IRTUAL)|W(?:HOLE|I(?:LD|THIN))|X(?:ML|PATH)|__COMPRESSED__)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"functions", vec![
        Rule::token(r"(?im)(A(?:BS|COS|LLNODES|S(?:CII|IN|STRING)|TAN(?:(?:2)?)|VE)|C(?:ASE|HOOSE(?:(?:N|SETS)?)|LUSTERSIZE|O(?:MBINE|RRELATION|S(?:(?:H)?)|UNT|VARIANCE)|RON)|D(?:ATASET|E(?:DUP|(?:FIN|NORMALIZ)E)|ISTRIBUT(?:E(?:(?:D)?)|ION))|E(?:BCDIC|NTH|RROR|V(?:ALUATE|ENT(?:(?:EXTRA|NAME)?))|X(?:ISTS|P))|F(?:AIL(?:(?:COD|MESSAG)E)|ETCH|ROMUNICODE)|G(?:ETISVALID|LOBAL|R(?:APH|OUP))|HA(?:SH(?:(?:32|64|CRC|MD5)?)|VING)|I(?:F|N(?:DEX|TFORMAT)|SVALID|TERATE)|JOIN|KEYUNICODE|L(?:ENGTH|I(?:BRARY|MIT)|N|O(?:CAL|G|OP))|M(?:A(?:TCH(?:ED|LENGTH|POSITION|TEXT|UNICODE)|[PX])|ERGE(?:(?:JOIN)?)|IN)|NO(?:LOCAL|NEMPTY|RMALIZE)|P(?:ARSE|IPE|OWER|R(?:ELOAD|O(?:CESS|JECT))|ULL)|R(?:AN(?:DOM|GE|K(?:(?:ED)?))|E(?:ALFORMAT|CORDOF|G(?:EX(?:FIND|REPLACE)|ROUP)|JECTED)|O(?:LLUP|UND(?:(?:UP)?)|W(?:(?:DIFF)?)))|S(?:AMPLE|ET|I(?:N(?:(?:H)?)|ZEOF)|O(?:APCALL|RT(?:(?:ED)?))|QRT|T(?:(?:EPP|OR)ED)|UM)|T(?:A(?:BLE|N(?:(?:H)?))|HISNODE|O(?:PN|UNICODE)|R(?:ANSFER|IM|UNCATE)|YPEOF)|UN(?:GROUP|ICODEORDER)|VARIANCE|W(?:HICH|ORKUNIT)|XML(?:DECODE|ENCODE|TEXT|UNICODE))\b", NAME_FUNCTION),
    ]);
    m.insert(r"hash", vec![Rule::token(r"(?im)^#.*$", COMMENT_PREPROC)]);
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
            Rule::token_to(r"(?im)\'", STRING, NewState::Pop(1)),
            Rule::token(r#"(?im)[^"\']+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for EclLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
