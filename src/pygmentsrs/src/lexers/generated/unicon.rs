//! AUTO-GENERATED from `pygments.pygments.lexers.unicon:UniconLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.unicon:UniconLexer:unicon

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: unicon
pub struct UniconLexer;

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
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token_to(r"(?m)class|method|procedure", KEYWORD_DECLARATION, NewState::Push(vec![r"subprogram"])),
        Rule::bygroups_to(r"(?m)(record)(\s+)(\w+)", vec![Some(KEYWORD_DECLARATION), Some(TEXT), Some(KEYWORD_TYPE)], NewState::Push(vec![r"type_def"])),
        Rule::token(r"(?m)(#line|\$C|\$Cend|\$define|\$else|\$endif|\$error|\$ifdef|\$ifndef|\$include|\$line|\$undef)\b", TokenType::new(&["Keyword", "PreProc"])),
        Rule::token(r"(?m)(&null|&fail)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)&allocated|&ascii|&clock|&collections|&column|&col|&control|&cset|&current|&dateline|&date|&digits|&dump|&errno|&errornumber|&errortext|&errorvalue|&error|&errout|&eventcode|&eventvalue|&eventsource|&e|&features|&file|&host|&input|&interval|&lcase|&letters|&level|&line|&ldrag|&lpress|&lrelease|&main|&mdrag|&meta|&mpress|&mrelease|&now|&output|&phi|&pick|&pi|&pos|&progname|&random|&rdrag|&regions|&resize|&row|&rpress|&rrelease|&shift|&source|&storage|&subject|&time|&trace|&ucase|&version|&window|&x|&y", KEYWORD_RESERVED),
        Rule::token(r"(?m)(by|of|not|to)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(global|local|static|abstract)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)package|link|import", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(all|break|c(?:ase|r(?:eate|itical))|d(?:efault|o)|e(?:lse|nd|very)|fail|i(?:f|mport|n(?:itial(?:(?:ly)?)|vocable))|next|re(?:peat|turn)|suspend|th(?:en|read)|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(A(?:bort|ctive|lert|ny|rb(?:(?:no)?)|ttrib)|B(?:al|g|reak(?:(?:x)?))|C(?:l(?:ip|one)|o(?:lor(?:(?:Value)?)|pyArea|uple))|Draw(?:Arc|C(?:ircle|u(?:(?:b|rv)e)|ylinder)|Disk|Image|Line|Po(?:int|lygon)|Rectangle|S(?:egment|phere|tring)|Torus)|E(?:raseArea|v(?:Get|Send|ent)|ye)|F(?:ail|ence|g|ill(?:Arc|Circle|Polygon|Rectangle)|ont|ree(?:Color|Space))|G(?:etSpace|oto(?:RC|XY))|I(?:dentityMatrix|n(?:Port|t86))|L(?:en|ower)|M(?:atrixMode|ultMatrix)|N(?:ewColor|o(?:rmals|tAny))|OutPort|P(?:a(?:lette(?:C(?:hars|olor)|Key)|ttern)|e(?:ek|nding)|ixel|layAudio|o(?:ke|pMatrix|s)|ush(?:Matrix|(?:Rotat|Scal|Translat)e))|QueryPointer|R(?:aise|e(?:adImage|fresh|m)|otate|pos|tab)|S(?:cale|pan|topAudio|ucceed|wi)|T(?:ab|ex(?:coord|t(?:Width|ure))|ranslate)|Uncouple|VAttrib|W(?:Attrib|Default|Flush|S(?:ection|ync)|in(?:Associate|Button|ColorDialog|EditRegion|FontDialog|MenuBar|OpenDialog|PlayMedia|S(?:aveDialog|crollBar|electDialog)|dowContents)|riteImage)|a(?:bs|cos|ny|r(?:gs|ray)|sin|tan(?:(?:h)?))|bal|c(?:allout|enter|h(?:ar|dir|mod|own|root)|l(?:(?:assnam|os)e)|o(?:fail|llect|n(?:(?:dva|structo)r)|py|s)|rypt|set|time)|d(?:b(?:columns|driver|keys|limits|product|tables)|e(?:l(?:ay|ete)|tab)|isplay|tor)|e(?:ntab|rrorclear|ventmask|x(?:ec|it|p))|f(?:cntl|dup|etch|i(?:eldnames|lepair|nd)|l(?:ock|ush)|ork|unction)|g(?:et(?:(?:ch(?:(?:e)?)|e(?:gid|nv|uid)|g(?:id|r)|host|p(?:grp|id|pid|w)|rusage|serv|timeofday|uid)?)|lobalnames|time)|hardlink|i(?:and|com|mage|n(?:sert|teger)|o(?:ctl|r)|s(?:hift|tate)|xor)|k(?:bhit|ey(?:(?:word)?)|ill)|l(?:eft|ist|o(?:ad(?:(?:func)?)|c(?:alnames|k)|g)|stat)|m(?:a(?:ny|tch|[px])|e(?:mber(?:(?:names)?)|thod(?:(?:(?:name)?)s))|in|kdir|ove|utex)|n(?:ame|umeric)|o(?:p(?:en(?:(?:cl)?)|rec)|rd)|p(?:ar(?:amnames|ent)|ipe|o(?:[ps])|roc|u(?:ll|sh|t))|r(?:e(?:a(?:d(?:link|[sy])|[dl])|ceive|move|name|pl|verse)|ight|mdir|tod|unerr)|s(?:ave|e(?:ek|lect|nd|rial|t(?:env|g(?:id|rent)|hostent|p(?:grp|went)|servent|uid)|[qt])|i(?:gnal|n)|ort(?:(?:f)?)|pawn|q(?:l|rt)|t(?:at(?:(?:icnames)?)|op|r(?:ing|ucture))|y(?:mlink|s(?:_errstr|tem|write)))|t(?:a(?:ble|[bn])|r(?:ap|im|uncate|ylock)|ype)|u(?:mask|nlock|pto|time)|variable|w(?:ait|here|rite(?:(?:s)?)))\b", NAME_FUNCTION),
        Rule::token(r"(?m)\b([+-]?([2-9]|[12][0-9]|3[0-6])[rR][0-9a-zA-Z]+)\b", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]*\.([0-9]*)([Ee][+-]?[0-9]*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b([+-]?[0-9]+[KMGTPkmgtp]?)\b", NUMBER_INTEGER),
        Rule::token(r"(?m)<@|<<@|>@|>>@|\.>|->|===|~===|\*\*|\+\+|--|\.|~==|~=|<=|>=|==|=|<<=|<<|>>=|>>|:=:|:=|->|<->|\+:=|\|", OPERATOR),
        Rule::token(r#"(?m)"(?:[^\\"]|\\.)*""#, STRING),
        Rule::token(r"(?m)'(?:[^\\']|\\.)*'", TokenType::new(&["Literal", "String", "Character"])),
        Rule::token(r"(?m)[*<>+=/&!?@~\\-]", OPERATOR),
        Rule::token(r"(?m)\^", OPERATOR),
        Rule::bygroups_g(r"(?m)(\w+)(\s*|[(,])", vec![Some(GroupAction::Token(NAME)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?m)[\[\]]", PUNCTUATION),
        Rule::token(r"(?m)<>|=>|[()|:;,.'`{}%&?]", PUNCTUATION),
        Rule::token(r"(?m)\n+", TEXT),
    ]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(
                r"(?m)\b([+-]?([2-9]|[12][0-9]|3[0-6])[rR][0-9a-zA-Z]+)\b",
                NUMBER_HEX,
            ),
            Rule::token(r"(?m)[+-]?[0-9]*\.([0-9]*)([Ee][+-]?[0-9]*)?", NUMBER_FLOAT),
            Rule::token(r"(?m)\b([+-]?[0-9]+[KMGTPkmgtp]?)\b", NUMBER_INTEGER),
        ],
    );
    m.insert(r"subprogram", vec![
        Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"#pop", r"formal_part"])),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r#"(?m)"[^"]+"|\w+"#, NAME_FUNCTION),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token_to(r"(?m)class|method|procedure", KEYWORD_DECLARATION, NewState::Push(vec![r"subprogram"])),
        Rule::bygroups_to(r"(?m)(record)(\s+)(\w+)", vec![Some(KEYWORD_DECLARATION), Some(TEXT), Some(KEYWORD_TYPE)], NewState::Push(vec![r"type_def"])),
        Rule::token(r"(?m)(#line|\$C|\$Cend|\$define|\$else|\$endif|\$error|\$ifdef|\$ifndef|\$include|\$line|\$undef)\b", TokenType::new(&["Keyword", "PreProc"])),
        Rule::token(r"(?m)(&null|&fail)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)&allocated|&ascii|&clock|&collections|&column|&col|&control|&cset|&current|&dateline|&date|&digits|&dump|&errno|&errornumber|&errortext|&errorvalue|&error|&errout|&eventcode|&eventvalue|&eventsource|&e|&features|&file|&host|&input|&interval|&lcase|&letters|&level|&line|&ldrag|&lpress|&lrelease|&main|&mdrag|&meta|&mpress|&mrelease|&now|&output|&phi|&pick|&pi|&pos|&progname|&random|&rdrag|&regions|&resize|&row|&rpress|&rrelease|&shift|&source|&storage|&subject|&time|&trace|&ucase|&version|&window|&x|&y", KEYWORD_RESERVED),
        Rule::token(r"(?m)(by|of|not|to)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(global|local|static|abstract)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)package|link|import", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(all|break|c(?:ase|r(?:eate|itical))|d(?:efault|o)|e(?:lse|nd|very)|fail|i(?:f|mport|n(?:itial(?:(?:ly)?)|vocable))|next|re(?:peat|turn)|suspend|th(?:en|read)|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(A(?:bort|ctive|lert|ny|rb(?:(?:no)?)|ttrib)|B(?:al|g|reak(?:(?:x)?))|C(?:l(?:ip|one)|o(?:lor(?:(?:Value)?)|pyArea|uple))|Draw(?:Arc|C(?:ircle|u(?:(?:b|rv)e)|ylinder)|Disk|Image|Line|Po(?:int|lygon)|Rectangle|S(?:egment|phere|tring)|Torus)|E(?:raseArea|v(?:Get|Send|ent)|ye)|F(?:ail|ence|g|ill(?:Arc|Circle|Polygon|Rectangle)|ont|ree(?:Color|Space))|G(?:etSpace|oto(?:RC|XY))|I(?:dentityMatrix|n(?:Port|t86))|L(?:en|ower)|M(?:atrixMode|ultMatrix)|N(?:ewColor|o(?:rmals|tAny))|OutPort|P(?:a(?:lette(?:C(?:hars|olor)|Key)|ttern)|e(?:ek|nding)|ixel|layAudio|o(?:ke|pMatrix|s)|ush(?:Matrix|(?:Rotat|Scal|Translat)e))|QueryPointer|R(?:aise|e(?:adImage|fresh|m)|otate|pos|tab)|S(?:cale|pan|topAudio|ucceed|wi)|T(?:ab|ex(?:coord|t(?:Width|ure))|ranslate)|Uncouple|VAttrib|W(?:Attrib|Default|Flush|S(?:ection|ync)|in(?:Associate|Button|ColorDialog|EditRegion|FontDialog|MenuBar|OpenDialog|PlayMedia|S(?:aveDialog|crollBar|electDialog)|dowContents)|riteImage)|a(?:bs|cos|ny|r(?:gs|ray)|sin|tan(?:(?:h)?))|bal|c(?:allout|enter|h(?:ar|dir|mod|own|root)|l(?:(?:assnam|os)e)|o(?:fail|llect|n(?:(?:dva|structo)r)|py|s)|rypt|set|time)|d(?:b(?:columns|driver|keys|limits|product|tables)|e(?:l(?:ay|ete)|tab)|isplay|tor)|e(?:ntab|rrorclear|ventmask|x(?:ec|it|p))|f(?:cntl|dup|etch|i(?:eldnames|lepair|nd)|l(?:ock|ush)|ork|unction)|g(?:et(?:(?:ch(?:(?:e)?)|e(?:gid|nv|uid)|g(?:id|r)|host|p(?:grp|id|pid|w)|rusage|serv|timeofday|uid)?)|lobalnames|time)|hardlink|i(?:and|com|mage|n(?:sert|teger)|o(?:ctl|r)|s(?:hift|tate)|xor)|k(?:bhit|ey(?:(?:word)?)|ill)|l(?:eft|ist|o(?:ad(?:(?:func)?)|c(?:alnames|k)|g)|stat)|m(?:a(?:ny|tch|[px])|e(?:mber(?:(?:names)?)|thod(?:(?:(?:name)?)s))|in|kdir|ove|utex)|n(?:ame|umeric)|o(?:p(?:en(?:(?:cl)?)|rec)|rd)|p(?:ar(?:amnames|ent)|ipe|o(?:[ps])|roc|u(?:ll|sh|t))|r(?:e(?:a(?:d(?:link|[sy])|[dl])|ceive|move|name|pl|verse)|ight|mdir|tod|unerr)|s(?:ave|e(?:ek|lect|nd|rial|t(?:env|g(?:id|rent)|hostent|p(?:grp|went)|servent|uid)|[qt])|i(?:gnal|n)|ort(?:(?:f)?)|pawn|q(?:l|rt)|t(?:at(?:(?:icnames)?)|op|r(?:ing|ucture))|y(?:mlink|s(?:_errstr|tem|write)))|t(?:a(?:ble|[bn])|r(?:ap|im|uncate|ylock)|ype)|u(?:mask|nlock|pto|time)|variable|w(?:ait|here|rite(?:(?:s)?)))\b", NAME_FUNCTION),
        Rule::token(r"(?m)\b([+-]?([2-9]|[12][0-9]|3[0-6])[rR][0-9a-zA-Z]+)\b", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]*\.([0-9]*)([Ee][+-]?[0-9]*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b([+-]?[0-9]+[KMGTPkmgtp]?)\b", NUMBER_INTEGER),
        Rule::token(r"(?m)<@|<<@|>@|>>@|\.>|->|===|~===|\*\*|\+\+|--|\.|~==|~=|<=|>=|==|=|<<=|<<|>>=|>>|:=:|:=|->|<->|\+:=|\|", OPERATOR),
        Rule::token(r#"(?m)"(?:[^\\"]|\\.)*""#, STRING),
        Rule::token(r"(?m)'(?:[^\\']|\\.)*'", TokenType::new(&["Literal", "String", "Character"])),
        Rule::token(r"(?m)[*<>+=/&!?@~\\-]", OPERATOR),
        Rule::token(r"(?m)\^", OPERATOR),
        Rule::bygroups_g(r"(?m)(\w+)(\s*|[(,])", vec![Some(GroupAction::Token(NAME)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?m)[\[\]]", PUNCTUATION),
        Rule::token(r"(?m)<>|=>|[()|:;,.'`{}%&?]", PUNCTUATION),
        Rule::token(r"(?m)\n+", TEXT),
    ]);
    m.insert(
        r"type_def",
        vec![Rule::token_to(
            r"(?m)\(",
            PUNCTUATION,
            NewState::Push(vec![r"formal_part"]),
        )],
    );
    m.insert(r"formal_part", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)(:string|:integer|:real)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)#.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token_to(r"(?m)class|method|procedure", KEYWORD_DECLARATION, NewState::Push(vec![r"subprogram"])),
        Rule::bygroups_to(r"(?m)(record)(\s+)(\w+)", vec![Some(KEYWORD_DECLARATION), Some(TEXT), Some(KEYWORD_TYPE)], NewState::Push(vec![r"type_def"])),
        Rule::token(r"(?m)(#line|\$C|\$Cend|\$define|\$else|\$endif|\$error|\$ifdef|\$ifndef|\$include|\$line|\$undef)\b", TokenType::new(&["Keyword", "PreProc"])),
        Rule::token(r"(?m)(&null|&fail)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)&allocated|&ascii|&clock|&collections|&column|&col|&control|&cset|&current|&dateline|&date|&digits|&dump|&errno|&errornumber|&errortext|&errorvalue|&error|&errout|&eventcode|&eventvalue|&eventsource|&e|&features|&file|&host|&input|&interval|&lcase|&letters|&level|&line|&ldrag|&lpress|&lrelease|&main|&mdrag|&meta|&mpress|&mrelease|&now|&output|&phi|&pick|&pi|&pos|&progname|&random|&rdrag|&regions|&resize|&row|&rpress|&rrelease|&shift|&source|&storage|&subject|&time|&trace|&ucase|&version|&window|&x|&y", KEYWORD_RESERVED),
        Rule::token(r"(?m)(by|of|not|to)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(global|local|static|abstract)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)package|link|import", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(all|break|c(?:ase|r(?:eate|itical))|d(?:efault|o)|e(?:lse|nd|very)|fail|i(?:f|mport|n(?:itial(?:(?:ly)?)|vocable))|next|re(?:peat|turn)|suspend|th(?:en|read)|until|while)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(A(?:bort|ctive|lert|ny|rb(?:(?:no)?)|ttrib)|B(?:al|g|reak(?:(?:x)?))|C(?:l(?:ip|one)|o(?:lor(?:(?:Value)?)|pyArea|uple))|Draw(?:Arc|C(?:ircle|u(?:(?:b|rv)e)|ylinder)|Disk|Image|Line|Po(?:int|lygon)|Rectangle|S(?:egment|phere|tring)|Torus)|E(?:raseArea|v(?:Get|Send|ent)|ye)|F(?:ail|ence|g|ill(?:Arc|Circle|Polygon|Rectangle)|ont|ree(?:Color|Space))|G(?:etSpace|oto(?:RC|XY))|I(?:dentityMatrix|n(?:Port|t86))|L(?:en|ower)|M(?:atrixMode|ultMatrix)|N(?:ewColor|o(?:rmals|tAny))|OutPort|P(?:a(?:lette(?:C(?:hars|olor)|Key)|ttern)|e(?:ek|nding)|ixel|layAudio|o(?:ke|pMatrix|s)|ush(?:Matrix|(?:Rotat|Scal|Translat)e))|QueryPointer|R(?:aise|e(?:adImage|fresh|m)|otate|pos|tab)|S(?:cale|pan|topAudio|ucceed|wi)|T(?:ab|ex(?:coord|t(?:Width|ure))|ranslate)|Uncouple|VAttrib|W(?:Attrib|Default|Flush|S(?:ection|ync)|in(?:Associate|Button|ColorDialog|EditRegion|FontDialog|MenuBar|OpenDialog|PlayMedia|S(?:aveDialog|crollBar|electDialog)|dowContents)|riteImage)|a(?:bs|cos|ny|r(?:gs|ray)|sin|tan(?:(?:h)?))|bal|c(?:allout|enter|h(?:ar|dir|mod|own|root)|l(?:(?:assnam|os)e)|o(?:fail|llect|n(?:(?:dva|structo)r)|py|s)|rypt|set|time)|d(?:b(?:columns|driver|keys|limits|product|tables)|e(?:l(?:ay|ete)|tab)|isplay|tor)|e(?:ntab|rrorclear|ventmask|x(?:ec|it|p))|f(?:cntl|dup|etch|i(?:eldnames|lepair|nd)|l(?:ock|ush)|ork|unction)|g(?:et(?:(?:ch(?:(?:e)?)|e(?:gid|nv|uid)|g(?:id|r)|host|p(?:grp|id|pid|w)|rusage|serv|timeofday|uid)?)|lobalnames|time)|hardlink|i(?:and|com|mage|n(?:sert|teger)|o(?:ctl|r)|s(?:hift|tate)|xor)|k(?:bhit|ey(?:(?:word)?)|ill)|l(?:eft|ist|o(?:ad(?:(?:func)?)|c(?:alnames|k)|g)|stat)|m(?:a(?:ny|tch|[px])|e(?:mber(?:(?:names)?)|thod(?:(?:(?:name)?)s))|in|kdir|ove|utex)|n(?:ame|umeric)|o(?:p(?:en(?:(?:cl)?)|rec)|rd)|p(?:ar(?:amnames|ent)|ipe|o(?:[ps])|roc|u(?:ll|sh|t))|r(?:e(?:a(?:d(?:link|[sy])|[dl])|ceive|move|name|pl|verse)|ight|mdir|tod|unerr)|s(?:ave|e(?:ek|lect|nd|rial|t(?:env|g(?:id|rent)|hostent|p(?:grp|went)|servent|uid)|[qt])|i(?:gnal|n)|ort(?:(?:f)?)|pawn|q(?:l|rt)|t(?:at(?:(?:icnames)?)|op|r(?:ing|ucture))|y(?:mlink|s(?:_errstr|tem|write)))|t(?:a(?:ble|[bn])|r(?:ap|im|uncate|ylock)|ype)|u(?:mask|nlock|pto|time)|variable|w(?:ait|here|rite(?:(?:s)?)))\b", NAME_FUNCTION),
        Rule::token(r"(?m)\b([+-]?([2-9]|[12][0-9]|3[0-6])[rR][0-9a-zA-Z]+)\b", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?[0-9]*\.([0-9]*)([Ee][+-]?[0-9]*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\b([+-]?[0-9]+[KMGTPkmgtp]?)\b", NUMBER_INTEGER),
        Rule::token(r"(?m)<@|<<@|>@|>>@|\.>|->|===|~===|\*\*|\+\+|--|\.|~==|~=|<=|>=|==|=|<<=|<<|>>=|>>|:=:|:=|->|<->|\+:=|\|", OPERATOR),
        Rule::token(r#"(?m)"(?:[^\\"]|\\.)*""#, STRING),
        Rule::token(r"(?m)'(?:[^\\']|\\.)*'", TokenType::new(&["Literal", "String", "Character"])),
        Rule::token(r"(?m)[*<>+=/&!?@~\\-]", OPERATOR),
        Rule::token(r"(?m)\^", OPERATOR),
        Rule::bygroups_g(r"(?m)(\w+)(\s*|[(,])", vec![Some(GroupAction::Token(NAME)), Some(GroupAction::UsingThis { state: None })]),
        Rule::token(r"(?m)[\[\]]", PUNCTUATION),
        Rule::token(r"(?m)<>|=>|[()|:;,.'`{}%&?]", PUNCTUATION),
        Rule::token(r"(?m)\n+", TEXT),
    ]);
    Table(m)
}

impl Lexer for UniconLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
