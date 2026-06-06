//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:EasytrieveLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:EasytrieveLexer:easytrieve

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: easytrieve
pub struct EasytrieveLexer;

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
        Rule::token(r"(?m)\*.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\n+", WHITESPACE),
        Rule::token_to(r"(?m)&[^ \'.,():\n*]+\.", NAME_VARIABLE, NewState::Push(vec![r"after_macro_argument"])),
        Rule::token(r"(?m)%[^ \'.,():\n*]+", NAME_VARIABLE),
        Rule::bygroups_to(r"(?m)(FILE|MACRO|REPORT)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"after_declaration"])),
        Rule::bygroups(r"(?m)(JOB|PARM)([ \'.,():\n])", vec![Some(KEYWORD_DECLARATION), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(A(?:FTER\-(?:BREAK|LINE|SCREEN)|IM|ND|TTR)|B(?:EFORE(?:(?:\-(?:BREAK|LINE|SCREEN))?)|USHU|Y)|C(?:A(?:LL|SE)|H(?:ECKPOINT|KP(?:(?:\-STATUS)?))|L(?:EAR|OSE)|O(?:L(?:(?:OR)?)|MMIT|NTROL|PY)|URSOR)|D(?:E(?:CLARE|F(?:AULT|INE)|LETE|NWA)|ISPLAY|LI|O|UPLICATE)|E(?:LSE(?:(?:\-IF)?)|N(?:D(?:(?:\-(?:CASE|DO|IF|PROC)|(?:PAG|TABL)E)?)|TER)|OF|Q|RROR|X(?:IT|TERNAL)|ZLIB)|F(?:1(?:[0123456789])|2(?:[0123456789])|3(?:[0123456])|ETCH|I(?:L(?:E\-STATUS|L)|NAL|RST(?:(?:\-DUP)?))|OR|[123456789])|G(?:ET|OTO|[EOQRT])|H(?:E(?:ADING|X)|IGH\-VALUES)|I(?:D(?:D|MS)|NSERT|[FN])|JUSTIFY|K(?:ANJI\-(?:DATE(?:(?:\-LONG)?)|TIME)|EY(?:(?:\-PRESSED)?)|OKUGO|UN)|L(?:AST\-DUP|EVEL|I(?:KE|N(?:E\-(?:COUNT|NUMBER)|[EK])|ST)|OW\-VALUES|[EQST])|M(?:A(?:CRO|SK|TCHED)|E(?:ND|SSAGE)|OVE|START)|N(?:EWPAGE|O(?:MASK|PRINT|T(?:(?:E)?)|VERIFY)|ULL|[EQ])|O(?:THERWISE|[FR])|P(?:A(?:GE\-(?:COUNT|NUMBER)|RM\-REGISTER|T(?:H\-ID|TERN)|[123])|ERFORM|O(?:INT|S)|R(?:I(?:MARY|NT)|O(?:CEDURE|GRAM))|UT)|R(?:E(?:AD|CORD(?:(?:\-(?:COUNT|LENGTH))?)|FRESH|LEASE|NUM|P(?:(?:EA|OR(?:(?:T\-INPU)?))T)|S(?:HOW|TART)|T(?:(?:RIEV|URN\-COD)E))|O(?:LLBACK|W))|S(?:CREEN|E(?:ARCH|CONDARY|LECT|QUENCE)|IZE|KIP|O(?:KAKU|RT)|QL|TOP|UM|YS(?:DATE(?:(?:\-LONG)?)|I(?:N|PT)|LST|PRINT|SNAP|TIME))|T(?:ALLY|ERM(?:\-(?:COLUMNS|NAME|ROWS)|INATION)|ITLE|O|R(?:ANSFER|C))|U(?:N(?:IQUE|TIL)|P(?:(?:DAT|PERCAS)E)|SER(?:(?:ID)?))|V(?:ALUE|ERIFY)|W(?:H(?:EN|ILE)|ORK|RITE)|X(?:DM|RST)|[DESWX])([ \'.,():\n])", vec![Some(KEYWORD_RESERVED), Some(OPERATOR)]),
        Rule::token(r"(?m)[.+\-/=\[\](){}<>;,&%¬]", OPERATOR),
        Rule::bygroups(r"(?m)([^ \'.,():\n*]+)(\s*)(\.?)(\s*)(PROC)(\s*\n)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD_DECLARATION), Some(WHITESPACE)]),
        Rule::token(r"(?m)[0-9]+\.[0-9]*", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)'(''|[^'])*'", STRING),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)[^ \'.,():\n*]+", NAME),
    ]);
    m.insert(r"after_declaration", vec![
        Rule::token(r"(?m)[^ \'.,():\n*]+", NAME_FUNCTION),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"after_macro_argument", vec![
        Rule::token_to(r"(?m)\*.*\n", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\s+", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?m)[.+\-/=\[\](){}<>;,&%¬]", OPERATOR, NewState::Pop(1)),
        Rule::token_to(r"(?m)'(''|[^'])*'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)[^ \'.,():\n*]+", NAME),
    ]);
    Table(m)
}

impl Lexer for EasytrieveLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
