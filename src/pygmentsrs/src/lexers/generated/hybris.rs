#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:HybrisLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:HybrisLexer:hybris

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: hybris
pub struct HybrisLexer;

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
        Rule::bygroups(r"(?ms)^(\s*(?:function|method|operator\s+)+?)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(KEYWORD), Some(NAME_FUNCTION), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?ms)[^\S\n]+", TEXT),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)@[a-zA-Z_][\w.]*", NAME_DECORATOR),
        Rule::token(r"(?ms)(break|case|catch|next|default|do|else|finally|for|foreach|of|unless|if|new|return|switch|me|throw|try|while)\b", KEYWORD),
        Rule::token(r"(?ms)(extends|private|protected|public|static|throws|function|method|operator)\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(true|false|null|__FILE__|__LINE__|__VERSION__|__LIB_PATH__|__INC_PATH__)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r"(?ms)(class|struct)(\s+)", vec![Some(KEYWORD_DECLARATION), Some(TEXT)], NewState::Push(vec![r"class"])),
        Rule::bygroups_to(r"(?ms)(import|include)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(TEXT)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?ms)(a(?:c(?:cept|os)|sin|tan(?:(?:2)?))|b(?:ase64(?:(?:de|en)code)|in(?:ary|d))|c(?:all(?:(?:_method)?)|eil|lose|o(?:n(?:nect|tains)|s(?:(?:h)?))|rc32)|d(?:ll(?:c(?:all(?:(?:_argv)?)|lose)|link|open)|yn_functions)|e(?:nv|val|x(?:ec|it|p))|f(?:abs|close|gets|i(?:le|nd)|loor|mod|o(?:pen|rk)|r(?:ead|omxml)|s(?:eek|ize)|tell|write)|g(?:c_(?:collect(?:(?:_threshold)?)|mm_(?:items|usage))|et(?:p(?:eername|id)|sockname))|h(?:as|ttp_(?:download|(?:ge|pos)t))|i(?:nput|s(?:a(?:lias|rray)|char|float|int|map|string))|join|k(?:eys|ill)|l(?:ength|isten|o(?:ad|g(?:(?:10)?)))|m(?:d5(?:(?:_file)?)|ethods|k(?:fifo|nod)|ount)|p(?:ack|c(?:(?:los|re_replac)e)|o(?:pen|[pw])|rint(?:(?:f|ln)?)|thread_(?:create(?:(?:_argv)?)|exit|join|kill))|re(?:ad(?:dir|line)|cv|(?:mov|plac)e)|s(?:e(?:nd|r(?:ial_(?:close|fcntl|get_(?:attr|(?:[io])speed)|open|read|set_(?:attr|(?:[io])speed)|write)|ver)|ttimeout)|ha(?:[12])|i(?:n(?:(?:h)?)|ze(?:(?:of)?))|leep|mtp_send|ocket|plit|qrt|tr(?:(?:dat|tim)e)|ubstr)|t(?:an(?:(?:h)?)|i(?:cks|me)|o(?:int|string|xml)|rim|ypeof)|u(?:mount(?:(?:2)?)|nmap|rl(?:(?:de|en)code)|s(?:er_functions|leep))|va(?:(?:lu|r_(?:nam|valu))es)|wait|xml_(?:load|parse))\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(C(?:GI|lientSocket|onsole)|D(?:irectory|ll)|Exception|File|MethodReference|P(?:ipe|rocess)|Runn(?:able|er)|S(?:(?:(?:erverS)?)ocket)|Thread)\b", KEYWORD_TYPE),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?ms)'\\.'|'[^\\]'|'\\u[0-9a-f]{4}'", STRING_CHAR),
        Rule::bygroups(r"(?ms)(\.)([a-zA-Z_]\w*)", vec![Some(OPERATOR), Some(NAME_ATTRIBUTE)]),
        Rule::token(r"(?ms)[a-zA-Z_]\w*:", NAME_LABEL),
        Rule::token(r"(?ms)[a-zA-Z_$]\w*", NAME),
        Rule::token(r"(?ms)[~^*!%&\[\](){}<>|+=:;,./?\-@]+", OPERATOR),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+L?", NUMBER_INTEGER),
        Rule::token(r"(?ms)\n", TEXT),
    ]);
    m.insert(
        r"class",
        vec![Rule::token_to(
            r"(?ms)[a-zA-Z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"import",
        vec![Rule::token_to(
            r"(?ms)[\w.]+\*?",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for HybrisLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
