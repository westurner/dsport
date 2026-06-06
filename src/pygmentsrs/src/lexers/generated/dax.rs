//! AUTO-GENERATED from `pygments.pygments.lexers.dax:DaxLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dax:DaxLexer:dax

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: dax
pub struct DaxLexer;

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
        Rule::token(r"(?m)--.*\n?", COMMENT_SINGLE),
        Rule::token(r"(?m)//.*\n?", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token(r"(?m)(?i)(a(?:bs|c(?:crint(?:(?:m)?)|o(?:(?:[st])h|[st]))|dd(?:(?:column|missingitem)s)|ll(?:(?:crossfiltered|except|noblankrow|selected)?)|mor(?:(?:degr|lin)c)|nd|pproximatedistinctcount|sin(?:(?:h)?)|tan(?:(?:h)?)|verage(?:(?:[ax])?))|b(?:eta\.(?:dist|inv)|it(?:and|lshift|or|rshift|xor)|lank)|c(?:al(?:culate(?:(?:table)?)|endar(?:(?:auto)?))|eiling|hisq\.(?:dist(?:(?:\.rt)?)|inv(?:(?:\.rt)?))|losingbalance(?:month|(?:quarte|yea)r)|o(?:alesce|lumnstatistics|mbin(?:(?:a|evalues)?)|n(?:catenate(?:(?:x)?)|fidence\.(?:norm|t)|tains(?:(?:row|string(?:(?:exact)?))?)|vert)|sh|th|u(?:nt(?:(?:ax|blank|rows|[ax])?)|p(?:day(?:bs|s(?:(?:nc)?))|n(?:cd|um)|pcd))|[st])|ross(?:filter|join)|u(?:m(?:ipmt|princ)|rren(?:cy|tgroup)|stomdata))|d(?:a(?:t(?:atable|e(?:(?:add|diff|s(?:between|(?:inperio|(?:[mqy])t)d)|value)?))|y)|b|db|e(?:(?:gree|tailrow)s)|i(?:s(?:c|tinct(?:(?:count(?:(?:noblank)?))?))|vide)|ollar(?:de|fr)|uration)|e(?:arlie(?:r|st)|date|ffect|ndof(?:month|(?:quarte|yea)r)|omonth|rror|v(?:aluateandlog|en)|x(?:act|cept|p(?:(?:on\.dist)?)))|f(?:a(?:ct|lse)|i(?:lter(?:(?:s)?)|nd|rst(?:date|nonblank(?:(?:value)?))|xed)|loor|ormat|v)|g(?:cd|e(?:nerate(?:(?:all|series)?)|omean(?:(?:x)?))|roupby)|h(?:as(?:h|one(?:filter|value))|our)|i(?:f(?:(?:(?:\.eage|erro)r)?)|gnore|n(?:dex|t(?:(?:ersect|rate)?))|pmt|s(?:after|blank|crossfiltered|e(?:mpty|rror|ven)|filtered|inscope|logical|n(?:ontext|umber)|o(?:\.ceiling|dd|norafter)|pmt|s(?:electedmeasure|ubtotal)|text))|ke(?:epfilters|ywordmatch)|l(?:ast(?:date|nonblank(?:(?:value)?))|cm|e(?:ft|n)|inest(?:(?:x)?)|n|o(?:g(?:(?:10)?)|okupvalue|wer))|m(?:ax(?:(?:[ax])?)|duration|edian(?:(?:x)?)|i(?:n(?:ute|[ax])|[dn])|o(?:d|nth)|round)|n(?:a(?:meof|tural(?:(?:inn|leftout)erjoin))|e(?:tworkdays|xt(?:day|month|(?:quarte|yea)r))|o(?:minal|nvisual|rm\.(?:dist|inv|s\.(?:dist|inv))|[tw])|per)|o(?:dd(?:(?:f(?:price|yield)|l(?:price|yield))?)|ffset|peningbalance(?:month|(?:quarte|yea)r)|r(?:(?:derby)?))|p(?:a(?:r(?:allelperiod|titionby)|th(?:(?:contains|item(?:(?:reverse)?)|length)?))|duration|er(?:centile(?:(?:\.(?:ex|in)|x\.(?:ex|in))c)|mut)|mt|o(?:isson\.dist|wer)|pmt|r(?:evious(?:day|month|(?:quarte|yea)r)|ice(?:(?:disc|mat)?)|oduct(?:(?:x)?))|[iv])|qu(?:arter|otient)|r(?:a(?:dians|n(?:d(?:(?:between)?)|k(?:\.eq|x))|te)|e(?:ceived|lated(?:(?:table)?)|movefilters|p(?:lace|t))|ight|o(?:llup(?:(?:addissubtotal|group|issubtotal)?)|und(?:(?:down|up)?)|w)|ri)|s(?:am(?:eperiodlastyear|ple(?:(?:axiswithlocalminmax)?))|e(?:arch|cond|lect(?:columns|ed(?:measure(?:(?:formatstring|name)?)|value)))|i(?:gn|n(?:(?:h)?))|ln|qrt(?:(?:pi)?)|t(?:artof(?:month|(?:quarte|yea)r)|dev(?:\.(?:[ps])|x\.(?:[ps])))|u(?:bstitute(?:(?:withindex)?)|m(?:(?:marize(?:(?:columns)?)|x)?))|witch|yd)|t(?:\.(?:dist(?:(?:\.(?:(?:[2r])t))?)|inv(?:(?:\.2t)?))|an(?:(?:h)?)|bill(?:eq|price|yield)|ime(?:(?:value)?)|o(?:csv|day|json|pn(?:(?:perlevel|skip)?)|tal(?:(?:[mqy])td))|r(?:eatas|im|u(?:e|nc)))|u(?:ni(?:c(?:har|ode)|on)|pper|ser(?:culture|elationship|name|objectid|principalname)|tc(?:now|today))|v(?:a(?:lue(?:(?:s)?)|r(?:\.(?:[ps])|x\.(?:[ps])))|db)|w(?:eek(?:day|num)|indow)|x(?:irr|npv)|y(?:ear(?:(?:frac)?)|ield(?:(?:disc|mat)?)))\b", NAME_FUNCTION),
        Rule::token(r"(?m)(?i)(a(?:sc|t)|b(?:o(?:olean|th)|y)|c(?:reate|urrency)|d(?:a(?:tetime|y)|e(?:fine|sc)|ouble)|evaluate|false|integer|m(?:easure|onth)|none|order|return|s(?:ingle|t(?:art|ring))|t(?:(?:abl|ru)e)|(?:v|ye)ar)\b", NAME_BUILTIN),
        Rule::token(r"(?m):=|[-+*\/=^]", OPERATOR),
        Rule::token(r"(?m)\b(IN|NOT)\b", OPERATOR_WORD),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)'(?:[^']|'')*'(?!')(?:\[[ \w]+\])?|\w+\[[ \w]+\]", NAME_ATTRIBUTE),
        Rule::token(r"(?m)\[[ \w]+\]", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?<!\w)(\d+\.?\d*|\.\d+\b)", NUMBER),
        Rule::token(r"(?m)[\[\](){}`,.]", PUNCTUATION),
        Rule::token(r"(?m).*\n", TEXT),
    ]);
    m.insert(r"multiline-comments", vec![
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
        Rule::token(r"(?m)[/*]", COMMENT_MULTILINE),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)"""#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^"]+"#, STRING),
    ]);
    Table(m)
}

impl Lexer for DaxLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
