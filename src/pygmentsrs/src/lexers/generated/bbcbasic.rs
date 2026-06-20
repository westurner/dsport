//! AUTO-GENERATED from `pygments.pygments.lexers.basic:BBCBasicLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:BBCBasicLexer:bbcbasic

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: bbcbasic
pub struct BbcbasicLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)[0-9]+", NAME_LABEL),
            Rule::bygroups(
                r"(?m)(\*)([^\n]*)",
                vec![Some(KEYWORD_PSEUDO), Some(COMMENT_SPECIAL)],
            ),
            Rule::default(NewState::Push(vec![r"code"])),
        ],
    );
    m.insert(r"code", vec![
        Rule::bygroups(r"(?m)(REM)([^\n]*)", vec![Some(KEYWORD_DECLARATION), Some(COMMENT_SINGLE)]),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Push(vec![r"root"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m):", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)(DEF)(\s*)(FN|PROC)([A-Za-z_@][\w@]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(KEYWORD_DECLARATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(FN|PROC)([A-Za-z_@][\w@]*)", vec![Some(KEYWORD), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(GOTO|GOSUB|THEN|RESTORE)(\s*)(\d+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?m)(TRUE|FALSE)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(PAGE|LOMEM|HIMEM|TIME|WIDTH|ERL|ERR|REPORT\$|POS|VPOS|VOICES)", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(A(?:BS|CS|DVAL|ND|S(?:[CN])|TN)|B(?:(?:GE|PU)T)|C(?:ALL|H(?:AIN|R\$)|L(?:EAR|OSE|[GS])|O(?:LOUR|S|UNT))|D(?:ATA|E(?:[FG])|I(?:[MV])|RAW)|E(?:LSE|N(?:D(?:(?:PROC)?)|VELOPE)|O(?:[FR])|R(?:ROR|[LR])|VAL|X(?:[PT]))|F(?:ALSE|N|OR)|G(?:COL|ET(?:(?:\$)?)|O(?:SUB|TO))|HIMEM(?:(?:)?)|I(?:F|N(?:KEY(?:(?:\$)?)|PUT|STR|T))|L(?:E(?:FT\$|[NT])|INE|N|O(?:CAL|G|MEM(?:(?:)?)))|M(?:ID\$|O(?:D(?:(?:E)?)|VE))|N(?:(?:EX|O)T)|O(?:FF|PEN(?:IN|OUT|UP)|SCLI|THERWISE|[NR])|P(?:AGE(?:(?:)?)|I|LOT|O(?:INT|S)|R(?:INT|OC)|TR(?:(?:)?))|R(?:AD|E(?:AD|M|P(?:(?:EA|OR)T)|STORE|TURN)|IGHT\$|ND|UN)|S(?:GN|IN|OUND|PC|QR|T(?:EP|OP|R(?:(?:(?:ING)?)\$)))|T(?:A(?:[BN])|HEN|IME(?:(?:)?)|O|R(?:(?:AC|U)E))|U(?:NTIL|SR)|V(?:AL|DU|POS)|WIDTH)", KEYWORD),
        Rule::token(r"(?m)(A(?:PPEND|UTO)|BEAT(?:(?:S)?)|C(?:ASE|IRCLE|RUNCH)|DELETE|E(?:DIT|LLIPSE|ND(?:CASE|IF|WHILE))|FILL|HELP|INSTALL(?:(?:)?)|L(?:I(?:BRARY|ST)|OAD|VAR)|MOUSE|NEW|O(?:F|LD|RIGIN|VERLAY)|POINT|QUIT|RE(?:CTANGLE|NUMBER)|S(?:AVE|TEREO|UM|WAP|YS)|T(?:E(?:MPO|XT(?:LOAD|SAVE))|INT|WIN(?:(?:O)?))|VOICE(?:(?:S)?)|W(?:AIT|H(?:EN|ILE)))", KEYWORD),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)%[01]{1,32}", NUMBER_BIN),
        Rule::token(r"(?m)&[0-9a-f]{1,8}", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]+\.[0-9]*(E[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\.[0-9]+(E[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?[0-9]+E[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)([A-Za-z_@][\w@]*[%$]?)", NAME_VARIABLE),
        Rule::token(r"(?m)([+\-]=|[$!|?+\-*/%^=><();]|>=|<=|<>|<<|>>|>>>|,)", OPERATOR),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^"\n]+"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\n", ERROR, NewState::Push(vec![r"root"])),
        ],
    );
    Table(m)
}

impl Lexer for BbcbasicLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
