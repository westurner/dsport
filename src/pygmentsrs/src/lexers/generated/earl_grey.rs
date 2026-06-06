//! AUTO-GENERATED from `pygments.pygments.lexers.javascript:EarlGreyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.javascript:EarlGreyLexer:earl_grey

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: earl-grey, earlgrey, eg
pub struct EarlGreyLexer;

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
        Rule::token(r"(?m)(?x)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?!\n)\s+
                (?!and|as|each\*|each|in|is|mod|of|or|when|where|with)
                (?=(?:[+\-*/~^<>%&|?!@#.])?[a-zA-Z$_](?:[\w$-]*[\w$])?)", TokenType::new(&["Keyword", "Control"])),
        Rule::bygroups(r#"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(?!\n)(\s+)(?=[\'"\d{\[(])"#, vec![Some(TokenType::new(&["Keyword", "Control"])), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(?x)
                (?:
                    (?<=[%=])|
                    (?<=[=\-]>)|
                    (?<=with|each|with)|
                    (?<=each\*|where)
                )(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<![+\-*/~^<>%&|?!@#.])(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::bygroups(r"(?m)(;;.*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?m)[\[\]{}:(),;]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(?<![\w\-$.])((?:(?:(?:Referenc|Typ)e)?)Error)(?![\w\-$.])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?x)
                (?<![\w$])
                E\.[\w$](?:[\w$\-]*[\w$])?
                (?:\.[\w$](?:[\w$\-]*[\w$])?)*
                (?=[({\[?!\s])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?<=\s|\[)(a(?:nd|s)|i(?:[ns])|not|o(?:[fr])|w(?:he(?:n|re)|ith))(?![\w$\-])", OPERATOR_WORD),
        Rule::token(r"(?m)[*@]?->", NAME_FUNCTION),
        Rule::token(r"(?m)[+\-*/~^<>%&|?!@#.]*=", OPERATOR_WORD),
        Rule::token(r"(?m)\.{2,3}", OPERATOR_WORD),
        Rule::token(r"(?m)([+*/~^<>&|?!]+)|([#\-](?=\s))|@@+(?=\s)|=+", OPERATOR),
        Rule::token(r"(?m)(?<![\w$\-])(var|let)(?:[^\w$])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?<![\w\-$.])(a(?:sync|wait)|break|c(?:hain|ontinue)|e(?:ach(?:(?:\*)?)|l(?:if|se)|xpr\-value)|gen|if|let|m(?:atch|(?:(?:eth)?)od)|pass|re(?:quire|turn)|var|yield)(?![\w\-$.])", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(?<![\w\-$.])(@|self|this)(?![\w\-$])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<![\w\-$.])(Array|Boolean|E(?:Node|rrorFactory)|Function|Number|Object|Promise|String)(?![\w\-$])", KEYWORD_TYPE),
        Rule::token(r"(?m)(?<![\w\-#.])(c(?:lone|on(?:s(?:tructor|ume)|tains))|e(?:numerate|qual)|get(?:\-(?:checker|pro(?:jector|perty))|Checker|Pro(?:jector|perty))|items|keys|ne(?:ighbours|qual)|object|pr(?:edicate|o(?:duct|misify))|r(?:ange|epr)|s(?:end|pawn)|take|zip)(?![\w\-.])", NAME_BUILTIN),
        Rule::token(r"(?m)(?<![\w\-$.])(false|null|true|undefined)(?![\w\-$.])", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\.)?([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=\s+[+\-*/~^<>%&|?!@#.]*\=\s)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::bygroups_to(r"(?m)(?x)
                (?:()([a-zA-Z$_](?:[\w$\-]*[\w$])?)|
                   (?<=[\s{\[(])(\.)([a-zA-Z$_](?:[\w$\-]*[\w$])?))
                (?=.*%)", vec![Some(PUNCTUATION), Some(NAME_TAG), Some(PUNCTUATION), Some(TokenType::new(&["Name", "Class", "Start"]))], NewState::Push(vec![r"dbs"])),
        Rule::token_to(r"(?m)[rR]?`", STRING_BACKTICK, NewState::Push(vec![r"bt"])),
        Rule::token_to(r"(?m)[rR]?```", STRING_BACKTICK, NewState::Push(vec![r"tbt"])),
        Rule::token(r"(?m)(?<=[\s\[{(,;])\.([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=[\s\]}),;])", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=\s+with(?:\s|\n))", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<!\s)(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[}\]).,;:\s])", vec![Some(PUNCTUATION), Some(TokenType::new(&["Name", "Field"]))]),
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[\[{(:])", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token_to(r#"(?m)(?:[rR]|[rR]\.[gmi]{1,3})?""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)(?:[rR]|[rR]\.[gmi]{1,3})?\'", STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token(r"(?m)#[a-zA-Z_][\w\-]*(?=[\s{(,;])", NAME_NAMESPACE),
        Rule::bygroups(r"(?m)(?<=[\s:;,])(\.{1,3}(?:[\w\-]*/)*)(\w(?:[\w\-]*\w)*)(?=[\s;,])", vec![Some(WHITESPACE), Some(TEXT)]),
        Rule::token(r"(?m)@([a-zA-Z$_](?:[\w$-]*[\w$])?)", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(\+\+|\-\-)?", vec![Some(TokenType::new(&["Name", "Symbol"])), Some(OPERATOR_WORD)]),
        Rule::token(r"(?m)\d+\.(?!\.)\d*([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)8r[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)2r[01]+", NUMBER_BIN),
        Rule::token(r"(?m)16r[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)([3-79]|[12][0-9]|3[0-6])r[a-zA-Z\d]+(\.[a-zA-Z\d]+)?", TokenType::new(&["Literal", "Number", "Radix"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"control", vec![
        Rule::token(r"(?m)(?x)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?!\n)\s+
                (?!and|as|each\*|each|in|is|mod|of|or|when|where|with)
                (?=(?:[+\-*/~^<>%&|?!@#.])?[a-zA-Z$_](?:[\w$-]*[\w$])?)", TokenType::new(&["Keyword", "Control"])),
        Rule::bygroups(r#"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(?!\n)(\s+)(?=[\'"\d{\[(])"#, vec![Some(TokenType::new(&["Keyword", "Control"])), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(?x)
                (?:
                    (?<=[%=])|
                    (?<=[=\-]>)|
                    (?<=with|each|with)|
                    (?<=each\*|where)
                )(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<![+\-*/~^<>%&|?!@#.])(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
    ]);
    m.insert(r"errors", vec![
        Rule::token(r"(?m)(?<![\w\-$.])((?:(?:(?:Referenc|Typ)e)?)Error)(?![\w\-$.])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?x)
                (?<![\w$])
                E\.[\w$](?:[\w$\-]*[\w$])?
                (?:\.[\w$](?:[\w$\-]*[\w$])?)*
                (?=[({\[?!\s])", NAME_EXCEPTION),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(?<![\w\-$.])(a(?:sync|wait)|break|c(?:hain|ontinue)|e(?:ach(?:(?:\*)?)|l(?:if|se)|xpr\-value)|gen|if|let|m(?:atch|(?:(?:eth)?)od)|pass|re(?:quire|turn)|var|yield)(?![\w\-$.])", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(?<![\w\-$.])(@|self|this)(?![\w\-$])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<![\w\-$.])(Array|Boolean|E(?:Node|rrorFactory)|Function|Number|Object|Promise|String)(?![\w\-$])", KEYWORD_TYPE),
    ]);
    m.insert(r"builtins", vec![
        Rule::token(r"(?m)(?<![\w\-#.])(c(?:lone|on(?:s(?:tructor|ume)|tains))|e(?:numerate|qual)|get(?:\-(?:checker|pro(?:jector|perty))|Checker|Pro(?:jector|perty))|items|keys|ne(?:ighbours|qual)|object|pr(?:edicate|o(?:duct|misify))|r(?:ange|epr)|s(?:end|pawn)|take|zip)(?![\w\-.])", NAME_BUILTIN),
        Rule::token(r"(?m)(?<![\w\-$.])(false|null|true|undefined)(?![\w\-$.])", NAME_CONSTANT),
    ]);
    m.insert(r"assignment", vec![
        Rule::bygroups(r"(?m)(\.)?([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=\s+[+\-*/~^<>%&|?!@#.]*\=\s)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
    ]);
    m.insert(r"nested", vec![
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=\s+with(?:\s|\n))", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<!\s)(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[}\]).,;:\s])", vec![Some(PUNCTUATION), Some(TokenType::new(&["Name", "Field"]))]),
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[\[{(:])", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"strings", vec![
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"_tmp_0", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"sqs", vec![
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"interpoling_string"])),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"_tmp_1", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"interpoling_string"])),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"tdqs", vec![
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"_tmp_2", vec![
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|\n|N\{.*?\}|u[a-fA-F0-9]{4}|U[a-fA-F0-9]{8}|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\\'"]"#, STRING),
        Rule::token(r#"(?m)[\'"\\]"#, STRING),
        Rule::token(r"(?m)\n", STRING),
    ]);
    m.insert(r"tuple", vec![
        Rule::token(r"(?m)#[a-zA-Z_][\w\-]*(?=[\s{(,;])", NAME_NAMESPACE),
    ]);
    m.insert(r"import_paths", vec![
        Rule::bygroups(r"(?m)(?<=[\s:;,])(\.{1,3}(?:[\w\-]*/)*)(\w(?:[\w\-]*\w)*)(?=[\s;,])", vec![Some(WHITESPACE), Some(TEXT)]),
    ]);
    m.insert(r"name", vec![
        Rule::token(r"(?m)@([a-zA-Z$_](?:[\w$-]*[\w$])?)", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(\+\+|\-\-)?", vec![Some(TokenType::new(&["Name", "Symbol"])), Some(OPERATOR_WORD)]),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)\d+\.(?!\.)\d*([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)8r[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)2r[01]+", NUMBER_BIN),
        Rule::token(r"(?m)16r[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)([3-79]|[12][0-9]|3[0-6])r[a-zA-Z\d]+(\.[a-zA-Z\d]+)?", TokenType::new(&["Literal", "Number", "Radix"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"dbs", vec![
        Rule::bygroups(r"(?m)(\.)([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=[.\[\s])", vec![Some(PUNCTUATION), Some(TokenType::new(&["Name", "Class", "DBS"]))]),
        Rule::bygroups(r"(?m)(\[)([\^#][a-zA-Z$_](?:[\w$\-]*[\w$])?)(\])", vec![Some(PUNCTUATION), Some(TokenType::new(&["Name", "Entity", "DBS"])), Some(PUNCTUATION)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)%", TokenType::new(&["Operator", "DBS"]), NewState::Pop(1)),
    ]);
    m.insert(r"interpoling_string", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)(?x)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?!\n)\s+
                (?!and|as|each\*|each|in|is|mod|of|or|when|where|with)
                (?=(?:[+\-*/~^<>%&|?!@#.])?[a-zA-Z$_](?:[\w$-]*[\w$])?)", TokenType::new(&["Keyword", "Control"])),
        Rule::bygroups(r#"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(?!\n)(\s+)(?=[\'"\d{\[(])"#, vec![Some(TokenType::new(&["Keyword", "Control"])), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(?x)
                (?:
                    (?<=[%=])|
                    (?<=[=\-]>)|
                    (?<=with|each|with)|
                    (?<=each\*|where)
                )(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<![+\-*/~^<>%&|?!@#.])(\s+)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)(:)", vec![Some(WHITESPACE), Some(TokenType::new(&["Keyword", "Control"])), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::bygroups(r"(?m)(;;.*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?m)[\[\]{}:(),;]", PUNCTUATION),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", TEXT),
        Rule::token(r"(?m)(?<![\w\-$.])((?:(?:(?:Referenc|Typ)e)?)Error)(?![\w\-$.])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?x)
                (?<![\w$])
                E\.[\w$](?:[\w$\-]*[\w$])?
                (?:\.[\w$](?:[\w$\-]*[\w$])?)*
                (?=[({\[?!\s])", NAME_EXCEPTION),
        Rule::token(r"(?m)(?<=\s|\[)(a(?:nd|s)|i(?:[ns])|not|o(?:[fr])|w(?:he(?:n|re)|ith))(?![\w$\-])", OPERATOR_WORD),
        Rule::token(r"(?m)[*@]?->", NAME_FUNCTION),
        Rule::token(r"(?m)[+\-*/~^<>%&|?!@#.]*=", OPERATOR_WORD),
        Rule::token(r"(?m)\.{2,3}", OPERATOR_WORD),
        Rule::token(r"(?m)([+*/~^<>&|?!]+)|([#\-](?=\s))|@@+(?=\s)|=+", OPERATOR),
        Rule::token(r"(?m)(?<![\w$\-])(var|let)(?:[^\w$])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?<![\w\-$.])(a(?:sync|wait)|break|c(?:hain|ontinue)|e(?:ach(?:(?:\*)?)|l(?:if|se)|xpr\-value)|gen|if|let|m(?:atch|(?:(?:eth)?)od)|pass|re(?:quire|turn)|var|yield)(?![\w\-$.])", KEYWORD_PSEUDO),
        Rule::token(r"(?m)(?<![\w\-$.])(@|self|this)(?![\w\-$])", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?<![\w\-$.])(Array|Boolean|E(?:Node|rrorFactory)|Function|Number|Object|Promise|String)(?![\w\-$])", KEYWORD_TYPE),
        Rule::token(r"(?m)(?<![\w\-#.])(c(?:lone|on(?:s(?:tructor|ume)|tains))|e(?:numerate|qual)|get(?:\-(?:checker|pro(?:jector|perty))|Checker|Pro(?:jector|perty))|items|keys|ne(?:ighbours|qual)|object|pr(?:edicate|o(?:duct|misify))|r(?:ange|epr)|s(?:end|pawn)|take|zip)(?![\w\-.])", NAME_BUILTIN),
        Rule::token(r"(?m)(?<![\w\-$.])(false|null|true|undefined)(?![\w\-$.])", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\.)?([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=\s+[+\-*/~^<>%&|?!@#.]*\=\s)", vec![Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::bygroups_to(r"(?m)(?x)
                (?:()([a-zA-Z$_](?:[\w$\-]*[\w$])?)|
                   (?<=[\s{\[(])(\.)([a-zA-Z$_](?:[\w$\-]*[\w$])?))
                (?=.*%)", vec![Some(PUNCTUATION), Some(NAME_TAG), Some(PUNCTUATION), Some(TokenType::new(&["Name", "Class", "Start"]))], NewState::Push(vec![r"dbs"])),
        Rule::token_to(r"(?m)[rR]?`", STRING_BACKTICK, NewState::Push(vec![r"bt"])),
        Rule::token_to(r"(?m)[rR]?```", STRING_BACKTICK, NewState::Push(vec![r"tbt"])),
        Rule::token(r"(?m)(?<=[\s\[{(,;])\.([a-zA-Z$_](?:[\w$\-]*[\w$])?)(?=[\s\]}),;])", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=\s+with(?:\s|\n))", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(?x)
                (?<!\s)(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[}\]).,;:\s])", vec![Some(PUNCTUATION), Some(TokenType::new(&["Name", "Field"]))]),
        Rule::bygroups(r"(?m)(?x)
                (?<=[\w$\]})])(\.)
                ([a-zA-Z$_](?:[\w$-]*[\w$])?)
                (?=[\[{(:])", vec![Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::token_to(r#"(?m)(?:[rR]|[rR]\.[gmi]{1,3})?""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)(?:[rR]|[rR]\.[gmi]{1,3})?\'", STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r#"(?m)""""#, STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token(r"(?m)#[a-zA-Z_][\w\-]*(?=[\s{(,;])", NAME_NAMESPACE),
        Rule::bygroups(r"(?m)(?<=[\s:;,])(\.{1,3}(?:[\w\-]*/)*)(\w(?:[\w\-]*\w)*)(?=[\s;,])", vec![Some(WHITESPACE), Some(TEXT)]),
        Rule::token(r"(?m)@([a-zA-Z$_](?:[\w$-]*[\w$])?)", NAME_VARIABLE_INSTANCE),
        Rule::bygroups(r"(?m)([a-zA-Z$_](?:[\w$-]*[\w$])?)(\+\+|\-\-)?", vec![Some(TokenType::new(&["Name", "Symbol"])), Some(OPERATOR_WORD)]),
        Rule::token(r"(?m)\d+\.(?!\.)\d*([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)8r[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)2r[01]+", NUMBER_BIN),
        Rule::token(r"(?m)16r[a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)([3-79]|[12][0-9]|3[0-6])r[a-zA-Z\d]+(\.[a-zA-Z\d]+)?", TokenType::new(&["Literal", "Number", "Radix"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
    ]);
    m.insert(r"bt", vec![
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Pop(1)),
        Rule::token(r"(?m)(?<!`)\n", STRING_BACKTICK),
        Rule::token(r"(?m)\^=?", STRING_ESCAPE),
        Rule::token(r"(?m).+", STRING_BACKTICK),
    ]);
    m.insert(r"tbt", vec![
        Rule::token_to(r"(?m)```", STRING_BACKTICK, NewState::Pop(1)),
        Rule::token(r"(?m)\n", STRING_BACKTICK),
        Rule::token(r"(?m)\^=?", STRING_ESCAPE),
        Rule::token(r"(?m)[^`]+", STRING_BACKTICK),
    ]);
    Table(m)
}

impl Lexer for EarlGreyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
