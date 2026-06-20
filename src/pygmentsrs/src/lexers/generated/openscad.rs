#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.openscad:OpenScadLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.openscad:OpenScadLexer:openscad

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: openscad
pub struct OpenscadLexer;

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
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token_to(r"(?m)//", COMMENT_SINGLE, NewState::Push(vec![r"comment-single"])),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comment-multi"])),
        Rule::token(r"(?m)[{}\[\]\(\),;:]", PUNCTUATION),
        Rule::token(r"(?m)[*!#%\-+=?/]", OPERATOR),
        Rule::token(r"(?m)<=|<|==|!=|>=|>|&&|\|\|", OPERATOR),
        Rule::token(r"(?m)\$(f[asn]|t|vp[rtd]|children)", OPERATOR),
        Rule::token(r"(?m)(undef|PI)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?m)(use|include)((?:\s|\\\\s)+)", vec![Some(KEYWORD_NAMESPACE), Some(TEXT)], NewState::Push(vec![r"includes"])),
        Rule::bygroups(r"(?m)(module)(\s*)([^\s\(]+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)(function)(\s*)([^\s\(]+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", LITERAL),
        Rule::token(r"(?m)\b(else|f(?:or|unction)|i(?:f|n(?:clude|tersection_for))|module|return|use)\b", KEYWORD),
        Rule::token(r"(?m)\b(a(?:bs|cos|s(?:(?:i|sig)n)|tan(?:(?:2)?))|c(?:eil|hr|ircle|o(?:lor|ncat|s)|ross|ube|ylinder)|d(?:ifference|xf_(?:cross|dim|linear_extrude))|e(?:cho|xp)|floor|hull|i(?:mport(?:(?:_dxf)?)|ntersection)|l(?:e(?:[nt])|inear_extrude|n|o(?:g|okup))|m(?:ax|i(?:n(?:(?:kowski)?)|rror)|ultmatrix)|norm|offset|p(?:arent_module|o(?:ly(?:(?:g|hedr)on)|w)|rojection)|r(?:ands|e(?:nder|size)|o(?:tate(?:(?:_extrude)?)|und))|s(?:cale|earch|i(?:(?:(?:g)?)n)|phere|q(?:rt|uare)|tr|urface)|t(?:an|ext|ranslate)|union|version(?:(?:_num)?))\b", NAME_BUILTIN),
        Rule::token(r"(?m)\bchildren\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r#"(?m)""".*?""""#, STRING_DOUBLE),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)-?\d+(\.\d+)?(e[+-]?\d+)?", NUMBER),
        Rule::token(r"(?m)\w+", NAME),
    ]);
    m.insert(
        r"includes",
        vec![Rule::bygroups(
            r"(?m)(<)([^>]*)(>)",
            vec![
                Some(PUNCTUATION),
                Some(COMMENT_PREPROCFILE),
                Some(PUNCTUATION),
            ],
        )],
    );
    m.insert(
        r"comment",
        vec![Rule::token(
            r"(?m):param: [a-zA-Z_]\w*|:returns?:|(FIXME|MARK|TODO):",
            COMMENT_SPECIAL,
        )],
    );
    m.insert(
        r"comment-single",
        vec![
            Rule::token_to(r"(?m)\n", TEXT, NewState::Pop(1)),
            Rule::token(
                r"(?m):param: [a-zA-Z_]\w*|:returns?:|(FIXME|MARK|TODO):",
                COMMENT_SPECIAL,
            ),
            Rule::token(r"(?m)[^\n]+", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"comment-multi",
        vec![
            Rule::token(
                r"(?m):param: [a-zA-Z_]\w*|:returns?:|(FIXME|MARK|TODO):",
                COMMENT_SPECIAL,
            ),
            Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for OpenscadLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
