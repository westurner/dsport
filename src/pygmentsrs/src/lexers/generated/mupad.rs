//! AUTO-GENERATED from `pygments.pygments.lexers.algebra:MuPADLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.algebra:MuPADLexer:mupad

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mupad
pub struct MupadLexer;

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
            Rule::token(r"(?m)//.*?$", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comment"]),
            ),
            Rule::token(r#"(?m)"(?:[^"\\]|\\.)*""#, STRING),
            Rule::token(r"(?m)\(|\)|\[|\]|\{|\}", PUNCTUATION),
            Rule::token(
                r"(?m)(?x)\b(?:
                next|break|end|
                axiom|end_axiom|category|end_category|domain|end_domain|inherits|
                if|%if|then|elif|else|end_if|
                case|of|do|otherwise|end_case|
                while|end_while|
                repeat|until|end_repeat|
                for|from|to|downto|step|end_for|
                proc|local|option|save|begin|end_proc|
                delete|frame
              )\b",
                KEYWORD,
            ),
            Rule::token(
                r"(?m)(?x)\b(?:
                DOM_ARRAY|DOM_BOOL|DOM_COMPLEX|DOM_DOMAIN|DOM_EXEC|DOM_EXPR|
                DOM_FAIL|DOM_FLOAT|DOM_FRAME|DOM_FUNC_ENV|DOM_HFARRAY|DOM_IDENT|
                DOM_INT|DOM_INTERVAL|DOM_LIST|DOM_NIL|DOM_NULL|DOM_POLY|DOM_PROC|
                DOM_PROC_ENV|DOM_RAT|DOM_SET|DOM_STRING|DOM_TABLE|DOM_VAR
              )\b",
                NAME_CLASS,
            ),
            Rule::token(
                r"(?m)(?x)\b(?:
                PI|EULER|E|CATALAN|
                NIL|FAIL|undefined|infinity|
                TRUE|FALSE|UNKNOWN
              )\b",
                NAME_CONSTANT,
            ),
            Rule::token(r"(?m)\b(?:dom|procname)\b", NAME_BUILTIN_PSEUDO),
            Rule::token(
                r"(?m)\.|,|:|;|=|\+|-|\*|/|\^|@|>|<|\$|\||!|\'|%|~=",
                OPERATOR,
            ),
            Rule::token(
                r"(?m)(?x)\b(?:
                and|or|not|xor|
                assuming|
                div|mod|
                union|minus|intersect|in|subset
              )\b",
                OPERATOR_WORD,
            ),
            Rule::token(r"(?m)\b(?:I|RDN_INF|RD_NINF|RD_NAN)\b", NUMBER),
            Rule::bygroups(
                r"(?m)(?x)
              ((?:[a-zA-Z_#][\w#]*|`[^`]*`)
              (?:::[a-zA-Z_#][\w#]*|`[^`]*`)*)(\s*)([(])",
                vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)],
            ),
            Rule::token(
                r"(?m)(?x)
              (?:[a-zA-Z_#][\w#]*|`[^`]*`)
              (?:::[a-zA-Z_#][\w#]*|`[^`]*`)*",
                NAME_VARIABLE,
            ),
            Rule::token(r"(?m)[0-9]+(?:\.[0-9]*)?(?:e[0-9]+)?", NUMBER),
            Rule::token(r"(?m)\.[0-9]+(?:e[0-9]+)?", NUMBER),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m).", TEXT),
        ],
    );
    m.insert(
        r"comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for MupadLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
