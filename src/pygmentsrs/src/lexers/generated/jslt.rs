//! AUTO-GENERATED from `pygments.pygments.lexers.jslt:JSLTLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jslt:JSLTLexer:jslt

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: jslt
pub struct JsltLexer;

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
        Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
        Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        Rule::token(r"(?m)-?(0|[1-9][0-9]*)", NUMBER_INTEGER),
        Rule::token(r"(?m)-?(0|[1-9][0-9]*)(.[0-9]+a)?([Ee][+-]?[0-9]+)", NUMBER_FLOAT),
        Rule::token(r#"(?m)"([^"\\]|\\.)*""#, STRING_DOUBLE),
        Rule::token(r"(?m)[(),:\[\]{}]", PUNCTUATION),
        Rule::token(r"(?m)(!=|[<=>]=?)", OPERATOR),
        Rule::token(r"(?m)[*+/|-]", OPERATOR),
        Rule::token(r"(?m)\.", OPERATOR),
        Rule::token_to(r"(?m)(import)(?=[^0-9A-Z_a-z-])", KEYWORD_NAMESPACE, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)(as)(?=[^0-9A-Z_a-z-])", KEYWORD_NAMESPACE, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r"(?m)(let)(?=[^0-9A-Z_a-z-])", KEYWORD_DECLARATION, NewState::Push(vec![r"_tmp_2"])),
        Rule::token_to(r"(?m)(def)(?=[^0-9A-Z_a-z-])", KEYWORD_DECLARATION, NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)(false|null|true)(?=[^0-9A-Z_a-z-])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(else|for|if)(?=[^0-9A-Z_a-z-])", KEYWORD),
        Rule::token(r"(?m)(and|or)(?=[^0-9A-Z_a-z-])", OPERATOR_WORD),
        Rule::token(r"(?m)(a(?:ll|(?:n|rra)y)|boolean|c(?:apture|eiling|ontains)|e(?:nds\-with|rror)|f(?:allback|l(?:atten|oor)|ormat\-time|rom\-json)|get\-key|hash\-int|i(?:ndex\-of|s\-(?:array|boolean|decimal|integer|number|object|string))|join|lowercase|m(?:ax|in|od)|n(?:o(?:[tw])|umber)|parse\-(?:time|url)|r(?:andom|eplace|ound)|s(?:ha256\-hex|ize|plit|t(?:arts\-with|ring)|um)|t(?:est|o\-json|rim)|uppercase|zip(?:(?:\-with\-index)?))(?=[^0-9A-Z_a-z-])", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*:[A-Z_a-z][0-9A-Z_a-z-]*", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*", NAME),
        Rule::token(r"(?m)\$[A-Z_a-z][0-9A-Z_a-z-]*", NAME_VARIABLE),
    ]);
    m.insert(
        r"import-path",
        vec![Rule::token_to(
            r#"(?m)"([^"]|\\.)*""#,
            STRING_SYMBOL,
            NewState::Push(vec![r"root"]),
        )],
    );
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"_tmp_0",
        vec![
            Rule::token_to(
                r#"(?m)"([^"]|\\.)*""#,
                STRING_SYMBOL,
                NewState::Push(vec![r"root"]),
            ),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"import-alias",
        vec![Rule::token_to(
            r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
            NAME_NAMESPACE,
            NewState::Push(vec![r"root"]),
        )],
    );
    m.insert(
        r"_tmp_1",
        vec![
            Rule::token_to(
                r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
                NAME_NAMESPACE,
                NewState::Push(vec![r"root"]),
            ),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"constant",
        vec![Rule::token_to(
            r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
            NAME_VARIABLE,
            NewState::Push(vec![r"root"]),
        )],
    );
    m.insert(
        r"_tmp_2",
        vec![
            Rule::token_to(
                r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
                NAME_VARIABLE,
                NewState::Push(vec![r"root"]),
            ),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"function",
        vec![Rule::token_to(
            r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
            NAME_FUNCTION,
            NewState::Push(vec![r"_tmp_4"]),
        )],
    );
    m.insert(
        r"function-parameter-list",
        vec![Rule::token_to(
            r"(?m)\(",
            PUNCTUATION,
            NewState::Push(vec![r"_tmp_5"]),
        )],
    );
    m.insert(
        r"function-parameters",
        vec![
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token(r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"_tmp_5",
        vec![
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Push(vec![r"root"])),
            Rule::token(r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*", NAME_VARIABLE),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"_tmp_4",
        vec![
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"_tmp_5"])),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"_tmp_3",
        vec![
            Rule::token_to(
                r"(?m)[A-Z_a-z][0-9A-Z_a-z-]*",
                NAME_FUNCTION,
                NewState::Push(vec![r"_tmp_4"]),
            ),
            Rule::token(r"(?m)[\t\n\f\r ]+", WHITESPACE),
            Rule::token(r"(?m)//.*(\n|\Z)", COMMENT_SINGLE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
        ],
    );
    Table(m)
}

impl Lexer for JsltLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
