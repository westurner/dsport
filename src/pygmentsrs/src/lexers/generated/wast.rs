//! AUTO-GENERATED from `pygments.pygments.lexers.webassembly:WatLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.webassembly:WatLexer:wast

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: wast, wat
pub struct WastLexer;

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
        Rule::token(r"(?m)(block|data|e(?:l(?:em|se)|nd|xport)|func(?:(?:ref)?)|global|i(?:f|mport)|lo(?:cal|op)|m(?:emory|odule|ut)|param|result|start|t(?:able|hen|ype))(?=[^a-z_\.])", KEYWORD),
        Rule::token_to(r"(?m)(b(?:lock|r(?:(?:_(?:if|table))?))|call(?:(?:_indirect)?)|drop|e(?:lse|nd)|f(?:32\.(?:a(?:bs|dd)|c(?:eil|o(?:n(?:st|vert_i(?:32_(?:[su])|64_(?:[su])))|pysign))|d(?:emote_f64|iv)|eq|floor|g(?:[et])|l(?:oad|[et])|m(?:ax|in|ul)|ne(?:(?:arest|g)?)|reinterpret_i32|s(?:qrt|tore|ub)|trunc)|64\.(?:a(?:bs|dd)|c(?:eil|o(?:n(?:st|vert_i(?:32_(?:[su])|64_(?:[su])))|pysign))|div|eq|floor|g(?:[et])|l(?:oad|[et])|m(?:ax|in|ul)|ne(?:(?:arest|g)?)|promote_f32|reinterpret_i64|s(?:qrt|tore|ub)|trunc))|global\.(?:(?:[gs])et)|i(?:32\.(?:a(?:(?:[dn])d)|c(?:lz|onst|tz)|div_(?:[su])|eq(?:(?:z)?)|g(?:e_(?:[su])|t_(?:[su]))|l(?:e_(?:[su])|oad(?:(?:16_(?:[su])|8_(?:[su]))?)|t_(?:[su]))|mul|ne|or|popcnt|r(?:e(?:interpret_f32|m_(?:[su]))|ot(?:[lr]))|s(?:h(?:l|r_(?:[su]))|tore(?:(?:16|8)?)|ub)|trunc_f(?:32_(?:[su])|64_(?:[su]))|wrap_i64|xor)|64\.(?:a(?:(?:[dn])d)|c(?:lz|onst|tz)|div_(?:[su])|e(?:q(?:(?:z)?)|xtend_i32_(?:[su]))|g(?:e_(?:[su])|t_(?:[su]))|l(?:e_(?:[su])|oad(?:(?:16_(?:[su])|32_(?:[su])|8_(?:[su]))?)|t_(?:[su]))|mul|ne|or|popcnt|r(?:e(?:interpret_f64|m_(?:[su]))|ot(?:[lr]))|s(?:h(?:l|r_(?:[su]))|tore(?:(?:16|32|8)?)|ub)|trunc_f(?:32_(?:[su])|64_(?:[su]))|xor)|f)|lo(?:cal\.(?:get|set|tee)|op)|memory\.(?:grow|size)|nop|return|select|unreachable)", NAME_BUILTIN, NewState::Push(vec![r"arguments"])),
        Rule::token(r"(?m)(f(?:32|64)|i(?:32|64))", KEYWORD_TYPE),
        Rule::token(r"(?m)\$[A-Za-z0-9!#$%&\'*+./:<=>?@\\^_`|~-]+", NAME_VARIABLE),
        Rule::token(r"(?m);;.*?$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)\(;", COMMENT_MULTILINE, NewState::Push(vec![r"nesting_comment"])),
        Rule::token(r"(?m)[+-]?0x[\dA-Fa-f](_?[\dA-Fa-f])*(.([\dA-Fa-f](_?[\dA-Fa-f])*)?)?([pP][+-]?[\dA-Fa-f](_?[\dA-Fa-f])*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d.\d(_?\d)*[eE][+-]?\d(_?\d)*", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d.\d(_?\d)*", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?\d.[eE][+-]?\d(_?\d)*", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?(inf|nan:0x[\dA-Fa-f](_?[\dA-Fa-f])*|nan)", NUMBER_FLOAT),
        Rule::token(r"(?m)[+-]?0x[\dA-Fa-f](_?[\dA-Fa-f])*", NUMBER_HEX),
        Rule::token(r"(?m)[+-]?\d(_?\d)*", NUMBER_INTEGER),
        Rule::token(r"(?m)[\(\)]", PUNCTUATION),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\s+", TEXT),
    ]);
    m.insert(r"nesting_comment", vec![
        Rule::token_to(r"(?m)\(;", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m);\)", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[^;(]+", COMMENT_MULTILINE),
        Rule::token(r"(?m)[;(]", COMMENT_MULTILINE),
    ]);
    m.insert(r"string", vec![
        Rule::token(r"(?m)\\[\dA-Fa-f][\dA-Fa-f]", STRING_ESCAPE),
        Rule::token(r"(?m)\\t", STRING_ESCAPE),
        Rule::token(r"(?m)\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\\r", STRING_ESCAPE),
        Rule::token(r#"(?m)\\""#, STRING_ESCAPE),
        Rule::token(r"(?m)\\'", STRING_ESCAPE),
        Rule::token(r"(?m)\\u\{[\dA-Fa-f](_?[\dA-Fa-f])*\}", STRING_ESCAPE),
        Rule::token(r"(?m)\\\\", STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"arguments", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::bygroups(r"(?m)(offset)(=)(0x[\dA-Fa-f](_?[\dA-Fa-f])*)", vec![Some(KEYWORD), Some(OPERATOR), Some(NUMBER_HEX)]),
        Rule::bygroups(r"(?m)(offset)(=)(\d(_?\d)*)", vec![Some(KEYWORD), Some(OPERATOR), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?m)(align)(=)(0x[\dA-Fa-f](_?[\dA-Fa-f])*)", vec![Some(KEYWORD), Some(OPERATOR), Some(NUMBER_HEX)]),
        Rule::bygroups(r"(?m)(align)(=)(\d(_?\d)*)", vec![Some(KEYWORD), Some(OPERATOR), Some(NUMBER_INTEGER)]),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for WastLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
