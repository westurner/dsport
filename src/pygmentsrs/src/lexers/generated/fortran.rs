//! AUTO-GENERATED from `pygments.pygments.lexers.fortran:FortranLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.fortran:FortranLexer:fortran

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: fortran, f90
pub struct FortranLexer;

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
        Rule::token(r"(?im)^#.*\n", COMMENT_PREPROC),
        Rule::token(r"(?im)!.*\n", COMMENT),
        Rule::token(r#"(?im)"(\\[0-7]+|\\[^0-7]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?im)'(\\[0-7]+|\\[^0-7]|[^'\\])*'", STRING_SINGLE),
        Rule::bygroups(r"(?im)\b(DO)(\s+)(CONCURRENT)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)\b(GO)(\s*)(TO)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)\b(A(?:BSTRACT|CCEPT|LL(?:(?:OCAT(?:(?:(?:ABL)?)E)|STOP)?)|RRAY|S(?:S(?:IGN|OCIATE)|YNCHRONOUS))|B(?:ACKSPACE|IND|LOCK(?:(?:DATA)?)|YTE)|C(?:A(?:LL|SE)|L(?:ASS|OSE)|O(?:DIMENSION|MMON|NT(?:AINS|I(?:GUOUS|NUE)))|RITICAL|YCLE)|D(?:ATA|E(?:ALLOCATE|CODE|FERRED)|IMENSION|O)|E(?:L(?:EMENTAL|SE(?:(?:IF)?))|N(?:CODE|D(?:(?:ASSOCIATE|BLOCK|DO|ENUM|F(?:ORALL|UNCTION)|I(?:F|NTERFACE)|MODULE|PROGRAM|S(?:ELECT|UB(?:(?:MODUL|ROUTIN)E))|(?:TYP|WHER)E)?)|TRY|UM(?:(?:ERATOR)?))|QUIVALENCE|RROR\ STOP|X(?:IT|T(?:E(?:NDS|RNAL)|RINSIC)))|F(?:I(?:LE|NAL)|OR(?:ALL|MAT)|UNCTION)|GENERIC|I(?:M(?:AGES|P(?:LICIT|ORT|URE))|N(?:CLUDE|QUIRE|T(?:E(?:NT|RFACE)|RINSIC))|[FS])|LOCK|M(?:EMORY|ODULE)|N(?:AMELIST|O(?:N(?:E|_(?:INTRINSIC|OVERRIDABLE))|PASS)|ULLIFY)|O(?:NLY|P(?:EN|TION(?:AL|S)))|P(?:A(?:RAMETER|SS|USE)|OINTER|R(?:I(?:NT|VATE)|O(?:CEDURE|GRAM|TECTED))|U(?:BLIC|RE))|RE(?:AD|CURSIVE|SULT|TURN|WIND)|S(?:AVE|E(?:LECT|QUENCE)|TOP|UB(?:(?:MODUL|ROUTIN)E)|YNC(?:(?:ALL|IMAGES|MEMORY)?))|T(?:ARGET|HEN|YPE)|U(?:NLOCK|SE)|(?:V(?:ALU|OLATIL)|W(?:H(?:ER|IL)|RIT))E)\s*\b", KEYWORD),
        Rule::token(r"(?im)\b(C(?:HARACTER|OMPLEX|_(?:BOOL|CHAR|DOUBLE(?:(?:_COMPLEX)?)|F(?:LOAT(?:(?:_COMPLEX)?)|UNPTR)|INT(?:(?:(?:16|32|64|8|MAX|PTR|_(?:FAST(?:16|32|64|8)|LEAST(?:16|32|64|8)))_T)?)|LONG(?:(?:_(?:DOUBLE(?:(?:_COMPLEX)?)|LONG))?)|PTR|S(?:HORT|I(?:GNED_CHAR|ZE_T))))|DOUBLE\ (?:COMPLEX|PRECISION)|INTEGER|(?:LOGIC|RE)AL)\s*\b", KEYWORD_TYPE),
        Rule::token(r"(?im)(\*\*|\*|\+|-|\/|<|>|<=|>=|==|\/=|=)", OPERATOR),
        Rule::token(r"(?im)(::)", KEYWORD_DECLARATION),
        Rule::token(r"(?im)[()\[\],:&%;.]", PUNCTUATION),
        Rule::token(r"(?im)\b(A(?:C(?:har|os(?:(?:H)?))|I(?:mag|nt)|Log|M(?:ax|in|od)|NInt|Sin(?:(?:H)?)|Tan(?:(?:H)?)|b(?:ort|s)|ccess|djust(?:[LR])|l(?:arm|l(?:(?:ocated)?))|n(?:[dy])|ssociated|tomic_(?:Define|Ref))|B(?:G(?:[ET])|L(?:[ET])|Test|es(?:JN|YN|sel_(?:J(?:[01N])|Y(?:[01N]))|[JY])|it_Size)|C(?:Abs|Cos|Exp|Log|PU_Time|S(?:hift|in|qRt)|Time|_(?:A(?:lert|ssociated)|Backspace|Carriage_Return|F(?:_P(?:(?:(?:rocP)?)ointer)|orm_Feed|unLoc)|Horizontal_Tab|Loc|N(?:ew_Line|ull_(?:(?:Cha|(?:Funp|P)t)r))|Sizeof|Vertical_Tab)|eiling|h(?:Dir|Mod|ar)|mplx|o(?:m(?:mand_Argument_Count|plex)|njg|s(?:(?:H)?)|unt))|D(?:A(?:Cos|Sin|Tan|bs)|Cos(?:(?:H)?)|DiM|E(?:rF(?:(?:C)?)|xp)|Int|Log|M(?:ax|in|od)|NInt|Prod|S(?:hift(?:[LR])|i(?:gn|n(?:(?:H)?))|qRt)|T(?:an(?:(?:H)?)|ime)|ate_and_Time|b(?:es(?:(?:[JY])N|[JY])|le)|i(?:M|gits)|ot_Product)|E(?:OShift|Time|psilon|rF(?:(?:C(?:(?:_Scaled)?))?)|x(?:ecute_Command_Line|it|p(?:(?:onent)?)|tends_Type_Of))|F(?:Date|Get(?:(?:C)?)|Num|Put(?:(?:C)?)|S(?:eek|tat)|Tell|indLoc|l(?:o(?:at|or)|ush)|raction)|G(?:Error|MTime|amma|et(?:Arg|CWD|Env|GId|Log|PId|UId|_(?:Command(?:(?:_Argument)?)|Environment_Variable)))|H(?:ostNm|uge|ypot)|I(?:A(?:Char|bs|ll|n(?:[dy])|rgC)|B(?:Clr|Set|its)|Char|D(?:Int|NInt|ate|iM)|E(?:Or|rrNo)|Fix|Or|Parity|Rand|S(?:hft(?:(?:C)?)|ign)|Time|mag(?:(?:Part|e_Index)?)|n(?:dex|t)|s(?:_(?:Contiguous|Iostat_E(?:nd|or))|aTty|o_C_Binding))|Ki(?:ll|nd)|L(?:Bound|CoBound|G(?:[et])|L(?:[et])|S(?:(?:hif|ta)t)|Time|en(?:(?:_Trim)?)|ink|nBlnk|o(?:g(?:_Gamma|ical)|ng|[cg]))|M(?:Clock|a(?:sk(?:[LR])|tMul|x(?:(?:Exponent|Loc|Val)?))|erge(?:(?:_Bits)?)|in(?:(?:Exponent|Loc|Val)?)|o(?:d(?:(?:ulo)?)|ve_Alloc)|vBits)|N(?:Int|e(?:arest|w_Line)|o(?:rm2|t)|u(?:ll|m_Images))|Or|P(?:Error|a(?:ck|rity)|r(?:e(?:cision|sent)|oduct))|R(?:RSpacing|Shift|a(?:dix|n(?:d(?:(?:om_(?:Number|Seed))?)|ge))|e(?:al(?:(?:Part)?)|name|peat|shape))|S(?:Rand|ame_Type_As|ca(?:le|n)|e(?:cond|lected_(?:(?:Char|Int|Real)_Kind)|t_Exponent)|h(?:ape|ift(?:[ALR])|ort)|i(?:gn(?:(?:al)?)|n(?:(?:H)?))|leep|ngl|p(?:acing|read)|qRt|t(?:at|orage_Size)|um|y(?:mLnk|stem(?:(?:_Clock)?)))|T(?:an(?:(?:H)?)|his_Image|i(?:me|ny)|r(?:a(?:ilZ|ns(?:fer|pose))|im)|tyNam)|U(?:Bound|CoBound|(?:Mas|n(?:lin|pac))k)|Verify|XOr|Z(?:Abs|Cos|Exp|Log|S(?:in|qRt)))\s*\b", NAME_BUILTIN),
        Rule::token(r"(?im)\.(true|false)\.", NAME_BUILTIN),
        Rule::token(r"(?im)\.(eq|ne|lt|le|gt|ge|not|and|or|eqv|neqv)\.", OPERATOR_WORD),
        Rule::token(r"(?im)[a-z][\w$]*", NAME),
        Rule::token(r"(?im)\d+(?![.e])(_([1-9]|[a-z]\w*))?", NUMBER_INTEGER),
        Rule::token(r"(?im)[+-]?\d*\.\d+([ed][-+]?\d+)?(_([1-9]|[a-z]\w*))?", NUMBER_FLOAT),
        Rule::token(r"(?im)[+-]?\d+\.\d*([ed][-+]?\d+)?(_([1-9]|[a-z]\w*))?", NUMBER_FLOAT),
        Rule::token(r"(?im)[+-]?\d+(\.\d*)?[ed][-+]?\d+(_([1-9]|[a-z]\w*))?", NUMBER_FLOAT),
        Rule::token(r"(?im)[\s]+", WHITESPACE),
    ]);
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?im)"(\\[0-7]+|\\[^0-7]|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?im)'(\\[0-7]+|\\[^0-7]|[^'\\])*'", STRING_SINGLE),
        ],
    );
    m.insert(r"core", vec![
        Rule::bygroups(r"(?im)\b(DO)(\s+)(CONCURRENT)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)\b(GO)(\s*)(TO)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)\b(A(?:BSTRACT|CCEPT|LL(?:(?:OCAT(?:(?:(?:ABL)?)E)|STOP)?)|RRAY|S(?:S(?:IGN|OCIATE)|YNCHRONOUS))|B(?:ACKSPACE|IND|LOCK(?:(?:DATA)?)|YTE)|C(?:A(?:LL|SE)|L(?:ASS|OSE)|O(?:DIMENSION|MMON|NT(?:AINS|I(?:GUOUS|NUE)))|RITICAL|YCLE)|D(?:ATA|E(?:ALLOCATE|CODE|FERRED)|IMENSION|O)|E(?:L(?:EMENTAL|SE(?:(?:IF)?))|N(?:CODE|D(?:(?:ASSOCIATE|BLOCK|DO|ENUM|F(?:ORALL|UNCTION)|I(?:F|NTERFACE)|MODULE|PROGRAM|S(?:ELECT|UB(?:(?:MODUL|ROUTIN)E))|(?:TYP|WHER)E)?)|TRY|UM(?:(?:ERATOR)?))|QUIVALENCE|RROR\ STOP|X(?:IT|T(?:E(?:NDS|RNAL)|RINSIC)))|F(?:I(?:LE|NAL)|OR(?:ALL|MAT)|UNCTION)|GENERIC|I(?:M(?:AGES|P(?:LICIT|ORT|URE))|N(?:CLUDE|QUIRE|T(?:E(?:NT|RFACE)|RINSIC))|[FS])|LOCK|M(?:EMORY|ODULE)|N(?:AMELIST|O(?:N(?:E|_(?:INTRINSIC|OVERRIDABLE))|PASS)|ULLIFY)|O(?:NLY|P(?:EN|TION(?:AL|S)))|P(?:A(?:RAMETER|SS|USE)|OINTER|R(?:I(?:NT|VATE)|O(?:CEDURE|GRAM|TECTED))|U(?:BLIC|RE))|RE(?:AD|CURSIVE|SULT|TURN|WIND)|S(?:AVE|E(?:LECT|QUENCE)|TOP|UB(?:(?:MODUL|ROUTIN)E)|YNC(?:(?:ALL|IMAGES|MEMORY)?))|T(?:ARGET|HEN|YPE)|U(?:NLOCK|SE)|(?:V(?:ALU|OLATIL)|W(?:H(?:ER|IL)|RIT))E)\s*\b", KEYWORD),
        Rule::token(r"(?im)\b(C(?:HARACTER|OMPLEX|_(?:BOOL|CHAR|DOUBLE(?:(?:_COMPLEX)?)|F(?:LOAT(?:(?:_COMPLEX)?)|UNPTR)|INT(?:(?:(?:16|32|64|8|MAX|PTR|_(?:FAST(?:16|32|64|8)|LEAST(?:16|32|64|8)))_T)?)|LONG(?:(?:_(?:DOUBLE(?:(?:_COMPLEX)?)|LONG))?)|PTR|S(?:HORT|I(?:GNED_CHAR|ZE_T))))|DOUBLE\ (?:COMPLEX|PRECISION)|INTEGER|(?:LOGIC|RE)AL)\s*\b", KEYWORD_TYPE),
        Rule::token(r"(?im)(\*\*|\*|\+|-|\/|<|>|<=|>=|==|\/=|=)", OPERATOR),
        Rule::token(r"(?im)(::)", KEYWORD_DECLARATION),
        Rule::token(r"(?im)[()\[\],:&%;.]", PUNCTUATION),
        Rule::token(r"(?im)\b(A(?:C(?:har|os(?:(?:H)?))|I(?:mag|nt)|Log|M(?:ax|in|od)|NInt|Sin(?:(?:H)?)|Tan(?:(?:H)?)|b(?:ort|s)|ccess|djust(?:[LR])|l(?:arm|l(?:(?:ocated)?))|n(?:[dy])|ssociated|tomic_(?:Define|Ref))|B(?:G(?:[ET])|L(?:[ET])|Test|es(?:JN|YN|sel_(?:J(?:[01N])|Y(?:[01N]))|[JY])|it_Size)|C(?:Abs|Cos|Exp|Log|PU_Time|S(?:hift|in|qRt)|Time|_(?:A(?:lert|ssociated)|Backspace|Carriage_Return|F(?:_P(?:(?:(?:rocP)?)ointer)|orm_Feed|unLoc)|Horizontal_Tab|Loc|N(?:ew_Line|ull_(?:(?:Cha|(?:Funp|P)t)r))|Sizeof|Vertical_Tab)|eiling|h(?:Dir|Mod|ar)|mplx|o(?:m(?:mand_Argument_Count|plex)|njg|s(?:(?:H)?)|unt))|D(?:A(?:Cos|Sin|Tan|bs)|Cos(?:(?:H)?)|DiM|E(?:rF(?:(?:C)?)|xp)|Int|Log|M(?:ax|in|od)|NInt|Prod|S(?:hift(?:[LR])|i(?:gn|n(?:(?:H)?))|qRt)|T(?:an(?:(?:H)?)|ime)|ate_and_Time|b(?:es(?:(?:[JY])N|[JY])|le)|i(?:M|gits)|ot_Product)|E(?:OShift|Time|psilon|rF(?:(?:C(?:(?:_Scaled)?))?)|x(?:ecute_Command_Line|it|p(?:(?:onent)?)|tends_Type_Of))|F(?:Date|Get(?:(?:C)?)|Num|Put(?:(?:C)?)|S(?:eek|tat)|Tell|indLoc|l(?:o(?:at|or)|ush)|raction)|G(?:Error|MTime|amma|et(?:Arg|CWD|Env|GId|Log|PId|UId|_(?:Command(?:(?:_Argument)?)|Environment_Variable)))|H(?:ostNm|uge|ypot)|I(?:A(?:Char|bs|ll|n(?:[dy])|rgC)|B(?:Clr|Set|its)|Char|D(?:Int|NInt|ate|iM)|E(?:Or|rrNo)|Fix|Or|Parity|Rand|S(?:hft(?:(?:C)?)|ign)|Time|mag(?:(?:Part|e_Index)?)|n(?:dex|t)|s(?:_(?:Contiguous|Iostat_E(?:nd|or))|aTty|o_C_Binding))|Ki(?:ll|nd)|L(?:Bound|CoBound|G(?:[et])|L(?:[et])|S(?:(?:hif|ta)t)|Time|en(?:(?:_Trim)?)|ink|nBlnk|o(?:g(?:_Gamma|ical)|ng|[cg]))|M(?:Clock|a(?:sk(?:[LR])|tMul|x(?:(?:Exponent|Loc|Val)?))|erge(?:(?:_Bits)?)|in(?:(?:Exponent|Loc|Val)?)|o(?:d(?:(?:ulo)?)|ve_Alloc)|vBits)|N(?:Int|e(?:arest|w_Line)|o(?:rm2|t)|u(?:ll|m_Images))|Or|P(?:Error|a(?:ck|rity)|r(?:e(?:cision|sent)|oduct))|R(?:RSpacing|Shift|a(?:dix|n(?:d(?:(?:om_(?:Number|Seed))?)|ge))|e(?:al(?:(?:Part)?)|name|peat|shape))|S(?:Rand|ame_Type_As|ca(?:le|n)|e(?:cond|lected_(?:(?:Char|Int|Real)_Kind)|t_Exponent)|h(?:ape|ift(?:[ALR])|ort)|i(?:gn(?:(?:al)?)|n(?:(?:H)?))|leep|ngl|p(?:acing|read)|qRt|t(?:at|orage_Size)|um|y(?:mLnk|stem(?:(?:_Clock)?)))|T(?:an(?:(?:H)?)|his_Image|i(?:me|ny)|r(?:a(?:ilZ|ns(?:fer|pose))|im)|tyNam)|U(?:Bound|CoBound|(?:Mas|n(?:lin|pac))k)|Verify|XOr|Z(?:Abs|Cos|Exp|Log|S(?:in|qRt)))\s*\b", NAME_BUILTIN),
        Rule::token(r"(?im)\.(true|false)\.", NAME_BUILTIN),
        Rule::token(r"(?im)\.(eq|ne|lt|le|gt|ge|not|and|or|eqv|neqv)\.", OPERATOR_WORD),
    ]);
    m.insert(
        r"nums",
        vec![
            Rule::token(r"(?im)\d+(?![.e])(_([1-9]|[a-z]\w*))?", NUMBER_INTEGER),
            Rule::token(
                r"(?im)[+-]?\d*\.\d+([ed][-+]?\d+)?(_([1-9]|[a-z]\w*))?",
                NUMBER_FLOAT,
            ),
            Rule::token(
                r"(?im)[+-]?\d+\.\d*([ed][-+]?\d+)?(_([1-9]|[a-z]\w*))?",
                NUMBER_FLOAT,
            ),
            Rule::token(
                r"(?im)[+-]?\d+(\.\d*)?[ed][-+]?\d+(_([1-9]|[a-z]\w*))?",
                NUMBER_FLOAT,
            ),
        ],
    );
    Table(m)
}

impl Lexer for FortranLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
