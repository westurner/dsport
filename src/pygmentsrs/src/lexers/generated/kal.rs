//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:KalLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:KalLexer:kal

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: kal
pub struct KalLexer;

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
        r"commentsandwhitespace",
        vec![
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#(?!##[^#]).*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
        ],
    );
    m.insert(
        r"functiondef",
        vec![
            Rule::bygroups_to(
                r"(?ms)([$a-zA-Z_][\w$]*)(\s*)",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#(?!##[^#]).*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
        ],
    );
    m.insert(
        r"classdef",
        vec![
            Rule::bygroups(
                r"(?ms)\b(inherits)(\s+)(from)\b",
                vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)],
            ),
            Rule::token_to(
                r"(?ms)([$a-zA-Z_][\w$]*)(?=\s*\n)",
                NAME_CLASS,
                NewState::Pop(1),
            ),
            Rule::token(r"(?ms)[$a-zA-Z_][\w$]*\b", NAME_CLASS),
            Rule::token(r"(?ms)\s+", WHITESPACE),
            Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
            Rule::bygroups(
                r"(?ms)(#(?!##[^#]).*?)(\n)",
                vec![Some(COMMENT_SINGLE), Some(WHITESPACE)],
            ),
        ],
    );
    m.insert(r"listcomprehension", vec![
        Rule::token_to(r"(?ms)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?ms)\b(property|value)\b", KEYWORD),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#(?!##[^#]).*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/(?! )(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)", STRING_REGEX),
        Rule::token(r"(?ms)\?|:|_(?=\n)|==?|!=|-(?!>)|[<>+*/-]=?", OPERATOR),
        Rule::token(r"(?ms)\b(and|or|isnt|is|not|but|bitwise|mod|\^|xor|exists|doesnt\s+exist)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?ms)(\([^()]+\))?(\s*)(>)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?ms)[{(]", PUNCTUATION),
        Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::Push(vec![r"listcomprehension"])),
        Rule::token(r"(?ms)[})\].,]", PUNCTUATION),
        Rule::token_to(r"(?ms)\b(function|method|task)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"functiondef"])),
        Rule::token_to(r"(?ms)\bclass\b", KEYWORD_DECLARATION, NewState::Push(vec![r"classdef"])),
        Rule::bygroups_to(r"(?ms)\b(safe(?=\s))?(\s*)(wait(?=\s))(\s+)(for)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"waitfor"])),
        Rule::token(r"(?ms)\b(me|this)(\.[$a-zA-Z_][\w.$]*)?\b", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?ms)(?<![.$])(run)(\s+)(in)(\s+)(parallel)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(except)(\s+)(when)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(fail)(\s+)(with)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(inherits)(\s+)(from)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?ms)(?<![.$])(break|c(?:atch|ontinue)|delete|else|finally|i(?:nstanceof|[fn])|new|o(?:f|therwise)|r(?:aise|eturn)|super|t(?:hrow|ry|ypeof)|un(?:less|til)|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?ms)(?<![.$])(Infinity|NaN|false|n(?:o(?:(?:ne|thing)?)|ull)|o(?:ff|n)|true|undefined|yes)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|S(?:tring|ymbol)|d(?:ecodeURI(?:(?:Component)?)|ocument)|e(?:ncodeURI(?:(?:Component)?)|val)|globalThis|is(?:Finite|NaN|SafeInteger)|p(?:(?:arse(?:Floa|In)|rin)t)|window)\b", NAME_BUILTIN),
        Rule::bygroups(r"(?ms)([$a-zA-Z_][\w.$]*)(\s*)(:|[+\-*/]?\=)?\b", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#(?!##[^#]).*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/(?! )(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)", STRING_REGEX),
        Rule::token(r"(?ms)\?|:|_(?=\n)|==?|!=|-(?!>)|[<>+*/-]=?", OPERATOR),
        Rule::token(r"(?ms)\b(and|or|isnt|is|not|but|bitwise|mod|\^|xor|exists|doesnt\s+exist)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?ms)(\([^()]+\))?(\s*)(>)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?ms)[{(]", PUNCTUATION),
        Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::Push(vec![r"listcomprehension"])),
        Rule::token(r"(?ms)[})\].,]", PUNCTUATION),
        Rule::token_to(r"(?ms)\b(function|method|task)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"functiondef"])),
        Rule::token_to(r"(?ms)\bclass\b", KEYWORD_DECLARATION, NewState::Push(vec![r"classdef"])),
        Rule::bygroups_to(r"(?ms)\b(safe(?=\s))?(\s*)(wait(?=\s))(\s+)(for)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"waitfor"])),
        Rule::token(r"(?ms)\b(me|this)(\.[$a-zA-Z_][\w.$]*)?\b", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?ms)(?<![.$])(run)(\s+)(in)(\s+)(parallel)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(except)(\s+)(when)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(fail)(\s+)(with)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(inherits)(\s+)(from)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?ms)(?<![.$])(break|c(?:atch|ontinue)|delete|else|finally|i(?:nstanceof|[fn])|new|o(?:f|therwise)|r(?:aise|eturn)|super|t(?:hrow|ry|ypeof)|un(?:less|til)|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?ms)(?<![.$])(Infinity|NaN|false|n(?:o(?:(?:ne|thing)?)|ull)|o(?:ff|n)|true|undefined|yes)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|S(?:tring|ymbol)|d(?:ecodeURI(?:(?:Component)?)|ocument)|e(?:ncodeURI(?:(?:Component)?)|val)|globalThis|is(?:Finite|NaN|SafeInteger)|p(?:(?:arse(?:Floa|In)|rin)t)|window)\b", NAME_BUILTIN),
        Rule::bygroups(r"(?ms)([$a-zA-Z_][\w.$]*)(\s*)(:|[+\-*/]?\=)?\b", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
    ]);
    m.insert(r"waitfor", vec![
        Rule::token_to(r"(?ms)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?ms)\bfrom\b", KEYWORD),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#(?!##[^#]).*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/(?! )(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)", STRING_REGEX),
        Rule::token(r"(?ms)\?|:|_(?=\n)|==?|!=|-(?!>)|[<>+*/-]=?", OPERATOR),
        Rule::token(r"(?ms)\b(and|or|isnt|is|not|but|bitwise|mod|\^|xor|exists|doesnt\s+exist)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?ms)(\([^()]+\))?(\s*)(>)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?ms)[{(]", PUNCTUATION),
        Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::Push(vec![r"listcomprehension"])),
        Rule::token(r"(?ms)[})\].,]", PUNCTUATION),
        Rule::token_to(r"(?ms)\b(function|method|task)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"functiondef"])),
        Rule::token_to(r"(?ms)\bclass\b", KEYWORD_DECLARATION, NewState::Push(vec![r"classdef"])),
        Rule::bygroups_to(r"(?ms)\b(safe(?=\s))?(\s*)(wait(?=\s))(\s+)(for)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"waitfor"])),
        Rule::token(r"(?ms)\b(me|this)(\.[$a-zA-Z_][\w.$]*)?\b", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?ms)(?<![.$])(run)(\s+)(in)(\s+)(parallel)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(except)(\s+)(when)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(fail)(\s+)(with)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(inherits)(\s+)(from)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?ms)(?<![.$])(break|c(?:atch|ontinue)|delete|else|finally|i(?:nstanceof|[fn])|new|o(?:f|therwise)|r(?:aise|eturn)|super|t(?:hrow|ry|ypeof)|un(?:less|til)|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?ms)(?<![.$])(Infinity|NaN|false|n(?:o(?:(?:ne|thing)?)|ull)|o(?:ff|n)|true|undefined|yes)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|S(?:tring|ymbol)|d(?:ecodeURI(?:(?:Component)?)|ocument)|e(?:ncodeURI(?:(?:Component)?)|val)|globalThis|is(?:Finite|NaN|SafeInteger)|p(?:(?:arse(?:Floa|In)|rin)t)|window)\b", NAME_BUILTIN),
        Rule::bygroups(r"(?ms)([$a-zA-Z_][\w.$]*)(\s*)(:|[+\-*/]?\=)?\b", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
    ]);
    m.insert(r"strings", vec![Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING)]);
    m.insert(r"interpoling_string", vec![
        Rule::token_to(r"(?ms)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)###[^#].*?###", COMMENT_MULTILINE),
        Rule::bygroups(r"(?ms)(#(?!##[^#]).*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?ms)/(?! )(\\.|[^\[/\\\n]|\[(\\.|[^\]\\\n])*])+/([gimuysd]+\b|\B)", STRING_REGEX),
        Rule::token(r"(?ms)\?|:|_(?=\n)|==?|!=|-(?!>)|[<>+*/-]=?", OPERATOR),
        Rule::token(r"(?ms)\b(and|or|isnt|is|not|but|bitwise|mod|\^|xor|exists|doesnt\s+exist)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?ms)(\([^()]+\))?(\s*)(>)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?ms)[{(]", PUNCTUATION),
        Rule::token_to(r"(?ms)\[", PUNCTUATION, NewState::Push(vec![r"listcomprehension"])),
        Rule::token(r"(?ms)[})\].,]", PUNCTUATION),
        Rule::token_to(r"(?ms)\b(function|method|task)\b", KEYWORD_DECLARATION, NewState::Push(vec![r"functiondef"])),
        Rule::token_to(r"(?ms)\bclass\b", KEYWORD_DECLARATION, NewState::Push(vec![r"classdef"])),
        Rule::bygroups_to(r"(?ms)\b(safe(?=\s))?(\s*)(wait(?=\s))(\s+)(for)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)], NewState::Push(vec![r"waitfor"])),
        Rule::token(r"(?ms)\b(me|this)(\.[$a-zA-Z_][\w.$]*)?\b", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?ms)(?<![.$])(run)(\s+)(in)(\s+)(parallel)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(except)(\s+)(when)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(fail)(\s+)(with)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(inherits)(\s+)(from)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?ms)(?<![.$])(for)(\s+)(parallel|series)?\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?ms)(?<![.$])(break|c(?:atch|ontinue)|delete|else|finally|i(?:nstanceof|[fn])|new|o(?:f|therwise)|r(?:aise|eturn)|super|t(?:hrow|ry|ypeof)|un(?:less|til)|wh(?:en|ile))\b", KEYWORD),
        Rule::token(r"(?ms)(?<![.$])(Infinity|NaN|false|n(?:o(?:(?:ne|thing)?)|ull)|o(?:ff|n)|true|undefined|yes)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(Array|Boolean|Date|Error|Function|Math|Number|Object|RegExp|S(?:tring|ymbol)|d(?:ecodeURI(?:(?:Component)?)|ocument)|e(?:ncodeURI(?:(?:Component)?)|val)|globalThis|is(?:Finite|NaN|SafeInteger)|p(?:(?:arse(?:Floa|In)|rin)t)|window)\b", NAME_BUILTIN),
        Rule::bygroups(r"(?ms)([$a-zA-Z_][\w.$]*)(\s*)(:|[+\-*/]?\=)?\b", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token_to(r#"(?ms)""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?ms)'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?ms)""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?ms)'", STRING, NewState::Push(vec![r"sqs"])),
    ]);
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?ms)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?ms)\\.|\'", STRING),
            Rule::token_to(
                r"(?ms)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"sqs",
        vec![
            Rule::token_to(r"(?ms)'", STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)#|\\.|""#, STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"tdqs",
        vec![
            Rule::token_to(r#"(?ms)""""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)\\.|\'|""#, STRING),
            Rule::token_to(
                r"(?ms)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpoling_string"]),
            ),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    m.insert(
        r"tsqs",
        vec![
            Rule::token_to(r"(?ms)'''", STRING, NewState::Pop(1)),
            Rule::token(r#"(?ms)#|\\.|\'|""#, STRING),
            Rule::token(r#"(?ms)[^#\\\'"]+"#, STRING),
        ],
    );
    Table(m)
}

impl Lexer for KalLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
