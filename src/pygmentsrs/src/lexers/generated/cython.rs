//! AUTO-GENERATED from `pygments.pygments.lexers.python:CythonLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.python:CythonLexer:cython

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: cython, pyx, pyrex
pub struct CythonLexer;

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
        Rule::bygroups(r#"(?m)^(\s*)("""(?:.|\n)*?""")"#, vec![Some(WHITESPACE), Some(STRING_DOC)]),
        Rule::bygroups(r"(?m)^(\s*)('''(?:.|\n)*?''')", vec![Some(WHITESPACE), Some(STRING_DOC)]),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)#.*$", COMMENT),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)\\\n", WHITESPACE),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(in|is|and|or|not)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)(<)([a-zA-Z0-9.?]+)(>)", vec![Some(PUNCTUATION), Some(KEYWORD_TYPE), Some(PUNCTUATION)]),
        Rule::token(r"(?m)!=|==|<<|>>|[-~+/*%=<>&^|.?]", OPERATOR),
        Rule::bygroups(r"(?m)(from)(\d+)(<=)(\s+)(<)(\d+)(:)", vec![Some(KEYWORD), Some(NUMBER_INTEGER), Some(OPERATOR), Some(WHITESPACE), Some(OPERATOR), Some(NAME), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(a(?:s(?:(?:sert|ync)?)|wait)|b(?:reak|y)|c(?:ontinue|typedef)|del|e(?:l(?:if|se)|x(?:cept(?:(?:\?)?)|ec))|f(?:inally|or|used)|g(?:(?:i|loba)l)|i(?:f|nclude)|lambda|n(?:amespace|ew|o(?:except|gil))|p(?:ass|rint)|r(?:aise|eturn)|try|w(?:hile|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(False|N(?:ULL|one)|True)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(DEF|IF|ELIF|ELSE)\b", COMMENT_PREPROC),
        Rule::bygroups_to(r"(?m)(def|property)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::bygroups_to(r"(?m)(cp?def)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"cdef"])),
        Rule::bygroups(r"(?m)(cdef)(:)", vec![Some(KEYWORD), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)(class|cppclass|struct)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"classname"])),
        Rule::bygroups_to(r"(?m)(from)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"fromimport"])),
        Rule::bygroups_to(r"(?m)(c?import)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?m)(?<!\.)(Py_ssize_t|__import__|a(?:bs|ll|(?:n|ppl)y)|b(?:asestring|in(?:(?:t)?)|ool|uffer|yte(?:array|s))|c(?:allable|h(?:(?:(?:a)?)r)|lassmethod|mp|o(?:erce|mp(?:ile|lex)))|d(?:elattr|i(?:ct|r|vmod)|ouble)|e(?:numerate|val|x(?:ecfile|it))|f(?:il(?:e|ter)|(?:loa|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:put|t(?:(?:ern)?))|s(?:instance|subclass)|ter)|l(?:en|ist|o(?:cals|ng))|m(?:a(?:[px])|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|roperty)|r(?:a(?:nge|w_input)|e(?:duce|load|pr|versed)|ound)|s(?:et(?:(?:attr)?)|ize_t|lice|orted|size_t|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|un(?:ic(?:hr|ode)|signed)|vars|xrange|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|cls|Ellipsis|NotImplemented)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|BaseException|DeprecationWarning|E(?:OFError|nvironmentError|xception)|F(?:loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|nde(?:(?:ntation|x)Error))|Key(?:Error|boardInterrupt)|LookupError|MemoryError|N(?:(?:ame|otImplemented)Error)|O(?:SError|verflow(?:Error|Warning))|PendingDeprecationWarning|R(?:eferenceError|untime(?:Error|Warning))|S(?:t(?:andardError|opIteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|ValueError|Warning|ZeroDivisionError)\b", NAME_EXCEPTION),
        Rule::token(r"(?m)`.*?`", STRING_BACKTICK),
        Rule::token_to(r#"(?m)(?:[rR]|[uU][rR]|[rR][uU])""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?m)(?:[rR]|[uU][rR]|[rR][uU])'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?m)(?:[rR]|[uU][rR]|[rR][uU])""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?m)(?:[rR]|[uU][rR]|[rR][uU])'", STRING, NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)[uU]?""""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)[uU]?'''", STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r#"(?m)[uU]?""#, STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token_to(r"(?m)[uU]?'", STRING, NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)@\w+", NAME_DECORATOR),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)0\d+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(a(?:s(?:(?:sert|ync)?)|wait)|b(?:reak|y)|c(?:ontinue|typedef)|del|e(?:l(?:if|se)|x(?:cept(?:(?:\?)?)|ec))|f(?:inally|or|used)|g(?:(?:i|loba)l)|i(?:f|nclude)|lambda|n(?:amespace|ew|o(?:except|gil))|p(?:ass|rint)|r(?:aise|eturn)|try|w(?:hile|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(False|N(?:ULL|one)|True)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(DEF|IF|ELIF|ELSE)\b", COMMENT_PREPROC),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(?<!\.)(Py_ssize_t|__import__|a(?:bs|ll|(?:n|ppl)y)|b(?:asestring|in(?:(?:t)?)|ool|uffer|yte(?:array|s))|c(?:allable|h(?:(?:(?:a)?)r)|lassmethod|mp|o(?:erce|mp(?:ile|lex)))|d(?:elattr|i(?:ct|r|vmod)|ouble)|e(?:numerate|val|x(?:ecfile|it))|f(?:il(?:e|ter)|(?:loa|rozense)t)|g(?:etattr|lobals)|h(?:as(?:attr|h)|ex)|i(?:d|n(?:put|t(?:(?:ern)?))|s(?:instance|subclass)|ter)|l(?:en|ist|o(?:cals|ng))|m(?:a(?:[px])|in)|next|o(?:bject|ct|pen|rd)|p(?:ow|roperty)|r(?:a(?:nge|w_input)|e(?:duce|load|pr|versed)|ound)|s(?:et(?:(?:attr)?)|ize_t|lice|orted|size_t|t(?:aticmethod|r)|u(?:m|per))|t(?:(?:upl|yp)e)|un(?:ic(?:hr|ode)|signed)|vars|xrange|zip)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?<!\.)(self|cls|Ellipsis|NotImplemented)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?<!\.)(A(?:(?:rithmetic|ssertion|ttribute)Error)|BaseException|DeprecationWarning|E(?:OFError|nvironmentError|xception)|F(?:loatingPointError|utureWarning)|GeneratorExit|I(?:OError|mport(?:Error|Warning)|nde(?:(?:ntation|x)Error))|Key(?:Error|boardInterrupt)|LookupError|MemoryError|N(?:(?:ame|otImplemented)Error)|O(?:SError|verflow(?:Error|Warning))|PendingDeprecationWarning|R(?:eferenceError|untime(?:Error|Warning))|S(?:t(?:andardError|opIteration)|y(?:ntax(?:Error|Warning)|stemE(?:rror|xit)))|T(?:(?:ab|ype)Error)|U(?:n(?:boundLocalError|icode(?:DecodeError|E(?:(?:(?:ncodeE)?)rror)|TranslateError|Warning))|serWarning)|ValueError|Warning|ZeroDivisionError)\b", NAME_EXCEPTION),
    ]);
    m.insert(
        r"backtick",
        vec![Rule::token(r"(?m)`.*?`", STRING_BACKTICK)],
    );
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(r"tdqs", vec![
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
    ]);
    m.insert(r"nl", vec![Rule::token(r"(?m)\n", STRING)]);
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"tsqs", vec![
        Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
    ]);
    m.insert(r"_tmp_2", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
    ]);
    m.insert(r"sqs", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
    ]);
    m.insert(r"_tmp_3", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)%(\([a-zA-Z0-9]+\))?[-#0 +]*([0-9]+|[*])?(\.([0-9]+|[*]))?[hlL]?[E-GXc-giorsux%]", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%\n]+"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)%", STRING),
    ]);
    m.insert(
        r"name",
        vec![
            Rule::token(r"(?m)@\w+", NAME_DECORATOR),
            Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
            Rule::token(r"(?m)0\d+", NUMBER_OCT),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(
                r"(?m)\d+L",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"funcname",
        vec![Rule::token_to(
            r"(?m)[a-zA-Z_]\w*",
            NAME_FUNCTION,
            NewState::Pop(1),
        )],
    );
    m.insert(
        r"cdef",
        vec![
            Rule::token(
                r"(?m)(public|readonly|extern|api|inline|packed)\b",
                KEYWORD_RESERVED,
            ),
            Rule::bygroups_to(
                r"(?m)(struct|enum|union|class|cppclass)\b(\s+)([a-zA-Z_]\w*)",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_]\w*)(\s*)(?=\()",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)([a-zA-Z_]\w*)(\s*)(?=[:,=#\n]|$)",
                vec![Some(NAME_VARIABLE), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups(
                r"(?m)([a-zA-Z_]\w*)(\s*)(,)",
                vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)from\b", KEYWORD, NewState::Pop(1)),
            Rule::token(r"(?m)as\b", KEYWORD),
            Rule::token_to(r"(?m):", PUNCTUATION, NewState::Pop(1)),
            Rule::token_to(r#"(?m)(?=["\'])"#, TEXT, NewState::Pop(1)),
            Rule::token(r"(?m)[a-zA-Z_]\w*", KEYWORD_TYPE),
            Rule::token(r"(?m).", TEXT),
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
    m.insert(
        r"import",
        vec![
            Rule::bygroups(
                r"(?m)(\s+)(as)(\s+)",
                vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)[a-zA-Z_][\w.]*", NAME_NAMESPACE),
            Rule::bygroups(
                r"(?m)(\s*)(,)(\s*)",
                vec![Some(WHITESPACE), Some(OPERATOR), Some(WHITESPACE)],
            ),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"fromimport",
        vec![
            Rule::bygroups_to(
                r"(?m)(\s+)(c?import)\b",
                vec![Some(WHITESPACE), Some(KEYWORD)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[a-zA-Z_.][\w.]*", NAME_NAMESPACE),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for CythonLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
