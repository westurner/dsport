#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.dsls:PuppetLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.dsls:PuppetLexer:puppet

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: puppet
pub struct PuppetLexer;

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
        Rule::bygroups(r"(?m)(\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT)]),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)(?i)(a(?:bsent|l(?:ert|ias)|u(?:dit|geas))|before|c(?:ase|heck|lass|o(?:mputer|n(?:(?:figur|tain)ed))|r(?:eate_resources|it|on))|d(?:e(?:bug|f(?:ault|ine(?:(?:d)?)))|irectory)|e(?:ls(?:e|if)|merg|rr|x(?:ec|tlookup))|f(?:a(?:il|lse)|ile(?:(?:bucket)?)|qdn_rand)|generate|host|i(?:f|mport|n(?:clude|fo|herits|line_template|stalled|terface))|k5login|l(?:atest|ink|oglevel)|m(?:a(?:cauthorization|il(?:alias|list))|cx|d5|ount(?:(?:ed)?))|n(?:agios_(?:co(?:mmand|ntact(?:(?:group)?))|host(?:(?:dependency|e(?:scalation|xtinfo)|group)?)|service(?:(?:dependency|e(?:scalation|xtinfo)|group)?)|timeperiod)|o(?:de|op|ti(?:ce|fy)))|p(?:ackage|resent|urged)|r(?:e(?:alize|gsubst|sources)|o(?:le|uter)|unning)|s(?:chedule(?:(?:d_task)?)|e(?:arch|l(?:boolean|module)|rvice)|h(?:a1|ellquote)|p(?:lit|rintf)|sh(?:(?:(?:_authorized_)?)key)|t(?:age|opped)|ubscribe)|t(?:ag(?:(?:ged)?)|emplate|idy|rue)|u(?:n(?:def|mounted)|ser)|v(?:ersioncmp|lan)|warning|yumrepo|z(?:fs|one|pool))\b", KEYWORD),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
        Rule::bygroups(r"(?m)(\$\S+)(\[)(\S+)(\])", vec![Some(NAME_VARIABLE), Some(PUNCTUATION), Some(STRING), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\$\S+", NAME_VARIABLE),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        Rule::token(r"(?m)(=>|\?|<|>|=|\+|-|/|\*|~|!|\|)", OPERATOR),
        Rule::token(r"(?m)(in|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r#"(?m)"([^"])*""#, STRING),
        Rule::token(r"(?m)'(\\'|[^'])*'", STRING),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"comments",
        vec![
            Rule::bygroups(r"(?m)(\s*)(#.*)$", vec![Some(WHITESPACE), Some(COMMENT)]),
            Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(?i)(a(?:bsent|l(?:ert|ias)|u(?:dit|geas))|before|c(?:ase|heck|lass|o(?:mputer|n(?:(?:figur|tain)ed))|r(?:eate_resources|it|on))|d(?:e(?:bug|f(?:ault|ine(?:(?:d)?)))|irectory)|e(?:ls(?:e|if)|merg|rr|x(?:ec|tlookup))|f(?:a(?:il|lse)|ile(?:(?:bucket)?)|qdn_rand)|generate|host|i(?:f|mport|n(?:clude|fo|herits|line_template|stalled|terface))|k5login|l(?:atest|ink|oglevel)|m(?:a(?:cauthorization|il(?:alias|list))|cx|d5|ount(?:(?:ed)?))|n(?:agios_(?:co(?:mmand|ntact(?:(?:group)?))|host(?:(?:dependency|e(?:scalation|xtinfo)|group)?)|service(?:(?:dependency|e(?:scalation|xtinfo)|group)?)|timeperiod)|o(?:de|op|ti(?:ce|fy)))|p(?:ackage|resent|urged)|r(?:e(?:alize|gsubst|sources)|o(?:le|uter)|unning)|s(?:chedule(?:(?:d_task)?)|e(?:arch|l(?:boolean|module)|rvice)|h(?:a1|ellquote)|p(?:lit|rintf)|sh(?:(?:(?:_authorized_)?)key)|t(?:age|opped)|ubscribe)|t(?:ag(?:(?:ged)?)|emplate|idy|rue)|u(?:n(?:def|mounted)|ser)|v(?:ersioncmp|lan)|warning|yumrepo|z(?:fs|one|pool))\b", KEYWORD),
    ]);
    m.insert(
        r"names",
        vec![
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME_ATTRIBUTE),
            Rule::bygroups(
                r"(?m)(\$\S+)(\[)(\S+)(\])",
                vec![
                    Some(NAME_VARIABLE),
                    Some(PUNCTUATION),
                    Some(STRING),
                    Some(PUNCTUATION),
                ],
            ),
            Rule::token(r"(?m)\$\S+", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
            Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(
                r"(?m)\d+L",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"operators",
        vec![
            Rule::token(r"(?m)(=>|\?|<|>|=|\+|-|/|\*|~|!|\|)", OPERATOR),
            Rule::token(r"(?m)(in|and|or|not)\b", OPERATOR_WORD),
        ],
    );
    m.insert(
        r"strings",
        vec![
            Rule::token(r#"(?m)"([^"])*""#, STRING),
            Rule::token(r"(?m)'(\\'|[^'])*'", STRING),
        ],
    );
    Table(m)
}

impl Lexer for PuppetLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
