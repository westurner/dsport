//! AUTO-GENERATED from `pygments.pygments.lexers.d:DLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.d:DLexer:d

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: d
pub struct DLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\+", COMMENT_MULTILINE, NewState::Push(vec![r"nested_comment"])),
        Rule::token(r"(?m)(__(?:gshared|parameters|traits|vector)|a(?:bstract|li(?:as|gn)|s(?:m|sert)|uto)|b(?:ody|reak)|c(?:a(?:s(?:[et])|tch)|lass|on(?:st|tinue))|d(?:e(?:bug|fault|le(?:(?:(?:ga)?)te)|precated)|o)|e(?:lse|num|x(?:port|tern))|f(?:inal(?:(?:ly)?)|or(?:(?:each(?:(?:_reverse)?))?)|unction)|goto|i(?:m(?:mutable|port)|n(?:out|terface|variant)|[fns])|lazy|m(?:ixin|odule)|n(?:(?:e|othro)w)|o(?:ut|verride)|p(?:ackage|r(?:agma|ivate|otected)|u(?:blic|re))|re(?:f|turn)|s(?:cope|hared|t(?:atic|ruct)|uper|witch|ynchronized)|t(?:emplate|h(?:is|row)|ry|ype(?:id|of))|uni(?:on|ttest)|v(?:ersion|olatile)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)(typedef)\b", TokenType::new(&["Keyword", "Removed"])),
        Rule::token(r"(?m)(b(?:ool|yte)|c(?:double|ent|float|har|real)|d(?:char|ouble)|float|i(?:double|float|nt|real)|long|real|short|u(?:byte|cent|int|long|short)|void|wchar)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(__(?:(?:DATE|EOF|F(?:ILE(?:(?:_FULL_PATH)?)|UNCTION)|LINE|MODULE|PRETTY_FUNCTION|TIME(?:(?:STAMP)?)|VE(?:NDOR|RSION))__))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)macro\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(string|wstring|dstring|size_t|ptrdiff_t)\b", NAME_BUILTIN),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F_]*\.[0-9a-fA-F_]+|[0-9a-fA-F_]+)[pP][+\-]?[0-9_]+[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9_]+(\.[0-9_]+[eE][+\-]?[0-9_]+|\.[0-9_]*|[eE][+\-]?[0-9_]+)[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)\.(0|[1-9][0-9_]*)([eE][+\-]?[0-9_]+)?[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[Bb][01_]+", NUMBER_BIN),
        Rule::token(r"(?m)0[0-7_]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)(0|[1-9][0-9_]*)([LUu]|Lu|LU|uL|UL)?", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(\\['"?\\abfnrtv]|\\x[0-9a-fA-F]{2}|\\[0-7]{1,3}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\&\w+;|.)'"#, STRING_CHAR),
        Rule::token(r#"(?m)r"[^"]*"[cwd]?"#, STRING),
        Rule::token(r"(?m)`[^`]*`[cwd]?", STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*"[cwd]?"#, STRING),
        Rule::token(r#"(?m)\\(['\"?\\abfnrtv]|x[0-9a-fA-F]{2}|[0-7]{1,3}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|&\w+;)"#, STRING),
        Rule::token(r#"(?m)x"[0-9a-fA-F_\s]*"[cwd]?"#, STRING),
        Rule::token_to(r#"(?m)q"\["#, STRING, NewState::Push(vec![r"delimited_bracket"])),
        Rule::token_to(r#"(?m)q"\("#, STRING, NewState::Push(vec![r"delimited_parenthesis"])),
        Rule::token_to(r#"(?m)q"<"#, STRING, NewState::Push(vec![r"delimited_angle"])),
        Rule::token_to(r#"(?m)q"\{"#, STRING, NewState::Push(vec![r"delimited_curly"])),
        Rule::token(r#"(?m)q"([a-zA-Z_]\w*)\n.*?\n\1""#, STRING),
        Rule::token(r#"(?m)q"(.).*?\1""#, STRING),
        Rule::token_to(r"(?m)q\{", STRING, NewState::Push(vec![r"token_string"])),
        Rule::token(r"(?m)@([a-zA-Z_]\w*)?", NAME_DECORATOR),
        Rule::token(r"(?m)(~=|\^=|%=|\*=|==|!>=|!<=|!<>=|!<>|!<|!>|!=|>>>=|>>>|>>=|>>|>=|<>=|<>|<<=|<<|<=|\+\+|\+=|--|-=|\|\||\|=|&&|&=|\.\.\.|\.\.|/=)|[/.&|\-+<>!()\[\]{}?,;:$=*%^~]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::bygroups(r"(?m)(#line)(\s)(.*)(\n)", vec![Some(COMMENT_SPECIAL), Some(WHITESPACE), Some(COMMENT_SPECIAL), Some(WHITESPACE)]),
    ]);
    m.insert(
        r"nested_comment",
        vec![
            Rule::token(r"(?m)[^+/]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?m)/\+", COMMENT_MULTILINE, NewState::PushSame),
            Rule::token_to(r"(?m)\+/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[+/]", COMMENT_MULTILINE),
        ],
    );
    m.insert(r"token_string", vec![
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"token_string_nest"])),
        Rule::token_to(r"(?m)\}", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\+", COMMENT_MULTILINE, NewState::Push(vec![r"nested_comment"])),
        Rule::token(r"(?m)(__(?:gshared|parameters|traits|vector)|a(?:bstract|li(?:as|gn)|s(?:m|sert)|uto)|b(?:ody|reak)|c(?:a(?:s(?:[et])|tch)|lass|on(?:st|tinue))|d(?:e(?:bug|fault|le(?:(?:(?:ga)?)te)|precated)|o)|e(?:lse|num|x(?:port|tern))|f(?:inal(?:(?:ly)?)|or(?:(?:each(?:(?:_reverse)?))?)|unction)|goto|i(?:m(?:mutable|port)|n(?:out|terface|variant)|[fns])|lazy|m(?:ixin|odule)|n(?:(?:e|othro)w)|o(?:ut|verride)|p(?:ackage|r(?:agma|ivate|otected)|u(?:blic|re))|re(?:f|turn)|s(?:cope|hared|t(?:atic|ruct)|uper|witch|ynchronized)|t(?:emplate|h(?:is|row)|ry|ype(?:id|of))|uni(?:on|ttest)|v(?:ersion|olatile)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)(typedef)\b", TokenType::new(&["Keyword", "Removed"])),
        Rule::token(r"(?m)(b(?:ool|yte)|c(?:double|ent|float|har|real)|d(?:char|ouble)|float|i(?:double|float|nt|real)|long|real|short|u(?:byte|cent|int|long|short)|void|wchar)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(__(?:(?:DATE|EOF|F(?:ILE(?:(?:_FULL_PATH)?)|UNCTION)|LINE|MODULE|PRETTY_FUNCTION|TIME(?:(?:STAMP)?)|VE(?:NDOR|RSION))__))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)macro\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(string|wstring|dstring|size_t|ptrdiff_t)\b", NAME_BUILTIN),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F_]*\.[0-9a-fA-F_]+|[0-9a-fA-F_]+)[pP][+\-]?[0-9_]+[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9_]+(\.[0-9_]+[eE][+\-]?[0-9_]+|\.[0-9_]*|[eE][+\-]?[0-9_]+)[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)\.(0|[1-9][0-9_]*)([eE][+\-]?[0-9_]+)?[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[Bb][01_]+", NUMBER_BIN),
        Rule::token(r"(?m)0[0-7_]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)(0|[1-9][0-9_]*)([LUu]|Lu|LU|uL|UL)?", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(\\['"?\\abfnrtv]|\\x[0-9a-fA-F]{2}|\\[0-7]{1,3}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\&\w+;|.)'"#, STRING_CHAR),
        Rule::token(r#"(?m)r"[^"]*"[cwd]?"#, STRING),
        Rule::token(r"(?m)`[^`]*`[cwd]?", STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*"[cwd]?"#, STRING),
        Rule::token(r#"(?m)\\(['\"?\\abfnrtv]|x[0-9a-fA-F]{2}|[0-7]{1,3}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|&\w+;)"#, STRING),
        Rule::token(r#"(?m)x"[0-9a-fA-F_\s]*"[cwd]?"#, STRING),
        Rule::token_to(r#"(?m)q"\["#, STRING, NewState::Push(vec![r"delimited_bracket"])),
        Rule::token_to(r#"(?m)q"\("#, STRING, NewState::Push(vec![r"delimited_parenthesis"])),
        Rule::token_to(r#"(?m)q"<"#, STRING, NewState::Push(vec![r"delimited_angle"])),
        Rule::token_to(r#"(?m)q"\{"#, STRING, NewState::Push(vec![r"delimited_curly"])),
        Rule::token(r#"(?m)q"([a-zA-Z_]\w*)\n.*?\n\1""#, STRING),
        Rule::token(r#"(?m)q"(.).*?\1""#, STRING),
        Rule::token_to(r"(?m)q\{", STRING, NewState::Push(vec![r"token_string"])),
        Rule::token(r"(?m)@([a-zA-Z_]\w*)?", NAME_DECORATOR),
        Rule::token(r"(?m)(~=|\^=|%=|\*=|==|!>=|!<=|!<>=|!<>|!<|!>|!=|>>>=|>>>|>>=|>>|>=|<>=|<>|<<=|<<|<=|\+\+|\+=|--|-=|\|\||\|=|&&|&=|\.\.\.|\.\.|/=)|[/.&|\-+<>!()\[\]{}?,;:$=*%^~]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::bygroups(r"(?m)(#line)(\s)(.*)(\n)", vec![Some(COMMENT_SPECIAL), Some(WHITESPACE), Some(COMMENT_SPECIAL), Some(WHITESPACE)]),
    ]);
    m.insert(r"token_string_nest", vec![
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(//.*?)(\n)", vec![Some(COMMENT_SINGLE), Some(WHITESPACE)]),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/\+", COMMENT_MULTILINE, NewState::Push(vec![r"nested_comment"])),
        Rule::token(r"(?m)(__(?:gshared|parameters|traits|vector)|a(?:bstract|li(?:as|gn)|s(?:m|sert)|uto)|b(?:ody|reak)|c(?:a(?:s(?:[et])|tch)|lass|on(?:st|tinue))|d(?:e(?:bug|fault|le(?:(?:(?:ga)?)te)|precated)|o)|e(?:lse|num|x(?:port|tern))|f(?:inal(?:(?:ly)?)|or(?:(?:each(?:(?:_reverse)?))?)|unction)|goto|i(?:m(?:mutable|port)|n(?:out|terface|variant)|[fns])|lazy|m(?:ixin|odule)|n(?:(?:e|othro)w)|o(?:ut|verride)|p(?:ackage|r(?:agma|ivate|otected)|u(?:blic|re))|re(?:f|turn)|s(?:cope|hared|t(?:atic|ruct)|uper|witch|ynchronized)|t(?:emplate|h(?:is|row)|ry|ype(?:id|of))|uni(?:on|ttest)|v(?:ersion|olatile)|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?m)(typedef)\b", TokenType::new(&["Keyword", "Removed"])),
        Rule::token(r"(?m)(b(?:ool|yte)|c(?:double|ent|float|har|real)|d(?:char|ouble)|float|i(?:double|float|nt|real)|long|real|short|u(?:byte|cent|int|long|short)|void|wchar)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(false|true|null)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(__(?:(?:DATE|EOF|F(?:ILE(?:(?:_FULL_PATH)?)|UNCTION)|LINE|MODULE|PRETTY_FUNCTION|TIME(?:(?:STAMP)?)|VE(?:NDOR|RSION))__))\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)macro\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)(string|wstring|dstring|size_t|ptrdiff_t)\b", NAME_BUILTIN),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F_]*\.[0-9a-fA-F_]+|[0-9a-fA-F_]+)[pP][+\-]?[0-9_]+[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9_]+(\.[0-9_]+[eE][+\-]?[0-9_]+|\.[0-9_]*|[eE][+\-]?[0-9_]+)[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)\.(0|[1-9][0-9_]*)([eE][+\-]?[0-9_]+)?[fFL]?[i]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[Bb][01_]+", NUMBER_BIN),
        Rule::token(r"(?m)0[0-7_]+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F_]+", NUMBER_HEX),
        Rule::token(r"(?m)(0|[1-9][0-9_]*)([LUu]|Lu|LU|uL|UL)?", NUMBER_INTEGER),
        Rule::token(r#"(?m)'(\\['"?\\abfnrtv]|\\x[0-9a-fA-F]{2}|\\[0-7]{1,3}|\\u[0-9a-fA-F]{4}|\\U[0-9a-fA-F]{8}|\\&\w+;|.)'"#, STRING_CHAR),
        Rule::token(r#"(?m)r"[^"]*"[cwd]?"#, STRING),
        Rule::token(r"(?m)`[^`]*`[cwd]?", STRING),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*"[cwd]?"#, STRING),
        Rule::token(r#"(?m)\\(['\"?\\abfnrtv]|x[0-9a-fA-F]{2}|[0-7]{1,3}|u[0-9a-fA-F]{4}|U[0-9a-fA-F]{8}|&\w+;)"#, STRING),
        Rule::token(r#"(?m)x"[0-9a-fA-F_\s]*"[cwd]?"#, STRING),
        Rule::token_to(r#"(?m)q"\["#, STRING, NewState::Push(vec![r"delimited_bracket"])),
        Rule::token_to(r#"(?m)q"\("#, STRING, NewState::Push(vec![r"delimited_parenthesis"])),
        Rule::token_to(r#"(?m)q"<"#, STRING, NewState::Push(vec![r"delimited_angle"])),
        Rule::token_to(r#"(?m)q"\{"#, STRING, NewState::Push(vec![r"delimited_curly"])),
        Rule::token(r#"(?m)q"([a-zA-Z_]\w*)\n.*?\n\1""#, STRING),
        Rule::token(r#"(?m)q"(.).*?\1""#, STRING),
        Rule::token_to(r"(?m)q\{", STRING, NewState::Push(vec![r"token_string"])),
        Rule::token(r"(?m)@([a-zA-Z_]\w*)?", NAME_DECORATOR),
        Rule::token(r"(?m)(~=|\^=|%=|\*=|==|!>=|!<=|!<>=|!<>|!<|!>|!=|>>>=|>>>|>>=|>>|>=|<>=|<>|<<=|<<|<=|\+\+|\+=|--|-=|\|\||\|=|&&|&=|\.\.\.|\.\.|/=)|[/.&|\-+<>!()\[\]{}?,;:$=*%^~]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::bygroups(r"(?m)(#line)(\s)(.*)(\n)", vec![Some(COMMENT_SPECIAL), Some(WHITESPACE), Some(COMMENT_SPECIAL), Some(WHITESPACE)]),
    ]);
    m.insert(
        r"delimited_bracket",
        vec![
            Rule::token(r"(?m)[^\[\]]+", STRING),
            Rule::token_to(
                r"(?m)\[",
                STRING,
                NewState::Push(vec![r"delimited_inside_bracket"]),
            ),
            Rule::token_to(r#"(?m)\]""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_inside_bracket",
        vec![
            Rule::token(r"(?m)[^\[\]]+", STRING),
            Rule::token_to(r"(?m)\[", STRING, NewState::PushSame),
            Rule::token_to(r"(?m)\]", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_parenthesis",
        vec![
            Rule::token(r"(?m)[^()]+", STRING),
            Rule::token_to(
                r"(?m)\(",
                STRING,
                NewState::Push(vec![r"delimited_inside_parenthesis"]),
            ),
            Rule::token_to(r#"(?m)\)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_inside_parenthesis",
        vec![
            Rule::token(r"(?m)[^()]+", STRING),
            Rule::token_to(r"(?m)\(", STRING, NewState::PushSame),
            Rule::token_to(r"(?m)\)", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_angle",
        vec![
            Rule::token(r"(?m)[^<>]+", STRING),
            Rule::token_to(
                r"(?m)<",
                STRING,
                NewState::Push(vec![r"delimited_inside_angle"]),
            ),
            Rule::token_to(r#"(?m)>""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_inside_angle",
        vec![
            Rule::token(r"(?m)[^<>]+", STRING),
            Rule::token_to(r"(?m)<", STRING, NewState::PushSame),
            Rule::token_to(r"(?m)>", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_curly",
        vec![
            Rule::token(r"(?m)[^{}]+", STRING),
            Rule::token_to(
                r"(?m)\{",
                STRING,
                NewState::Push(vec![r"delimited_inside_curly"]),
            ),
            Rule::token_to(r#"(?m)\}""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"delimited_inside_curly",
        vec![
            Rule::token(r"(?m)[^{}]+", STRING),
            Rule::token_to(r"(?m)\{", STRING, NewState::PushSame),
            Rule::token_to(r"(?m)\}", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for DLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
