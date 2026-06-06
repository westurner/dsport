//! AUTO-GENERATED from `pygments.pygments.lexers.usd:UsdLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.usd:UsdLexer:usd

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: usd, usda
pub struct UsdLexer;

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
        Rule::bygroups(r"(?m)(custom)([ \t]+)(uniform)(\s+)(\w+(?:\[\])?)(\s+)(\w+(?:\:\w+)*)(?:(\.)(timeSamples))?(\s*)(=)", vec![Some(TokenType::new(&["Keyword", "Token"])), Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Token"])), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(TEXT), Some(TokenType::new(&["Name", "Keyword", "Tokens"])), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(custom)([ \t]+)(\w+(?:\[\])?)(\s+)(\w+(?:\:\w+)*)(?:(\.)(timeSamples))?(\s*)(=)", vec![Some(TokenType::new(&["Keyword", "Token"])), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(TEXT), Some(TokenType::new(&["Name", "Keyword", "Tokens"])), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(uniform)([ \t]+)(\w+(?:\[\])?)(\s+)(\w+(?:\:\w+)*)(?:(\.)(timeSamples))?(\s*)(=)", vec![Some(TokenType::new(&["Keyword", "Token"])), Some(WHITESPACE), Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(TEXT), Some(TokenType::new(&["Name", "Keyword", "Tokens"])), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(\w+(?:\[\])?)([ \t]+)(\w+(?:\:\w+)*)(?:(\.)(timeSamples))?(\s*)(=)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_ATTRIBUTE), Some(TEXT), Some(TokenType::new(&["Name", "Keyword", "Tokens"])), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?m)\b(c(?:l(?:(?:as|ip)s)|ustom(?:(?:Data)?))|d(?:ef|ictionary)|inherits|over|payload|re(?:ferences|l)|subLayers|timeSamples|uniform|variant(?:Set(?:(?:s)?)|s))\b", TokenType::new(&["Keyword", "Tokens"])),
        Rule::token(r"(?m)\b(a(?:ctive|piSchemas)|defaultPrim|e(?:(?:lementSiz|ndTimeCod)e)|hidden|in(?:stanceable|terpolation)|kind|startTimeCode|upAxis)\b", TokenType::new(&["Name", "Builtins"])),
        Rule::token(r"(?m)\b(extent|xformOpOrder)\b", NAME_ATTRIBUTE),
        Rule::token(r"(?m)\b\w+:[\w:]+\b", NAME_ATTRIBUTE),
        Rule::token(r"(?m)\b(a(?:(?:d|ppen)d)|delete|prepend|reorder)\b", OPERATOR),
        Rule::token(r"(?m)asset\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)bool\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color3d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color3f\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color3h\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color4d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color4f\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)color4h\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)double\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)double2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)double3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)double4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)float\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)float2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)float3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)float4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)frame4d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)half\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)half2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)half3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)half4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)int\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)int2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)int3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)int4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)keyword\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)matrix2d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)matrix3d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)matrix4d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)normal3d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)normal3f\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)normal3h\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)point3d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)point3f\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)point3h\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)quatd\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)quatf\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)quath\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)string\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)syn\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)token\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uchar\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uchar2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uchar3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uchar4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uint\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uint2\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uint3\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)uint4\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)usdaType\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)vector3d\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)vector3f\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)vector3h\[\]", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(asset|bool|color(?:3(?:[dfh])|4(?:[dfh]))|double(?:(?:[234])?)|f(?:loat(?:(?:[234])?)|rame4d)|half(?:(?:[234])?)|int(?:(?:[234])?)|keyword|matrix(?:(?:[234])d)|normal3(?:[dfh])|point3(?:[dfh])|quat(?:[dfh])|s(?:tring|yn)|token|u(?:char(?:(?:[234])?)|int(?:(?:[234])?)|sdaType)|vector3(?:[dfh]))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[(){}\[\]]", PUNCTUATION),
        Rule::token(r"(?m)#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token(r"(?m)=", OPERATOR),
        Rule::token(r"(?m)[-]*([0-9]*[.])?[0-9]+(?:e[+-]*\d+)?", NUMBER),
        Rule::token(r"(?m)'''(?:.|\n)*?'''", STRING),
        Rule::token(r#"(?m)"""(?:.|\n)*?""""#, STRING),
        Rule::token(r"(?m)'.*?'", STRING),
        Rule::token(r#"(?m)".*?""#, STRING),
        Rule::token(r"(?m)<(\.\./)*([\w/]+|[\w/]+\.\w+[\w:]*)>", NAME_NAMESPACE),
        Rule::token(r"(?m)@.*?@", STRING_INTERPOL),
        Rule::token(r#"(?m)\(.*"[.\\n]*".*\)"#, STRING_DOC),
        Rule::token(r"(?m)\A#usda .+$", COMMENT_HASHBANG),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\w+", TEXT),
        Rule::token(r"(?m)[_:.]+", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for UsdLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
