//! AUTO-GENERATED from `pygments.pygments.lexers.asm:HsailLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:HsailLexer:hsail

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hsail, hsa
pub struct HsailLexer;

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
        Rule::token(r"(?m)(\n|\s)+", WHITESPACE),
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
        Rule::token(r#"(?m)"[^"]*?""#, STRING),
        Rule::token(r"(?m)@[a-zA-Z_][\w.]*:?", NAME_LABEL),
        Rule::token(r"(?m)(\$(c|s|d|q)[0-9]+)\b", TokenType::new(&["Name", "Variable", "Anonymous"])),
        Rule::token(r"(?m)kernarg_(u8x4|s8x4|u16x2|s16x2|u8x8|s8x8|u16x4|s16x4|u32x2|s32x2|u8x16|s8x16|u16x8|s16x8|u32x4|s32x4|u64x2|s64x2|f16x2|f16x4|f16x8|f32x2|f32x4|f64x2|u8|s8|u16|s16|u32|s32|u64|s64|b128|b8|b16|b32|b64|b1|f16|f32|f64|roimg|woimg|rwimg|samp|sig32|sig64)", KEYWORD_TYPE),
        Rule::token(r"(?m)\$(full|base|small|large|default|zero|near)", KEYWORD),
        Rule::token(r"(?m)(decl|e(?:nable(?:(?:break|detect)exceptions)|xtension)|function|indirect|kernel|m(?:(?:ax(?:(?:dynamicgroup|flat(?:grid|workgroup))siz)|odul)e)|pr(?:agma|og)|require(?:d(?:dim|(?:grid|workgroup)size)|nopartialworkgroups)|signature)\b", KEYWORD),
        Rule::token(r"(?m)((_ftz)?(_up|_down|_zero|_near))", KEYWORD),
        Rule::token(r"(?m)_(u8x4|s8x4|u16x2|s16x2|u8x8|s8x8|u16x4|s16x4|u32x2|s32x2|u8x16|s8x16|u16x8|s16x8|u32x4|s32x4|u64x2|s64x2|f16x2|f16x4|f16x8|f32x2|f32x4|f64x2|u8|s8|u16|s16|u32|s32|u64|s64|b128|b8|b16|b32|b64|b1|f16|f32|f64|roimg|woimg|rwimg|samp|sig32|sig64)", KEYWORD),
        Rule::token(r"(?m)_((align\(\d+\))|(width\((\d+|all)\)))", KEYWORD),
        Rule::token(r"(?m)_kernarg", KEYWORD),
        Rule::token(r"(?m)(nop|imagefence)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:1d(?:(?:[ab])?)|2d(?:(?:a(?:(?:depth)?)|depth)?)|3d|a(?:dd(?:(?:ressing)?)|gent|nd|rray)|c(?:as|hannel(?:order|type)|oord)|d(?:epth|owni(?:(?:_sat)?))|e(?:q(?:(?:u)?)|xch)|filter|g(?:eu|lobal|tu|[et])|height|kernarg|l(?:(?:[et])u|[det])|m(?:ax|in)|n(?:an|e(?:(?:ari(?:(?:_sat)?)|u)?)|um)|or|p(?:_sat|(?:[ps])_sat|[ps])|r(?:eadonly|lx)|s(?:_sat|c(?:a(?:cq|r)|rel)|downi(?:(?:_sat)?)|eq(?:(?:u)?)|g(?:(?:[et])u|[et])|l(?:(?:[et])u|[et])|n(?:an|e(?:(?:ari(?:(?:_sat)?)|u)?)|um)|p_sat|s_sat|u(?:b|pi(?:(?:_sat)?))|ystem|zeroi(?:(?:_sat)?)|[pst])|upi(?:(?:_sat)?)|v(?:[234])|w(?:ave|g|idth|rap(?:(?:de|in)c))|xor|zeroi(?:(?:_sat)?)|[ps])|a(?:bs|ctivelane(?:count|id|mask|permute)|dd(?:(?:queuewriteindex)?)|lloca|nd|r(?:g|rivefbar)|tomic)|b(?:arrier|it(?:align|extract|insert|mask|rev|select)|orrow|r|ytealign)|c(?:a(?:ll|rry|squeuewriteindex)|br|eil|l(?:eardetectexcept|ock)|m(?:ov|p)|o(?:mbine|pysign)|u(?:id|rrentwork(?:groupsize|itemflatid))|vt)|d(?:ebugtrap|i(?:[mv]))|expand|f(?:loor|ma|ract|tos)|g(?:etdetectexcept|lobal|r(?:id(?:groups|size)|oup(?:(?:baseptr)?)))|i(?:call|nitfbar)|joinfbar|kernargbaseptr|l(?:aneid|d(?:(?:image|queuereadindex|[af])?)|e(?:avefbar|rp))|m(?:a(?:d24(?:(?:hi)?)|x(?:(?:cu|wave)id)|[dx])|in|ov|ul(?:(?:24(?:(?:hi)?)|hi)?))|n(?:cos|e(?:g|xp2)|fma|log2|ot|r(?:cp|sqrt)|s(?:in|qrt)|ullptr)|or|p(?:ack(?:(?:cvt|et(?:completionsig|id))?)|rivate)|querysampler|r(?:e(?:adonly|leasefbar|[mt])|int)|s(?:ad|br|call|e(?:mentp|tdetectexcept)|h(?:uffle|[lr])|pill|qrt|t(?:(?:image|of|queuereadindex)?)|ub)|trunc|unpack(?:(?:cvt|hi|lo)?)|w(?:a(?:itfbar|ve(?:barrier|id))|ork(?:group(?:id|size)|item(?:(?:(?:abs|flat(?:(?:abs)?))?)id)))|xor)", KEYWORD),
        Rule::token(r"(?m)i[1-9]\d*", KEYWORD),
        Rule::token(r"(?m)&[a-zA-Z_][\w.]*", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)%[a-zA-Z_][\w.]*", NAME_VARIABLE),
        Rule::token(r"(?m)0[xX](([0-9a-fA-F]+\.[0-9a-fA-F]*)|([0-9a-fA-F]*\.[0-9a-fA-F]+))[pP][+-]?\d+", NUMBER_HEX),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)0((h|H)[0-9a-fA-F]{4}|(f|F)[0-9a-fA-F]{8}|(d|D)[0-9a-fA-F]{16})", NUMBER_FLOAT),
        Rule::token(r"(?m)((\d+\.)|(\d*\.\d+))[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[=<>{}\[\]()*.,:;!]|x\b", PUNCTUATION),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)(\n|\s)+", WHITESPACE),
    ]);
    m.insert(r"comments", vec![
        Rule::token(r"(?m)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?m)//.*?\n", COMMENT_SINGLE),
    ]);
    m.insert(r"keyword", vec![
        Rule::token(r"(?m)kernarg_(u8x4|s8x4|u16x2|s16x2|u8x8|s8x8|u16x4|s16x4|u32x2|s32x2|u8x16|s8x16|u16x8|s16x8|u32x4|s32x4|u64x2|s64x2|f16x2|f16x4|f16x8|f32x2|f32x4|f64x2|u8|s8|u16|s16|u32|s32|u64|s64|b128|b8|b16|b32|b64|b1|f16|f32|f64|roimg|woimg|rwimg|samp|sig32|sig64)", KEYWORD_TYPE),
        Rule::token(r"(?m)\$(full|base|small|large|default|zero|near)", KEYWORD),
        Rule::token(r"(?m)(decl|e(?:nable(?:(?:break|detect)exceptions)|xtension)|function|indirect|kernel|m(?:(?:ax(?:(?:dynamicgroup|flat(?:grid|workgroup))siz)|odul)e)|pr(?:agma|og)|require(?:d(?:dim|(?:grid|workgroup)size)|nopartialworkgroups)|signature)\b", KEYWORD),
        Rule::token(r"(?m)((_ftz)?(_up|_down|_zero|_near))", KEYWORD),
        Rule::token(r"(?m)_(u8x4|s8x4|u16x2|s16x2|u8x8|s8x8|u16x4|s16x4|u32x2|s32x2|u8x16|s8x16|u16x8|s16x8|u32x4|s32x4|u64x2|s64x2|f16x2|f16x4|f16x8|f32x2|f32x4|f64x2|u8|s8|u16|s16|u32|s32|u64|s64|b128|b8|b16|b32|b64|b1|f16|f32|f64|roimg|woimg|rwimg|samp|sig32|sig64)", KEYWORD),
        Rule::token(r"(?m)_((align\(\d+\))|(width\((\d+|all)\)))", KEYWORD),
        Rule::token(r"(?m)_kernarg", KEYWORD),
        Rule::token(r"(?m)(nop|imagefence)\b", KEYWORD),
        Rule::token(r"(?m)(_(?:1d(?:(?:[ab])?)|2d(?:(?:a(?:(?:depth)?)|depth)?)|3d|a(?:dd(?:(?:ressing)?)|gent|nd|rray)|c(?:as|hannel(?:order|type)|oord)|d(?:epth|owni(?:(?:_sat)?))|e(?:q(?:(?:u)?)|xch)|filter|g(?:eu|lobal|tu|[et])|height|kernarg|l(?:(?:[et])u|[det])|m(?:ax|in)|n(?:an|e(?:(?:ari(?:(?:_sat)?)|u)?)|um)|or|p(?:_sat|(?:[ps])_sat|[ps])|r(?:eadonly|lx)|s(?:_sat|c(?:a(?:cq|r)|rel)|downi(?:(?:_sat)?)|eq(?:(?:u)?)|g(?:(?:[et])u|[et])|l(?:(?:[et])u|[et])|n(?:an|e(?:(?:ari(?:(?:_sat)?)|u)?)|um)|p_sat|s_sat|u(?:b|pi(?:(?:_sat)?))|ystem|zeroi(?:(?:_sat)?)|[pst])|upi(?:(?:_sat)?)|v(?:[234])|w(?:ave|g|idth|rap(?:(?:de|in)c))|xor|zeroi(?:(?:_sat)?)|[ps])|a(?:bs|ctivelane(?:count|id|mask|permute)|dd(?:(?:queuewriteindex)?)|lloca|nd|r(?:g|rivefbar)|tomic)|b(?:arrier|it(?:align|extract|insert|mask|rev|select)|orrow|r|ytealign)|c(?:a(?:ll|rry|squeuewriteindex)|br|eil|l(?:eardetectexcept|ock)|m(?:ov|p)|o(?:mbine|pysign)|u(?:id|rrentwork(?:groupsize|itemflatid))|vt)|d(?:ebugtrap|i(?:[mv]))|expand|f(?:loor|ma|ract|tos)|g(?:etdetectexcept|lobal|r(?:id(?:groups|size)|oup(?:(?:baseptr)?)))|i(?:call|nitfbar)|joinfbar|kernargbaseptr|l(?:aneid|d(?:(?:image|queuereadindex|[af])?)|e(?:avefbar|rp))|m(?:a(?:d24(?:(?:hi)?)|x(?:(?:cu|wave)id)|[dx])|in|ov|ul(?:(?:24(?:(?:hi)?)|hi)?))|n(?:cos|e(?:g|xp2)|fma|log2|ot|r(?:cp|sqrt)|s(?:in|qrt)|ullptr)|or|p(?:ack(?:(?:cvt|et(?:completionsig|id))?)|rivate)|querysampler|r(?:e(?:adonly|leasefbar|[mt])|int)|s(?:ad|br|call|e(?:mentp|tdetectexcept)|h(?:uffle|[lr])|pill|qrt|t(?:(?:image|of|queuereadindex)?)|ub)|trunc|unpack(?:(?:cvt|hi|lo)?)|w(?:a(?:itfbar|ve(?:barrier|id))|ork(?:group(?:id|size)|item(?:(?:(?:abs|flat(?:(?:abs)?))?)id)))|xor)", KEYWORD),
        Rule::token(r"(?m)i[1-9]\d*", KEYWORD),
    ]);
    Table(m)
}

impl Lexer for HsailLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
