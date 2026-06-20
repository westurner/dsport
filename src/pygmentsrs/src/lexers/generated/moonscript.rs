//! AUTO-GENERATED from `pygments.pygments.lexers.scripting:MoonScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.scripting:MoonScriptLexer:moonscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: moonscript, moon
pub struct MoonscriptLexer;

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
        r"root",
        vec![
            Rule::token(r"(?m)#!(.*?)$", COMMENT_PREPROC),
            Rule::default(NewState::Push(vec![r"base"])),
        ],
    );
    m.insert(r"base", vec![
        Rule::token(r"(?m)--.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(?i)(\d*\.\d+|\d+\.\d*)(e[+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)\d+e[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)0x[0-9a-f]*", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(?s)\[(=*)\[.*?\]\1\]", STRING),
        Rule::token(r"(?m)(->|=>)", NAME_FUNCTION),
        Rule::token(r"(?m):[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(==|!=|~=|<=|>=|\.\.\.|\.\.|[=+\-*/%^<>#!.\\:])", OPERATOR),
        Rule::token(r"(?m)[;,]", PUNCTUATION),
        Rule::token(r"(?m)[\[\]{}()]", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:", NAME_VARIABLE),
        Rule::token(r"(?m)(and|break|class|do|e(?:lse(?:(?:if)?)|x(?:port|tends))|f(?:or|rom)|i(?:mport|[fn])|not|or|return|s(?:uper|witch)|then|using|w(?:h(?:en|ile)|ith))\b", KEYWORD),
        Rule::token(r"(?m)(true|false|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(self)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)@@?([a-zA-Z_]\w*)?", NAME_VARIABLE_CLASS),
        Rule::token(r"(?m)[A-Z]\w*", NAME_CLASS),
        Rule::token(r"(?m)(_(?:G|VERSION)|assert|bit32\.(?:arshift|b(?:and|not|or|test|xor)|extract|l(?:rotate|shift)|r(?:eplace|rotate|shift))|co(?:llectgarbage|routine\.(?:c(?:(?:los|reat)e)|isyieldable|r(?:esume|unning)|status|wrap|yield))|d(?:ebug\.(?:debug|get(?:hook|info|local|metatable|registry|u(?:(?:p|ser)value))|set(?:hook|local|(?:metatabl|u(?:(?:p|ser)valu))e)|traceback|upvalue(?:id|join))|ofile)|error|getmetatable|i(?:o\.(?:close|flush|input|lines|o(?:pen|utput)|popen|read|std(?:err|in|out)|(?:t(?:mpfil|yp)|writ)e)|pairs)|load(?:(?:file)?)|math\.(?:a(?:bs|cos|sin|tan(?:(?:2)?))|c(?:eil|os(?:(?:h)?))|deg|exp|f(?:loor|mod|rexp)|huge|l(?:dexp|og)|m(?:ax(?:(?:integer)?)|in(?:(?:integer)?)|odf)|p(?:i|ow)|ra(?:d|ndom(?:(?:seed)?))|s(?:in(?:(?:h)?)|qrt)|t(?:an(?:(?:h)?)|ointeger|ype)|ult)|next|os\.(?:clock|d(?:(?:at|ifftim)e)|ex(?:ecute|it)|getenv|(?:re(?:mov|nam)|setlocal|t(?:(?:i|mpna)m))e)|p(?:a(?:ckage\.(?:c(?:onfig|path)|load(?:ed|lib)|p(?:ath|reload)|search(?:ers|path))|irs)|call|rint)|r(?:aw(?:equal|get|len|set)|equire)|s(?:e(?:lect|tmetatable)|tring\.(?:byte|char|dump|f(?:ind|ormat)|g(?:match|sub)|l(?:en|ower)|match|pack(?:(?:size)?)|re(?:p|verse)|sub|u(?:npack|pper)))|t(?:able\.(?:concat|insert|move|pack|remove|sort|unpack)|o(?:number|string)|ype)|utf8\.(?:c(?:har(?:(?:pattern)?)|ode(?:point|s))|len|offset)|warn|xpcall)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Za-z_]\w*", NAME),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_1"])),
    ]);
    m.insert(
        r"stringescape",
        vec![Rule::token(
            r#"(?m)\\([abfnrtv\\"']|\d{1,3})"#,
            STRING_ESCAPE,
        )],
    );
    m.insert(
        r"sqs",
        vec![
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r#"(?m)#|\\.|""#, STRING),
            Rule::token(r#"(?m)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(r"strings", vec![Rule::token(r#"(?m)[^#\\\'"]+"#, STRING)]);
    m.insert(
        r"_tmp_0",
        vec![
            Rule::token(r#"(?m)\\([abfnrtv\\"']|\d{1,3})"#, STRING_ESCAPE),
            Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r#"(?m)#|\\.|""#, STRING),
            Rule::token(r#"(?m)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.|\'", STRING),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r"(?m)#", STRING),
            Rule::token(r#"(?m)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"_tmp_1",
        vec![
            Rule::token(r#"(?m)\\([abfnrtv\\"']|\d{1,3})"#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r"(?m)\\.|\'", STRING),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r"(?m)#", STRING),
            Rule::token(r#"(?m)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(r"interpoling_string", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)--.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)(?i)(\d*\.\d+|\d+\.\d*)(e[+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)\d+e[+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(?i)0x[0-9a-f]*", NUMBER_HEX),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(?s)\[(=*)\[.*?\]\1\]", STRING),
        Rule::token(r"(?m)(->|=>)", NAME_FUNCTION),
        Rule::token(r"(?m):[a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)(==|!=|~=|<=|>=|\.\.\.|\.\.|[=+\-*/%^<>#!.\\:])", OPERATOR),
        Rule::token(r"(?m)[;,]", PUNCTUATION),
        Rule::token(r"(?m)[\[\]{}()]", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:", NAME_VARIABLE),
        Rule::token(r"(?m)(and|break|class|do|e(?:lse(?:(?:if)?)|x(?:port|tends))|f(?:or|rom)|i(?:mport|[fn])|not|or|return|s(?:uper|witch)|then|using|w(?:h(?:en|ile)|ith))\b", KEYWORD),
        Rule::token(r"(?m)(true|false|nil)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)(self)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)@@?([a-zA-Z_]\w*)?", NAME_VARIABLE_CLASS),
        Rule::token(r"(?m)[A-Z]\w*", NAME_CLASS),
        Rule::token(r"(?m)(_(?:G|VERSION)|assert|bit32\.(?:arshift|b(?:and|not|or|test|xor)|extract|l(?:rotate|shift)|r(?:eplace|rotate|shift))|co(?:llectgarbage|routine\.(?:c(?:(?:los|reat)e)|isyieldable|r(?:esume|unning)|status|wrap|yield))|d(?:ebug\.(?:debug|get(?:hook|info|local|metatable|registry|u(?:(?:p|ser)value))|set(?:hook|local|(?:metatabl|u(?:(?:p|ser)valu))e)|traceback|upvalue(?:id|join))|ofile)|error|getmetatable|i(?:o\.(?:close|flush|input|lines|o(?:pen|utput)|popen|read|std(?:err|in|out)|(?:t(?:mpfil|yp)|writ)e)|pairs)|load(?:(?:file)?)|math\.(?:a(?:bs|cos|sin|tan(?:(?:2)?))|c(?:eil|os(?:(?:h)?))|deg|exp|f(?:loor|mod|rexp)|huge|l(?:dexp|og)|m(?:ax(?:(?:integer)?)|in(?:(?:integer)?)|odf)|p(?:i|ow)|ra(?:d|ndom(?:(?:seed)?))|s(?:in(?:(?:h)?)|qrt)|t(?:an(?:(?:h)?)|ointeger|ype)|ult)|next|os\.(?:clock|d(?:(?:at|ifftim)e)|ex(?:ecute|it)|getenv|(?:re(?:mov|nam)|setlocal|t(?:(?:i|mpna)m))e)|p(?:a(?:ckage\.(?:c(?:onfig|path)|load(?:ed|lib)|p(?:ath|reload)|search(?:ers|path))|irs)|call|rint)|r(?:aw(?:equal|get|len|set)|equire)|s(?:e(?:lect|tmetatable)|tring\.(?:byte|char|dump|f(?:ind|ormat)|g(?:match|sub)|l(?:en|ower)|match|pack(?:(?:size)?)|re(?:p|verse)|sub|u(?:npack|pper)))|t(?:able\.(?:concat|insert|move|pack|remove|sort|unpack)|o(?:number|string)|ype)|utf8\.(?:c(?:har(?:(?:pattern)?)|ode(?:point|s))|len|offset)|warn|xpcall)\b", NAME_BUILTIN),
        Rule::token(r"(?m)[A-Za-z_]\w*", NAME),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_1"])),
    ]);
    m.insert(
        r"ws",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)(?:\s+(?!\s))", WHITESPACE),
        ],
    );
    m.insert(
        r"varname",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)(?:\s+(?!\s))", WHITESPACE),
            Rule::token_to(r"(?m)\.\.", OPERATOR, NewState::Pop(1)),
            Rule::token(r"(?m)[.:]", PUNCTUATION),
            Rule::token(r"(?m)(?:[^\W\d]\w*)(?=\s*[.:])", NAME_PROPERTY),
            Rule::token_to(
                r"(?m)(?:[^\W\d]\w*)(?=\s*\()",
                NAME_FUNCTION,
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)(?:[^\W\d]\w*)", NAME_PROPERTY, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"funcname",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)(?:\s+(?!\s))", WHITESPACE),
            Rule::token(r"(?m)[.:]", PUNCTUATION),
            Rule::token(r"(?m)(?:[^\W\d]\w*)(?=\s*[.:])", NAME_CLASS),
            Rule::token_to(r"(?m)(?:[^\W\d]\w*)", NAME_FUNCTION, NewState::Pop(1)),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"goto",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)(?:\s+(?!\s))", WHITESPACE),
            Rule::token_to(r"(?m)(?:[^\W\d]\w*)", NAME_LABEL, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"label",
        vec![
            Rule::token(
                r"(?m)(?:--\[(?P<level>=*)\[[\w\W]*?\](?P=level)\])",
                COMMENT_MULTILINE,
            ),
            Rule::token(r"(?m)(?:--.*$)", COMMENT_SINGLE),
            Rule::token(r"(?m)(?:\s+(?!\s))", WHITESPACE),
            Rule::token_to(r"(?m)::", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)(?:[^\W\d]\w*)", NAME_LABEL),
        ],
    );
    Table(m)
}

impl Lexer for MoonscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
