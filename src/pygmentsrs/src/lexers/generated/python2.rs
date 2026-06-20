#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.python:Python2Lexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.python:Python2Lexer:python2

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: python2, py2
pub struct Python2Lexer;

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
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)\A#!.+$", COMMENT_HASHBANG),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)\\\n", TEXT),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(in|is|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|[-~+/*%=<>&^|.]", OPERATOR),
        Rule::token(r"(?m)(as(?:(?:sert)?)|break|continue|del|e(?:l(?:if|se)|x(?:cept|ec))|f(?:inally|or)|global|if|lambda|p(?:ass|rint)|r(?:aise|eturn)|try|w(?:hile|ith)|yield(?:(?:\ from)?))\b", KEYWORD),
        Rule::bygroups_to(r"(?m)(def)((?:\s|\\\s)+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::bygroups_to(r"(?m)(class)((?:\s|\\\s)+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::bygroups_to(r"(?m)(from)((?:\s|\\\s)+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"fromimport"])),
        Rule::bygroups_to(r"(?m)(import)((?:\s|\\\s)+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?m)(?<!\.)(__import__|a(?:bs|ll|(?:n|ppl)y)|b(?:asestring|in|ool|uffer|yte(?:array|s))|c(?:allable|hr|lassmethod|mp|o(?:erce|mp(?:ile|lex)))|d(?:elattr|i(?:ct|r|vmod))|e(?:numerate|val|x(?:ecfile|it))|f(?:il(?:e|ter)|(?:loa|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:put|t(?:(?:ern)?))|s(?:instance|subclass)|ter)|l(?:en|ist|o(?:cals|ng))|m(?:a(?:[px])|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|roperty)|r(?:a(?:nge|w_input)|e(?:duce|load|pr|versed)|ound)|s(?:et(?:(?:attr)?)|lice|orted|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|unic(?:hr|ode)|vars|xrange|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|None|Ellipsis|NotImplemented|False|True|cls)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|BaseException|DeprecationWarning|E(?:OFError|nvironmentError|xception)|F(?:loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|nde(?:(?:ntation|x)Error))|Key(?:Error|boardInterrupt)|LookupError|MemoryError|N(?:(?:ame|otImplemented)Error)|O(?:SError|verflow(?:Error|Warning))|PendingDeprecationWarning|R(?:eferenceError|untime(?:Error|Warning))|S(?:t(?:andardError|opIteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|V(?:(?:MS|alue)Error)|W(?:arning|indowsError)|ZeroDivisionError)\b", NAME_EXCEPTION),
        Rule::token(r"(?m)(__(?:(?:a(?:bs|(?:[dn])d)|c(?:all|mp|o(?:erce|mplex|ntains))|d(?:el(?:(?:attr|ete|item|slice)?)|iv(?:(?:mod)?))|e(?:nter|q|xit)|flo(?:at|ordiv)|g(?:et(?:(?:attr(?:(?:ibute)?)|item|slice)?)|[et])|h(?:ash|ex)|i(?:a(?:(?:[dn])d)|div|floordiv|lshift|m(?:od|ul)|n(?:dex|it|stancecheck|(?:(?:ver)?)t)|o(?:[pr])|pow|rshift|sub|t(?:er|ruediv)|xor)|l(?:en|ong|shift|[et])|m(?:issing|od|ul)|n(?:e(?:(?:[gw])?)|onzero)|o(?:ct|[pr])|po(?:[sw])|r(?:a(?:(?:[dn])d)|cmp|div(?:(?:mod)?)|e(?:pr|versed)|floordiv|lshift|m(?:od|ul)|o(?:[pr])|pow|rshift|s(?:hift|ub)|truediv|xor)|s(?:et(?:(?:attr|item|slice)?)|tr|ub(?:(?:classcheck)?))|truediv|unicode|xor)__))\b", NAME_FUNCTION_MAGIC),
        Rule::token(r"(?m)(__(?:(?:bases|c(?:l(?:ass|osure)|ode)|d(?:efaults|ict|oc)|f(?:ile|unc)|globals|m(?:etaclass|odule|ro)|name|s(?:elf|lots)|weakref)__))\b", NAME_VARIABLE_MAGIC),
        Rule::token(r"(?m)`.*?`", STRING_BACKTICK),
        Rule::bygroups_to(r#"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)([rR]|[uUbB][rR]|[rR][uUbB])(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::bygroups_to(r#"(?m)([uUbB]?)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)([uUbB]?)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)([uUbB]?)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)([uUbB]?)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)@[\w.]+", NAME_DECORATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(as(?:(?:sert)?)|break|continue|del|e(?:l(?:if|se)|x(?:cept|ec))|f(?:inally|or)|global|if|lambda|p(?:ass|rint)|r(?:aise|eturn)|try|w(?:hile|ith)|yield(?:(?:\ from)?))\b", KEYWORD),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(?<!\.)(__import__|a(?:bs|ll|(?:n|ppl)y)|b(?:asestring|in|ool|uffer|yte(?:array|s))|c(?:allable|hr|lassmethod|mp|o(?:erce|mp(?:ile|lex)))|d(?:elattr|i(?:ct|r|vmod))|e(?:numerate|val|x(?:ecfile|it))|f(?:il(?:e|ter)|(?:loa|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:put|t(?:(?:ern)?))|s(?:instance|subclass)|ter)|l(?:en|ist|o(?:cals|ng))|m(?:a(?:[px])|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|roperty)|r(?:a(?:nge|w_input)|e(?:duce|load|pr|versed)|ound)|s(?:et(?:(?:attr)?)|lice|orted|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|unic(?:hr|ode)|vars|xrange|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|None|Ellipsis|NotImplemented|False|True|cls)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|BaseException|DeprecationWarning|E(?:OFError|nvironmentError|xception)|F(?:loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|nde(?:(?:ntation|x)Error))|Key(?:Error|boardInterrupt)|LookupError|MemoryError|N(?:(?:ame|otImplemented)Error)|O(?:SError|verflow(?:Error|Warning))|PendingDeprecationWarning|R(?:eferenceError|untime(?:Error|Warning))|S(?:t(?:andardError|opIteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|V(?:(?:MS|alue)Error)|W(?:arning|indowsError)|ZeroDivisionError)\b", NAME_EXCEPTION),
    ]);
    m.insert(r"magicfuncs", vec![
        Rule::token(r"(?m)(__(?:(?:a(?:bs|(?:[dn])d)|c(?:all|mp|o(?:erce|mplex|ntains))|d(?:el(?:(?:attr|ete|item|slice)?)|iv(?:(?:mod)?))|e(?:nter|q|xit)|flo(?:at|ordiv)|g(?:et(?:(?:attr(?:(?:ibute)?)|item|slice)?)|[et])|h(?:ash|ex)|i(?:a(?:(?:[dn])d)|div|floordiv|lshift|m(?:od|ul)|n(?:dex|it|stancecheck|(?:(?:ver)?)t)|o(?:[pr])|pow|rshift|sub|t(?:er|ruediv)|xor)|l(?:en|ong|shift|[et])|m(?:issing|od|ul)|n(?:e(?:(?:[gw])?)|onzero)|o(?:ct|[pr])|po(?:[sw])|r(?:a(?:(?:[dn])d)|cmp|div(?:(?:mod)?)|e(?:pr|versed)|floordiv|lshift|m(?:od|ul)|o(?:[pr])|pow|rshift|s(?:hift|ub)|truediv|xor)|s(?:et(?:(?:attr|item|slice)?)|tr|ub(?:(?:classcheck)?))|truediv|unicode|xor)__))\b", NAME_FUNCTION_MAGIC),
    ]);
    m.insert(r"magicvars", vec![
        Rule::token(r"(?m)(__(?:(?:bases|c(?:l(?:ass|osure)|ode)|d(?:efaults|ict|oc)|f(?:ile|unc)|globals|m(?:etaclass|odule|ro)|name|s(?:elf|lots)|weakref)__))\b", NAME_VARIABLE_MAGIC),
    ]);
    m.insert(
        r"backtick",
        vec![Rule::token(r"(?m)`.*?`", STRING_BACKTICK)],
    );
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
            Rule::token(r"(?m)\n", STRING_DOUBLE),
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
        Rule::token(r"(?m)\n", STRING_DOUBLE),
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
            Rule::token(r"(?m)\n", STRING_SINGLE),
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
        Rule::token(r"(?m)\n", STRING_SINGLE),
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
    m.insert(
        r"name",
        vec![
            Rule::token(r"(?m)@[\w.]+", NAME_DECORATOR),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?j?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+[eE][+-]?[0-9]+j?", NUMBER_FLOAT),
            Rule::token(r"(?m)0[0-7]+j?", NUMBER_OCT),
            Rule::token(r"(?m)0[bB][01]+", NUMBER_BIN),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(
                r"(?m)\d+L",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)\d+j?", NUMBER_INTEGER),
        ],
    );
    m.insert(r"funcname", vec![
        Rule::token(r"(?m)(__(?:(?:a(?:bs|(?:[dn])d)|c(?:all|mp|o(?:erce|mplex|ntains))|d(?:el(?:(?:attr|ete|item|slice)?)|iv(?:(?:mod)?))|e(?:nter|q|xit)|flo(?:at|ordiv)|g(?:et(?:(?:attr(?:(?:ibute)?)|item|slice)?)|[et])|h(?:ash|ex)|i(?:a(?:(?:[dn])d)|div|floordiv|lshift|m(?:od|ul)|n(?:dex|it|stancecheck|(?:(?:ver)?)t)|o(?:[pr])|pow|rshift|sub|t(?:er|ruediv)|xor)|l(?:en|ong|shift|[et])|m(?:issing|od|ul)|n(?:e(?:(?:[gw])?)|onzero)|o(?:ct|[pr])|po(?:[sw])|r(?:a(?:(?:[dn])d)|cmp|div(?:(?:mod)?)|e(?:pr|versed)|floordiv|lshift|m(?:od|ul)|o(?:[pr])|pow|rshift|s(?:hift|ub)|truediv|xor)|s(?:et(?:(?:attr|item|slice)?)|tr|ub(?:(?:classcheck)?))|truediv|unicode|xor)__))\b", NAME_FUNCTION_MAGIC),
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(
        r"classname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_]\w*",
            NAME_CLASS,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"import",
        vec![
            Rule::token(r"(?m)(?:[ \t]|\\\n)+", TEXT),
            Rule::token(r"(?m)as\b", KEYWORD_NAMESPACE),
            Rule::token(r"(?m),", OPERATOR),
            Rule::token(r"(?m)[a-zA-Z_][\w.]*", NAME_NAMESPACE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fromimport",
        vec![
            Rule::token(r"(?m)(?:[ \t]|\\\n)+", TEXT),
            Rule::token_to(r"(?m)import\b", KEYWORD_NAMESPACE, NewState::Pop(1)),
            Rule::token_to(r"(?m)None\b", NAME_BUILTIN_PSEUDO, NewState::Pop(1)),
            Rule::token(r"(?m)[a-zA-Z_.][\w.]*", NAME_NAMESPACE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for Python2Lexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
