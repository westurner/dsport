//! AUTO-GENERATED from `pygments.pygments.lexers.futhark:FutharkLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.futhark:FutharkLexer:futhark

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: futhark
pub struct FutharkLexer;

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
        Rule::token(r"(?m)--(.*?)$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\(\)", PUNCTUATION),
        Rule::token(r"(?m)\b(if|then|else|def|let|loop|in|with|type|type~|type^|val|entry|for|while|do|case|match|include|import|module|open|local|assert|_)(?!\')\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64|bool)(?!\')\b", KEYWORD_TYPE),
        Rule::token(r"(?m)#\[([a-zA-Z_\(\) ]*)\]", COMMENT_PREPROC),
        Rule::token(r"(?m)[#!]?([a-zA-Z_][a-zA-Z_0-9']*\.)*[a-zA-Z_][a-zA-Z_0-9']*", NAME),
        Rule::token(r"(?m)\\", OPERATOR),
        Rule::token(r"(?m)[-+/%=!><|&*^][-+/%=!><|&*^.]*", OPERATOR),
        Rule::token(r"(?m)[\]\[(),:;`{}?.\'~^]", PUNCTUATION),
        Rule::token(r"(?m)0[xX]_*[\da-fA-F](_*[\da-fA-F])*_*[pP][+-]?\d(_*\d)*(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX]_*[\da-fA-F](_*[\da-fA-F])*\.[\da-fA-F](_*[\da-fA-F])*(_*[pP][+-]?\d(_*\d)*)?(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_*\d)*_*[eE][+-]?\d(_*\d)*(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_*\d)*\.\d(_*\d)*(_*[eE][+-]?\d(_*\d)*)?(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[bB]_*[01](_*[01])*(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_BIN),
        Rule::token(r"(?m)0[xX]_*[\da-fA-F](_*[\da-fA-F])*(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_HEX),
        Rule::token(r"(?m)\d(_*\d)*(i8|i16|i32|i64|u8|u16|u32|u64|f32|f64)?", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"character"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\[[a-zA-Z_\d]*\]", KEYWORD_TYPE),
        Rule::token(r"(?m)\(\)", NAME_BUILTIN),
    ]);
    m.insert(
        r"character",
        vec![
            Rule::token_to(r"(?m)[^\\']'", STRING_CHAR, NewState::Pop(1)),
            Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
            Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
            Rule::token_to(r"(?m)\\", STRING_ESCAPE, NewState::Push(vec![r"escape"])),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(r"escape", vec![
        Rule::token_to(r#"(?m)[abfnrtv"\'&\\]"#, STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\^[\]\[A-ZÀ-ÖØ-ÞĀĂĄĆĈĊČĎĐĒĔĖĘĚĜĞĠĢĤĦĨĪĬĮİĲĴĶĹĻĽĿŁŃŅŇŊŌŎŐŒŔŖŘŚŜŞŠŢŤŦŨŪŬŮŰŲŴŶŸ-ŹŻŽƁ-ƂƄƆ-ƇƉ-ƋƎ-ƑƓ-ƔƖ-ƘƜ-ƝƟ-ƠƢƤƦ-ƧƩƬƮ-ƯƱ-ƳƵƷ-ƸƼǄǇǊǍǏǑǓǕǗǙǛǞǠǢǤǦǨǪǬǮǱǴǶ-ǸǺǼǾȀȂȄȆȈȊȌȎȐȒȔȖȘȚȜȞȠȢȤȦȨȪȬȮȰȲȺ-ȻȽ-ȾɁɃ-ɆɈɊɌɎͰͲͶͿΆΈ-ΊΌΎ-ΏΑ-ΡΣ-ΫϏϒ-ϔϘϚϜϞϠϢϤϦϨϪϬϮϴϷϹ-ϺϽ-ЯѠѢѤѦѨѪѬѮѰѲѴѶѸѺѼѾҀҊҌҎҐҒҔҖҘҚҜҞҠҢҤҦҨҪҬҮҰҲҴҶҸҺҼҾӀ-ӁӃӅӇӉӋӍӐӒӔӖӘӚӜӞӠӢӤӦӨӪӬӮӰӲӴӶӸӺӼӾԀԂԄԆԈԊԌԎԐԒԔԖԘԚԜԞԠԢԤԦԨԪԬԮԱ-ՖႠ-ჅჇჍᎠ-ᏵᲐ-ᲺᲽ-ᲿḀḂḄḆḈḊḌḎḐḒḔḖḘḚḜḞḠḢḤḦḨḪḬḮḰḲḴḶḸḺḼḾṀṂṄṆṈṊṌṎṐṒṔṖṘṚṜṞṠṢṤṦṨṪṬṮṰṲṴṶṸṺṼṾẀẂẄẆẈẊẌẎẐẒẔẞẠẢẤẦẨẪẬẮẰẲẴẶẸẺẼẾỀỂỄỆỈỊỌỎỐỒỔỖỘỚỜỞỠỢỤỦỨỪỬỮỰỲỴỶỸỺỼỾἈ-ἏἘ-ἝἨ-ἯἸ-ἿὈ-ὍὙὛὝὟὨ-ὯᾸ-ΆῈ-ΉῘ-ΊῨ-ῬῸ-Ώℂℇℋ-ℍℐ-ℒℕℙ-ℝℤΩℨK-ℭℰ-ℳℾ-ℿⅅↃⰀ-ⰮⱠⱢ-ⱤⱧⱩⱫⱭ-ⱰⱲⱵⱾ-ⲀⲂⲄⲆⲈⲊⲌⲎⲐⲒⲔⲖⲘⲚⲜⲞⲠⲢⲤⲦⲨⲪⲬⲮⲰⲲⲴⲶⲸⲺⲼⲾⳀⳂⳄⳆⳈⳊⳌⳎⳐⳒⳔⳖⳘⳚⳜⳞⳠⳢⳫⳭⳲꙀꙂꙄꙆꙈꙊꙌꙎꙐꙒꙔꙖꙘꙚꙜꙞꙠꙢꙤꙦꙨꙪꙬꚀꚂꚄꚆꚈꚊꚌꚎꚐꚒꚔꚖꚘꚚꜢꜤꜦꜨꜪꜬꜮꜲꜴꜶꜸꜺꜼꜾꝀꝂꝄꝆꝈꝊꝌꝎꝐꝒꝔꝖꝘꝚꝜꝞꝠꝢꝤꝦꝨꝪꝬꝮꝹꝻꝽ-ꝾꞀꞂꞄꞆꞋꞍꞐꞒꞖꞘꞚꞜꞞꞠꞢꞤꞦꞨꞪ-ꞮꞰ-ꞴꞶꞸＡ-Ｚ𐐀-𐐧𐒰-𐓓𐲀-𐲲𑢠-𑢿𖹀-𖹟𝐀-𝐙𝐴-𝑍𝑨-𝒁𝒜𝒞-𝒟𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒵𝓐-𝓩𝔄-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔸-𝔹𝔻-𝔾𝕀-𝕄𝕆𝕊-𝕐𝕬-𝖅𝖠-𝖹𝗔-𝗭𝘈-𝘡𝘼-𝙕𝙰-𝚉𝚨-𝛀𝛢-𝛺𝜜-𝜴𝝖-𝝮𝞐-𝞨𝟊𞤀-𞤡@^_]", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)NUL|SOH|[SE]TX|EOT|ENQ|ACK|BEL|BS|HT|LF|VT|FF|CR|S[OI]|DLE|DC[1-4]|NAK|SYN|ETB|CAN|EM|SUB|ESC|[FGRU]S|SP|DEL", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)o[0-7]+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)x[\da-fA-F]+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::token_to(r"(?m)\d+", STRING_ESCAPE, NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(\s+)(\\)", vec![Some(WHITESPACE), Some(STRING_ESCAPE)], NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for FutharkLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
