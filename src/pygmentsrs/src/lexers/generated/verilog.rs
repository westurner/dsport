//! AUTO-GENERATED from `pygments.pygments.lexers.hdl:VerilogLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.hdl:VerilogLexer:verilog

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: verilog, v
pub struct VerilogLexer;

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
        Rule::token_to(r"(?m)^\s*`define", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?m)/(\\\n)?/(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[{}#@]", PUNCTUATION),
        Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)L?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([0-9]+)|(\'h)[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)([0-9]+)|(\'b)[01]+", NUMBER_BIN),
        Rule::token(r"(?m)([0-9]+)|(\'d)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?m)([0-9]+)|(\'o)[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)\'[01xz]", NUMBER),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.;\']", PUNCTUATION),
        Rule::token(r"(?m)`[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)^(\s*)(package)(\s+)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(TEXT)]),
        Rule::bygroups_to(r"(?m)^(\s*)(import)(\s+)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(TEXT)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?m)(a(?:lways(?:(?:_(?:comb|ff|latch))?)|nd|ssign|utomatic)|b(?:egin|reak|uf(?:(?:if(?:[01]))?))|c(?:ase(?:(?:[xz])?)|mos|on(?:st|tinue))|d(?:e(?:assign|f(?:ault|param))|isable|o)|e(?:dge|lse|n(?:d(?:(?:case|function|generate|module|p(?:(?:ackag|rimitiv)e)|specify|ta(?:ble|sk))?)|um)|vent)|f(?:inal|or(?:(?:ce|ever|k)?)|unction)|gen(?:erate|var)|highz(?:[01])|i(?:f|n(?:itial|out|put|teger))|join|l(?:arge|ocalparam)|m(?:acromodule|edium|odule)|n(?:and|egedge|mos|o(?:tif(?:[01])|[rt]))|o(?:r|utput)|p(?:a(?:cked|rameter)|mos|osedge|rimitive|ull(?:down|up|[01]))|r(?:cmos|e(?:f|lease|peat|turn)|nmos|pmos|tran(?:(?:if(?:[01]))?))|s(?:calared|igned|mall|pec(?:ify|param)|tr(?:ength|ing|ong(?:[01])|uct))|t(?:a(?:ble|sk)|ran(?:(?:if(?:[01]))?)|ype(?:(?:def)?))|unsigned|v(?:ar|(?:ectore|oi)d)|w(?:ait|eak(?:[01])|hile)|x(?:(?:(?:n)?)or))\b", KEYWORD),
        Rule::token(r"(?m)`(a(?:ccelerate|utoexpand_vectornets)|celldefine|default_nettype|e(?:ls(?:e|if)|nd(?:celldefine|if|protect(?:(?:ed)?))|xpand_vectornets)|i(?:f(?:(?:(?:n)?)def)|nclude)|no(?:accelerate|expand_vectornets|remove_(?:(?:gate|net)names)|unconnected_drive)|protect(?:(?:ed)?)|re(?:move_(?:(?:gate|net)names)|setall)|timescale|un(?:connected_drive|def))\b", COMMENT_PREPROC),
        Rule::token(r"(?m)\$(bits(?:(?:to(?:(?:(?:short)?)real))?)|countdrivers|display|f(?:close|display|inish|loor|monitor|open|(?:strob|writ)e)|getpattern|history|i(?:n(?:csave|put)|tor)|key|l(?:ist|og)|monitor(?:(?:o(?:ff|n))?)|no(?:key|log)|printtimescale|r(?:andom|e(?:a(?:dmem(?:[bh])|lt(?:ime|obits))|s(?:et(?:(?:_(?:count|value))?)|tart))|toi)|s(?:ave|c(?:(?:al|op)e)|ho(?:(?:rtrealtobit|w(?:scope|var(?:(?:iable)?)))s)|readmem(?:[bh])|t(?:ime|op|robe))|time(?:(?:format)?)|write)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(b(?:it|yte)|int(?:(?:eger)?)|lo(?:gic|ngint)|re(?:al(?:(?:time)?)|g)|s(?:hortint|upply(?:[01]))|t(?:ime|ri(?:(?:and|or|reg|[01])?))|uwire|w(?:and|ire|orshortreal))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:(?!:)", NAME_LABEL),
        Rule::token(r"(?m)\$?[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\\(\S+)", NAME),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    m.insert(
        r"macro",
        vec![
            Rule::token(r"(?m)[^/\n]+", COMMENT_PREPROC),
            Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)//.*?\n", COMMENT_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)/", COMMENT_PREPROC),
            Rule::token(r"(?m)(?<=\\)\n", COMMENT_PREPROC),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"import",
        vec![Rule::token_to(
            r"(?m)[\w:]+\*?",
            NAME_NAMESPACE,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for VerilogLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
