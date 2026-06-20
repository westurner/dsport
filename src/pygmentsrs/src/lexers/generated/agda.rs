#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.haskell:AgdaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.haskell:AgdaLexer:agda

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: agda
pub struct AgdaLexer;

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
        Rule::bygroups(r"(?m)^(\s*)([^\s(){}]+)(\s*)(:)(\s*)", vec![Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(OPERATOR_WORD), Some(WHITESPACE)]),
        Rule::token(r"(?m)--(?![!#$%&*+./<=>?@^|_~:\\]).*?$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\{!", TokenType::new(&["Comment", "Directive"]), NewState::Push(vec![r"hole"])),
        Rule::token(r"(?m)\b(abstract|codata|coinductive|constructor|data|do|eta-equality|field|forall|hiding|in|inductive|infix|infixl|infixr|instance|interleaved|let|macro|mutual|no-eta-equality|opaque|open|overlap|pattern|postulate|primitive|private|quote|quoteTerm|record|renaming|rewrite|syntax|tactic|unfolding|unquote|unquoteDecl|unquoteDef|using|variable|where|with)(?!\')\b", KEYWORD_RESERVED),
        Rule::bygroups_to(r"(?m)(import|module)(\s+)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE)], NewState::Push(vec![r"module"])),
        Rule::token(r"(?m)\b(Set|Prop)[\u2080-\u2089]*\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(\(|\)|\{|\})", OPERATOR),
        Rule::token(r"(?m)(\.{1,3}|\||\u03BB|\u2200|\u2192|:|=|->)", OPERATOR_WORD),
        Rule::token(r"(?m)\d+[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+\.\d+([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)'", STRING_CHAR, NewState::Push(vec![r"character"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[^\s(){}]+", TEXT),
        Rule::token(r"(?m)\s+?", WHITESPACE),
    ]);
    m.insert(
        r"hole",
        vec![
            Rule::token(r"(?m)[^!{}]+", TokenType::new(&["Comment", "Directive"])),
            Rule::token_to(
                r"(?m)\{!",
                TokenType::new(&["Comment", "Directive"]),
                NewState::PushSame,
            ),
            Rule::token_to(
                r"(?m)!\}",
                TokenType::new(&["Comment", "Directive"]),
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[!{}]", TokenType::new(&["Comment", "Directive"])),
        ],
    );
    m.insert(
        r"module",
        vec![
            Rule::token_to(
                r"(?m)\{-",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token_to(r"(?m)[a-zA-Z][\w.\']*", NAME, NewState::Pop(1)),
            Rule::token(r"(?m)[\W0-9_]+", TEXT),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^-{}]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)-\}", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[-{}]", COMMENT_MULTILINE),
        ],
    );
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

impl Lexer for AgdaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
