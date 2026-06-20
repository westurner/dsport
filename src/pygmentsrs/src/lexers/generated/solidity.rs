#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.solidity:SolidityLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.solidity:SolidityLexer:solidity

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: solidity
pub struct SolidityLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)\bpragma\s+solidity\b", KEYWORD, NewState::Push(vec![r"pragma"])),
        Rule::bygroups(r"(?m)\b(contract)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_ENTITY)]),
        Rule::bygroups(r"(?m)\b(address|bool|(?:(?:bytes|hash|int|string|uint)(?:8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?))\b(\s+)((?:external|public|internal|private)\s+)?([a-zA-Z_]\w*)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(KEYWORD), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)\b(enum|event|function|struct)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_TYPE), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::token(r"(?m)\b(msg|block|tx)\.([A-Za-z_][a-zA-Z0-9_]*)\b", KEYWORD),
        Rule::token(r"(?m)\b(b(?:(?:loc|rea)k)|con(?:st(?:ant|ructor)|t(?:inue|ract))|do|e(?:lse|xternal)|f(?:alse|or|unction)|i(?:mport|n(?:herited|ternal)|[fs])|library|m(?:apping|emory|odifier|sg)|new|p(?:ayable|rivate|ublic)|re(?:quire|turn(?:(?:s)?))|s(?:truct|uicide)|t(?:h(?:is|row)|rue|x)|var|while)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(keccak256)\b", NAME_BUILTIN),
        Rule::token(r"(?m)\b(address|bool|(?:(?:bytes|hash|int|string|uint)(?:8|16|24|32|40|48|56|64|72|80|88|96|104|112|120|128|136|144|152|160|168|176|184|192|200|208|216|224|232|240|248|256)?))\b", KEYWORD_TYPE),
        Rule::token(r#"(?m)("(\\"|.)*?")"#, STRING_DOUBLE),
        Rule::token(r"(?m)('(\\'|.)*?')", STRING_SINGLE),
        Rule::token(r"(?m)\b0[xX][0-9a-fA-F]+\b", NUMBER_HEX),
        Rule::token(r"(?m)\b\d+\b", TokenType::new(&["Literal", "Number", "Decimal"])),
        Rule::token(r"(?m)[a-zA-Z_]\w*", TEXT),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[.;{}(),\[\]]", PUNCTUATION),
    ]);
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\n", WHITESPACE),
        ],
    );
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"constants",
        vec![
            Rule::token(r#"(?m)("(\\"|.)*?")"#, STRING_DOUBLE),
            Rule::token(r"(?m)('(\\'|.)*?')", STRING_SINGLE),
            Rule::token(r"(?m)\b0[xX][0-9a-fA-F]+\b", NUMBER_HEX),
            Rule::token(
                r"(?m)\b\d+\b",
                TokenType::new(&["Literal", "Number", "Decimal"]),
            ),
        ],
    );
    m.insert(
        r"pragma",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)//(\n|[\w\W]*?[^\\]\n)", COMMENT_SINGLE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*?[*](\\\n)?/", COMMENT_MULTILINE),
            Rule::token(r"(?m)/(\\\n)?[*][\w\W]*", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?m)(\^|>=|<)(\s*)(\d+\.\d+\.\d+)",
                vec![Some(OPERATOR), Some(WHITESPACE), Some(KEYWORD)],
            ),
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for SolidityLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
