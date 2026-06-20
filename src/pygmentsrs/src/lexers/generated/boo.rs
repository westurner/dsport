//! AUTO-GENERATED from `pygments.pygments.lexers.dotnet:BooLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dotnet:BooLexer:boo

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: boo
pub struct BooLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(#|//).*$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[\]{}:(),.;\[]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(in|is|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)/(\\\\|\\[^\\]|[^/\\\s])/", STRING_REGEX),
        Rule::token(r"(?m)@/(\\\\|\\[^\\]|[^/\\])*/", STRING_REGEX),
        Rule::token(r"(?m)=~|!=|==|<<|>>|[-+/*%=<>&^|]", OPERATOR),
        Rule::token(r"(?m)(as|abstract|callable|constructor|destructor|do|import|enum|event|final|get|interface|internal|of|override|partial|private|protected|public|return|set|static|struct|transient|virtual|yield|super|and|break|cast|continue|elif|else|ensure|except|for|given|goto|if|in|is|isa|not|or|otherwise|pass|raise|ref|try|unless|when|while|from|as)\b", KEYWORD),
        Rule::token(r"(?m)def(?=\s+\(.*?\))", KEYWORD),
        Rule::bygroups_to(r"(?m)(def)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::bygroups_to(r"(?m)(class)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::bygroups_to(r"(?m)(namespace)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"namespace"])),
        Rule::token(r"(?m)(?<!\.)(true|false|null|self|__eval__|__switch__|array|assert|checked|enumerate|filter|getter|len|lock|map|matrix|max|min|normalArrayIndexing|print|property|range|rawArrayIndexing|required|typeof|unchecked|using|yieldAll|zip)\b", NAME_BUILTIN),
        Rule::token(r#"(?m)"""(\\\\|\\"|.*?)""""#, STRING_DOUBLE),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([fF][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9][0-9.]*(ms?|d|h|s)", NUMBER),
        Rule::token(r"(?m)0\d+", NUMBER_OCT),
        Rule::token(r"(?m)0x[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)[*]/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[^/*]", COMMENT_MULTILINE),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"funcname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_]\w*",
            NAME_FUNCTION,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"classname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"namespace",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_][\w.]*",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for BooLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
