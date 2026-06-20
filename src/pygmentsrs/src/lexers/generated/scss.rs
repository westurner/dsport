#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.css:ScssLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.css:ScssLexer:scss

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: scss
pub struct ScssLexer;

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
            Rule::token(r"(?ims)\s+", WHITESPACE),
            Rule::token(r"(?ims)//.*?\n", COMMENT_SINGLE),
            Rule::token(r"(?ims)/\*.*?\*/", COMMENT_MULTILINE),
            Rule::token_to(r"(?ims)@import", KEYWORD, NewState::Push(vec![r"value"])),
            Rule::token_to(r"(?ims)@for", KEYWORD, NewState::Push(vec![r"for"])),
            Rule::token_to(
                r"(?ims)@(debug|warn|if|while)",
                KEYWORD,
                NewState::Push(vec![r"value"]),
            ),
            Rule::bygroups_to(
                r"(?ims)(@mixin)( [\w-]+)",
                vec![Some(KEYWORD), Some(NAME_FUNCTION)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::bygroups_to(
                r"(?ims)(@include)( [\w-]+)",
                vec![Some(KEYWORD), Some(NAME_DECORATOR)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::token_to(r"(?ims)@extend", KEYWORD, NewState::Push(vec![r"selector"])),
            Rule::bygroups_to(
                r"(?ims)(@media)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::token_to(r"(?ims)@[\w-]+", KEYWORD, NewState::Push(vec![r"selector"])),
            Rule::bygroups_to(
                r"(?ims)(\$[\w-]*\w)([ \t]*:)",
                vec![Some(NAME_VARIABLE), Some(OPERATOR)],
                NewState::Push(vec![r"value"]),
            ),
            Rule::default(NewState::Push(vec![r"selector"])),
        ],
    );
    m.insert(
        r"attr",
        vec![
            Rule::token(r#"(?ims)[^\s:="\[]+"#, NAME_ATTRIBUTE),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token_to(r"(?ims)[ \t]*:", OPERATOR, NewState::Push(vec![r"value"])),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"inline-comment",
        vec![
            Rule::token(
                r"(?ims)(\\#|#(?=[^{])|\*(?=[^/])|[^#*])+",
                COMMENT_MULTILINE,
            ),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token_to(r"(?ims)\*/", COMMENT, NewState::Pop(1)),
        ],
    );
    m.insert(r"value", vec![
        Rule::token(r"(?ims)[ \t]+", WHITESPACE),
        Rule::token(r"(?ims)[!$][\w-]+", NAME_VARIABLE),
        Rule::token_to(r"(?ims)url\(", STRING_OTHER, NewState::Push(vec![r"string-url"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*(?=\()", NAME_FUNCTION),
        Rule::token(r"(?ims)(\-webkit\-line\-clamp|a(?:b(?:(?:ov|solut)e)|ccent\-color|l(?:ign(?:\-(?:content|items|self)|ment\-baseline)|l|ways)|nimation(?:(?:\-(?:d(?:elay|(?:irec|ura)tion)|fill\-mode|iteration\-count|name|play\-state|timing\-function))?)|ppearance|rmenian|spect\-ratio|u(?:ral|to)|void|zimuth)|b(?:a(?:ck(?:face\-visibility|ground(?:(?:\-(?:attachment|blend\-mode|c(?:lip|olor)|image|origin|position|repeat|size))?))|seline(?:(?:\-s(?:hift|ource))?))|e(?:hind|low)|idi\-override|l(?:ink|ock(?:(?:\-(?:ellipsis|s(?:ize|tep(?:(?:\-(?:align|insert|round|size))?))))?))|o(?:ld(?:(?:er)?)|okmark\-(?:l(?:(?:ab|ev)el)|state)|rder(?:(?:\-(?:b(?:lock(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?)|o(?:ttom(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|undary))|col(?:lapse|or)|end\-(?:(?:end|start)\-radius)|i(?:mage(?:(?:\-(?:outset|repeat|s(?:(?:li|our)ce)|width))?)|nline(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?))|left(?:(?:\-(?:color|style|width))?)|r(?:adius|ight(?:(?:\-(?:color|style|width))?))|s(?:pacing|t(?:art\-(?:(?:end|start)\-radius)|yle))|top(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|width))?)|t(?:h|tom)|x\-(?:decoration\-break|s(?:hadow|izing|nap)))|reak\-(?:after|(?:befor|insid)e))|c(?:a(?:p(?:(?:italiz|tion\-sid)e)|ret(?:(?:\-(?:color|shape))?))|enter(?:(?:\-(?:(?:lef|righ)t))?)|hains|ircle|jk\-ideographic|l(?:ear|ip(?:(?:\-(?:path|rule))?)|ose\-quote)|o(?:l(?:lapse|or(?:(?:\-(?:adjust|interpolation\-filters|scheme))?)|umn(?:\-(?:count|fill|gap|rule(?:(?:\-(?:color|style|width))?)|span|width)|s))|n(?:densed|t(?:ain(?:(?:\-intrinsic\-(?:block\-size|height|inline\-size|size|width)|er(?:(?:\-(?:(?:nam|typ)e))?))?)|ent(?:(?:\-visibility)?)|inu(?:e|ous)))|unter\-(?:(?:incremen|(?:(?:re)?)se)t))|ross(?:(?:hair)?)|u(?:e(?:(?:\-(?:after|before))?)|rs(?:ive|or)))|d(?:ashed|e(?:cimal(?:(?:\-leading\-zero)?)|fault)|i(?:gits|rection|s(?:c|play))|o(?:minant\-baseline|tted|uble))|e(?:\-resize|levation|m(?:bed|pty\-cells)|x(?:(?:pand|tra\-(?:condens|expand))ed))|f(?:a(?:ntasy|r\-(?:(?:lef|righ)t)|st(?:(?:er)?))|i(?:l(?:l(?:(?:\-(?:break|color|image|o(?:pacity|rigin)|position|r(?:epeat|ule)|size))?)|ter)|xed)|l(?:ex(?:(?:\-(?:basis|direction|flow|grow|shrink|wrap))?)|o(?:at(?:(?:\-(?:defer|offset|reference))?)|od\-(?:color|opacity)|w(?:(?:\-(?:from|into))?)))|o(?:nt(?:(?:\-(?:f(?:amily|eature\-settings)|kerning|language\-override|optical\-sizing|palette|s(?:ize(?:(?:\-adjust)?)|t(?:retch|yle)|ynthesis(?:(?:\-(?:s(?:mall\-caps|tyle)|weight))?))|varia(?:nt(?:(?:\-(?:alternates|caps|e(?:ast\-asian|moji)|ligatures|numeric|position))?)|tion\-settings)|weight))?)|otnote\-(?:(?:displa|polic)y)|rced\-color\-adjust))|g(?:ap|eorgian|lyph\-orientation\-vertical|r(?:id(?:(?:\-(?:a(?:rea|uto\-(?:columns|flow|rows))|column(?:(?:\-(?:end|start))?)|row(?:(?:\-(?:end|start))?)|template(?:(?:\-(?:(?:area|column|row)s))?)))?)|oove))|h(?:anging\-punctuation|e(?:brew|ight|lp)|i(?:d(?:den|e)|gh(?:(?:er)?)|ragana(?:(?:\-iroha)?))|yphen(?:ate\-(?:character|limit\-(?:chars|l(?:ast|ines)|zone))|s))|i(?:con|mage\-(?:orientation|re(?:ndering|solution))|n(?:herit|itial\-letter(?:(?:\-(?:align|wrap))?)|line(?:(?:\-(?:siz(?:e|ing)|table))?)|put\-security|s(?:et(?:(?:(?:\-(?:block(?:(?:\-(?:end|start))?)|inline(?:(?:\-(?:end|start))?)))?)?)|ide)|vert)|solation|talic)|justify(?:(?:\-(?:content|items|self))?)|katakana(?:(?:\-iroha)?)|l(?:a(?:ndscape|rge(?:(?:r)?))|e(?:ading\-trim|ft(?:(?:\-side|wards)?)|tter\-spacing|vel)|i(?:ght(?:(?:e|ing\-colo)r)|ne\-(?:break|clamp|grid|height(?:(?:\-step)?)|padding|snap|through)|st\-(?:item|style(?:(?:\-(?:image|position|type))?)))|o(?:ud|w(?:(?:er(?:(?:\-(?:alpha|greek|roman)|case)?))?))|tr)|m(?:a(?:r(?:gin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom|reak)|inline(?:(?:\-(?:end|start))?)|left|right|t(?:op|rim)))?)|ker(?:(?:\-(?:end|knockout\-(?:(?:lef|righ)t)|mid|pattern|s(?:egment|ide|tart)))?))|sk(?:(?:\-(?:border(?:(?:\-(?:mode|outset|repeat|s(?:(?:li|our)ce)|width))?)|c(?:lip|omposite)|image|mode|origin|position|repeat|(?:siz|typ)e))?)|x\-(?:block\-size|height|inline\-size|lines|width))|e(?:dium|ssage\-box)|i(?:ddle|n\-(?:block\-size|height|in(?:line\-size|trinsic\-sizing)|width)|x(?:(?:\-blend\-mode)?))|onospace)|n(?:\-resize|a(?:rrower|v\-(?:down|left|right|up))|e\-resize|o(?:\-(?:close\-quote|open\-quote|repeat)|ne|rmal|wrap)|w\-resize)|o(?:b(?:ject\-(?:fit|overflow|position|view\-box)|lique)|ffset(?:(?:\-(?:anchor|distance|p(?:ath|osition)|rotate))?)|nce|p(?:acity|en\-quote)|r(?:der|phans)|ut(?:line(?:(?:\-(?:color|offset|style|width))?)|s(?:et|ide))|ver(?:flow(?:(?:\-(?:anchor|block|clip\-margin|inline|wrap|[xy]))?)|line|scroll\-behavior(?:(?:\-(?:block|inline|[xy]))?)))|p(?:a(?:dding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|ge(?:(?:\-break\-(?:after|(?:befor|insid)e))?)|use(?:(?:\-(?:after|before))?))|erspective(?:(?:\-origin)?)|itch(?:(?:\-range)?)|la(?:ce\-(?:content|items|self)|y\-during)|o(?:inter(?:(?:\-events)?)|rtrait|sition)|r(?:int\-color\-adjust|operty\-name)|x)|quotes|r(?:e(?:gion\-fragment|lative|peat(?:(?:\-(?:[xy]))?)|s(?:ize|t(?:(?:\-(?:after|before))?)))|gb|i(?:chness|dge|ght(?:(?:\-side|wards)?))|o(?:tate|w\-gap)|u(?:by\-(?:align|merge|overhang|position)|nning))|s(?:\-resize|ans\-serif|c(?:ale|roll(?:(?:\-(?:behavior|margin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|padding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|snap\-(?:align|stop|type))|bar\-(?:color|gutter|width))?))|e(?:\-resize|mi\-(?:(?:condens|expand)ed)|parate|rif)|h(?:ape\-(?:i(?:mage\-threshold|nside)|margin|outside)|ow)|ilent|low(?:(?:er)?)|mall(?:\-cap(?:s|tion)|er)|o(?:ft|lid)|p(?:atial\-navigation\-(?:(?:actio|contai|functio)n)|e(?:ak(?:(?:\-(?:as|header|numeral|punctuation))?)|ech\-rate|ll\-out))|quare|t(?:at(?:ic|us\-bar)|r(?:ess|ing\-set|oke(?:(?:\-(?:align(?:(?:ment)?)|break|color|dash(?:\-(?:corner|justify)|a(?:djust|rray)|corner|offset)|image|line(?:cap|join)|miterlimit|o(?:pacity|rigin)|position|repeat|size|width))?)))|uper|w\-resize)|t(?:ab(?:\-size|le\-(?:c(?:aption|ell|olumn(?:(?:\-group)?))|footer\-group|header\-group|layout|row(?:(?:\-group)?)))|ext(?:(?:\-(?:align(?:(?:\-(?:all|last))?)|bottom|combine\-upright|decoration(?:(?:\-(?:color|line|s(?:kip(?:(?:\-(?:box|in(?:k|set)|s(?:elf|paces)))?)|tyle)|thickness))?)|e(?:dge|mphasis(?:(?:\-(?:color|position|s(?:kip|tyle)))?))|group\-align|indent|justify|o(?:rientation|verflow)|s(?:hadow|pac(?:e\-(?:collapse|trim)|ing))|t(?:op|ransform)|underline\-(?:offset|position)|wrap))?)|hi(?:ck|n)|op|rans(?:form(?:(?:\-(?:box|origin|style))?)|ition(?:(?:\-(?:d(?:elay|uration)|property|timing\-function))?)|late|parent))|u(?:ltra\-(?:(?:condens|expand)ed)|n(?:derline|icode\-bidi)|pper(?:\-(?:alpha|(?:lati|roma)n)|case)|rl|ser\-select)|v(?:ertical\-align|isib(?:ility|le)|o(?:ice\-(?:balance|duration|family|pitch|ra(?:(?:ng|t)e)|stress|volume)|lume))|w(?:\-resize|ait|hite\-space|i(?:d(?:er|ows|th)|ll\-change)|ord\-(?:b(?:oundary\-(?:(?:detect|expans)ion)|reak)|spacing|wrap)|r(?:ap\-(?:after|before|flow|inside|through)|iting\-mode))|x(?:\-(?:fast|high|l(?:arge|o(?:ud|w))|s(?:mall|oft))|x\-(?:large|small))|yes|z\-index)\b", NAME_CONSTANT),
        Rule::token(r"(?ims)(a(?:liceblue|ntiquewhite|qua(?:(?:marine)?)|zure)|b(?:eige|isque|l(?:a(?:ck|nchedalmond)|ue(?:(?:violet)?))|rown|urlywood)|c(?:adetblue|h(?:(?:artreus|ocolat)e)|or(?:al|n(?:flowerblue|silk))|(?:rimso|ya)n)|d(?:ark(?:blue|cyan|g(?:oldenrod|r(?:ay|e(?:en|y)))|khaki|magenta|o(?:livegreen|r(?:ange|chid))|red|s(?:almon|eagreen|late(?:blue|gr(?:(?:[ae])y)))|turquoise|violet)|eep(?:pink|skyblue)|imgr(?:(?:[ae])y)|odgerblue)|f(?:irebrick|loralwhite|orestgreen|uchsia)|g(?:ainsboro|hostwhite|old(?:(?:enrod)?)|r(?:ay|e(?:en(?:(?:yellow)?)|y)))|ho(?:neydew|tpink)|i(?:ndi(?:anred|go)|vory)|khaki|l(?:a(?:vender(?:(?:blush)?)|wngreen)|emonchiffon|i(?:ght(?:blue|c(?:oral|yan)|g(?:oldenrodyellow|r(?:ay|e(?:en|y)))|pink|s(?:almon|eagreen|kyblue|lategr(?:(?:[ae])y)|teelblue)|yellow)|me(?:(?:green)?)|nen))|m(?:a(?:genta|roon)|edium(?:aquamarine|blue|orchid|purple|s(?:eagreen|lateblue|pringgreen)|turquoise|violetred)|i(?:dnightblue|ntcream|styrose)|occasin)|nav(?:ajowhite|y)|o(?:l(?:dlace|ive(?:(?:drab)?))|r(?:ange(?:(?:red)?)|chid))|p(?:a(?:le(?:g(?:oldenrod|reen)|turquoise|violetred)|payawhip)|e(?:achpuff|ru)|ink|lum|(?:owderblu|urpl)e)|r(?:e(?:beccapurple|d)|o(?:sybrown|yalblue))|s(?:a(?:(?:ddlebrow|lmo|ndybrow)n)|ea(?:green|shell)|i(?:enna|lver)|kyblue|late(?:blue|gr(?:(?:[ae])y))|now|pringgreen|teelblue)|t(?:an|eal|histle|omato|ransparent|urquoise)|violet|wh(?:eat|ite(?:(?:smoke)?))|yellow(?:(?:green)?))\b", NAME_ENTITY),
        Rule::token(r"(?ims)(aqua|bl(?:ack|ue)|fuchsia|gr(?:ay|een)|lime|maroon|navy|olive|purple|red|silver|teal|white|yellow)\b", NAME_BUILTIN),
        Rule::token(r"(?ims)\!(important|default)", NAME_EXCEPTION),
        Rule::token(r"(?ims)(true|false)", TokenType::new(&["Name", "Pseudo"])),
        Rule::token(r"(?ims)(and|or|not)", OPERATOR_WORD),
        Rule::token_to(r"(?ims)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"inline-comment"])),
        Rule::token(r"(?ims)//[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?ims)\#[a-z0-9]{1,6}", NUMBER_HEX),
        Rule::bygroups(r"(?ims)(-?\d+)(\%|[a-z]+)?", vec![Some(NUMBER_INTEGER), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?ims)(-?\d*\.\d+)(\%|[a-z]+)?", vec![Some(NUMBER_FLOAT), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?ims)#\{", STRING_INTERPOL, NewState::Push(vec![r"interpolation"])),
        Rule::token(r"(?ims)[~^*!&%<>|+=@:,./?-]+", OPERATOR),
        Rule::token(r"(?ims)[\[\]()]+", PUNCTUATION),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string-double"])),
        Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Push(vec![r"string-single"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*", NAME),
        Rule::token(r"(?ims)\n", WHITESPACE),
        Rule::token_to(r"(?ims)[;{}]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"interpolation", vec![
        Rule::token_to(r"(?ims)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?ims)[ \t]+", WHITESPACE),
        Rule::token(r"(?ims)[!$][\w-]+", NAME_VARIABLE),
        Rule::token_to(r"(?ims)url\(", STRING_OTHER, NewState::Push(vec![r"string-url"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*(?=\()", NAME_FUNCTION),
        Rule::token(r"(?ims)(\-webkit\-line\-clamp|a(?:b(?:(?:ov|solut)e)|ccent\-color|l(?:ign(?:\-(?:content|items|self)|ment\-baseline)|l|ways)|nimation(?:(?:\-(?:d(?:elay|(?:irec|ura)tion)|fill\-mode|iteration\-count|name|play\-state|timing\-function))?)|ppearance|rmenian|spect\-ratio|u(?:ral|to)|void|zimuth)|b(?:a(?:ck(?:face\-visibility|ground(?:(?:\-(?:attachment|blend\-mode|c(?:lip|olor)|image|origin|position|repeat|size))?))|seline(?:(?:\-s(?:hift|ource))?))|e(?:hind|low)|idi\-override|l(?:ink|ock(?:(?:\-(?:ellipsis|s(?:ize|tep(?:(?:\-(?:align|insert|round|size))?))))?))|o(?:ld(?:(?:er)?)|okmark\-(?:l(?:(?:ab|ev)el)|state)|rder(?:(?:\-(?:b(?:lock(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?)|o(?:ttom(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|undary))|col(?:lapse|or)|end\-(?:(?:end|start)\-radius)|i(?:mage(?:(?:\-(?:outset|repeat|s(?:(?:li|our)ce)|width))?)|nline(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?))|left(?:(?:\-(?:color|style|width))?)|r(?:adius|ight(?:(?:\-(?:color|style|width))?))|s(?:pacing|t(?:art\-(?:(?:end|start)\-radius)|yle))|top(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|width))?)|t(?:h|tom)|x\-(?:decoration\-break|s(?:hadow|izing|nap)))|reak\-(?:after|(?:befor|insid)e))|c(?:a(?:p(?:(?:italiz|tion\-sid)e)|ret(?:(?:\-(?:color|shape))?))|enter(?:(?:\-(?:(?:lef|righ)t))?)|hains|ircle|jk\-ideographic|l(?:ear|ip(?:(?:\-(?:path|rule))?)|ose\-quote)|o(?:l(?:lapse|or(?:(?:\-(?:adjust|interpolation\-filters|scheme))?)|umn(?:\-(?:count|fill|gap|rule(?:(?:\-(?:color|style|width))?)|span|width)|s))|n(?:densed|t(?:ain(?:(?:\-intrinsic\-(?:block\-size|height|inline\-size|size|width)|er(?:(?:\-(?:(?:nam|typ)e))?))?)|ent(?:(?:\-visibility)?)|inu(?:e|ous)))|unter\-(?:(?:incremen|(?:(?:re)?)se)t))|ross(?:(?:hair)?)|u(?:e(?:(?:\-(?:after|before))?)|rs(?:ive|or)))|d(?:ashed|e(?:cimal(?:(?:\-leading\-zero)?)|fault)|i(?:gits|rection|s(?:c|play))|o(?:minant\-baseline|tted|uble))|e(?:\-resize|levation|m(?:bed|pty\-cells)|x(?:(?:pand|tra\-(?:condens|expand))ed))|f(?:a(?:ntasy|r\-(?:(?:lef|righ)t)|st(?:(?:er)?))|i(?:l(?:l(?:(?:\-(?:break|color|image|o(?:pacity|rigin)|position|r(?:epeat|ule)|size))?)|ter)|xed)|l(?:ex(?:(?:\-(?:basis|direction|flow|grow|shrink|wrap))?)|o(?:at(?:(?:\-(?:defer|offset|reference))?)|od\-(?:color|opacity)|w(?:(?:\-(?:from|into))?)))|o(?:nt(?:(?:\-(?:f(?:amily|eature\-settings)|kerning|language\-override|optical\-sizing|palette|s(?:ize(?:(?:\-adjust)?)|t(?:retch|yle)|ynthesis(?:(?:\-(?:s(?:mall\-caps|tyle)|weight))?))|varia(?:nt(?:(?:\-(?:alternates|caps|e(?:ast\-asian|moji)|ligatures|numeric|position))?)|tion\-settings)|weight))?)|otnote\-(?:(?:displa|polic)y)|rced\-color\-adjust))|g(?:ap|eorgian|lyph\-orientation\-vertical|r(?:id(?:(?:\-(?:a(?:rea|uto\-(?:columns|flow|rows))|column(?:(?:\-(?:end|start))?)|row(?:(?:\-(?:end|start))?)|template(?:(?:\-(?:(?:area|column|row)s))?)))?)|oove))|h(?:anging\-punctuation|e(?:brew|ight|lp)|i(?:d(?:den|e)|gh(?:(?:er)?)|ragana(?:(?:\-iroha)?))|yphen(?:ate\-(?:character|limit\-(?:chars|l(?:ast|ines)|zone))|s))|i(?:con|mage\-(?:orientation|re(?:ndering|solution))|n(?:herit|itial\-letter(?:(?:\-(?:align|wrap))?)|line(?:(?:\-(?:siz(?:e|ing)|table))?)|put\-security|s(?:et(?:(?:(?:\-(?:block(?:(?:\-(?:end|start))?)|inline(?:(?:\-(?:end|start))?)))?)?)|ide)|vert)|solation|talic)|justify(?:(?:\-(?:content|items|self))?)|katakana(?:(?:\-iroha)?)|l(?:a(?:ndscape|rge(?:(?:r)?))|e(?:ading\-trim|ft(?:(?:\-side|wards)?)|tter\-spacing|vel)|i(?:ght(?:(?:e|ing\-colo)r)|ne\-(?:break|clamp|grid|height(?:(?:\-step)?)|padding|snap|through)|st\-(?:item|style(?:(?:\-(?:image|position|type))?)))|o(?:ud|w(?:(?:er(?:(?:\-(?:alpha|greek|roman)|case)?))?))|tr)|m(?:a(?:r(?:gin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom|reak)|inline(?:(?:\-(?:end|start))?)|left|right|t(?:op|rim)))?)|ker(?:(?:\-(?:end|knockout\-(?:(?:lef|righ)t)|mid|pattern|s(?:egment|ide|tart)))?))|sk(?:(?:\-(?:border(?:(?:\-(?:mode|outset|repeat|s(?:(?:li|our)ce)|width))?)|c(?:lip|omposite)|image|mode|origin|position|repeat|(?:siz|typ)e))?)|x\-(?:block\-size|height|inline\-size|lines|width))|e(?:dium|ssage\-box)|i(?:ddle|n\-(?:block\-size|height|in(?:line\-size|trinsic\-sizing)|width)|x(?:(?:\-blend\-mode)?))|onospace)|n(?:\-resize|a(?:rrower|v\-(?:down|left|right|up))|e\-resize|o(?:\-(?:close\-quote|open\-quote|repeat)|ne|rmal|wrap)|w\-resize)|o(?:b(?:ject\-(?:fit|overflow|position|view\-box)|lique)|ffset(?:(?:\-(?:anchor|distance|p(?:ath|osition)|rotate))?)|nce|p(?:acity|en\-quote)|r(?:der|phans)|ut(?:line(?:(?:\-(?:color|offset|style|width))?)|s(?:et|ide))|ver(?:flow(?:(?:\-(?:anchor|block|clip\-margin|inline|wrap|[xy]))?)|line|scroll\-behavior(?:(?:\-(?:block|inline|[xy]))?)))|p(?:a(?:dding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|ge(?:(?:\-break\-(?:after|(?:befor|insid)e))?)|use(?:(?:\-(?:after|before))?))|erspective(?:(?:\-origin)?)|itch(?:(?:\-range)?)|la(?:ce\-(?:content|items|self)|y\-during)|o(?:inter(?:(?:\-events)?)|rtrait|sition)|r(?:int\-color\-adjust|operty\-name)|x)|quotes|r(?:e(?:gion\-fragment|lative|peat(?:(?:\-(?:[xy]))?)|s(?:ize|t(?:(?:\-(?:after|before))?)))|gb|i(?:chness|dge|ght(?:(?:\-side|wards)?))|o(?:tate|w\-gap)|u(?:by\-(?:align|merge|overhang|position)|nning))|s(?:\-resize|ans\-serif|c(?:ale|roll(?:(?:\-(?:behavior|margin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|padding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|snap\-(?:align|stop|type))|bar\-(?:color|gutter|width))?))|e(?:\-resize|mi\-(?:(?:condens|expand)ed)|parate|rif)|h(?:ape\-(?:i(?:mage\-threshold|nside)|margin|outside)|ow)|ilent|low(?:(?:er)?)|mall(?:\-cap(?:s|tion)|er)|o(?:ft|lid)|p(?:atial\-navigation\-(?:(?:actio|contai|functio)n)|e(?:ak(?:(?:\-(?:as|header|numeral|punctuation))?)|ech\-rate|ll\-out))|quare|t(?:at(?:ic|us\-bar)|r(?:ess|ing\-set|oke(?:(?:\-(?:align(?:(?:ment)?)|break|color|dash(?:\-(?:corner|justify)|a(?:djust|rray)|corner|offset)|image|line(?:cap|join)|miterlimit|o(?:pacity|rigin)|position|repeat|size|width))?)))|uper|w\-resize)|t(?:ab(?:\-size|le\-(?:c(?:aption|ell|olumn(?:(?:\-group)?))|footer\-group|header\-group|layout|row(?:(?:\-group)?)))|ext(?:(?:\-(?:align(?:(?:\-(?:all|last))?)|bottom|combine\-upright|decoration(?:(?:\-(?:color|line|s(?:kip(?:(?:\-(?:box|in(?:k|set)|s(?:elf|paces)))?)|tyle)|thickness))?)|e(?:dge|mphasis(?:(?:\-(?:color|position|s(?:kip|tyle)))?))|group\-align|indent|justify|o(?:rientation|verflow)|s(?:hadow|pac(?:e\-(?:collapse|trim)|ing))|t(?:op|ransform)|underline\-(?:offset|position)|wrap))?)|hi(?:ck|n)|op|rans(?:form(?:(?:\-(?:box|origin|style))?)|ition(?:(?:\-(?:d(?:elay|uration)|property|timing\-function))?)|late|parent))|u(?:ltra\-(?:(?:condens|expand)ed)|n(?:derline|icode\-bidi)|pper(?:\-(?:alpha|(?:lati|roma)n)|case)|rl|ser\-select)|v(?:ertical\-align|isib(?:ility|le)|o(?:ice\-(?:balance|duration|family|pitch|ra(?:(?:ng|t)e)|stress|volume)|lume))|w(?:\-resize|ait|hite\-space|i(?:d(?:er|ows|th)|ll\-change)|ord\-(?:b(?:oundary\-(?:(?:detect|expans)ion)|reak)|spacing|wrap)|r(?:ap\-(?:after|before|flow|inside|through)|iting\-mode))|x(?:\-(?:fast|high|l(?:arge|o(?:ud|w))|s(?:mall|oft))|x\-(?:large|small))|yes|z\-index)\b", NAME_CONSTANT),
        Rule::token(r"(?ims)(a(?:liceblue|ntiquewhite|qua(?:(?:marine)?)|zure)|b(?:eige|isque|l(?:a(?:ck|nchedalmond)|ue(?:(?:violet)?))|rown|urlywood)|c(?:adetblue|h(?:(?:artreus|ocolat)e)|or(?:al|n(?:flowerblue|silk))|(?:rimso|ya)n)|d(?:ark(?:blue|cyan|g(?:oldenrod|r(?:ay|e(?:en|y)))|khaki|magenta|o(?:livegreen|r(?:ange|chid))|red|s(?:almon|eagreen|late(?:blue|gr(?:(?:[ae])y)))|turquoise|violet)|eep(?:pink|skyblue)|imgr(?:(?:[ae])y)|odgerblue)|f(?:irebrick|loralwhite|orestgreen|uchsia)|g(?:ainsboro|hostwhite|old(?:(?:enrod)?)|r(?:ay|e(?:en(?:(?:yellow)?)|y)))|ho(?:neydew|tpink)|i(?:ndi(?:anred|go)|vory)|khaki|l(?:a(?:vender(?:(?:blush)?)|wngreen)|emonchiffon|i(?:ght(?:blue|c(?:oral|yan)|g(?:oldenrodyellow|r(?:ay|e(?:en|y)))|pink|s(?:almon|eagreen|kyblue|lategr(?:(?:[ae])y)|teelblue)|yellow)|me(?:(?:green)?)|nen))|m(?:a(?:genta|roon)|edium(?:aquamarine|blue|orchid|purple|s(?:eagreen|lateblue|pringgreen)|turquoise|violetred)|i(?:dnightblue|ntcream|styrose)|occasin)|nav(?:ajowhite|y)|o(?:l(?:dlace|ive(?:(?:drab)?))|r(?:ange(?:(?:red)?)|chid))|p(?:a(?:le(?:g(?:oldenrod|reen)|turquoise|violetred)|payawhip)|e(?:achpuff|ru)|ink|lum|(?:owderblu|urpl)e)|r(?:e(?:beccapurple|d)|o(?:sybrown|yalblue))|s(?:a(?:(?:ddlebrow|lmo|ndybrow)n)|ea(?:green|shell)|i(?:enna|lver)|kyblue|late(?:blue|gr(?:(?:[ae])y))|now|pringgreen|teelblue)|t(?:an|eal|histle|omato|ransparent|urquoise)|violet|wh(?:eat|ite(?:(?:smoke)?))|yellow(?:(?:green)?))\b", NAME_ENTITY),
        Rule::token(r"(?ims)(aqua|bl(?:ack|ue)|fuchsia|gr(?:ay|een)|lime|maroon|navy|olive|purple|red|silver|teal|white|yellow)\b", NAME_BUILTIN),
        Rule::token(r"(?ims)\!(important|default)", NAME_EXCEPTION),
        Rule::token(r"(?ims)(true|false)", TokenType::new(&["Name", "Pseudo"])),
        Rule::token(r"(?ims)(and|or|not)", OPERATOR_WORD),
        Rule::token_to(r"(?ims)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"inline-comment"])),
        Rule::token(r"(?ims)//[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?ims)\#[a-z0-9]{1,6}", NUMBER_HEX),
        Rule::bygroups(r"(?ims)(-?\d+)(\%|[a-z]+)?", vec![Some(NUMBER_INTEGER), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?ims)(-?\d*\.\d+)(\%|[a-z]+)?", vec![Some(NUMBER_FLOAT), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?ims)#\{", STRING_INTERPOL, NewState::Push(vec![r"interpolation"])),
        Rule::token(r"(?ims)[~^*!&%<>|+=@:,./?-]+", OPERATOR),
        Rule::token(r"(?ims)[\[\]()]+", PUNCTUATION),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string-double"])),
        Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Push(vec![r"string-single"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*", NAME),
        Rule::token(r"(?ims)\n", WHITESPACE),
        Rule::token_to(r"(?ims)[;{}]", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(
        r"selector",
        vec![
            Rule::token(r"(?ims)[ \t]+", WHITESPACE),
            Rule::token_to(
                r"(?ims)\:",
                NAME_DECORATOR,
                NewState::Push(vec![r"pseudo-class"]),
            ),
            Rule::token_to(r"(?ims)\.", NAME_CLASS, NewState::Push(vec![r"class"])),
            Rule::token_to(r"(?ims)\#", NAME_NAMESPACE, NewState::Push(vec![r"id"])),
            Rule::token(r"(?ims)[\w-]+", NAME_TAG),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token(r"(?ims)&", KEYWORD),
            Rule::token(r"(?ims)[~^*!&\[\]()<>|+=@:;,./?-]", OPERATOR),
            Rule::token_to(
                r#"(?ims)""#,
                STRING_DOUBLE,
                NewState::Push(vec![r"string-double"]),
            ),
            Rule::token_to(
                r"(?ims)'",
                STRING_SINGLE,
                NewState::Push(vec![r"string-single"]),
            ),
            Rule::token(r"(?ims)\n", WHITESPACE),
            Rule::token_to(r"(?ims)[;{}]", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string-double",
        vec![
            Rule::token(r##"(?ims)(\\.|#(?=[^\n{])|[^\n"#])+"##, STRING_DOUBLE),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string-single",
        vec![
            Rule::token(r"(?ims)(\\.|#(?=[^\n{])|[^\n'#])+", STRING_SINGLE),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string-url",
        vec![
            Rule::token(r"(?ims)(\\#|#(?=[^\n{])|[^\n#)])+", STRING_OTHER),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::token_to(r"(?ims)\)", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"pseudo-class",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_DECORATOR),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"class",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_CLASS),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"id",
        vec![
            Rule::token(r"(?ims)[\w-]+", NAME_NAMESPACE),
            Rule::token_to(
                r"(?ims)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpolation"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"for", vec![
        Rule::token(r"(?ims)(from|to|through)", OPERATOR_WORD),
        Rule::token(r"(?ims)[ \t]+", WHITESPACE),
        Rule::token(r"(?ims)[!$][\w-]+", NAME_VARIABLE),
        Rule::token_to(r"(?ims)url\(", STRING_OTHER, NewState::Push(vec![r"string-url"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*(?=\()", NAME_FUNCTION),
        Rule::token(r"(?ims)(\-webkit\-line\-clamp|a(?:b(?:(?:ov|solut)e)|ccent\-color|l(?:ign(?:\-(?:content|items|self)|ment\-baseline)|l|ways)|nimation(?:(?:\-(?:d(?:elay|(?:irec|ura)tion)|fill\-mode|iteration\-count|name|play\-state|timing\-function))?)|ppearance|rmenian|spect\-ratio|u(?:ral|to)|void|zimuth)|b(?:a(?:ck(?:face\-visibility|ground(?:(?:\-(?:attachment|blend\-mode|c(?:lip|olor)|image|origin|position|repeat|size))?))|seline(?:(?:\-s(?:hift|ource))?))|e(?:hind|low)|idi\-override|l(?:ink|ock(?:(?:\-(?:ellipsis|s(?:ize|tep(?:(?:\-(?:align|insert|round|size))?))))?))|o(?:ld(?:(?:er)?)|okmark\-(?:l(?:(?:ab|ev)el)|state)|rder(?:(?:\-(?:b(?:lock(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?)|o(?:ttom(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|undary))|col(?:lapse|or)|end\-(?:(?:end|start)\-radius)|i(?:mage(?:(?:\-(?:outset|repeat|s(?:(?:li|our)ce)|width))?)|nline(?:(?:\-(?:color|end(?:(?:\-(?:color|style|width))?)|st(?:art(?:(?:\-(?:color|style|width))?)|yle)|width))?))|left(?:(?:\-(?:color|style|width))?)|r(?:adius|ight(?:(?:\-(?:color|style|width))?))|s(?:pacing|t(?:art\-(?:(?:end|start)\-radius)|yle))|top(?:(?:\-(?:color|left\-radius|right\-radius|style|width))?)|width))?)|t(?:h|tom)|x\-(?:decoration\-break|s(?:hadow|izing|nap)))|reak\-(?:after|(?:befor|insid)e))|c(?:a(?:p(?:(?:italiz|tion\-sid)e)|ret(?:(?:\-(?:color|shape))?))|enter(?:(?:\-(?:(?:lef|righ)t))?)|hains|ircle|jk\-ideographic|l(?:ear|ip(?:(?:\-(?:path|rule))?)|ose\-quote)|o(?:l(?:lapse|or(?:(?:\-(?:adjust|interpolation\-filters|scheme))?)|umn(?:\-(?:count|fill|gap|rule(?:(?:\-(?:color|style|width))?)|span|width)|s))|n(?:densed|t(?:ain(?:(?:\-intrinsic\-(?:block\-size|height|inline\-size|size|width)|er(?:(?:\-(?:(?:nam|typ)e))?))?)|ent(?:(?:\-visibility)?)|inu(?:e|ous)))|unter\-(?:(?:incremen|(?:(?:re)?)se)t))|ross(?:(?:hair)?)|u(?:e(?:(?:\-(?:after|before))?)|rs(?:ive|or)))|d(?:ashed|e(?:cimal(?:(?:\-leading\-zero)?)|fault)|i(?:gits|rection|s(?:c|play))|o(?:minant\-baseline|tted|uble))|e(?:\-resize|levation|m(?:bed|pty\-cells)|x(?:(?:pand|tra\-(?:condens|expand))ed))|f(?:a(?:ntasy|r\-(?:(?:lef|righ)t)|st(?:(?:er)?))|i(?:l(?:l(?:(?:\-(?:break|color|image|o(?:pacity|rigin)|position|r(?:epeat|ule)|size))?)|ter)|xed)|l(?:ex(?:(?:\-(?:basis|direction|flow|grow|shrink|wrap))?)|o(?:at(?:(?:\-(?:defer|offset|reference))?)|od\-(?:color|opacity)|w(?:(?:\-(?:from|into))?)))|o(?:nt(?:(?:\-(?:f(?:amily|eature\-settings)|kerning|language\-override|optical\-sizing|palette|s(?:ize(?:(?:\-adjust)?)|t(?:retch|yle)|ynthesis(?:(?:\-(?:s(?:mall\-caps|tyle)|weight))?))|varia(?:nt(?:(?:\-(?:alternates|caps|e(?:ast\-asian|moji)|ligatures|numeric|position))?)|tion\-settings)|weight))?)|otnote\-(?:(?:displa|polic)y)|rced\-color\-adjust))|g(?:ap|eorgian|lyph\-orientation\-vertical|r(?:id(?:(?:\-(?:a(?:rea|uto\-(?:columns|flow|rows))|column(?:(?:\-(?:end|start))?)|row(?:(?:\-(?:end|start))?)|template(?:(?:\-(?:(?:area|column|row)s))?)))?)|oove))|h(?:anging\-punctuation|e(?:brew|ight|lp)|i(?:d(?:den|e)|gh(?:(?:er)?)|ragana(?:(?:\-iroha)?))|yphen(?:ate\-(?:character|limit\-(?:chars|l(?:ast|ines)|zone))|s))|i(?:con|mage\-(?:orientation|re(?:ndering|solution))|n(?:herit|itial\-letter(?:(?:\-(?:align|wrap))?)|line(?:(?:\-(?:siz(?:e|ing)|table))?)|put\-security|s(?:et(?:(?:(?:\-(?:block(?:(?:\-(?:end|start))?)|inline(?:(?:\-(?:end|start))?)))?)?)|ide)|vert)|solation|talic)|justify(?:(?:\-(?:content|items|self))?)|katakana(?:(?:\-iroha)?)|l(?:a(?:ndscape|rge(?:(?:r)?))|e(?:ading\-trim|ft(?:(?:\-side|wards)?)|tter\-spacing|vel)|i(?:ght(?:(?:e|ing\-colo)r)|ne\-(?:break|clamp|grid|height(?:(?:\-step)?)|padding|snap|through)|st\-(?:item|style(?:(?:\-(?:image|position|type))?)))|o(?:ud|w(?:(?:er(?:(?:\-(?:alpha|greek|roman)|case)?))?))|tr)|m(?:a(?:r(?:gin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom|reak)|inline(?:(?:\-(?:end|start))?)|left|right|t(?:op|rim)))?)|ker(?:(?:\-(?:end|knockout\-(?:(?:lef|righ)t)|mid|pattern|s(?:egment|ide|tart)))?))|sk(?:(?:\-(?:border(?:(?:\-(?:mode|outset|repeat|s(?:(?:li|our)ce)|width))?)|c(?:lip|omposite)|image|mode|origin|position|repeat|(?:siz|typ)e))?)|x\-(?:block\-size|height|inline\-size|lines|width))|e(?:dium|ssage\-box)|i(?:ddle|n\-(?:block\-size|height|in(?:line\-size|trinsic\-sizing)|width)|x(?:(?:\-blend\-mode)?))|onospace)|n(?:\-resize|a(?:rrower|v\-(?:down|left|right|up))|e\-resize|o(?:\-(?:close\-quote|open\-quote|repeat)|ne|rmal|wrap)|w\-resize)|o(?:b(?:ject\-(?:fit|overflow|position|view\-box)|lique)|ffset(?:(?:\-(?:anchor|distance|p(?:ath|osition)|rotate))?)|nce|p(?:acity|en\-quote)|r(?:der|phans)|ut(?:line(?:(?:\-(?:color|offset|style|width))?)|s(?:et|ide))|ver(?:flow(?:(?:\-(?:anchor|block|clip\-margin|inline|wrap|[xy]))?)|line|scroll\-behavior(?:(?:\-(?:block|inline|[xy]))?)))|p(?:a(?:dding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|ge(?:(?:\-break\-(?:after|(?:befor|insid)e))?)|use(?:(?:\-(?:after|before))?))|erspective(?:(?:\-origin)?)|itch(?:(?:\-range)?)|la(?:ce\-(?:content|items|self)|y\-during)|o(?:inter(?:(?:\-events)?)|rtrait|sition)|r(?:int\-color\-adjust|operty\-name)|x)|quotes|r(?:e(?:gion\-fragment|lative|peat(?:(?:\-(?:[xy]))?)|s(?:ize|t(?:(?:\-(?:after|before))?)))|gb|i(?:chness|dge|ght(?:(?:\-side|wards)?))|o(?:tate|w\-gap)|u(?:by\-(?:align|merge|overhang|position)|nning))|s(?:\-resize|ans\-serif|c(?:ale|roll(?:(?:\-(?:behavior|margin(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|padding(?:(?:\-(?:b(?:lock(?:(?:\-(?:end|start))?)|ottom)|inline(?:(?:\-(?:end|start))?)|left|right|top))?)|snap\-(?:align|stop|type))|bar\-(?:color|gutter|width))?))|e(?:\-resize|mi\-(?:(?:condens|expand)ed)|parate|rif)|h(?:ape\-(?:i(?:mage\-threshold|nside)|margin|outside)|ow)|ilent|low(?:(?:er)?)|mall(?:\-cap(?:s|tion)|er)|o(?:ft|lid)|p(?:atial\-navigation\-(?:(?:actio|contai|functio)n)|e(?:ak(?:(?:\-(?:as|header|numeral|punctuation))?)|ech\-rate|ll\-out))|quare|t(?:at(?:ic|us\-bar)|r(?:ess|ing\-set|oke(?:(?:\-(?:align(?:(?:ment)?)|break|color|dash(?:\-(?:corner|justify)|a(?:djust|rray)|corner|offset)|image|line(?:cap|join)|miterlimit|o(?:pacity|rigin)|position|repeat|size|width))?)))|uper|w\-resize)|t(?:ab(?:\-size|le\-(?:c(?:aption|ell|olumn(?:(?:\-group)?))|footer\-group|header\-group|layout|row(?:(?:\-group)?)))|ext(?:(?:\-(?:align(?:(?:\-(?:all|last))?)|bottom|combine\-upright|decoration(?:(?:\-(?:color|line|s(?:kip(?:(?:\-(?:box|in(?:k|set)|s(?:elf|paces)))?)|tyle)|thickness))?)|e(?:dge|mphasis(?:(?:\-(?:color|position|s(?:kip|tyle)))?))|group\-align|indent|justify|o(?:rientation|verflow)|s(?:hadow|pac(?:e\-(?:collapse|trim)|ing))|t(?:op|ransform)|underline\-(?:offset|position)|wrap))?)|hi(?:ck|n)|op|rans(?:form(?:(?:\-(?:box|origin|style))?)|ition(?:(?:\-(?:d(?:elay|uration)|property|timing\-function))?)|late|parent))|u(?:ltra\-(?:(?:condens|expand)ed)|n(?:derline|icode\-bidi)|pper(?:\-(?:alpha|(?:lati|roma)n)|case)|rl|ser\-select)|v(?:ertical\-align|isib(?:ility|le)|o(?:ice\-(?:balance|duration|family|pitch|ra(?:(?:ng|t)e)|stress|volume)|lume))|w(?:\-resize|ait|hite\-space|i(?:d(?:er|ows|th)|ll\-change)|ord\-(?:b(?:oundary\-(?:(?:detect|expans)ion)|reak)|spacing|wrap)|r(?:ap\-(?:after|before|flow|inside|through)|iting\-mode))|x(?:\-(?:fast|high|l(?:arge|o(?:ud|w))|s(?:mall|oft))|x\-(?:large|small))|yes|z\-index)\b", NAME_CONSTANT),
        Rule::token(r"(?ims)(a(?:liceblue|ntiquewhite|qua(?:(?:marine)?)|zure)|b(?:eige|isque|l(?:a(?:ck|nchedalmond)|ue(?:(?:violet)?))|rown|urlywood)|c(?:adetblue|h(?:(?:artreus|ocolat)e)|or(?:al|n(?:flowerblue|silk))|(?:rimso|ya)n)|d(?:ark(?:blue|cyan|g(?:oldenrod|r(?:ay|e(?:en|y)))|khaki|magenta|o(?:livegreen|r(?:ange|chid))|red|s(?:almon|eagreen|late(?:blue|gr(?:(?:[ae])y)))|turquoise|violet)|eep(?:pink|skyblue)|imgr(?:(?:[ae])y)|odgerblue)|f(?:irebrick|loralwhite|orestgreen|uchsia)|g(?:ainsboro|hostwhite|old(?:(?:enrod)?)|r(?:ay|e(?:en(?:(?:yellow)?)|y)))|ho(?:neydew|tpink)|i(?:ndi(?:anred|go)|vory)|khaki|l(?:a(?:vender(?:(?:blush)?)|wngreen)|emonchiffon|i(?:ght(?:blue|c(?:oral|yan)|g(?:oldenrodyellow|r(?:ay|e(?:en|y)))|pink|s(?:almon|eagreen|kyblue|lategr(?:(?:[ae])y)|teelblue)|yellow)|me(?:(?:green)?)|nen))|m(?:a(?:genta|roon)|edium(?:aquamarine|blue|orchid|purple|s(?:eagreen|lateblue|pringgreen)|turquoise|violetred)|i(?:dnightblue|ntcream|styrose)|occasin)|nav(?:ajowhite|y)|o(?:l(?:dlace|ive(?:(?:drab)?))|r(?:ange(?:(?:red)?)|chid))|p(?:a(?:le(?:g(?:oldenrod|reen)|turquoise|violetred)|payawhip)|e(?:achpuff|ru)|ink|lum|(?:owderblu|urpl)e)|r(?:e(?:beccapurple|d)|o(?:sybrown|yalblue))|s(?:a(?:(?:ddlebrow|lmo|ndybrow)n)|ea(?:green|shell)|i(?:enna|lver)|kyblue|late(?:blue|gr(?:(?:[ae])y))|now|pringgreen|teelblue)|t(?:an|eal|histle|omato|ransparent|urquoise)|violet|wh(?:eat|ite(?:(?:smoke)?))|yellow(?:(?:green)?))\b", NAME_ENTITY),
        Rule::token(r"(?ims)(aqua|bl(?:ack|ue)|fuchsia|gr(?:ay|een)|lime|maroon|navy|olive|purple|red|silver|teal|white|yellow)\b", NAME_BUILTIN),
        Rule::token(r"(?ims)\!(important|default)", NAME_EXCEPTION),
        Rule::token(r"(?ims)(true|false)", TokenType::new(&["Name", "Pseudo"])),
        Rule::token(r"(?ims)(and|or|not)", OPERATOR_WORD),
        Rule::token_to(r"(?ims)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"inline-comment"])),
        Rule::token(r"(?ims)//[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?ims)\#[a-z0-9]{1,6}", NUMBER_HEX),
        Rule::bygroups(r"(?ims)(-?\d+)(\%|[a-z]+)?", vec![Some(NUMBER_INTEGER), Some(KEYWORD_TYPE)]),
        Rule::bygroups(r"(?ims)(-?\d*\.\d+)(\%|[a-z]+)?", vec![Some(NUMBER_FLOAT), Some(KEYWORD_TYPE)]),
        Rule::token_to(r"(?ims)#\{", STRING_INTERPOL, NewState::Push(vec![r"interpolation"])),
        Rule::token(r"(?ims)[~^*!&%<>|+=@:,./?-]+", OPERATOR),
        Rule::token(r"(?ims)[\[\]()]+", PUNCTUATION),
        Rule::token_to(r#"(?ims)""#, STRING_DOUBLE, NewState::Push(vec![r"string-double"])),
        Rule::token_to(r"(?ims)'", STRING_SINGLE, NewState::Push(vec![r"string-single"])),
        Rule::token(r"(?ims)[a-z_-][\w-]*", NAME),
        Rule::token(r"(?ims)\n", WHITESPACE),
        Rule::token_to(r"(?ims)[;{}]", PUNCTUATION, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for ScssLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
