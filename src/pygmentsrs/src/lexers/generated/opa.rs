//! AUTO-GENERATED from `pygments.pygments.lexers.ml:OpaLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ml:OpaLexer:opa

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: opa
pub struct OpaLexer;

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
        r"escape-sequence",
        vec![
            Rule::token(r#"(?m)\\[\\"\'ntr}]"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
        ],
    );
    m.insert(
        r"comments-and-spaces",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
        ],
    );
    m.insert(r"root", vec![
        Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
        Rule::token(r"(?m)//.*?$", COMMENT),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\b(a(?:nd|s)|begin|c(?:ase|lient|ss)|d(?:atabase|[bo])|e(?:lse|nd|xternal)|f(?:orall|unction)|i(?:f|mport)|m(?:atch|odule)|or|pa(?:ckage|rser)|rec|server|t(?:hen|ype)|val|with|xml_parser)\b", KEYWORD),
        Rule::token(r"(?m)@(([a-zA-Z_]\w*)|(`[^`]*`))\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)-?.[\d]+([eE][+\-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+.\d*([eE][+\-]?\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+[eE][+\-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)#[\da-fA-F]{3,6}", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r#"(?m)'(?:(\\[\\\"'ntbr ])|(\\[0-9]{3})|(\\x[0-9a-fA-F]{2})|.)'"#, STRING_CHAR),
        Rule::token_to(r"(?m)\{", OPERATOR, NewState::PushSame),
        Rule::token_to(r"(?m)\}", OPERATOR, NewState::Pop(1)),
        Rule::token_to(r"(?m)<(?=[a-zA-Z>])", STRING_SINGLE, NewState::Push(vec![r"html-open-tag"])),
        Rule::token(r"(?m)[@?!]?(/\w+)+(\[_\])?", NAME_VARIABLE),
        Rule::token(r"(?m)<-(?![.=\-<>,@~%/+?*&^!])", NAME_VARIABLE),
        Rule::token(r"(?m)\b([A-Z]\w*)(?=\.)", NAME_NAMESPACE),
        Rule::token(r"(?m)=(?![.=\-<>,@~%/+?*&^!])", KEYWORD),
        Rule::token(r"(?m)([.=\-<>,@~%/+?*&^!])+", OPERATOR),
        Rule::token(r"(?m)([()\[\],;|])+", OPERATOR),
        Rule::token_to(r"(?m):", OPERATOR, NewState::Push(vec![r"type"])),
        Rule::token(r"(?m)'(([a-zA-Z_]\w*)|(`[^`]*`))", KEYWORD_TYPE),
        Rule::token(r"(?m)#(([a-zA-Z_]\w*)|(`[^`]*`))", STRING_SINGLE),
        Rule::token(r"(?m)#(?=\{)", STRING_SINGLE),
        Rule::token(r"(?m)(([a-zA-Z_]\w*)|(`[^`]*`))", TEXT),
    ]);
    m.insert(
        r"type",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)->", KEYWORD_TYPE),
            Rule::default(NewState::Push(vec![
                r"#pop",
                r"type-lhs-1",
                r"type-with-slash",
            ])),
        ],
    );
    m.insert(
        r"type-1",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)\(",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type-tuple"]),
            ),
            Rule::token_to(
                r"(?m)~?\{",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type-record"]),
            ),
            Rule::token_to(
                r"(?m)(([a-zA-Z_]\w*)|(`[^`]*`))\(",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type-tuple"]),
            ),
            Rule::token_to(
                r"(?m)(([a-zA-Z_]\w*)|(`[^`]*`))",
                KEYWORD_TYPE,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)'(([a-zA-Z_]\w*)|(`[^`]*`))", KEYWORD_TYPE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type-with-slash",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::default(NewState::Push(vec![r"#pop", r"slash-type-1", r"type-1"])),
        ],
    );
    m.insert(
        r"slash-type-1",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)/",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type-1"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type-lhs-1",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)->",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type"]),
            ),
            Rule::token_to(
                r"(?m)(?=,)",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type-arrow"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type-arrow",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m),(?=[^:]*?->)",
                KEYWORD_TYPE,
                NewState::Push(vec![r"type-with-slash"]),
            ),
            Rule::token_to(
                r"(?m)->",
                KEYWORD_TYPE,
                NewState::Push(vec![r"#pop", r"type"]),
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type-tuple",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)[^()/*]+", KEYWORD_TYPE),
            Rule::token(r"(?m)[/*]", KEYWORD_TYPE),
            Rule::token_to(r"(?m)\(", KEYWORD_TYPE, NewState::PushSame),
            Rule::token_to(r"(?m)\)", KEYWORD_TYPE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"type-record",
        vec![
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::Push(vec![r"nested-comment"])),
            Rule::token(r"(?m)//.*?$", COMMENT),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r"(?m)[^{}/*]+", KEYWORD_TYPE),
            Rule::token(r"(?m)[/*]", KEYWORD_TYPE),
            Rule::token_to(r"(?m)\{", KEYWORD_TYPE, NewState::PushSame),
            Rule::token_to(r"(?m)\}", KEYWORD_TYPE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"nested-comment",
        vec![
            Rule::token(r"(?m)[^/*]+", COMMENT),
            Rule::token_to(r"(?m)/\*", COMMENT, NewState::PushSame),
            Rule::token_to(r"(?m)\*/", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)[/*]", COMMENT),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r#"(?m)[^\\"{]+"#, STRING_DOUBLE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", OPERATOR, NewState::Push(vec![r"root"])),
            Rule::token(r#"(?m)\\[\\"\'ntr}]"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"single-string",
        vec![
            Rule::token(r"(?m)[^\\\'{]+", STRING_DOUBLE),
            Rule::token_to(r"(?m)\'", STRING_DOUBLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", OPERATOR, NewState::Push(vec![r"root"])),
            Rule::token(r#"(?m)\\[\\"\'ntr}]"#, STRING_ESCAPE),
            Rule::token(r"(?m)\\[0-9]{3}", STRING_ESCAPE),
            Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"html-open-tag",
        vec![
            Rule::token_to(
                r"(?m)[\w\-:]+",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"html-attr"]),
            ),
            Rule::token_to(
                r"(?m)>",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"html-content"]),
            ),
        ],
    );
    m.insert(
        r"html-end-tag",
        vec![Rule::token_to(
            r"(?m)[\w\-:]*>",
            STRING_SINGLE,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"html-attr",
        vec![
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token_to(
                r"(?m)[\w\-:]+=",
                STRING_SINGLE,
                NewState::Push(vec![r"html-attr-value"]),
            ),
            Rule::token_to(r"(?m)/>", STRING_SINGLE, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)>",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"html-content"]),
            ),
        ],
    );
    m.insert(
        r"html-attr-value",
        vec![
            Rule::token_to(
                r"(?m)'",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"single-string"]),
            ),
            Rule::token_to(
                r#"(?m)""#,
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"string"]),
            ),
            Rule::token_to(
                r"(?m)#(([a-zA-Z_]\w*)|(`[^`]*`))",
                STRING_SINGLE,
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)#(?=\{)",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"root"]),
            ),
            Rule::token_to(r#"(?m)[^"\'{`=<>]+"#, STRING_SINGLE, NewState::Pop(1)),
            Rule::token_to(r"(?m)\{", OPERATOR, NewState::Push(vec![r"#pop", r"root"])),
        ],
    );
    m.insert(
        r"html-content",
        vec![
            Rule::token_to(r"(?m)<!--", COMMENT, NewState::Push(vec![r"html-comment"])),
            Rule::token_to(
                r"(?m)</",
                STRING_SINGLE,
                NewState::Push(vec![r"#pop", r"html-end-tag"]),
            ),
            Rule::token_to(
                r"(?m)<",
                STRING_SINGLE,
                NewState::Push(vec![r"html-open-tag"]),
            ),
            Rule::token_to(r"(?m)\{", OPERATOR, NewState::Push(vec![r"root"])),
            Rule::token(r"(?m)[^<{]+", STRING_SINGLE),
        ],
    );
    m.insert(
        r"html-comment",
        vec![
            Rule::token_to(r"(?m)-->", COMMENT, NewState::Pop(1)),
            Rule::token(r"(?m)[^\-]+|-", COMMENT),
        ],
    );
    Table(m)
}

impl Lexer for OpaLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
