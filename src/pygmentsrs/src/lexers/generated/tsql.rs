//! AUTO-GENERATED from `pygments.pygments.lexers.sql:TransactSqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:TransactSqlLexer:tsql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tsql, t-sql
pub struct TsqlLexer;

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
        Rule::token(r"(?im)--.*[$|\n]?", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token(r"(?im)(!(?:[<=>])|%=|\&=|\*=|\+=|\-=|/=|::|<(?:[=>])|(?:[>\^|])=|[%&*+\-/<=>\^|~])", OPERATOR),
        Rule::token(r"(?im)(a(?:ll|n(?:[dy]))|between|ex(?:cept|ists)|in(?:(?:tersect)?)|like|not|or|some|union)\b", OPERATOR_WORD),
        Rule::token(r"(?im)(bi(?:gint|nary|t)|c(?:(?:ha|urso)r)|d(?:ate(?:(?:time(?:(?:2|offset)?))?)|ecimal)|float|hierarchyid|i(?:mage|nt)|money|n(?:char|text|umeric|varchar)|real|s(?:mall(?:datetime|int|money)|ql_variant)|t(?:able|ext|i(?:me(?:(?:stamp)?)|nyint))|uniqueidentifier|var(?:binary|char)|xml)\b", NAME_CLASS),
        Rule::token(r"(?im)(\$partition|a(?:bs|cos|pp(?:_name|lock_(?:mode|test))|s(?:cii|in|semblyproperty)|t(?:an|n2)|vg)|binary_checksum|c(?:ast|e(?:iling|rt(?:encoded|privatekey))|h(?:ar(?:(?:index)?)|ecksum(?:(?:_agg)?)|oose)|o(?:l(?:_(?:length|name)|umnproperty)|mpress|n(?:cat|nectionproperty|text_info|vert)|unt(?:(?:_big)?)|[st])|ur(?:rent_(?:request_id|t(?:imestamp|ransaction_id)|user)|sor_status))|d(?:a(?:t(?:abase(?:_principal_id|propertyex)|e(?:add|diff(?:(?:_big)?)|fromparts|name|part|time(?:(?:(?:2|offset)?)fromparts)))|y)|b_(?:id|name)|e(?:compress|grees|nse_rank)|ifference)|e(?:omonth|rror_(?:line|message|number|procedure|s(?:everity|tate))|xp)|f(?:ile(?:_(?:id(?:(?:ex)?)|name)|group(?:_(?:id|name)|property)|property)|loor|ormat(?:(?:message)?)|ulltext(?:(?:catalog|service)property))|g(?:et(?:_filestream_transaction_context|ansinull|(?:(?:utc)?)date)|rouping(?:(?:_id)?))|h(?:as_perms_by_name|ost_(?:id|name))|i(?:if|ndex(?:_col|(?:(?:key_)?)property)|s(?:_(?:(?:(?:(?:(?:srv)?)role)?)member)|date|json|nu(?:ll|meric)))|json_(?:modify|query|value)|l(?:e(?:ft|n)|o(?:g(?:(?:10)?)|wer)|trim)|m(?:ax|in(?:(?:_active_rowversion)?)|onth)|n(?:char|ew(?:(?:(?:sequential)?)id)|tile)|o(?:bject(?:_(?:definition|id|(?:(?:schema_)?)name)|property(?:(?:ex)?))|pen(?:datasource|json|query|rowset|xml)|riginal_(?:db_name|login))|p(?:a(?:rse(?:(?:name)?)|tindex)|ermissions|i|ower|wd(?:compare|encrypt))|quotename|r(?:a(?:dians|n(?:[dk]))|e(?:(?:pl(?:ac|icat)|vers)e)|ight|o(?:und|w(?:_number|count_big))|trim)|s(?:c(?:hema_(?:id|name)|ope_identity)|e(?:rverproperty|ssion_(?:context|user))|i(?:(?:(?:g)?)n)|malldatetimefromparts|oundex|p(?:(?:_helplanguag|ac)e)|q(?:rt|uare)|t(?:ats_date|dev(?:(?:p)?)|r(?:(?:ing_(?:escape|split))?)|uff)|u(?:bstring|m|ser_(?:id|name|s(?:id|name)))|witchoffset|ys(?:datetime(?:(?:offset)?)|tem_user|utcdatetime))|t(?:an|ext(?:ptr|valid)|imefromparts|odatetimeoffset|ry_(?:c(?:(?:as|onver)t)|parse)|ype(?:_(?:id|name)|property))|u(?:nicode|pper|ser_(?:id|name))|var(?:(?:p)?)|xact_state|year)\b", NAME_FUNCTION),
        Rule::bygroups(r"(?im)(goto)(\s+)(\w+\b)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?im)(a(?:bsolute|ction|d(?:min|[ad])|fter|ggregate|l(?:ias|l(?:(?:ocate)?)|ter)|n(?:[dy])|r(?:e|ray)|s(?:c|ensitive|sertion|ymmetric)|tomic|uthorization|vg|[st])|b(?:ackup|e(?:fore|(?:gi|twee)n)|i(?:nary|t(?:(?:_length)?))|lob|o(?:olean|th)|r(?:ea(?:dth|k)|owse)|ulk|y)|c(?:a(?:ll(?:(?:ed)?)|rdinality|s(?:cade(?:(?:d)?)|[et])|t(?:alog|ch))|h(?:ar(?:(?:_length|acter(?:(?:_length)?))?)|eck(?:(?:point)?))|l(?:ass|o(?:b|se)|ustered)|o(?:alesce|l(?:l(?:at(?:e|ion)|ect)|umn)|m(?:mit|p(?:letion|ute))|n(?:dition|nect(?:(?:ion)?)|str(?:aint(?:(?:s)?)|uctor)|t(?:ains(?:(?:table)?)|inue)|vert)|rr(?:(?:esponding)?)|unt|var_(?:(?:po|sam)p))|r(?:eate|oss)|u(?:be|me_dist|r(?:rent(?:(?:_(?:catalog|d(?:ate|efault_transform_group)|path|role|schema|t(?:ime(?:(?:stamp)?)|ransform_group_for_type)|user))?)|sor))|ycle)|d(?:a(?:t(?:abase|[ae])|y)|bcc|e(?:allocate|c(?:(?:imal|lare)?)|f(?:ault|err(?:able|ed))|lete|ny|pth|ref|s(?:c(?:(?:ri(?:be|ptor))?)|tr(?:oy|uctor))|terministic)|i(?:agnostics|ctionary|s(?:connect|k|t(?:inct|ributed)))|o(?:main|uble)|rop|ump|ynamic)|e(?:ach|l(?:ement|se)|nd(?:(?:\-exec)?)|quals|rrlvl|scape|very|x(?:cept(?:(?:ion)?)|ec(?:(?:ute)?)|i(?:sts|t)|t(?:ernal|ract)))|f(?:alse|etch|i(?:l(?:e|(?:lfacto|te)r)|rst)|loat|o(?:r(?:(?:(?:eig|tra)n)?)|und)|r(?:ee(?:(?:text(?:(?:table)?))?)|om)|u(?:ll(?:(?:texttable)?)|(?:nct|s)ion))|g(?:e(?:neral|t)|lobal|o(?:(?:to)?)|r(?:ant|oup(?:(?:ing)?)))|h(?:aving|o(?:ld(?:(?:lock)?)|st|ur))|i(?:dentity(?:(?:_insert|col)?)|gnore|mmediate|n(?:clude|d(?:ex|icator)|itial(?:ize|ly)|ner|out|put|se(?:nsitive|rt)|t(?:(?:e(?:ger|r(?:sect(?:(?:ion)?)|val))|o)?))|solation|terate|[fns])|join|k(?:ey|ill)|l(?:a(?:nguage|rge|st|teral)|e(?:ading|ft|ss|vel)|i(?:ke(?:(?:_regex)?)|mit|neno)|n|o(?:ad|ca(?:l(?:(?:time(?:(?:stamp)?))?)|tor)|wer))|m(?:a(?:tch|[px])|e(?:mber|rge|thod)|in(?:(?:ute)?)|o(?:d(?:(?:if(?:ies|y)|ule)?)|nth)|ultiset)|n(?:a(?:mes|t(?:(?:ion|ur)al))|c(?:har|lob)|e(?:w|xt)|o(?:(?:check|n(?:clustered|e)|rmalize|t)?)|u(?:ll(?:(?:if)?)|meric))|o(?:bject|c(?:currences_regex|tet_length)|ff(?:(?:sets)?)|ld|nly|p(?:e(?:n(?:(?:datasource|query|rowset|xml)?)|ration)|tion)|rd(?:er|inality)|ut(?:(?:er|put)?)|ver(?:(?:la(?:ps|y))?)|[fnr])|p(?:a(?:d|r(?:ameter(?:(?:s)?)|ti(?:al|tion))|scal|th)|ercent(?:(?:_rank|ile_(?:cont|disc))?)|ivot|lan|os(?:ition(?:(?:_regex)?)|tfix)|r(?:e(?:cision|fix|order|(?:par|serv)e)|i(?:mary|nt|or|vileges)|oc(?:(?:edure)?))|ublic)|r(?:a(?:iserror|nge)|e(?:a(?:d(?:s|text)|[dl])|c(?:(?:onfigur|ursiv)e)|f(?:(?:erenc(?:es|ing))?)|gr_(?:avg(?:[xy])|count|intercept|r2|s(?:lope|x(?:[xy])|yy))|l(?:(?:ativ|eas)e)|plication|s(?:t(?:ore|rict)|ult)|turn(?:(?:s)?)|v(?:ert|oke))|ight|o(?:l(?:e|l(?:back|up))|utine|w(?:(?:count|guidcol|s)?))|ule)|s(?:ave(?:(?:point)?)|c(?:hema|ope|roll)|e(?:arch|c(?:ond|tion|urityaudit)|lect|mantic(?:(?:keyphrase|similarity(?:(?:details)?))table)|nsitive|quence|ssion(?:(?:_user)?)|t(?:(?:s|user)?))|hutdown|i(?:milar|ze)|mallint|ome|p(?:ace|ecific(?:(?:type)?))|ql(?:(?:c(?:a|ode)|e(?:rror|xception)|state|warning)?)|t(?:a(?:rt|t(?:e(?:(?:ment)?)|i(?:c|stics)))|ddev_(?:(?:po|sam)p)|ructure)|u(?:b(?:multiset|string(?:(?:_regex)?))|m)|y(?:mmetric|stem(?:(?:_user)?)))|t(?:able(?:(?:sample)?)|e(?:mporary|(?:rminat|xtsiz)e)|h(?:an|en|row)|ime(?:(?:stamp|zone_(?:hour|minute))?)|o(?:(?:p)?)|r(?:a(?:iling|n(?:(?:s(?:action|lat(?:e(?:(?:_regex)?)|ion)))?))|eat|i(?:gger|m)|u(?:(?:(?:ncat)?)e)|y(?:(?:_convert)?))|sequal)|u(?:escape|n(?:der|i(?:on|que)|known|(?:nes|pivo)t)|p(?:date(?:(?:text)?)|per)|s(?:age|e(?:(?:r)?)|ing))|v(?:a(?:lue(?:(?:s)?)|r(?:_(?:(?:po|sam)p)|char|iable|ying))|iew)|w(?:aitfor|h(?:e(?:n(?:(?:ever)?)|re)|ile)|i(?:dth_bucket|ndow|th(?:(?:in|out)?))|ork|rite(?:(?:text)?))|xml(?:a(?:gg|ttributes)|binary|c(?:(?:as|o(?:mmen|nca))t)|document|e(?:lement|xists)|forest|iterate|namespaces|p(?:arse|i)|query|serialize|t(?:able|ext)|validate)|year|zone)\b", KEYWORD),
        Rule::bygroups(r"(?im)(\[)([^\]]+)(\])", vec![Some(OPERATOR), Some(NAME), Some(OPERATOR)]),
        Rule::token(r"(?im)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)\.[0-9]+(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r#"(?im)"(""|[^"])*""#, STRING_SYMBOL),
        Rule::token(r"(?im)[;(),.]", PUNCTUATION),
        Rule::token(r"(?im)@@\w+", NAME_BUILTIN),
        Rule::token(r"(?im)@\w+", NAME_VARIABLE),
        Rule::bygroups(r"(?im)(\w+)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?im)#?#?\w+", NAME),
        Rule::token(r"(?im)\?", NAME_VARIABLE_MAGIC),
    ]);
    m.insert(r"multiline-comments", vec![
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comments"])),
        Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)[^/*]+", COMMENT_MULTILINE),
        Rule::token(r"(?im)[/*]", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for TsqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
