//! AUTO-GENERATED from `pygments.pygments.lexers.devicetree:DevicetreeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.devicetree:DevicetreeLexer:devicetree

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: devicetree, dts
pub struct DevicetreeLexer;

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
        r"macro",
        vec![
            Rule::bygroups(
                r"(?m)(#include)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROCFILE),
                ],
            ),
            Rule::bygroups(
                r"(?m)(#define)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROC),
                ],
            ),
            Rule::bygroups(
                r#"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)("[^\n{]+")"#,
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROCFILE),
                ],
            ),
            Rule::bygroups(
                r"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n;{]*)([;]?)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROC),
                    Some(PUNCTUATION),
                ],
            ),
        ],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
            Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"statements", vec![
        Rule::bygroups_to(r#"(?m)(L?)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::bygroups_to(r"(?m)([^\s{}/*]*)(\s*)(:)", vec![Some(NAME_LABEL), Some(TEXT), Some(PUNCTUATION)], NewState::Pop(1)),
        Rule::token(r"(?m)(\#(?:(?:address|size)\-cells)|compatible|d(?:evice_type|ma\-ranges)|model|name|phandle|r(?:anges|eg)|status|virtual\-reg)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)([~!%^&*+=|?:<>/#-])", OPERATOR),
        Rule::token(r"(?m)[(){},.\]]", PUNCTUATION),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"bytestring"])),
        Rule::token(r"(?m)[a-zA-Z_][\w-]*(?=(?:\s*,\s*(?:/[*][^*/]*?[*]/\s*)*[a-zA-Z_][\w-]*)*\s*(?:/[*][^*/]*?[*]/\s*)*[=;])", NAME),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
    ]);
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\\\n", TEXT),
            Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?m)(#include)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROCFILE),
                ],
            ),
            Rule::bygroups(
                r"(?m)(#define)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROC),
                ],
            ),
            Rule::bygroups(
                r#"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)("[^\n{]+")"#,
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROCFILE),
                ],
            ),
            Rule::bygroups(
                r"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n;{]*)([;]?)",
                vec![
                    Some(COMMENT_PREPROC),
                    Some(COMMENT_MULTILINE),
                    Some(COMMENT_PREPROC),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::bygroups_to(
                r"(?m)(&)(?:([A-Za-z_]\w*)|(\{)([^}]+)(\}))(\s*(?:/[*][^*/]*?[*]/\s*)*)(\{)",
                vec![
                    Some(OPERATOR),
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                    Some(NAME_NAMESPACE),
                    Some(PUNCTUATION),
                    Some(COMMENT_MULTILINE),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"node"]),
            ),
            Rule::bygroups_to(
                r"(?m)([^/*@\s&]+|/)(@?)((?:0x)?[0-9a-fA-F,]*)(\s*(?:/[*][^*/]*?[*]/\s*)*)(\{)",
                vec![
                    Some(NAME_FUNCTION),
                    Some(OPERATOR),
                    Some(NUMBER_INTEGER),
                    Some(COMMENT_MULTILINE),
                    Some(PUNCTUATION),
                ],
                NewState::Push(vec![r"node"]),
            ),
            Rule::default(NewState::Push(vec![r"statement"])),
        ],
    );
    m.insert(r"statement", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups_to(r#"(?m)(L?)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::bygroups_to(r"(?m)([^\s{}/*]*)(\s*)(:)", vec![Some(NAME_LABEL), Some(TEXT), Some(PUNCTUATION)], NewState::Pop(1)),
        Rule::token(r"(?m)(\#(?:(?:address|size)\-cells)|compatible|d(?:evice_type|ma\-ranges)|model|name|phandle|r(?:anges|eg)|status|virtual\-reg)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)([~!%^&*+=|?:<>/#-])", OPERATOR),
        Rule::token(r"(?m)[(){},.\]]", PUNCTUATION),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"bytestring"])),
        Rule::token(r"(?m)[a-zA-Z_][\w-]*(?=(?:\s*,\s*(?:/[*][^*/]*?[*]/\s*)*[a-zA-Z_][\w-]*)*\s*(?:/[*][^*/]*?[*]/\s*)*[=;])", NAME),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"node", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::bygroups(r"(?m)(#include)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)", vec![Some(COMMENT_PREPROC), Some(COMMENT_MULTILINE), Some(COMMENT_PREPROCFILE)]),
        Rule::bygroups(r"(?m)(#define)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n]+)", vec![Some(COMMENT_PREPROC), Some(COMMENT_MULTILINE), Some(COMMENT_PREPROC)]),
        Rule::bygroups(r#"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)("[^\n{]+")"#, vec![Some(COMMENT_PREPROC), Some(COMMENT_MULTILINE), Some(COMMENT_PREPROCFILE)]),
        Rule::bygroups(r"(?m)(/[^*/{]+/)(\s*(?:/[*][^*/]*?[*]/\s*)*)([^\n;{]*)([;]?)", vec![Some(COMMENT_PREPROC), Some(COMMENT_MULTILINE), Some(COMMENT_PREPROC), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)(&)(?:([A-Za-z_]\w*)|(\{)([^}]+)(\}))(\s*(?:/[*][^*/]*?[*]/\s*)*)(\{)", vec![Some(OPERATOR), Some(NAME_FUNCTION), Some(PUNCTUATION), Some(NAME_NAMESPACE), Some(PUNCTUATION), Some(COMMENT_MULTILINE), Some(PUNCTUATION)], NewState::PushSame),
        Rule::bygroups_to(r"(?m)([^/*@\s&]+|/)(@?)((?:0x)?[0-9a-fA-F,]*)(\s*(?:/[*][^*/]*?[*]/\s*)*)(\{)", vec![Some(NAME_FUNCTION), Some(OPERATOR), Some(NUMBER_INTEGER), Some(COMMENT_MULTILINE), Some(PUNCTUATION)], NewState::PushSame),
        Rule::bygroups_to(r#"(?m)(L?)(")"#, vec![Some(STRING_AFFIX), Some(STRING)], NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::bygroups_to(r"(?m)([^\s{}/*]*)(\s*)(:)", vec![Some(NAME_LABEL), Some(TEXT), Some(PUNCTUATION)], NewState::Pop(1)),
        Rule::token(r"(?m)(\#(?:(?:address|size)\-cells)|compatible|d(?:evice_type|ma\-ranges)|model|name|phandle|r(?:anges|eg)|status|virtual\-reg)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)([~!%^&*+=|?:<>/#-])", OPERATOR),
        Rule::token(r"(?m)[(){},.\]]", PUNCTUATION),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"bytestring"])),
        Rule::token(r"(?m)[a-zA-Z_][\w-]*(?=(?:\s*,\s*(?:/[*][^*/]*?[*]/\s*)*[a-zA-Z_][\w-]*)*\s*(?:/[*][^*/]*?[*]/\s*)*[=;])", NAME),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
        Rule::token_to(r"(?m)\};", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(
        r"bytestring",
        vec![
            Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)[0-9a-fA-F]{2}", NUMBER_HEX),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for DevicetreeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
