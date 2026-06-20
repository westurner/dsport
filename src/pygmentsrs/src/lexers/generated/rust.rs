//! AUTO-GENERATED from `pygments.pygments.lexers.rust:RustLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.rust:RustLexer:rust

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: rust, rs
pub struct RustLexer;

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
            Rule::token(r"(?m)#![^\[\r\n].*$", COMMENT_PREPROC),
            Rule::default(NewState::Push(vec![r"base"])),
        ],
    );
    m.insert(r"base", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//!.*?\n", STRING_DOC),
        Rule::token(r"(?m)///(\n|[^/].*?\n)", STRING_DOC),
        Rule::token(r"(?m)//(.*?)\n", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*\*(\n|[^/*])", STRING_DOC, NewState::Push(vec![r"doccomment"])),
        Rule::token_to(r"(?m)/\*!", STRING_DOC, NewState::Push(vec![r"doccomment"])),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\$([a-zA-Z_]\w*|\(,?|\),?|,?)", COMMENT_PREPROC),
        Rule::token(r"(?m)(a(?:s(?:(?:ync)?)|wait)|box|c(?:onst|rate)|dyn|e(?:lse|xtern)|for|i(?:mpl|[fn])|loop|m(?:atch|ove|ut)|pub|re(?:f|turn)|s(?:tatic|uper)|trait|(?:u(?:nsaf|s)|wh(?:er|il))e)\b", KEYWORD),
        Rule::token(r"(?m)(abstract|become|do|final|macro|override|priv|t(?:ry|ypeof)|unsized|virtual|yield)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(true|false)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)self\b", NAME_BUILTIN_PSEUDO),
        Rule::token_to(r"(?m)mod\b", KEYWORD, NewState::Push(vec![r"modname"])),
        Rule::token(r"(?m)let\b", KEYWORD_DECLARATION),
        Rule::token_to(r"(?m)fn\b", KEYWORD, NewState::Push(vec![r"funcname"])),
        Rule::token_to(r"(?m)(struct|enum|type|union)\b", KEYWORD, NewState::Push(vec![r"typename"])),
        Rule::bygroups(r"(?m)(default)(\s+)(type|fn)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?m)(bool|char|f(?:32|64)|i(?:1(?:28|6)|32|64|8|size)|str|u(?:1(?:28|6)|32|64|8|size))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[sS]elf\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(As(?:Mut|Ref)|Box|C(?:lone|opy)|D(?:efault|oubleEndedIterator|rop)|E(?:q|rr|x(?:actSizeIterator|tend))|F(?:n(?:(?:Mut|Once)?)|rom)|I(?:nto(?:(?:Iterator)?)|terator)|None|O(?:k|ption|rd)|Partial(?:Eq|Ord)|Result|S(?:end|ized|ome|tring|ync)|To(?:Owned|String)|Unpin|Vec|drop)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(as(?:m|sert(?:(?:_(?:eq|ne))?))|c(?:fg|o(?:lumn|mpile_error|ncat(?:(?:_idents)?)))|d(?:bg|ebug_assert(?:(?:_(?:eq|ne))?))|e(?:nv|print(?:(?:ln)?))|f(?:ile|ormat(?:(?:_args(?:(?:_nl)?))?))|global_asm|i(?:nclude(?:(?:_(?:bytes|str))?)|s_(?:(?:a(?:arch64|rm)|mips(?:(?:64)?)|powerpc(?:(?:64)?)|x86)_feature_detected))|l(?:ine|lvm_asm|og_syntax)|m(?:a(?:(?:cro_rul|tch)es)|odule_path)|option_env|p(?:anic|rint(?:(?:ln)?))|stringify|t(?:hread_local|odo|race_macros)|un(?:implemented|reachable)|vec|write(?:(?:ln)?))!", NAME_FUNCTION_MAGIC),
        Rule::token(r"(?m)::\b", PUNCTUATION),
        Rule::token_to(r"(?m)(?::|->)", PUNCTUATION, NewState::Push(vec![r"typename"])),
        Rule::bygroups(r"(?m)(break|continue)(\b\s*)(\'[A-Za-z_]\w*)?", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r#"(?m)'(\\['"\\nrt]|\\x[0-7][0-9a-fA-F]|\\0|\\u\{[0-9a-fA-F]{1,6}\}|.)'"#, STRING_CHAR),
        Rule::token(r#"(?m)b'(\\['"\\nrt]|\\x[0-9a-fA-F]{2}|\\0|\\u\{[0-9a-fA-F]{1,6}\}|.)'"#, STRING_CHAR),
        Rule::token_to(r"(?m)0b[01_]+", NUMBER_BIN, NewState::Push(vec![r"number_lit"])),
        Rule::token_to(r"(?m)0o[0-7_]+", NUMBER_OCT, NewState::Push(vec![r"number_lit"])),
        Rule::token_to(r"(?m)0[xX][0-9a-fA-F_]+", NUMBER_HEX, NewState::Push(vec![r"number_lit"])),
        Rule::token_to(r"(?m)[0-9][0-9_]*(\.[0-9_]+[eE][+\-]?[0-9_]+|\.[0-9_]*(?!\.)|[eE][+\-]?[0-9_]+)", NUMBER_FLOAT, NewState::Push(vec![r"number_lit"])),
        Rule::token_to(r"(?m)[0-9][0-9_]*", NUMBER_INTEGER, NewState::Push(vec![r"number_lit"])),
        Rule::token_to(r#"(?m)b""#, STRING, NewState::Push(vec![r"bytestring"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r#"(?m)(?s)b?r(#*)".*?"\1"#, STRING),
        Rule::token_to(r"(?m)'", OPERATOR, NewState::Push(vec![r"lifetime"])),
        Rule::token(r"(?m)\.\.=?", OPERATOR),
        Rule::token(r"(?m)[{}()\[\],.;]", PUNCTUATION),
        Rule::token(r"(?m)[+\-*/%&|<>^!~@=:?]", OPERATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)r#[a-zA-Z_]\w*", NAME),
        Rule::token_to(r"(?m)#!?\[", COMMENT_PREPROC, NewState::Push(vec![r"attribute["])),
        Rule::token(r"(?m)#", PUNCTUATION),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"doccomment",
        vec![
            Rule::token(r"(?m)[^*/]+", STRING_DOC),
            Rule::token_to(r"(?m)/\*", STRING_DOC, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", STRING_DOC, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", STRING_DOC),
        ],
    );
    m.insert(
        r"modname",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_NAMESPACE, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"funcname",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(r"typename", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)&", KEYWORD_PSEUDO),
        Rule::token_to(r"(?m)'", OPERATOR, NewState::Push(vec![r"lifetime"])),
        Rule::token(r"(?m)(As(?:Mut|Ref)|Box|C(?:lone|opy)|D(?:efault|oubleEndedIterator|rop)|E(?:q|rr|x(?:actSizeIterator|tend))|F(?:n(?:(?:Mut|Once)?)|rom)|I(?:nto(?:(?:Iterator)?)|terator)|None|O(?:k|ption|rd)|Partial(?:Eq|Ord)|Result|S(?:end|ized|ome|tring|ync)|To(?:Owned|String)|Unpin|Vec|drop)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(bool|char|f(?:32|64)|i(?:1(?:28|6)|32|64|8|size)|str|u(?:1(?:28|6)|32|64|8|size))\b", KEYWORD_TYPE),
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"lifetime",
        vec![
            Rule::token(r"(?m)(static|_)", NAME_BUILTIN),
            Rule::token(r"(?m)[a-zA-Z_]+\w*", NAME_ATTRIBUTE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"number_lit",
        vec![
            Rule::token_to(r"(?m)[ui](8|16|32|64|size)", KEYWORD, NewState::Pop(1)),
            Rule::token_to(r"(?m)f(32|64)", KEYWORD, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\['"\\nrt]|\\x[0-7][0-9a-fA-F]|\\0|\\u\{[0-9a-fA-F]{1,6}\}"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    m.insert(
        r"bytestring",
        vec![
            Rule::token(r"(?m)\\x[89a-fA-F][0-9a-fA-F]", STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\['"\\nrt]|\\x[0-7][0-9a-fA-F]|\\0|\\u\{[0-9a-fA-F]{1,6}\}"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?m)[^\\"]+"#, STRING),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    m.insert(
        r"attribute_common",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(
                r"(?m)\[",
                COMMENT_PREPROC,
                NewState::Push(vec![r"attribute["]),
            ),
        ],
    );
    m.insert(
        r"attribute[",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token_to(
                r"(?m)\[",
                COMMENT_PREPROC,
                NewState::Push(vec![r"attribute["]),
            ),
            Rule::token_to(r"(?m)\]", COMMENT_PREPROC, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\]\[]+"#, COMMENT_PREPROC),
        ],
    );
    Table(m)
}

impl Lexer for RustLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
