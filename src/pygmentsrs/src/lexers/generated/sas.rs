//! AUTO-GENERATED from `pygments.pygments.lexers.sas:SASLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sas:SASLexer:sas

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: sas
pub struct SasLexer;

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
        Rule::token(r"(?im)^\s*\*.*?;", COMMENT),
        Rule::token(r"(?im)/\*.*?\*/", COMMENT),
        Rule::token(r"(?im)^\s*\*(.|\n)*?;", COMMENT_MULTILINE),
        Rule::token(r"(?im)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token(r"(?im)(^|;)\s*(proc \w+|data|run|quit)[\s;]", KEYWORD_RESERVED),
        Rule::token_to(r"(?im)^\s*(datalines|cards)\s*;\s*$", KEYWORD, NewState::Push(vec![r"data"])),
        Rule::token_to(r"(?im)\n?^\s*%?put ", KEYWORD, NewState::Push(vec![r"log-messages"])),
        Rule::token(r"(?im)\b(a(?:bort|rray|ttrib)|by|c(?:a(?:ll|rds(?:(?:4)?)|tname)|ontinue)|d(?:atalines(?:(?:4)?)|el(?:ete|im(?:(?:iter)?))|isplay|m|rop)|e(?:ndsas|rror)|f(?:ile(?:(?:name)?)|o(?:otnote|rmat))|goto|in(?:(?:f(?:ile|ormat)|put)?)|keep|l(?:abel|e(?:ave|ngth)|i(?:bname|nk|st)|ostcard)|m(?:erge|issing|odify)|o(?:ptions|ut(?:(?:put)?))|p(?:age|ut)|re(?:direct|move|name|place|t(?:(?:ai|ur)n))|s(?:e(?:(?:(?:lec)?)t)|kip|t(?:artsas|op)|ystask)|title|update|w(?:aitsas|here|indow)|x)\b", KEYWORD),
        Rule::token(r"(?im)\b(a(?:dd|lter|nd|s)|c(?:ascade|heck|reate)|d(?:e(?:(?:let|scrib)e)|istinct|rop)|f(?:oreign|rom)|group|having|in(?:(?:dex|sert|to)?)|key|like|m(?:essage|odify|sgtype)|n(?:ot|ull)|o(?:rder|[nr])|primary|re(?:ferences|s(?:(?:e|tric)t))|se(?:(?:(?:lec)?)t)|table|u(?:(?:niqu|pdat)e)|v(?:alidate|iew)|where)\b", KEYWORD),
        Rule::token(r"(?im)\b(do|e(?:lse|nd)|if|then|until|while)\b", KEYWORD),
        Rule::token(r"(?im)%(bquote|c(?:mpres|ompstor)|d(?:atatyp|isplay|o)|e(?:lse|nd|val)|g(?:lobal|oto)|i(?:f|n(?:dex|put))|keydef|l(?:abel|e(?:ft|ngth|t)|o(?:cal|wcase))|m(?:acro|end)|nr(?:bquote|quote|str)|put|q(?:cmpres|l(?:eft|owcase)|s(?:can|ubstr|ysfunc)|trim|u(?:(?:ot|pcas)e))|s(?:can|tr|u(?:bstr|perq)|ys(?:call|e(?:valf|xec)|func|get|lput|prod|r(?:c|put)))|t(?:hen|o|rim)|u(?:n(?:quote|til)|pcase)|verify|w(?:hile|indow))\b", NAME_BUILTIN),
        Rule::token(r"(?im)\b(a(?:bs|ddr|iry|r(?:cos|sin)|t(?:an|tr(?:[cn])))|b(?:and|etainv|lshift|not|or|rshift|xor|yte)|c(?:df|e(?:il|xist)|inv|lose|nonct|o(?:llate|mp(?:bl|ound|ress)|s(?:(?:h)?))|ss|urobs|v)|d(?:a(?:cc(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|iry|te(?:(?:jul|part|time)?)|y)|close|e(?:p(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|quote)|hms|i(?:gamma|nfo|[fm])|num|op(?:en|tn(?:ame|um))|r(?:ead|opnote)|sname)|e(?:rf(?:(?:c)?)|x(?:ist|p))|f(?:append|c(?:lose|ol)|delete|e(?:tch(?:(?:obs)?)|xist)|get|i(?:le(?:exist|name|ref)|n(?:fo|v)|p(?:name(?:(?:l)?)|state))|loor|no(?:nct|te)|op(?:en|tn(?:ame|um))|p(?:o(?:int|s)|ut)|r(?:e(?:(?:a|win)d)|len)|sep|uzz|write)|g(?:am(?:inv|ma)|et(?:option|var(?:[cn])))|h(?:bound|ms|o(?:sthelp|ur))|i(?:bessel|n(?:dex(?:(?:[cw])?)|put(?:(?:[cn])?)|t(?:(?:ck|nx|rr)?))|rr)|j(?:bessel|uldate)|kurtosis|l(?:ag|bound|e(?:ft|ngth)|gamma|ib(?:name|ref)|o(?:g(?:(?:10|2|(?:p(?:[dm])|sd)f)?)|wcase))|m(?:ax|dy|ean|in(?:(?:ute)?)|o(?:d|nth|pen|rt))|n(?:(?:etpv|miss|o(?:rmal|te)|pv)?)|o(?:pen|rdinal)|p(?:athname|df|eek(?:(?:c)?)|mf|o(?:i(?:nt|sson)|ke)|rob(?:b(?:eta|nml)|chi|gam|hypr|it|n(?:egb|orm)|[ft])|ut(?:(?:[cn])?))|q(?:tr|uote)|r(?:an(?:bin|cau|exp|g(?:am|e)|k|nor|poi|t(?:bl|ri)|uni)|e(?:peat|solve|verse|wind)|ight|ound)|s(?:aving|can|df|econd|i(?:gn|n(?:(?:h)?))|kewness|oundex|pedis|qrt|t(?:d(?:(?:err)?)|fips|name(?:(?:l)?))|u(?:bstr|m)|y(?:mget|s(?:get|msg|prod|rc|tem)))|t(?:an(?:(?:h)?)|i(?:me(?:(?:part)?)|nv)|nonct|oday|r(?:an(?:slate|wrd)|i(?:gamma|m(?:(?:n)?))|unc))|u(?:niform|pcase|ss)|v(?:ar(?:(?:fmt|infmt|l(?:abel|en)|n(?:ame|um)|ray(?:(?:x)?)|type)?)|erify|format(?:(?:(?:[dnw])x|[dnwx])?)|in(?:array(?:(?:x)?)|format(?:(?:(?:[dnw])x|[dnwx])?))|l(?:abel(?:(?:x)?)|ength(?:(?:x)?))|name(?:(?:x)?)|type(?:(?:x)?))|weekday|y(?:ear|yq)|zip(?:fips|name(?:(?:l)?)|state))\(", NAME_BUILTIN),
        Rule::token(r"(?im)&[a-z_]\w*\.?", NAME_VARIABLE),
        Rule::token(r"(?im)%[a-z_]\w*", NAME_FUNCTION),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string_dquote"])),
        Rule::token(r"(?im)(null|missing|_all_|_automatic_|_character_|_n_|_infile_|_name_|_null_|_numeric_|_user_|_webout_)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)\b[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)(E[+-]?[0-9]+)?i?\b", NUMBER),
        Rule::token(r"(?im).", TEXT),
    ]);
    m.insert(r"comments", vec![
        Rule::token(r"(?im)^\s*\*.*?;", COMMENT),
        Rule::token(r"(?im)/\*.*?\*/", COMMENT),
        Rule::token(r"(?im)^\s*\*(.|\n)*?;", COMMENT_MULTILINE),
        Rule::token(r"(?im)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
    ]);
    m.insert(r"proc-data", vec![
        Rule::token(r"(?im)(^|;)\s*(proc \w+|data|run|quit)[\s;]", KEYWORD_RESERVED),
    ]);
    m.insert(r"cards-datalines", vec![
        Rule::token_to(r"(?im)^\s*(datalines|cards)\s*;\s*$", KEYWORD, NewState::Push(vec![r"data"])),
    ]);
    m.insert(r"logs", vec![
        Rule::token_to(r"(?im)\n?^\s*%?put ", KEYWORD, NewState::Push(vec![r"log-messages"])),
    ]);
    m.insert(r"general", vec![
        Rule::token(r"(?im)\b(a(?:bort|rray|ttrib)|by|c(?:a(?:ll|rds(?:(?:4)?)|tname)|ontinue)|d(?:atalines(?:(?:4)?)|el(?:ete|im(?:(?:iter)?))|isplay|m|rop)|e(?:ndsas|rror)|f(?:ile(?:(?:name)?)|o(?:otnote|rmat))|goto|in(?:(?:f(?:ile|ormat)|put)?)|keep|l(?:abel|e(?:ave|ngth)|i(?:bname|nk|st)|ostcard)|m(?:erge|issing|odify)|o(?:ptions|ut(?:(?:put)?))|p(?:age|ut)|re(?:direct|move|name|place|t(?:(?:ai|ur)n))|s(?:e(?:(?:(?:lec)?)t)|kip|t(?:artsas|op)|ystask)|title|update|w(?:aitsas|here|indow)|x)\b", KEYWORD),
        Rule::token(r"(?im)\b(a(?:dd|lter|nd|s)|c(?:ascade|heck|reate)|d(?:e(?:(?:let|scrib)e)|istinct|rop)|f(?:oreign|rom)|group|having|in(?:(?:dex|sert|to)?)|key|like|m(?:essage|odify|sgtype)|n(?:ot|ull)|o(?:rder|[nr])|primary|re(?:ferences|s(?:(?:e|tric)t))|se(?:(?:(?:lec)?)t)|table|u(?:(?:niqu|pdat)e)|v(?:alidate|iew)|where)\b", KEYWORD),
        Rule::token(r"(?im)\b(do|e(?:lse|nd)|if|then|until|while)\b", KEYWORD),
        Rule::token(r"(?im)%(bquote|c(?:mpres|ompstor)|d(?:atatyp|isplay|o)|e(?:lse|nd|val)|g(?:lobal|oto)|i(?:f|n(?:dex|put))|keydef|l(?:abel|e(?:ft|ngth|t)|o(?:cal|wcase))|m(?:acro|end)|nr(?:bquote|quote|str)|put|q(?:cmpres|l(?:eft|owcase)|s(?:can|ubstr|ysfunc)|trim|u(?:(?:ot|pcas)e))|s(?:can|tr|u(?:bstr|perq)|ys(?:call|e(?:valf|xec)|func|get|lput|prod|r(?:c|put)))|t(?:hen|o|rim)|u(?:n(?:quote|til)|pcase)|verify|w(?:hile|indow))\b", NAME_BUILTIN),
        Rule::token(r"(?im)\b(a(?:bs|ddr|iry|r(?:cos|sin)|t(?:an|tr(?:[cn])))|b(?:and|etainv|lshift|not|or|rshift|xor|yte)|c(?:df|e(?:il|xist)|inv|lose|nonct|o(?:llate|mp(?:bl|ound|ress)|s(?:(?:h)?))|ss|urobs|v)|d(?:a(?:cc(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|iry|te(?:(?:jul|part|time)?)|y)|close|e(?:p(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|quote)|hms|i(?:gamma|nfo|[fm])|num|op(?:en|tn(?:ame|um))|r(?:ead|opnote)|sname)|e(?:rf(?:(?:c)?)|x(?:ist|p))|f(?:append|c(?:lose|ol)|delete|e(?:tch(?:(?:obs)?)|xist)|get|i(?:le(?:exist|name|ref)|n(?:fo|v)|p(?:name(?:(?:l)?)|state))|loor|no(?:nct|te)|op(?:en|tn(?:ame|um))|p(?:o(?:int|s)|ut)|r(?:e(?:(?:a|win)d)|len)|sep|uzz|write)|g(?:am(?:inv|ma)|et(?:option|var(?:[cn])))|h(?:bound|ms|o(?:sthelp|ur))|i(?:bessel|n(?:dex(?:(?:[cw])?)|put(?:(?:[cn])?)|t(?:(?:ck|nx|rr)?))|rr)|j(?:bessel|uldate)|kurtosis|l(?:ag|bound|e(?:ft|ngth)|gamma|ib(?:name|ref)|o(?:g(?:(?:10|2|(?:p(?:[dm])|sd)f)?)|wcase))|m(?:ax|dy|ean|in(?:(?:ute)?)|o(?:d|nth|pen|rt))|n(?:(?:etpv|miss|o(?:rmal|te)|pv)?)|o(?:pen|rdinal)|p(?:athname|df|eek(?:(?:c)?)|mf|o(?:i(?:nt|sson)|ke)|rob(?:b(?:eta|nml)|chi|gam|hypr|it|n(?:egb|orm)|[ft])|ut(?:(?:[cn])?))|q(?:tr|uote)|r(?:an(?:bin|cau|exp|g(?:am|e)|k|nor|poi|t(?:bl|ri)|uni)|e(?:peat|solve|verse|wind)|ight|ound)|s(?:aving|can|df|econd|i(?:gn|n(?:(?:h)?))|kewness|oundex|pedis|qrt|t(?:d(?:(?:err)?)|fips|name(?:(?:l)?))|u(?:bstr|m)|y(?:mget|s(?:get|msg|prod|rc|tem)))|t(?:an(?:(?:h)?)|i(?:me(?:(?:part)?)|nv)|nonct|oday|r(?:an(?:slate|wrd)|i(?:gamma|m(?:(?:n)?))|unc))|u(?:niform|pcase|ss)|v(?:ar(?:(?:fmt|infmt|l(?:abel|en)|n(?:ame|um)|ray(?:(?:x)?)|type)?)|erify|format(?:(?:(?:[dnw])x|[dnwx])?)|in(?:array(?:(?:x)?)|format(?:(?:(?:[dnw])x|[dnwx])?))|l(?:abel(?:(?:x)?)|ength(?:(?:x)?))|name(?:(?:x)?)|type(?:(?:x)?))|weekday|y(?:ear|yq)|zip(?:fips|name(?:(?:l)?)|state))\(", NAME_BUILTIN),
        Rule::token(r"(?im)&[a-z_]\w*\.?", NAME_VARIABLE),
        Rule::token(r"(?im)%[a-z_]\w*", NAME_FUNCTION),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string_dquote"])),
        Rule::token(r"(?im)(null|missing|_all_|_automatic_|_character_|_n_|_infile_|_name_|_null_|_numeric_|_user_|_webout_)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)\b[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)(E[+-]?[0-9]+)?i?\b", NUMBER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?im)\b(a(?:bort|rray|ttrib)|by|c(?:a(?:ll|rds(?:(?:4)?)|tname)|ontinue)|d(?:atalines(?:(?:4)?)|el(?:ete|im(?:(?:iter)?))|isplay|m|rop)|e(?:ndsas|rror)|f(?:ile(?:(?:name)?)|o(?:otnote|rmat))|goto|in(?:(?:f(?:ile|ormat)|put)?)|keep|l(?:abel|e(?:ave|ngth)|i(?:bname|nk|st)|ostcard)|m(?:erge|issing|odify)|o(?:ptions|ut(?:(?:put)?))|p(?:age|ut)|re(?:direct|move|name|place|t(?:(?:ai|ur)n))|s(?:e(?:(?:(?:lec)?)t)|kip|t(?:artsas|op)|ystask)|title|update|w(?:aitsas|here|indow)|x)\b", KEYWORD),
        Rule::token(r"(?im)\b(a(?:dd|lter|nd|s)|c(?:ascade|heck|reate)|d(?:e(?:(?:let|scrib)e)|istinct|rop)|f(?:oreign|rom)|group|having|in(?:(?:dex|sert|to)?)|key|like|m(?:essage|odify|sgtype)|n(?:ot|ull)|o(?:rder|[nr])|primary|re(?:ferences|s(?:(?:e|tric)t))|se(?:(?:(?:lec)?)t)|table|u(?:(?:niqu|pdat)e)|v(?:alidate|iew)|where)\b", KEYWORD),
        Rule::token(r"(?im)\b(do|e(?:lse|nd)|if|then|until|while)\b", KEYWORD),
        Rule::token(r"(?im)%(bquote|c(?:mpres|ompstor)|d(?:atatyp|isplay|o)|e(?:lse|nd|val)|g(?:lobal|oto)|i(?:f|n(?:dex|put))|keydef|l(?:abel|e(?:ft|ngth|t)|o(?:cal|wcase))|m(?:acro|end)|nr(?:bquote|quote|str)|put|q(?:cmpres|l(?:eft|owcase)|s(?:can|ubstr|ysfunc)|trim|u(?:(?:ot|pcas)e))|s(?:can|tr|u(?:bstr|perq)|ys(?:call|e(?:valf|xec)|func|get|lput|prod|r(?:c|put)))|t(?:hen|o|rim)|u(?:n(?:quote|til)|pcase)|verify|w(?:hile|indow))\b", NAME_BUILTIN),
        Rule::token(r"(?im)\b(a(?:bs|ddr|iry|r(?:cos|sin)|t(?:an|tr(?:[cn])))|b(?:and|etainv|lshift|not|or|rshift|xor|yte)|c(?:df|e(?:il|xist)|inv|lose|nonct|o(?:llate|mp(?:bl|ound|ress)|s(?:(?:h)?))|ss|urobs|v)|d(?:a(?:cc(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|iry|te(?:(?:jul|part|time)?)|y)|close|e(?:p(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|quote)|hms|i(?:gamma|nfo|[fm])|num|op(?:en|tn(?:ame|um))|r(?:ead|opnote)|sname)|e(?:rf(?:(?:c)?)|x(?:ist|p))|f(?:append|c(?:lose|ol)|delete|e(?:tch(?:(?:obs)?)|xist)|get|i(?:le(?:exist|name|ref)|n(?:fo|v)|p(?:name(?:(?:l)?)|state))|loor|no(?:nct|te)|op(?:en|tn(?:ame|um))|p(?:o(?:int|s)|ut)|r(?:e(?:(?:a|win)d)|len)|sep|uzz|write)|g(?:am(?:inv|ma)|et(?:option|var(?:[cn])))|h(?:bound|ms|o(?:sthelp|ur))|i(?:bessel|n(?:dex(?:(?:[cw])?)|put(?:(?:[cn])?)|t(?:(?:ck|nx|rr)?))|rr)|j(?:bessel|uldate)|kurtosis|l(?:ag|bound|e(?:ft|ngth)|gamma|ib(?:name|ref)|o(?:g(?:(?:10|2|(?:p(?:[dm])|sd)f)?)|wcase))|m(?:ax|dy|ean|in(?:(?:ute)?)|o(?:d|nth|pen|rt))|n(?:(?:etpv|miss|o(?:rmal|te)|pv)?)|o(?:pen|rdinal)|p(?:athname|df|eek(?:(?:c)?)|mf|o(?:i(?:nt|sson)|ke)|rob(?:b(?:eta|nml)|chi|gam|hypr|it|n(?:egb|orm)|[ft])|ut(?:(?:[cn])?))|q(?:tr|uote)|r(?:an(?:bin|cau|exp|g(?:am|e)|k|nor|poi|t(?:bl|ri)|uni)|e(?:peat|solve|verse|wind)|ight|ound)|s(?:aving|can|df|econd|i(?:gn|n(?:(?:h)?))|kewness|oundex|pedis|qrt|t(?:d(?:(?:err)?)|fips|name(?:(?:l)?))|u(?:bstr|m)|y(?:mget|s(?:get|msg|prod|rc|tem)))|t(?:an(?:(?:h)?)|i(?:me(?:(?:part)?)|nv)|nonct|oday|r(?:an(?:slate|wrd)|i(?:gamma|m(?:(?:n)?))|unc))|u(?:niform|pcase|ss)|v(?:ar(?:(?:fmt|infmt|l(?:abel|en)|n(?:ame|um)|ray(?:(?:x)?)|type)?)|erify|format(?:(?:(?:[dnw])x|[dnwx])?)|in(?:array(?:(?:x)?)|format(?:(?:(?:[dnw])x|[dnwx])?))|l(?:abel(?:(?:x)?)|ength(?:(?:x)?))|name(?:(?:x)?)|type(?:(?:x)?))|weekday|y(?:ear|yq)|zip(?:fips|name(?:(?:l)?)|state))\(", NAME_BUILTIN),
    ]);
    m.insert(r"vars-strings", vec![
        Rule::token(r"(?im)&[a-z_]\w*\.?", NAME_VARIABLE),
        Rule::token(r"(?im)%[a-z_]\w*", NAME_FUNCTION),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string_dquote"])),
    ]);
    m.insert(r"special", vec![
        Rule::token(r"(?im)(null|missing|_all_|_automatic_|_character_|_n_|_infile_|_name_|_null_|_numeric_|_user_|_webout_)", KEYWORD_CONSTANT),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?im)\b[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)(E[+-]?[0-9]+)?i?\b", NUMBER),
    ]);
    m.insert(r"data", vec![
        Rule::token_to(r"(?im)(.|\n)*^\s*;\s*$", OTHER, NewState::Pop(1)),
    ]);
    m.insert(r"log-messages", vec![
        Rule::token_to(r"(?im)NOTE(:|-).*", GENERIC, NewState::Pop(1)),
        Rule::token_to(r"(?im)WARNING(:|-).*", GENERIC_EMPH, NewState::Pop(1)),
        Rule::token_to(r"(?im)ERROR(:|-).*", GENERIC_ERROR, NewState::Pop(1)),
        Rule::token(r"(?im)\b(a(?:bort|rray|ttrib)|by|c(?:a(?:ll|rds(?:(?:4)?)|tname)|ontinue)|d(?:atalines(?:(?:4)?)|el(?:ete|im(?:(?:iter)?))|isplay|m|rop)|e(?:ndsas|rror)|f(?:ile(?:(?:name)?)|o(?:otnote|rmat))|goto|in(?:(?:f(?:ile|ormat)|put)?)|keep|l(?:abel|e(?:ave|ngth)|i(?:bname|nk|st)|ostcard)|m(?:erge|issing|odify)|o(?:ptions|ut(?:(?:put)?))|p(?:age|ut)|re(?:direct|move|name|place|t(?:(?:ai|ur)n))|s(?:e(?:(?:(?:lec)?)t)|kip|t(?:artsas|op)|ystask)|title|update|w(?:aitsas|here|indow)|x)\b", KEYWORD),
        Rule::token(r"(?im)\b(a(?:dd|lter|nd|s)|c(?:ascade|heck|reate)|d(?:e(?:(?:let|scrib)e)|istinct|rop)|f(?:oreign|rom)|group|having|in(?:(?:dex|sert|to)?)|key|like|m(?:essage|odify|sgtype)|n(?:ot|ull)|o(?:rder|[nr])|primary|re(?:ferences|s(?:(?:e|tric)t))|se(?:(?:(?:lec)?)t)|table|u(?:(?:niqu|pdat)e)|v(?:alidate|iew)|where)\b", KEYWORD),
        Rule::token(r"(?im)\b(do|e(?:lse|nd)|if|then|until|while)\b", KEYWORD),
        Rule::token(r"(?im)%(bquote|c(?:mpres|ompstor)|d(?:atatyp|isplay|o)|e(?:lse|nd|val)|g(?:lobal|oto)|i(?:f|n(?:dex|put))|keydef|l(?:abel|e(?:ft|ngth|t)|o(?:cal|wcase))|m(?:acro|end)|nr(?:bquote|quote|str)|put|q(?:cmpres|l(?:eft|owcase)|s(?:can|ubstr|ysfunc)|trim|u(?:(?:ot|pcas)e))|s(?:can|tr|u(?:bstr|perq)|ys(?:call|e(?:valf|xec)|func|get|lput|prod|r(?:c|put)))|t(?:hen|o|rim)|u(?:n(?:quote|til)|pcase)|verify|w(?:hile|indow))\b", NAME_BUILTIN),
        Rule::token(r"(?im)\b(a(?:bs|ddr|iry|r(?:cos|sin)|t(?:an|tr(?:[cn])))|b(?:and|etainv|lshift|not|or|rshift|xor|yte)|c(?:df|e(?:il|xist)|inv|lose|nonct|o(?:llate|mp(?:bl|ound|ress)|s(?:(?:h)?))|ss|urobs|v)|d(?:a(?:cc(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|iry|te(?:(?:jul|part|time)?)|y)|close|e(?:p(?:db(?:(?:sl)?)|s(?:l|yd)|tab)|quote)|hms|i(?:gamma|nfo|[fm])|num|op(?:en|tn(?:ame|um))|r(?:ead|opnote)|sname)|e(?:rf(?:(?:c)?)|x(?:ist|p))|f(?:append|c(?:lose|ol)|delete|e(?:tch(?:(?:obs)?)|xist)|get|i(?:le(?:exist|name|ref)|n(?:fo|v)|p(?:name(?:(?:l)?)|state))|loor|no(?:nct|te)|op(?:en|tn(?:ame|um))|p(?:o(?:int|s)|ut)|r(?:e(?:(?:a|win)d)|len)|sep|uzz|write)|g(?:am(?:inv|ma)|et(?:option|var(?:[cn])))|h(?:bound|ms|o(?:sthelp|ur))|i(?:bessel|n(?:dex(?:(?:[cw])?)|put(?:(?:[cn])?)|t(?:(?:ck|nx|rr)?))|rr)|j(?:bessel|uldate)|kurtosis|l(?:ag|bound|e(?:ft|ngth)|gamma|ib(?:name|ref)|o(?:g(?:(?:10|2|(?:p(?:[dm])|sd)f)?)|wcase))|m(?:ax|dy|ean|in(?:(?:ute)?)|o(?:d|nth|pen|rt))|n(?:(?:etpv|miss|o(?:rmal|te)|pv)?)|o(?:pen|rdinal)|p(?:athname|df|eek(?:(?:c)?)|mf|o(?:i(?:nt|sson)|ke)|rob(?:b(?:eta|nml)|chi|gam|hypr|it|n(?:egb|orm)|[ft])|ut(?:(?:[cn])?))|q(?:tr|uote)|r(?:an(?:bin|cau|exp|g(?:am|e)|k|nor|poi|t(?:bl|ri)|uni)|e(?:peat|solve|verse|wind)|ight|ound)|s(?:aving|can|df|econd|i(?:gn|n(?:(?:h)?))|kewness|oundex|pedis|qrt|t(?:d(?:(?:err)?)|fips|name(?:(?:l)?))|u(?:bstr|m)|y(?:mget|s(?:get|msg|prod|rc|tem)))|t(?:an(?:(?:h)?)|i(?:me(?:(?:part)?)|nv)|nonct|oday|r(?:an(?:slate|wrd)|i(?:gamma|m(?:(?:n)?))|unc))|u(?:niform|pcase|ss)|v(?:ar(?:(?:fmt|infmt|l(?:abel|en)|n(?:ame|um)|ray(?:(?:x)?)|type)?)|erify|format(?:(?:(?:[dnw])x|[dnwx])?)|in(?:array(?:(?:x)?)|format(?:(?:(?:[dnw])x|[dnwx])?))|l(?:abel(?:(?:x)?)|ength(?:(?:x)?))|name(?:(?:x)?)|type(?:(?:x)?))|weekday|y(?:ear|yq)|zip(?:fips|name(?:(?:l)?)|state))\(", NAME_BUILTIN),
        Rule::token(r"(?im)&[a-z_]\w*\.?", NAME_VARIABLE),
        Rule::token(r"(?im)%[a-z_]\w*", NAME_FUNCTION),
        Rule::token_to(r"(?im)\'", STRING, NewState::Push(vec![r"string_squote"])),
        Rule::token_to(r#"(?im)""#, STRING, NewState::Push(vec![r"string_dquote"])),
        Rule::token(r"(?im)(null|missing|_all_|_automatic_|_character_|_n_|_infile_|_name_|_null_|_numeric_|_user_|_webout_)", KEYWORD_CONSTANT),
        Rule::token(r"(?im)\b[+-]?([0-9]+(\.[0-9]+)?|\.[0-9]+|\.)(E[+-]?[0-9]+)?i?\b", NUMBER),
    ]);
    m.insert(r"string_squote", vec![
        Rule::token_to(r"(?im)'", STRING, NewState::Pop(1)),
        Rule::token(r#"(?im)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?im)[^$\'\\]+", STRING),
        Rule::token(r"(?im)[$\'\\]", STRING),
    ]);
    m.insert(r"string_dquote", vec![
        Rule::token_to(r#"(?im)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?im)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token_to(r"(?im)&", NAME_VARIABLE, NewState::Push(vec![r"validvar"])),
        Rule::token(r#"(?im)[^$&"\\]+"#, STRING),
        Rule::token(r#"(?im)[$"\\]"#, STRING),
    ]);
    m.insert(r"validvar", vec![
        Rule::token_to(r"(?im)[a-z_]\w*\.?", NAME_VARIABLE, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for SasLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
