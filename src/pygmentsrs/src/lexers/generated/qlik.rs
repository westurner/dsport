//! AUTO-GENERATED from `pygments.pygments.lexers.qlik:QlikLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.qlik:QlikLexer:qlik

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: qlik, qlikview, qliksense, qlikscript
pub struct QlikLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"comment", vec![
        Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?im)[^*]+", COMMENT_MULTILINE),
    ]);
    m.insert(r"numerics", vec![
        Rule::token(r"(?im)\b\d+\.\d+(e\d+)?[fd]?\b", NUMBER_FLOAT),
        Rule::token(r"(?im)\b\d+\b", NUMBER_INTEGER),
    ]);
    m.insert(r"interp", vec![
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r"(?im)'", STRING, NewState::Pop(1)),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?im)[^'$]+", STRING),
        Rule::token(r"(?im)\$", STRING),
    ]);
    m.insert(r"assignment", vec![
        Rule::token_to(r"(?im);", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?im)//.*\n", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?im)(let|set)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"assignment"])),
        Rule::token(r"(?im)\b(and|bit(?:and|not|(?:(?:x)?)or)|follows|like|not|or|precedes|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?im)(A(?:dd|lias|utoNumber)|B(?:inary|uffer)|C(?:USTOM|o(?:mment\ (?:field(?:(?:s)?)|table(?:(?:s)?))|n(?:catenate|nect))|rosstable)|D(?:e(?:(?:clar|riv)e)|i(?:rect(?:(?:\ Que|o)ry)|sconnect)|rop\ (?:field(?:(?:s)?)|table(?:(?:s)?)))|Execute|F(?:irst|lushLog|orce|rom)|Generic|Hierarchy(?:(?:BelongsTo)?)|In(?:ner|tervalMatch)|Join|Keep|L(?:IB|e(?:(?:(?:f)?)t)|o(?:ad|osen\ Table))|M(?:ap(?:(?:ping)?)|erge)|N(?:oConcatenate|ullAs(?:Null|Value))|O(?:DBC|LEBD|uter)|Partial\ reload|Qualify|R(?:e(?:m|name\ (?:field(?:(?:s)?)|table(?:(?:s)?))|place)|ight)|S(?:QL(?:(?:(?:Column|T(?:(?:abl|yp)e))s)?)|ample|e(?:arch|ction|lect|mantic|t)|leep|t(?:ar|ore))|T(?:ag|race)|Un(?:less|map|qualify|tag)|When|a(?:ccess|pplication|s(?:(?:c)?)|utogenerate)|ca(?:pitalization|se(?:(?:\ (?:lower|mixed|upper))?))|d(?:e(?:fault|sc)|i(?:mension|stinct)|o)|e(?:ach|lse|nd(?:(?:if)?)|x(?:clude|it|plicit|tension))|f(?:or|rom(?:(?:_field)?))|group\ by|i(?:mplicit|n(?:(?:clud|lin)e)|[fn])|loop|measure|next|order\ by|re(?:sident|turn)|s(?:cript|witch(?:(?:)?))|t(?:hen|otal)|u(?:n(?:less|til)|sing)|w(?:h(?:(?:er|il)e)|ith))\b", KEYWORD),
        Rule::token(r"(?im)[a-z]\w*:", KEYWORD_DECLARATION),
        Rule::token(r"(?im)(BrokenWeeks|C(?:ollationLocale|reateSearchIndexOnReload)|D(?:a(?:teFormat|yNames)|ecimalSep)|First(?:MonthOfYear|WeekDay)|Long(?:(?:Day|Month)Names)|Mon(?:ey(?:DecimalSep|Format|ThousandSep)|thNames)|Nu(?:ll(?:Display|Interpret|Value)|mericalAbbreviation)|O(?:penUrlTimeout|therSymbol)|QvWork(?:Path|Root)|ReferenceDay|StripComments|T(?:housandSep|ime(?:(?:(?:stamp)?)Format))|Verbatim|Win(?:Path|Root)|cd|errormode|floppy|hide(?:(?:pre|suf)fix)|include|must_include|null|qv(?:path|root)|scripterror(?:(?:(?:coun|lis)t)?))\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?im)(A(?:RGB|dd(?:(?:Month|Year)s)|pply(?:Codepage|Map)|ttribute|uthor|vg)|C(?:HI(?:DIST|INV)|apitalize|h(?:i2Test_(?:chi2|df|p)|r)|lientPlatform|o(?:lor(?:(?:Map(?:Hue|Jet)|mix(?:[12]))?)|mputerName|n(?:cat|nectString)|rrel|unt))|D(?:ate(?:(?:\#)?)|ocument(?:Name|Path|Title)|ual)|E(?:mptyIsNull|ngineVersion|valuate|xists)|F(?:DIST|INV|V|i(?:eld(?:Index|N(?:ame|umber)|Value(?:(?:Count)?))|le(?:BaseName|Dir|Extension|Name|Path|(?:Siz|Tim)e)|ndOneOf|rst(?:(?:(?:Sorted)?)Value))|ractile(?:(?:Exc)?))|G(?:MT|e(?:o(?:AggrGeometry|BoundingBox|CountVertex|Get(?:BoundingBox|PolygonCenter)|InvProjectGeometry|MakePoint|Project(?:(?:Geometry)?)|ReduceGeometry)|t(?:AlternativeCount|C(?:ollationLocale|urrentSelections)|ExcludedCount|F(?:ieldSelections|olderPath)|NotSelectedCount|Object(?:Dimension|Field(?:(?:)?)|Measure)|PossibleCount|RegistryString|SelectedCount)))|H(?:SL|ash(?:1(?:28|60)|256))|I(?:RR|n(?:dex|terval(?:(?:\#)?))|s(?:Nu(?:ll|m)|PartialReload|Text)|terNo)|K(?:eepChar|urtosis)|L(?:INEST_(?:BLINEST_df|r2|s(?:e(?:[bmy])|sreg)|[fm])|Trim|astValue|e(?:ft|n|venshteinDist)|inest_ssresid|o(?:calTime|okUp|wer))|M(?:a(?:pSubstring|x(?:(?:String)?))|edian|i(?:nString|ssingCount|[dn])|o(?:de|ney(?:(?:\#)?)))|N(?:ORM(?:DIST|INV)|PV|oOf(?:(?:Field|Row|Table)s)|u(?:ll(?:(?:Count)?)|m(?:(?:\#|ericCount)?)))|O(?:SUser|nly|rd)|P(?:V|eek|mt|r(?:evious|oductVersion)|urgeChar)|Qvd(?:CreateTime|FieldName|NoOf(?:(?:Fiel|Recor)ds)|TableName)|R(?:GB|Trim|a(?:nge(?:Avg|Co(?:rrel|unt)|Fractile|IRR|Kurtosis|M(?:ax(?:(?:String)?)|i(?:n(?:(?:String)?)|ssingCount)|ode)|N(?:PV|u(?:(?:ll|meric)Count))|Only|S(?:kew|tdev|um)|TextCount|X(?:IRR|NPV))|te)|e(?:cNo|loadTime|p(?:eat|lace))|ight|owNo)|S(?:TEYX|kew|t(?:ateName|dev|err)|u(?:b(?:Field|StringCount)|m)|ysColor)|T(?:DIST|INV|ableN(?:ame|umber)|ext(?:(?:Between|Count)?)|ime(?:(?:\#|stamp(?:(?:\#)?))?)|rim)|U(?:TC|pper)|ValueL(?:ist|oop)|X(?:IRR|NPV)|a(?:ge|lt|utonumber(?:(?:hash(?:128|256))?))|b(?:itcount|l(?:ack|ue)|rown)|c(?:eil|lass|o(?:alesce|mbin|nverttolocaltime)|yan)|d(?:a(?:rkgray|y(?:(?:end|lightsaving|n(?:ame|umberof(?:(?:quarte|yea)r))|start)?))|iv)|e(?:(?:ven|xp)?)|f(?:a(?:bs|ct|lse)|irstworkdate|loor|mod|rac)|green|hour|i(?:f|n(?:day(?:(?:totime)?)|lunarweek(?:(?:todate)?)|month(?:(?:s(?:(?:todate)?)|todate)?)|quarter(?:(?:todate)?)|week(?:(?:todate)?)|year(?:(?:todate)?)))|l(?:astworkdate|ight(?:blue|cyan|gr(?:ay|een)|magenta|red)|og(?:(?:10)?)|unarweek(?:end|name|start))|m(?:a(?:genta|ke(?:(?:dat|tim|weekdat)e)|tch)|i(?:nute|xmatch)|o(?:d|nth(?:(?:end|name|s(?:end|name|(?:(?:s)?)tart))?)))|n(?:Per|etworkdays|ow)|odd|p(?:ermut|i(?:(?:ck)?)|ow)|quarter(?:end|name|start)|r(?:(?:an|e|oun)d)|s(?:e(?:cond|tdateyear(?:(?:month)?))|ign|qr(?:(?:t)?))|t(?:imezone|oday|rue|test(?:1(?:_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper))|_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)))|w(?:eek(?:(?:day|end|name|start|year)?)|hite|ildmatch)|ye(?:ar(?:(?:end|name|start|todate)?)|llow)|ztest(?:_(?:conf|dif|lower|s(?:ig|terr)|upper|z)|w_(?:conf|dif|lower|s(?:ig|terr)|upper|z)))(?=\s*\()", NAME_BUILTIN, NewState::Push(vec![r"function"])),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token_to(r#"(?im)""#, STRING_SYMBOL, NewState::Push(vec![r"field_name_quote"])),
        Rule::token_to(r"(?im)\[", STRING_SYMBOL, NewState::Push(vec![r"field_name_bracket"])),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)\b\d+\.\d+(e\d+)?[fd]?\b", NUMBER_FLOAT),
        Rule::token(r"(?im)\b\d+\b", NUMBER_INTEGER),
        Rule::token(r"(?im)(<(?:[<=>])|>(?:[=>])|[&*+\-/<=>])", OPERATOR),
        Rule::token(r"(?im)'.+?'", STRING),
        Rule::token(r"(?im)\b\w+\b", TEXT),
        Rule::token(r"(?im)[,;.()\\/]", PUNCTUATION),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?im)//.*\n", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?im)(let|set)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"assignment"])),
        Rule::token(r"(?im)\b(and|bit(?:and|not|(?:(?:x)?)or)|follows|like|not|or|precedes|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?im)(A(?:dd|lias|utoNumber)|B(?:inary|uffer)|C(?:USTOM|o(?:mment\ (?:field(?:(?:s)?)|table(?:(?:s)?))|n(?:catenate|nect))|rosstable)|D(?:e(?:(?:clar|riv)e)|i(?:rect(?:(?:\ Que|o)ry)|sconnect)|rop\ (?:field(?:(?:s)?)|table(?:(?:s)?)))|Execute|F(?:irst|lushLog|orce|rom)|Generic|Hierarchy(?:(?:BelongsTo)?)|In(?:ner|tervalMatch)|Join|Keep|L(?:IB|e(?:(?:(?:f)?)t)|o(?:ad|osen\ Table))|M(?:ap(?:(?:ping)?)|erge)|N(?:oConcatenate|ullAs(?:Null|Value))|O(?:DBC|LEBD|uter)|Partial\ reload|Qualify|R(?:e(?:m|name\ (?:field(?:(?:s)?)|table(?:(?:s)?))|place)|ight)|S(?:QL(?:(?:(?:Column|T(?:(?:abl|yp)e))s)?)|ample|e(?:arch|ction|lect|mantic|t)|leep|t(?:ar|ore))|T(?:ag|race)|Un(?:less|map|qualify|tag)|When|a(?:ccess|pplication|s(?:(?:c)?)|utogenerate)|ca(?:pitalization|se(?:(?:\ (?:lower|mixed|upper))?))|d(?:e(?:fault|sc)|i(?:mension|stinct)|o)|e(?:ach|lse|nd(?:(?:if)?)|x(?:clude|it|plicit|tension))|f(?:or|rom(?:(?:_field)?))|group\ by|i(?:mplicit|n(?:(?:clud|lin)e)|[fn])|loop|measure|next|order\ by|re(?:sident|turn)|s(?:cript|witch(?:(?:)?))|t(?:hen|otal)|u(?:n(?:less|til)|sing)|w(?:h(?:(?:er|il)e)|ith))\b", KEYWORD),
        Rule::token(r"(?im)[a-z]\w*:", KEYWORD_DECLARATION),
        Rule::token(r"(?im)(BrokenWeeks|C(?:ollationLocale|reateSearchIndexOnReload)|D(?:a(?:teFormat|yNames)|ecimalSep)|First(?:MonthOfYear|WeekDay)|Long(?:(?:Day|Month)Names)|Mon(?:ey(?:DecimalSep|Format|ThousandSep)|thNames)|Nu(?:ll(?:Display|Interpret|Value)|mericalAbbreviation)|O(?:penUrlTimeout|therSymbol)|QvWork(?:Path|Root)|ReferenceDay|StripComments|T(?:housandSep|ime(?:(?:(?:stamp)?)Format))|Verbatim|Win(?:Path|Root)|cd|errormode|floppy|hide(?:(?:pre|suf)fix)|include|must_include|null|qv(?:path|root)|scripterror(?:(?:(?:coun|lis)t)?))\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?im)(A(?:RGB|dd(?:(?:Month|Year)s)|pply(?:Codepage|Map)|ttribute|uthor|vg)|C(?:HI(?:DIST|INV)|apitalize|h(?:i2Test_(?:chi2|df|p)|r)|lientPlatform|o(?:lor(?:(?:Map(?:Hue|Jet)|mix(?:[12]))?)|mputerName|n(?:cat|nectString)|rrel|unt))|D(?:ate(?:(?:\#)?)|ocument(?:Name|Path|Title)|ual)|E(?:mptyIsNull|ngineVersion|valuate|xists)|F(?:DIST|INV|V|i(?:eld(?:Index|N(?:ame|umber)|Value(?:(?:Count)?))|le(?:BaseName|Dir|Extension|Name|Path|(?:Siz|Tim)e)|ndOneOf|rst(?:(?:(?:Sorted)?)Value))|ractile(?:(?:Exc)?))|G(?:MT|e(?:o(?:AggrGeometry|BoundingBox|CountVertex|Get(?:BoundingBox|PolygonCenter)|InvProjectGeometry|MakePoint|Project(?:(?:Geometry)?)|ReduceGeometry)|t(?:AlternativeCount|C(?:ollationLocale|urrentSelections)|ExcludedCount|F(?:ieldSelections|olderPath)|NotSelectedCount|Object(?:Dimension|Field(?:(?:)?)|Measure)|PossibleCount|RegistryString|SelectedCount)))|H(?:SL|ash(?:1(?:28|60)|256))|I(?:RR|n(?:dex|terval(?:(?:\#)?))|s(?:Nu(?:ll|m)|PartialReload|Text)|terNo)|K(?:eepChar|urtosis)|L(?:INEST_(?:BLINEST_df|r2|s(?:e(?:[bmy])|sreg)|[fm])|Trim|astValue|e(?:ft|n|venshteinDist)|inest_ssresid|o(?:calTime|okUp|wer))|M(?:a(?:pSubstring|x(?:(?:String)?))|edian|i(?:nString|ssingCount|[dn])|o(?:de|ney(?:(?:\#)?)))|N(?:ORM(?:DIST|INV)|PV|oOf(?:(?:Field|Row|Table)s)|u(?:ll(?:(?:Count)?)|m(?:(?:\#|ericCount)?)))|O(?:SUser|nly|rd)|P(?:V|eek|mt|r(?:evious|oductVersion)|urgeChar)|Qvd(?:CreateTime|FieldName|NoOf(?:(?:Fiel|Recor)ds)|TableName)|R(?:GB|Trim|a(?:nge(?:Avg|Co(?:rrel|unt)|Fractile|IRR|Kurtosis|M(?:ax(?:(?:String)?)|i(?:n(?:(?:String)?)|ssingCount)|ode)|N(?:PV|u(?:(?:ll|meric)Count))|Only|S(?:kew|tdev|um)|TextCount|X(?:IRR|NPV))|te)|e(?:cNo|loadTime|p(?:eat|lace))|ight|owNo)|S(?:TEYX|kew|t(?:ateName|dev|err)|u(?:b(?:Field|StringCount)|m)|ysColor)|T(?:DIST|INV|ableN(?:ame|umber)|ext(?:(?:Between|Count)?)|ime(?:(?:\#|stamp(?:(?:\#)?))?)|rim)|U(?:TC|pper)|ValueL(?:ist|oop)|X(?:IRR|NPV)|a(?:ge|lt|utonumber(?:(?:hash(?:128|256))?))|b(?:itcount|l(?:ack|ue)|rown)|c(?:eil|lass|o(?:alesce|mbin|nverttolocaltime)|yan)|d(?:a(?:rkgray|y(?:(?:end|lightsaving|n(?:ame|umberof(?:(?:quarte|yea)r))|start)?))|iv)|e(?:(?:ven|xp)?)|f(?:a(?:bs|ct|lse)|irstworkdate|loor|mod|rac)|green|hour|i(?:f|n(?:day(?:(?:totime)?)|lunarweek(?:(?:todate)?)|month(?:(?:s(?:(?:todate)?)|todate)?)|quarter(?:(?:todate)?)|week(?:(?:todate)?)|year(?:(?:todate)?)))|l(?:astworkdate|ight(?:blue|cyan|gr(?:ay|een)|magenta|red)|og(?:(?:10)?)|unarweek(?:end|name|start))|m(?:a(?:genta|ke(?:(?:dat|tim|weekdat)e)|tch)|i(?:nute|xmatch)|o(?:d|nth(?:(?:end|name|s(?:end|name|(?:(?:s)?)tart))?)))|n(?:Per|etworkdays|ow)|odd|p(?:ermut|i(?:(?:ck)?)|ow)|quarter(?:end|name|start)|r(?:(?:an|e|oun)d)|s(?:e(?:cond|tdateyear(?:(?:month)?))|ign|qr(?:(?:t)?))|t(?:imezone|oday|rue|test(?:1(?:_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper))|_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)))|w(?:eek(?:(?:day|end|name|start|year)?)|hite|ildmatch)|ye(?:ar(?:(?:end|name|start|todate)?)|llow)|ztest(?:_(?:conf|dif|lower|s(?:ig|terr)|upper|z)|w_(?:conf|dif|lower|s(?:ig|terr)|upper|z)))(?=\s*\()", NAME_BUILTIN, NewState::Push(vec![r"function"])),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token_to(r#"(?im)""#, STRING_SYMBOL, NewState::Push(vec![r"field_name_quote"])),
        Rule::token_to(r"(?im)\[", STRING_SYMBOL, NewState::Push(vec![r"field_name_bracket"])),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)\b\d+\.\d+(e\d+)?[fd]?\b", NUMBER_FLOAT),
        Rule::token(r"(?im)\b\d+\b", NUMBER_INTEGER),
        Rule::token(r"(?im)(<(?:[<=>])|>(?:[=>])|[&*+\-/<=>])", OPERATOR),
        Rule::token(r"(?im)'.+?'", STRING),
        Rule::token(r"(?im)\b\w+\b", TEXT),
        Rule::token(r"(?im)[,;.()\\/]", PUNCTUATION),
    ]);
    m.insert(r"field_name_quote", vec![
        Rule::token_to(r#"(?im)""#, STRING_SYMBOL, NewState::Pop(1)),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r#"(?im)[^\"$]+"#, STRING_SYMBOL),
        Rule::token(r"(?im)\$", STRING_SYMBOL),
    ]);
    m.insert(r"field_name_bracket", vec![
        Rule::token_to(r"(?im)\]", STRING_SYMBOL, NewState::Pop(1)),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token(r"(?im)[^\]$]+", STRING_SYMBOL),
        Rule::token(r"(?im)\$", STRING_SYMBOL),
    ]);
    m.insert(r"function", vec![
        Rule::token_to(r"(?im)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?im)//.*\n", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?im)(let|set)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE)], NewState::Push(vec![r"assignment"])),
        Rule::token(r"(?im)\b(and|bit(?:and|not|(?:(?:x)?)or)|follows|like|not|or|precedes|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?im)(A(?:dd|lias|utoNumber)|B(?:inary|uffer)|C(?:USTOM|o(?:mment\ (?:field(?:(?:s)?)|table(?:(?:s)?))|n(?:catenate|nect))|rosstable)|D(?:e(?:(?:clar|riv)e)|i(?:rect(?:(?:\ Que|o)ry)|sconnect)|rop\ (?:field(?:(?:s)?)|table(?:(?:s)?)))|Execute|F(?:irst|lushLog|orce|rom)|Generic|Hierarchy(?:(?:BelongsTo)?)|In(?:ner|tervalMatch)|Join|Keep|L(?:IB|e(?:(?:(?:f)?)t)|o(?:ad|osen\ Table))|M(?:ap(?:(?:ping)?)|erge)|N(?:oConcatenate|ullAs(?:Null|Value))|O(?:DBC|LEBD|uter)|Partial\ reload|Qualify|R(?:e(?:m|name\ (?:field(?:(?:s)?)|table(?:(?:s)?))|place)|ight)|S(?:QL(?:(?:(?:Column|T(?:(?:abl|yp)e))s)?)|ample|e(?:arch|ction|lect|mantic|t)|leep|t(?:ar|ore))|T(?:ag|race)|Un(?:less|map|qualify|tag)|When|a(?:ccess|pplication|s(?:(?:c)?)|utogenerate)|ca(?:pitalization|se(?:(?:\ (?:lower|mixed|upper))?))|d(?:e(?:fault|sc)|i(?:mension|stinct)|o)|e(?:ach|lse|nd(?:(?:if)?)|x(?:clude|it|plicit|tension))|f(?:or|rom(?:(?:_field)?))|group\ by|i(?:mplicit|n(?:(?:clud|lin)e)|[fn])|loop|measure|next|order\ by|re(?:sident|turn)|s(?:cript|witch(?:(?:)?))|t(?:hen|otal)|u(?:n(?:less|til)|sing)|w(?:h(?:(?:er|il)e)|ith))\b", KEYWORD),
        Rule::token(r"(?im)[a-z]\w*:", KEYWORD_DECLARATION),
        Rule::token(r"(?im)(BrokenWeeks|C(?:ollationLocale|reateSearchIndexOnReload)|D(?:a(?:teFormat|yNames)|ecimalSep)|First(?:MonthOfYear|WeekDay)|Long(?:(?:Day|Month)Names)|Mon(?:ey(?:DecimalSep|Format|ThousandSep)|thNames)|Nu(?:ll(?:Display|Interpret|Value)|mericalAbbreviation)|O(?:penUrlTimeout|therSymbol)|QvWork(?:Path|Root)|ReferenceDay|StripComments|T(?:housandSep|ime(?:(?:(?:stamp)?)Format))|Verbatim|Win(?:Path|Root)|cd|errormode|floppy|hide(?:(?:pre|suf)fix)|include|must_include|null|qv(?:path|root)|scripterror(?:(?:(?:coun|lis)t)?))\b", KEYWORD_CONSTANT),
        Rule::token_to(r"(?im)(A(?:RGB|dd(?:(?:Month|Year)s)|pply(?:Codepage|Map)|ttribute|uthor|vg)|C(?:HI(?:DIST|INV)|apitalize|h(?:i2Test_(?:chi2|df|p)|r)|lientPlatform|o(?:lor(?:(?:Map(?:Hue|Jet)|mix(?:[12]))?)|mputerName|n(?:cat|nectString)|rrel|unt))|D(?:ate(?:(?:\#)?)|ocument(?:Name|Path|Title)|ual)|E(?:mptyIsNull|ngineVersion|valuate|xists)|F(?:DIST|INV|V|i(?:eld(?:Index|N(?:ame|umber)|Value(?:(?:Count)?))|le(?:BaseName|Dir|Extension|Name|Path|(?:Siz|Tim)e)|ndOneOf|rst(?:(?:(?:Sorted)?)Value))|ractile(?:(?:Exc)?))|G(?:MT|e(?:o(?:AggrGeometry|BoundingBox|CountVertex|Get(?:BoundingBox|PolygonCenter)|InvProjectGeometry|MakePoint|Project(?:(?:Geometry)?)|ReduceGeometry)|t(?:AlternativeCount|C(?:ollationLocale|urrentSelections)|ExcludedCount|F(?:ieldSelections|olderPath)|NotSelectedCount|Object(?:Dimension|Field(?:(?:)?)|Measure)|PossibleCount|RegistryString|SelectedCount)))|H(?:SL|ash(?:1(?:28|60)|256))|I(?:RR|n(?:dex|terval(?:(?:\#)?))|s(?:Nu(?:ll|m)|PartialReload|Text)|terNo)|K(?:eepChar|urtosis)|L(?:INEST_(?:BLINEST_df|r2|s(?:e(?:[bmy])|sreg)|[fm])|Trim|astValue|e(?:ft|n|venshteinDist)|inest_ssresid|o(?:calTime|okUp|wer))|M(?:a(?:pSubstring|x(?:(?:String)?))|edian|i(?:nString|ssingCount|[dn])|o(?:de|ney(?:(?:\#)?)))|N(?:ORM(?:DIST|INV)|PV|oOf(?:(?:Field|Row|Table)s)|u(?:ll(?:(?:Count)?)|m(?:(?:\#|ericCount)?)))|O(?:SUser|nly|rd)|P(?:V|eek|mt|r(?:evious|oductVersion)|urgeChar)|Qvd(?:CreateTime|FieldName|NoOf(?:(?:Fiel|Recor)ds)|TableName)|R(?:GB|Trim|a(?:nge(?:Avg|Co(?:rrel|unt)|Fractile|IRR|Kurtosis|M(?:ax(?:(?:String)?)|i(?:n(?:(?:String)?)|ssingCount)|ode)|N(?:PV|u(?:(?:ll|meric)Count))|Only|S(?:kew|tdev|um)|TextCount|X(?:IRR|NPV))|te)|e(?:cNo|loadTime|p(?:eat|lace))|ight|owNo)|S(?:TEYX|kew|t(?:ateName|dev|err)|u(?:b(?:Field|StringCount)|m)|ysColor)|T(?:DIST|INV|ableN(?:ame|umber)|ext(?:(?:Between|Count)?)|ime(?:(?:\#|stamp(?:(?:\#)?))?)|rim)|U(?:TC|pper)|ValueL(?:ist|oop)|X(?:IRR|NPV)|a(?:ge|lt|utonumber(?:(?:hash(?:128|256))?))|b(?:itcount|l(?:ack|ue)|rown)|c(?:eil|lass|o(?:alesce|mbin|nverttolocaltime)|yan)|d(?:a(?:rkgray|y(?:(?:end|lightsaving|n(?:ame|umberof(?:(?:quarte|yea)r))|start)?))|iv)|e(?:(?:ven|xp)?)|f(?:a(?:bs|ct|lse)|irstworkdate|loor|mod|rac)|green|hour|i(?:f|n(?:day(?:(?:totime)?)|lunarweek(?:(?:todate)?)|month(?:(?:s(?:(?:todate)?)|todate)?)|quarter(?:(?:todate)?)|week(?:(?:todate)?)|year(?:(?:todate)?)))|l(?:astworkdate|ight(?:blue|cyan|gr(?:ay|een)|magenta|red)|og(?:(?:10)?)|unarweek(?:end|name|start))|m(?:a(?:genta|ke(?:(?:dat|tim|weekdat)e)|tch)|i(?:nute|xmatch)|o(?:d|nth(?:(?:end|name|s(?:end|name|(?:(?:s)?)tart))?)))|n(?:Per|etworkdays|ow)|odd|p(?:ermut|i(?:(?:ck)?)|ow)|quarter(?:end|name|start)|r(?:(?:an|e|oun)d)|s(?:e(?:cond|tdateyear(?:(?:month)?))|ign|qr(?:(?:t)?))|t(?:imezone|oday|rue|test(?:1(?:_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper))|_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)|w_(?:conf|d(?:(?:(?:i)?)f)|lower|s(?:ig|terr)|t|upper)))|w(?:eek(?:(?:day|end|name|start|year)?)|hite|ildmatch)|ye(?:ar(?:(?:end|name|start|todate)?)|llow)|ztest(?:_(?:conf|dif|lower|s(?:ig|terr)|upper|z)|w_(?:conf|dif|lower|s(?:ig|terr)|upper|z)))(?=\s*\()", NAME_BUILTIN, NewState::Push(vec![r"function"])),
        Rule::bygroups(r"(?im)(\$\()(\w+)(\))", vec![Some(STRING_INTERPOL), Some(NAME_VARIABLE), Some(STRING_INTERPOL)]),
        Rule::token_to(r#"(?im)""#, STRING_SYMBOL, NewState::Push(vec![r"field_name_quote"])),
        Rule::token_to(r"(?im)\[", STRING_SYMBOL, NewState::Push(vec![r"field_name_bracket"])),
        Rule::token_to(r"(?im)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?im)\b\d+\.\d+(e\d+)?[fd]?\b", NUMBER_FLOAT),
        Rule::token(r"(?im)\b\d+\b", NUMBER_INTEGER),
        Rule::token(r"(?im)(<(?:[<=>])|>(?:[=>])|[&*+\-/<=>])", OPERATOR),
        Rule::token(r"(?im)'.+?'", STRING),
        Rule::token(r"(?im)\b\w+\b", TEXT),
        Rule::token(r"(?im)[,;.()\\/]", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for QlikLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
