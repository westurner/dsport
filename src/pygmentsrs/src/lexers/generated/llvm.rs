#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.asm:LlvmLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.asm:LlvmLexer:llvm

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: llvm
pub struct LlvmLexer;

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
        Rule::token(r"(?m)(\n|\s+)+", WHITESPACE),
        Rule::token(r"(?m);.*?\n", COMMENT),
        Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"c-comment"])),
        Rule::token(r#"(?m)(([-a-zA-Z$._][\w\-$.]*|"[^"]*?")|(\d+))\s*:"#, NAME_LABEL),
        Rule::token(r"(?m)(a(?:arch64_(?:(?:(?:sve_)?)vector_pcs)|cq(?:_rel|uire)|dd(?:(?:rspace(?:(?:cast)?))?)|fn|l(?:i(?:as(?:(?:ee)?)|gn(?:(?:Log2|stack)?))|l(?:Ones|oc(?:a|size))|ways(?:(?:[Ii])nline))|mdgpu_(?:cs|es|g(?:fx|s)|hs|kernel|(?:[lpv])s)|n(?:yregcc|[dy])|ppending|r(?:cp|g(?:memonly|s)|m_a(?:(?:apcs(?:(?:_vfp)?)|pcs)cc))|s(?:hr|m)|t(?:omic(?:(?:rmw)?)|tributes)|v(?:ailable_externally|r_(?:(?:intr|signal)cc)))|b(?:it(?:(?:Mask|cast)?)|lock(?:address|count)|r(?:(?:anchFunnel)?)|uiltin|y(?:Arg|ref|te(?:(?:Array)?)|val))|c(?:a(?:ll(?:(?:br|e(?:[er])|s)?)|nAutoHide|tch(?:(?:pad|ret|switch)?))|c(?:(?:c)?)|fguard_checkcc|leanup(?:(?:pad|ret)?)|mpxchg|o(?:ld(?:(?:cc)?)|m(?:dat|mon)|n(?:(?:stan|trac|vergen)t))|ritical|xx_fast_tlscc)|d(?:atalayout|e(?:clare|f(?:ault|ine)|plibs|referenceable(?:(?:_or_null)?))|istinct|ll(?:(?:ex|im)port)|so(?:Local|_(?:local(?:(?:_equivalent)?)|preemptable)))|e(?:q|x(?:act(?:(?:match)?)|t(?:ern(?:_weak|al(?:(?:ly_initialized)?))|ract(?:element|value))))|f(?:a(?:dd|lse|st(?:(?:cc)?))|cmp|div|ence|ilter|lags|mul|neg|p(?:ext|t(?:o(?:(?:[su])i)|runc))|r(?:e(?:eze|m)|om)|sub|unc(?:Flags|tion))|g(?:etelementptr|hccc|lobal|uid|[cv])|h(?:ash|hvm(?:(?:(?:_c)?)cc)|idden|ot(?:(?:ness)?))|i(?:cmp|func|mmarg|n(?:a(?:ccessiblemem(?:(?:(?:_or_argmem)?)only)|lloca)|bounds|dir(?:(?:ectbr)?)|fo|itialexec|line(?:(?:Bits|hint)?)|r(?:ange|eg)|s(?:ert(?:element|value)|ts)|t(?:e(?:l(?:_ocl_bicc|dialect)|rnal)|toptr)|voke))|jumptable|kind|l(?:a(?:ndingpad|rgest)|i(?:nk(?:age|once(?:(?:_odr)?))|ve)|o(?:ad|cal(?:_unnamed_addr|(?:dynami|exe)c))|shr)|m(?:ax|etadata|in(?:(?:size)?)|o(?:dule|notonic)|sp430_intrcc|u(?:l|st(?:progress|tail)))|n(?:a(?:ked|me|nd)|e(?:(?:st)?)|inf|nan|o(?:Inline|Recurse|alias|builtin|c(?:a(?:llback|pture)|f_check)|duplicate(?:(?:s)?)|free|i(?:mplicitfloat|nline)|merge|n(?:e|lazybind|null)|profile|re(?:curse|dzone|turn)|sync|t(?:EligibleToImport|ail)|un(?:def|wind))|s(?:[wz])|u(?:ll(?:(?:_pointer_is_valid)?)|w))|o(?:eq|ffset|g(?:[et])|l(?:[et])|ne|p(?:aque|t(?:forfuzzing|(?:non|siz)e))|r(?:(?:d)?))|p(?:a(?:r(?:am(?:(?:s)?)|tition)|th)|ersonality|hi|oison|r(?:e(?:allocated|fix|serve_(?:(?:all|most)cc))|ivate|o(?:logue|tected))|t(?:rtoint|x_(?:device|kernel)))|re(?:a(?:d(?:None|Only|none|only)|ssoc)|fs|l(?:bf|ease)|s(?:ByArg|ume)|t(?:(?:urn(?:DoesNotAlias|ed|s_twice))?))|s(?:a(?:festack|mesize|nitize_(?:address|hwaddress|mem(?:ory|tag)|thread))|div|e(?:ction|(?:lec|q_cs|x)t)|g(?:[et])|h(?:adowcallstack|l|ufflevector)|i(?:deeffect|gnext|ngle(?:(?:Impl(?:(?:Name)?))?)|tofp|zeM1(?:(?:BitWidth)?))|l(?:[et])|ource_filename|p(?:eculat(?:able|ive_load_hardening)|ir_(?:func|kernel)|lat)|re(?:[mt])|sp(?:(?:req|strong)?)|t(?:ore|rictfp)|u(?:b|mmar(?:ies|y))|wi(?:ft(?:cc|error|self)|tch)|yncscope)|t(?:a(?:il(?:(?:cc)?)|rget)|hread_local|o(?:(?:ken)?)|r(?:iple|u(?:e|nc))|ype(?:(?:CheckedLoad(?:(?:(?:Const)?)VCalls)|IdInfo|Test(?:(?:(?:Assume(?:(?:(?:Const)?)VCall)|Re)?)s)|id(?:(?:CompatibleVTable)?))?))|u(?:div|eq|g(?:[et])|itofp|l(?:[et])|m(?:ax|in)|n(?:def|i(?:(?:form|que)RetVal)|known|named_addr|ordered|reachable|sat|wind|[eo])|rem|selistorder(?:(?:_bb)?)|wtable)|v(?:FuncId|TableFuncs|a(?:_arg|r(?:Flags|iable))|call_visibility|irt(?:Func|ualConstProp)|o(?:id|latile)|scale)|w(?:e(?:ak(?:(?:_odr)?)|bkit_jscc)|i(?:llreturn|n64cc|thin)|pdRes(?:(?:olutions)?)|riteonly)|x(?:86_(?:64_sysvcc|fastcallcc|intrcc|mmx|(?:reg|std|this|vector)callcc)|chg|or)|ze(?:ro(?:ext|initializer)|xt)|[cx])\b", KEYWORD),
        Rule::token(r"(?m)(bfloat|double|f(?:loat|p128)|half|label|metadata|p(?:pc_fp128|tr)|token|void|x86_(?:amx|fp80|mmx))", KEYWORD_TYPE),
        Rule::token(r"(?m)i[1-9]\d*", KEYWORD_TYPE),
        Rule::token(r#"(?m)%([-a-zA-Z$._][\w\-$.]*|"[^"]*?")"#, NAME_VARIABLE),
        Rule::token(r#"(?m)@([-a-zA-Z$._][\w\-$.]*|"[^"]*?")"#, NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)%\d+", TokenType::new(&["Name", "Variable", "Anonymous"])),
        Rule::token(r"(?m)@\d+", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)#\d+", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)!([-a-zA-Z$._][\w\-$.]*|"[^"]*?")"#, NAME_VARIABLE),
        Rule::token(r"(?m)!\d+", TokenType::new(&["Name", "Variable", "Anonymous"])),
        Rule::token(r#"(?m)c?"[^"]*?""#, STRING),
        Rule::token(r"(?m)0[xX][KLMHR]?[a-fA-F0-9]+", NUMBER),
        Rule::token(r"(?m)-?\d+(?:[.]\d+)?(?:[eE][-+]?\d+(?:[.]\d+)?)?", NUMBER),
        Rule::token(r"(?m)[=<>{}\[\]()*.,!]|x\b", PUNCTUATION),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)(\n|\s+)+", WHITESPACE),
            Rule::token(r"(?m);.*?\n", COMMENT),
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"c-comment"])),
        ],
    );
    m.insert(r"keyword", vec![
        Rule::token(r"(?m)(a(?:arch64_(?:(?:(?:sve_)?)vector_pcs)|cq(?:_rel|uire)|dd(?:(?:rspace(?:(?:cast)?))?)|fn|l(?:i(?:as(?:(?:ee)?)|gn(?:(?:Log2|stack)?))|l(?:Ones|oc(?:a|size))|ways(?:(?:[Ii])nline))|mdgpu_(?:cs|es|g(?:fx|s)|hs|kernel|(?:[lpv])s)|n(?:yregcc|[dy])|ppending|r(?:cp|g(?:memonly|s)|m_a(?:(?:apcs(?:(?:_vfp)?)|pcs)cc))|s(?:hr|m)|t(?:omic(?:(?:rmw)?)|tributes)|v(?:ailable_externally|r_(?:(?:intr|signal)cc)))|b(?:it(?:(?:Mask|cast)?)|lock(?:address|count)|r(?:(?:anchFunnel)?)|uiltin|y(?:Arg|ref|te(?:(?:Array)?)|val))|c(?:a(?:ll(?:(?:br|e(?:[er])|s)?)|nAutoHide|tch(?:(?:pad|ret|switch)?))|c(?:(?:c)?)|fguard_checkcc|leanup(?:(?:pad|ret)?)|mpxchg|o(?:ld(?:(?:cc)?)|m(?:dat|mon)|n(?:(?:stan|trac|vergen)t))|ritical|xx_fast_tlscc)|d(?:atalayout|e(?:clare|f(?:ault|ine)|plibs|referenceable(?:(?:_or_null)?))|istinct|ll(?:(?:ex|im)port)|so(?:Local|_(?:local(?:(?:_equivalent)?)|preemptable)))|e(?:q|x(?:act(?:(?:match)?)|t(?:ern(?:_weak|al(?:(?:ly_initialized)?))|ract(?:element|value))))|f(?:a(?:dd|lse|st(?:(?:cc)?))|cmp|div|ence|ilter|lags|mul|neg|p(?:ext|t(?:o(?:(?:[su])i)|runc))|r(?:e(?:eze|m)|om)|sub|unc(?:Flags|tion))|g(?:etelementptr|hccc|lobal|uid|[cv])|h(?:ash|hvm(?:(?:(?:_c)?)cc)|idden|ot(?:(?:ness)?))|i(?:cmp|func|mmarg|n(?:a(?:ccessiblemem(?:(?:(?:_or_argmem)?)only)|lloca)|bounds|dir(?:(?:ectbr)?)|fo|itialexec|line(?:(?:Bits|hint)?)|r(?:ange|eg)|s(?:ert(?:element|value)|ts)|t(?:e(?:l(?:_ocl_bicc|dialect)|rnal)|toptr)|voke))|jumptable|kind|l(?:a(?:ndingpad|rgest)|i(?:nk(?:age|once(?:(?:_odr)?))|ve)|o(?:ad|cal(?:_unnamed_addr|(?:dynami|exe)c))|shr)|m(?:ax|etadata|in(?:(?:size)?)|o(?:dule|notonic)|sp430_intrcc|u(?:l|st(?:progress|tail)))|n(?:a(?:ked|me|nd)|e(?:(?:st)?)|inf|nan|o(?:Inline|Recurse|alias|builtin|c(?:a(?:llback|pture)|f_check)|duplicate(?:(?:s)?)|free|i(?:mplicitfloat|nline)|merge|n(?:e|lazybind|null)|profile|re(?:curse|dzone|turn)|sync|t(?:EligibleToImport|ail)|un(?:def|wind))|s(?:[wz])|u(?:ll(?:(?:_pointer_is_valid)?)|w))|o(?:eq|ffset|g(?:[et])|l(?:[et])|ne|p(?:aque|t(?:forfuzzing|(?:non|siz)e))|r(?:(?:d)?))|p(?:a(?:r(?:am(?:(?:s)?)|tition)|th)|ersonality|hi|oison|r(?:e(?:allocated|fix|serve_(?:(?:all|most)cc))|ivate|o(?:logue|tected))|t(?:rtoint|x_(?:device|kernel)))|re(?:a(?:d(?:None|Only|none|only)|ssoc)|fs|l(?:bf|ease)|s(?:ByArg|ume)|t(?:(?:urn(?:DoesNotAlias|ed|s_twice))?))|s(?:a(?:festack|mesize|nitize_(?:address|hwaddress|mem(?:ory|tag)|thread))|div|e(?:ction|(?:lec|q_cs|x)t)|g(?:[et])|h(?:adowcallstack|l|ufflevector)|i(?:deeffect|gnext|ngle(?:(?:Impl(?:(?:Name)?))?)|tofp|zeM1(?:(?:BitWidth)?))|l(?:[et])|ource_filename|p(?:eculat(?:able|ive_load_hardening)|ir_(?:func|kernel)|lat)|re(?:[mt])|sp(?:(?:req|strong)?)|t(?:ore|rictfp)|u(?:b|mmar(?:ies|y))|wi(?:ft(?:cc|error|self)|tch)|yncscope)|t(?:a(?:il(?:(?:cc)?)|rget)|hread_local|o(?:(?:ken)?)|r(?:iple|u(?:e|nc))|ype(?:(?:CheckedLoad(?:(?:(?:Const)?)VCalls)|IdInfo|Test(?:(?:(?:Assume(?:(?:(?:Const)?)VCall)|Re)?)s)|id(?:(?:CompatibleVTable)?))?))|u(?:div|eq|g(?:[et])|itofp|l(?:[et])|m(?:ax|in)|n(?:def|i(?:(?:form|que)RetVal)|known|named_addr|ordered|reachable|sat|wind|[eo])|rem|selistorder(?:(?:_bb)?)|wtable)|v(?:FuncId|TableFuncs|a(?:_arg|r(?:Flags|iable))|call_visibility|irt(?:Func|ualConstProp)|o(?:id|latile)|scale)|w(?:e(?:ak(?:(?:_odr)?)|bkit_jscc)|i(?:llreturn|n64cc|thin)|pdRes(?:(?:olutions)?)|riteonly)|x(?:86_(?:64_sysvcc|fastcallcc|intrcc|mmx|(?:reg|std|this|vector)callcc)|chg|or)|ze(?:ro(?:ext|initializer)|xt)|[cx])\b", KEYWORD),
        Rule::token(r"(?m)(bfloat|double|f(?:loat|p128)|half|label|metadata|p(?:pc_fp128|tr)|token|void|x86_(?:amx|fp80|mmx))", KEYWORD_TYPE),
        Rule::token(r"(?m)i[1-9]\d*", KEYWORD_TYPE),
    ]);
    m.insert(
        r"c-comment",
        vec![
            Rule::token(r"(?m)[^*]+", COMMENT),
            Rule::token_to(r"(?m)\*/", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)\*", COMMENT),
        ],
    );
    Table(m)
}

impl Lexer for LlvmLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
