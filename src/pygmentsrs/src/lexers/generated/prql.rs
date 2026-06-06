//! AUTO-GENERATED from `pygments.pygments.lexers.prql:PrqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.prql:PrqlLexer:prql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: prql
pub struct PrqlLexer;

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
        Rule::token(r"(?m)#!.*", STRING_DOC),
        Rule::token(r"(?m)#.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups_to(r"(?m)^(\s*)(module)(\s*)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"imports"])),
        Rule::token(r"(?m)(bool|float|int(?:(?:1(?:28|6)|32|64|8)?)|(?:se|tex)t)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)^prql ", KEYWORD_RESERVED),
        Rule::token(r"(?m)let", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(case|int(?:ernal|o)|(?:modul|typ)e)\b", KEYWORD),
        Rule::token(r"(?m)(false|null|true)\b", KEYWORD_CONSTANT),
        Rule::bygroups_to(r#"(?m)(f)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)(f)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)(f)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)(f)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::bygroups_to(r#"(?m)(s)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_4"])),
        Rule::bygroups_to(r"(?m)(s)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_5"])),
        Rule::bygroups_to(r#"(?m)(s)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_6"])),
        Rule::bygroups_to(r"(?m)(s)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_7"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_8"])),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Push(vec![r"_tmp_9"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_10"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_11"])),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}T\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}", LITERAL_DATE),
        Rule::token(r"(?m)@\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(\d(?:_?\d)*\.(?:\d(?:_?\d)*)?|(?:\d(?:_?\d)*)?\.\d(?:_?\d)*)([eE][+-]?\d(?:_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(?:_?\d)*[eE][+-]?\d(?:_?\d)*j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO](?:_?[0-7])+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB](?:_?[01])+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX](?:_?[a-fA-F0-9])+", NUMBER_HEX),
        Rule::token(r"(?m)\d(?:_?\d)*", NUMBER_INTEGER),
        Rule::token(r"(?m)->|=>|==|!=|>=|<=|~=|&&|\|\||\?\?|\/\/", OPERATOR),
        Rule::token(r"(?m)[-~+/*%=<>&^|.@]", OPERATOR),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(_(?:eq|is_null)|a(?:ny|s|verage)|co(?:ncat_array|unt)|every|f(?:(?:irs|rom_tex)t)|in|l(?:a(?:g|st)|ead|ower)|m(?:ax|in)|r(?:ank(?:(?:_dense)?)|ead_(?:csv|parquet)|o(?:und|w_number))|s(?:tddev|um)|tuple_(?:every|(?:ma|zi)p)|upper)\b", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Za-z_][a-zA-Z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)^[A-Za-z_][a-zA-Z0-9_]*", KEYWORD),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)(case|int(?:ernal|o)|(?:modul|typ)e)\b", KEYWORD),
        Rule::token(r"(?m)(false|null|true)\b", KEYWORD_CONSTANT),
    ]);
    m.insert(r"expr", vec![
        Rule::bygroups_to(r#"(?m)(f)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)(f)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)(f)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)(f)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::bygroups_to(r#"(?m)(s)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_4"])),
        Rule::bygroups_to(r"(?m)(s)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_5"])),
        Rule::bygroups_to(r#"(?m)(s)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_6"])),
        Rule::bygroups_to(r"(?m)(s)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_7"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_8"])),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Push(vec![r"_tmp_9"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_10"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_11"])),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}T\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}", LITERAL_DATE),
        Rule::token(r"(?m)@\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(\d(?:_?\d)*\.(?:\d(?:_?\d)*)?|(?:\d(?:_?\d)*)?\.\d(?:_?\d)*)([eE][+-]?\d(?:_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(?:_?\d)*[eE][+-]?\d(?:_?\d)*j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO](?:_?[0-7])+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB](?:_?[01])+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX](?:_?[a-fA-F0-9])+", NUMBER_HEX),
        Rule::token(r"(?m)\d(?:_?\d)*", NUMBER_INTEGER),
        Rule::token(r"(?m)->|=>|==|!=|>=|<=|~=|&&|\|\||\?\?|\/\/", OPERATOR),
        Rule::token(r"(?m)[-~+/*%=<>&^|.@]", OPERATOR),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(_(?:eq|is_null)|a(?:ny|s|verage)|co(?:ncat_array|unt)|every|f(?:(?:irs|rom_tex)t)|in|l(?:a(?:g|st)|ead|ower)|m(?:ax|in)|r(?:ank(?:(?:_dense)?)|ead_(?:csv|parquet)|o(?:und|w_number))|s(?:tddev|um)|tuple_(?:every|(?:ma|zi)p)|upper)\b", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Za-z_][a-zA-Z0-9_]*", NAME_VARIABLE),
    ]);
    m.insert(r"fstringescape", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(r"stringescape", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(r"bytesescape", vec![
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
    ]);
    m.insert(r"tdqf", vec![
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
    ]);
    m.insert(r"fstrings-double", vec![
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_0", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
    ]);
    m.insert(r"tsqf", vec![
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)\n", STRING_SINGLE),
    ]);
    m.insert(r"fstrings-single", vec![
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
    ]);
    m.insert(r"_tmp_1", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)\n", STRING_SINGLE),
    ]);
    m.insert(r"dqf", vec![
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_2", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
    ]);
    m.insert(r"sqf", vec![
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
    ]);
    m.insert(r"_tmp_3", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
    ]);
    m.insert(r"_tmp_4", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_5", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)\n", STRING_SINGLE),
    ]);
    m.insert(r"_tmp_6", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_7", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\}", STRING_INTERPOL),
        Rule::token_to(r"(?m)\{", STRING_INTERPOL, NewState::Push(vec![r"expr-inside-fstring"])),
        Rule::token(r#"(?m)[^\\\'"{}\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
    ]);
    m.insert(r"tdqs", vec![
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_DOUBLE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
    ]);
    m.insert(r"strings-double", vec![
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_8", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_DOUBLE),
        Rule::token(r"(?m)\n", STRING_DOUBLE),
    ]);
    m.insert(r"tsqs", vec![
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_SINGLE),
        Rule::token(r"(?m)\n", STRING_SINGLE),
    ]);
    m.insert(r"strings-single", vec![
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_SINGLE),
    ]);
    m.insert(r"_tmp_9", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_SINGLE),
        Rule::token(r"(?m)\n", STRING_SINGLE),
    ]);
    m.insert(r"dqs", vec![
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_DOUBLE),
    ]);
    m.insert(r"_tmp_10", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?m)\\\\|\\"|\\\n"#, STRING_ESCAPE),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_DOUBLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_DOUBLE),
    ]);
    m.insert(r"sqs", vec![
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_SINGLE),
    ]);
    m.insert(r"_tmp_11", vec![
        Rule::token(r"(?m)\\(N\{.*?\}|u\{[a-fA-F0-9]{1,6}\})", STRING_ESCAPE),
        Rule::token(r#"(?m)\\([\\bfnrt"\']|\n|x[a-fA-F0-9]{2}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)\\\\|\\'|\\\n", STRING_ESCAPE),
        Rule::token(r"(?m)\{((\w+)((\.\w+)|(\[[^\]]+\]))*)?(\:(.?[<>=\^])?[-+ ]?#?0?(\d+)?,?(\.\d+)?[E-GXb-gnosx%]?)?\}", STRING_INTERPOL),
        Rule::token(r#"(?m)[^\\\'"%{\n]+"#, STRING_SINGLE),
        Rule::token(r#"(?m)[\'"\\]"#, STRING_SINGLE),
        Rule::token(r"(?m)%|(\{{1,2})", STRING_SINGLE),
    ]);
    m.insert(r"numbers", vec![
        Rule::token(r"(?m)(\d(?:_?\d)*\.(?:\d(?:_?\d)*)?|(?:\d(?:_?\d)*)?\.\d(?:_?\d)*)([eE][+-]?\d(?:_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(?:_?\d)*[eE][+-]?\d(?:_?\d)*j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO](?:_?[0-7])+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB](?:_?[01])+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX](?:_?[a-fA-F0-9])+", NUMBER_HEX),
        Rule::token(r"(?m)\d(?:_?\d)*", NUMBER_INTEGER),
    ]);
    m.insert(r"functions", vec![
        Rule::token(r"(?m)(_(?:eq|is_null)|a(?:ny|s|verage)|co(?:ncat_array|unt)|every|f(?:(?:irs|rom_tex)t)|in|l(?:a(?:g|st)|ead|ower)|m(?:ax|in)|r(?:ank(?:(?:_dense)?)|ead_(?:csv|parquet)|o(?:und|w_number))|s(?:tddev|um)|tuple_(?:every|(?:ma|zi)p)|upper)\b", NAME_FUNCTION),
    ]);
    m.insert(r"expr-inside-fstring", vec![
        Rule::token_to(r"(?m)[{(\[]", PUNCTUATION, NewState::Push(vec![r"expr-inside-fstring-inner"])),
        Rule::token_to(r"(?m)(=\s*)?\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token_to(r"(?m)(=\s*)?:", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups_to(r#"(?m)(f)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)(f)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)(f)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)(f)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::bygroups_to(r#"(?m)(s)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_4"])),
        Rule::bygroups_to(r"(?m)(s)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_5"])),
        Rule::bygroups_to(r#"(?m)(s)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_6"])),
        Rule::bygroups_to(r"(?m)(s)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_7"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_8"])),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Push(vec![r"_tmp_9"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_10"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_11"])),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}T\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}", LITERAL_DATE),
        Rule::token(r"(?m)@\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(\d(?:_?\d)*\.(?:\d(?:_?\d)*)?|(?:\d(?:_?\d)*)?\.\d(?:_?\d)*)([eE][+-]?\d(?:_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(?:_?\d)*[eE][+-]?\d(?:_?\d)*j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO](?:_?[0-7])+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB](?:_?[01])+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX](?:_?[a-fA-F0-9])+", NUMBER_HEX),
        Rule::token(r"(?m)\d(?:_?\d)*", NUMBER_INTEGER),
        Rule::token(r"(?m)->|=>|==|!=|>=|<=|~=|&&|\|\||\?\?|\/\/", OPERATOR),
        Rule::token(r"(?m)[-~+/*%=<>&^|.@]", OPERATOR),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(_(?:eq|is_null)|a(?:ny|s|verage)|co(?:ncat_array|unt)|every|f(?:(?:irs|rom_tex)t)|in|l(?:a(?:g|st)|ead|ower)|m(?:ax|in)|r(?:ank(?:(?:_dense)?)|ead_(?:csv|parquet)|o(?:und|w_number))|s(?:tddev|um)|tuple_(?:every|(?:ma|zi)p)|upper)\b", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Za-z_][a-zA-Z0-9_]*", NAME_VARIABLE),
    ]);
    m.insert(r"expr-inside-fstring-inner", vec![
        Rule::token_to(r"(?m)[{(\[]", PUNCTUATION, NewState::Push(vec![r"expr-inside-fstring-inner"])),
        Rule::token_to(r"(?m)[\])}]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups_to(r#"(?m)(f)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_0"])),
        Rule::bygroups_to(r"(?m)(f)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_1"])),
        Rule::bygroups_to(r#"(?m)(f)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_2"])),
        Rule::bygroups_to(r"(?m)(f)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_3"])),
        Rule::bygroups_to(r#"(?m)(s)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_4"])),
        Rule::bygroups_to(r"(?m)(s)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_5"])),
        Rule::bygroups_to(r#"(?m)(s)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"_tmp_6"])),
        Rule::bygroups_to(r"(?m)(s)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"_tmp_7"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(""")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"tdqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(''')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"tsqs"])),
        Rule::bygroups_to(r#"(?m)(?i)(r)(")"#, vec![Some(STRING_AFFIX), Some(STRING_DOUBLE)], NewState::Push(vec![r"dqs"])),
        Rule::bygroups_to(r"(?m)(?i)(r)(')", vec![Some(STRING_AFFIX), Some(STRING_SINGLE)], NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)""""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_8"])),
        Rule::token_to(r"(?m)'''", STRING_SINGLE, NewState::Push(vec![r"_tmp_9"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"_tmp_10"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"_tmp_11"])),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}T\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)@\d{4}-\d{2}-\d{2}", LITERAL_DATE),
        Rule::token(r"(?m)@\d{2}(:\d{2})?(:\d{2})?(\.\d{1,6})?(Z|[+-]\d{1,2}(:\d{1,2})?)?", LITERAL_DATE),
        Rule::token(r"(?m)[^\S\n]+", TEXT),
        Rule::token(r"(?m)(\d(?:_?\d)*\.(?:\d(?:_?\d)*)?|(?:\d(?:_?\d)*)?\.\d(?:_?\d)*)([eE][+-]?\d(?:_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(?:_?\d)*[eE][+-]?\d(?:_?\d)*j?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[oO](?:_?[0-7])+", NUMBER_OCT),
        Rule::token(r"(?m)0[bB](?:_?[01])+", NUMBER_BIN),
        Rule::token(r"(?m)0[xX](?:_?[a-fA-F0-9])+", NUMBER_HEX),
        Rule::token(r"(?m)\d(?:_?\d)*", NUMBER_INTEGER),
        Rule::token(r"(?m)->|=>|==|!=|>=|<=|~=|&&|\|\||\?\?|\/\/", OPERATOR),
        Rule::token(r"(?m)[-~+/*%=<>&^|.@]", OPERATOR),
        Rule::token(r"(?m)[\]{}:(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(_(?:eq|is_null)|a(?:ny|s|verage)|co(?:ncat_array|unt)|every|f(?:(?:irs|rom_tex)t)|in|l(?:a(?:g|st)|ead|ower)|m(?:ax|in)|r(?:ank(?:(?:_dense)?)|ead_(?:csv|parquet)|o(?:und|w_number))|s(?:tddev|um)|tuple_(?:every|(?:ma|zi)p)|upper)\b", NAME_FUNCTION),
        Rule::token(r"(?m)[A-Za-z_][a-zA-Z0-9_]*", NAME_VARIABLE),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)-(?!\})", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)\{-", COMMENT_MULTILINE, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[^-}]", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)-\}", COMMENT_MULTILINE, NewState::Pop(1)),
    ]);
    m.insert(r"imports", vec![
        Rule::token_to(r"(?m)\w+(\.\w+)*", NAME_CLASS, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for PrqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
