//! AUTO-GENERATED from `pygments.pygments.lexers.basic:VBScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:VBScriptLexer:vbscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: vbscript
pub struct VbscriptLexer;

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
        Rule::token(r"(?im)'[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)&h[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)\.[0-9]+(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)#.+#", STRING),
        Rule::bygroups_to(r"(?im)(dim)(\s+)([a-z_][a-z0-9_]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE)], NewState::Push(vec![r"dim_more"])),
        Rule::bygroups(r"(?im)(function|sub)(\s+)([a-z_][a-z0-9_]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)(class)(\s+)([a-z_][a-z0-9_]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?im)(const)(\s+)([a-z_][a-z0-9_]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CONSTANT)]),
        Rule::bygroups(r"(?im)(end)(\s+)(class|function|if|property|sub|with)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(on)(\s+)(error)(\s+)(goto)(\s+)(0)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?im)(on)(\s+)(error)(\s+)(resume)(\s+)(next)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(option)(\s+)(explicit)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(property)(\s+)(get|let|set)(\s+)([a-z_][a-z0-9_]*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_PROPERTY)]),
        Rule::token(r"(?im)rem\s.*[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?im)(By(?:Ref|Val)|GetRef|c(?:a(?:ll|se)|lass)|do|e(?:ach|lse(?:(?:if)?)|nd|rase|x(?:ecute|it))|f(?:or|unction(?:(?:)?))|global|if|l(?:et|oop)|ne(?:w|xt)|p(?:rivate|ublic)|redim|s(?:e(?:(?:(?:lec)?)t)|ub)|then|w(?:end|hile|ith))\b", KEYWORD),
        Rule::token(r"(?im)(<(?:[=>])|>=|[$&*+\-/<=>\\\^|])", OPERATOR),
        Rule::token(r"(?im)(and|eqv|i(?:mp|s)|mod|not|(?:(?:x)?)or)\b", OPERATOR_WORD),
        Rule::token(r"(?im)(False|True|vb(?:A(?:bort(?:(?:RetryIgnore)?)|pplicationModal|rray)|B(?:inaryCompare|l(?:ack|ue)|(?:ool|yt)e)|C(?:ancel|r(?:(?:Lf|itical)?)|urrency|yan)|D(?:at(?:aObject|e)|efaultButton(?:[1234])|ouble)|E(?:mpty|rror|xclamation)|F(?:alse|irst(?:FullWeek|Jan1)|ormFeed|riday)|G(?:eneralDate|reen)|I(?:gnore|n(?:formation|teger))|L(?:f|ong(?:(?:(?:Dat|Tim)e)?))|M(?:agenta|onday|sgBox(?:HelpButton|R(?:ight|tlReading)|SetForeground))|N(?:ewLine|o|ull(?:(?:Char|String)?))|O(?:K(?:(?:Cancel|Only)?)|bject(?:(?:Error)?))|Question|Re(?:d|try(?:(?:Cancel)?))|S(?:aturday|hort(?:(?:Dat|Tim)e)|ingle|tring|unday|ystemModal)|T(?:ab|extCompare|hursday|rue|uesday)|Use(?:Default|System(?:(?:)?))|V(?:ariant|erticalTab)|W(?:ednesday|hite)|Ye(?:llow|s(?:(?:No(?:(?:Cancel)?))?))))\b", NAME_CONSTANT),
        Rule::token(r"(?im)(A(?:bs|rray|sc|tn)|C(?:B(?:ool|yte)|Cur|D(?:ate|bl)|Int|Lng|S(?:ng|tr)|hr|os|reateObject)|Da(?:te(?:(?:Add|Diff|Part|Serial|Value)?)|y)|E(?:val|xp)|F(?:i(?:lter|x)|ormat(?:Currency|DateTime|Number|Percent))|Get(?:Locale|Object)|H(?:ex|our)|I(?:n(?:Str|t)|s(?:Array|Date|Empty|Nu(?:ll|meric)|Object))|Join|L(?:Bound|Case|Trim|e(?:ft|n)|o(?:adPicture|g))|M(?:i(?:d|nute)|onth(?:(?:Name)?)|sgBox)|Now|Oct|R(?:GB|Trim|andomize|e(?:gExp|place)|ight|(?:(?:ou)?)nd)|S(?:criptEngine(?:(?:(?:Build|M(?:(?:aj|in)or))Version)?)|e(?:cond|tLocale)|gn|p(?:ace|lit)|qr|tr(?:Comp|Reverse|ing))|T(?:an|ime(?:(?:Serial|Value|r)?)|rim|ypeName)|U(?:Bound|Case)|VarType|Weekday(?:(?:Name)?)|Year|inStrRev)\b", NAME_BUILTIN),
        Rule::token(r"(?im)(D(?:ebug|ictionary|rive(?:(?:s)?))|Err|F(?:ile(?:(?:SystemObject|s)?)|older(?:(?:s)?))|Match(?:(?:es)?)|RegExp|Submatches|TextStream)\b", NAME_BUILTIN),
        Rule::token(r"(?im)[a-z_][a-z0-9_]*", NAME),
        Rule::token(r"(?im)\b_\n", OPERATOR),
        Rule::token(r"(?im)([(),.:])", PUNCTUATION),
        Rule::token(r"(?im).+(\n)?", ERROR),
    ]);
    m.insert(r"dim_more", vec![
        Rule::bygroups(r"(?im)(\s*)(,)(\s*)([a-z_][a-z0-9]*)", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?im)[^"\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?im)\"\""#, STRING_DOUBLE),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token_to(r"(?im)\n", ERROR, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for VbscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
