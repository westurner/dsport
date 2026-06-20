//! AUTO-GENERATED from `pygments.pygments.lexers.gdscript:GDScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.gdscript:GDScriptLexer:gdscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gdscript, gd
pub struct GdscriptLexer;

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
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::bygroups(r#"(?m)^(\s*)([rRuUbB]{,2})("""(?:.|\n)*?""")"#, vec![Some(WHITESPACE), Some(STRING_AFFIX), Some(STRING_DOC)]),
        Rule::bygroups(r"(?m)^(\s*)([rRuUbB]{,2})('''(?:.|\n)*?''')", vec![Some(WHITESPACE), Some(STRING_AFFIX), Some(STRING_DOC)]),
        Rule::token(r"(?m)[^\S\n]+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(in|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|&&|\+=|-=|\*=|/=|%=|&=|\|=|\|\||[-~+/*%=<>&^.!|$]", OPERATOR),
        Rule::token(r"(?m)(a(?:nd|s)|break(?:(?:point)?)|c(?:lass(?:(?:_name)?)|on(?:st|tinue))|e(?:l(?:if|se)|num|x(?:port|tends))|f(?:or|unc)|i(?:[fns])|ma(?:ster(?:(?:sync)?)|tch)|not|o(?:nready|r)|p(?:ass|uppet(?:(?:sync)?))|re(?:mote(?:(?:sync)?)|turn)|s(?:etget|ignal|tatic)|tool|var|while)\b", KEYWORD),
        Rule::bygroups_to(r"(?m)(func)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::bygroups_to(r"(?m)(class)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::token(r"(?m)(?<!\.)(Color(?:[8N])|a(?:bs|cos|s(?:in|sert)|tan(?:(?:2)?))|bytes2var|c(?:eil|har|lamp|o(?:nvert|s(?:(?:h)?)))|d(?:b2linear|e(?:c(?:imals|time)|g2rad)|ict2inst)|e(?:ase|xp)|f(?:loor|mod|posmod|uncref)|hash|i(?:nst(?:2dict|ance_from_id)|s_(?:inf|nan))|l(?:erp|inear2db|o(?:ad|g))|m(?:ax|in)|nearest_po2|p(?:ow|r(?:eload|int(?:(?:_stack|err|raw|[st])?)))|r(?:a(?:d2deg|n(?:d(?:_(?:range|seed)|omize|[fi])|ge))|ound)|s(?:eed|i(?:gn|n(?:(?:h)?))|qrt|t(?:epify|r(?:(?:2var)?)))|t(?:an(?:(?:(?:h)?)?)|ype(?:_exist|of))|var2(?:bytes|str)|weakref|yield)\b", NAME_BUILTIN),
        Rule::token(r"(?m)((?<!\.)(self|false|true)|(PI|TAU|NAN|INF))\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(Array|Basis|Color|Dictionary|NodePath(?:(?:)?)|Object|P(?:acked(?:(?:Byte|Color|Float(?:32|64)|Int(?:32|64)|String|Vector(?:[23]))Array)|lane)|Quat|R(?:ID|ect(?:[23]))|String|Transform(?:(?:2D)?)|Vector(?:[23])|bool|float|int|null|void)\b", TokenType::new(&["Name", "Builtin", "Type"])),
        Rule::bygroups_to(r#"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::bygroups_to(r#"(?m)([uUbB]?)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)([uUbB]?)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)([uUbB]?)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)([uUbB]?)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(a(?:nd|s)|break(?:(?:point)?)|c(?:lass(?:(?:_name)?)|on(?:st|tinue))|e(?:l(?:if|se)|num|x(?:port|tends))|f(?:or|unc)|i(?:[fns])|ma(?:ster(?:(?:sync)?)|tch)|not|o(?:nready|r)|p(?:ass|uppet(?:(?:sync)?))|re(?:mote(?:(?:sync)?)|turn)|s(?:etget|ignal|tatic)|tool|var|while)\b", KEYWORD),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(?<!\.)(Color(?:[8N])|a(?:bs|cos|s(?:in|sert)|tan(?:(?:2)?))|bytes2var|c(?:eil|har|lamp|o(?:nvert|s(?:(?:h)?)))|d(?:b2linear|e(?:c(?:imals|time)|g2rad)|ict2inst)|e(?:ase|xp)|f(?:loor|mod|posmod|uncref)|hash|i(?:nst(?:2dict|ance_from_id)|s_(?:inf|nan))|l(?:erp|inear2db|o(?:ad|g))|m(?:ax|in)|nearest_po2|p(?:ow|r(?:eload|int(?:(?:_stack|err|raw|[st])?)))|r(?:a(?:d2deg|n(?:d(?:_(?:range|seed)|omize|[fi])|ge))|ound)|s(?:eed|i(?:gn|n(?:(?:h)?))|qrt|t(?:epify|r(?:(?:2var)?)))|t(?:an(?:(?:(?:h)?)?)|ype(?:_exist|of))|var2(?:bytes|str)|weakref|yield)\b", NAME_BUILTIN),
        Rule::token(r"(?m)((?<!\.)(self|false|true)|(PI|TAU|NAN|INF))\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(Array|Basis|Color|Dictionary|NodePath(?:(?:)?)|Object|P(?:acked(?:(?:Byte|Color|Float(?:32|64)|Int(?:32|64)|String|Vector(?:[23]))Array)|lane)|Quat|R(?:ID|ect(?:[23]))|String|Transform(?:(?:2D)?)|Vector(?:[23])|bool|float|int|null|void)\b", TokenType::new(&["Name", "Builtin", "Type"])),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(
        r"tdqs",
        vec![
            Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
            Rule::token(r"(?m)%", STRING_DOUBLE),
            Rule::token(r"(?m)\n", WHITESPACE),
        ],
    );
    m.insert(
        r"strings-double",
        vec![
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
            Rule::token(r"(?m)%", STRING_DOUBLE),
        ],
    );
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%", STRING_DOUBLE),
        Rule::token(r"(?m)\n", WHITESPACE),
    ]);
    m.insert(
        r"tsqs",
        vec![
            Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_SINGLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
            Rule::token(r"(?m)%", STRING_SINGLE),
            Rule::token(r"(?m)\n", WHITESPACE),
        ],
    );
    m.insert(
        r"strings-single",
        vec![
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_SINGLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
            Rule::token(r"(?m)%", STRING_SINGLE),
        ],
    );
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%", STRING_SINGLE),
        Rule::token(r"(?m)\n", WHITESPACE),
    ]);
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_DOUBLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
            Rule::token(r"(?m)%", STRING_DOUBLE),
        ],
    );
    m.insert(r"_tmp_2", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%", STRING_DOUBLE),
    ]);
    m.insert(
        r"sqs",
        vec![
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
            Rule::token(
                r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]",
                STRING_INTERPOL,
            ),
            Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_SINGLE),
            Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
            Rule::token(r"(?m)%", STRING_SINGLE),
        ],
    );
    m.insert(r"_tmp_3", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)%(\(\w+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%", STRING_SINGLE),
    ]);
    m.insert(r"name", vec![Rule::token(r"(?m)[a-zA-Z_]\w*", NAME)]);
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"funcname",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"classname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    Table(m)
}

impl Lexer for GdscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
