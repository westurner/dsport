//! AUTO-GENERATED from `pygments.pygments.lexers.felix:FelixLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.felix:FelixLexer:felix

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: felix, flx
pub struct FelixLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)(axiom|ctor|fun|gen|proc|reduce|union)\b", KEYWORD, NewState::Push(vec![r"funcname"])),
        Rule::token_to(r"(?m)(c(?:class|lass|struct)|obj|struct)\b", KEYWORD, NewState::Push(vec![r"classname"])),
        Rule::token_to(r"(?m)(instance|module|typeclass)\b", KEYWORD, NewState::Push(vec![r"modulename"])),
        Rule::token(r"(?m)(_(?:(?:deref)?)|a(?:ll|s(?:(?:sert)?)|ttempt)|c(?:a(?:ll(?:(?:back)?)|se(?:(?:no)?))|class|o(?:de|mpound)|types)|do(?:(?:ne|wnto)?)|e(?:l(?:if|se)|n(?:d(?:attempt|case|if|match)|um)|x(?:cept(?:(?:ions)?)|pect))|f(?:inally|or(?:(?:all|get|k)?)|unctor)|goto|i(?:dent|f|n(?:complete|herit|(?:stan|terfa)ce))|jump|l(?:ambda|oop)|m(?:atch|odule)|n(?:amespace|ew|o(?:expand|nterm))|o(?:bj|f|pen)|parse|r(?:aise|e(?:g(?:exp|lex|match)|name|turn))|t(?:he(?:(?:n)?)|o|ype(?:(?:case|def|match|of)?))|upto|w(?:h(?:en|ilst)|ith)|yield)\b", KEYWORD),
        Rule::token(r"(?m)(_gc_(?:pointer|type)|body|co(?:(?:mmen|ns)t)|export|header|inline|lval|macro|no(?:inline|return)|p(?:ackage|od|r(?:ivate|operty)|ubli(?:c|sh))|requires|todo|use|virtual)\b", NAME_DECORATOR),
        Rule::token(r"(?m)(def|let|ref|va(?:[lr]))\b", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(a(?:ddress|(?:n|rra)y)|b(?:ool|yte)|c(?:address|har(?:(?:(?:(?:c)?)p)?)|o(?:mplex|nt)|vaddress)|d(?:complex|imaginary|ouble)|float|i(?:maginary|nt(?:(?:16|32|64|8)?))|l(?:complex|double|i(?:maginary|st)|ong|value)|o(?:(?:ffse|p)t)|s(?:hort|lice|tring)|tiny|u(?:char(?:(?:(?:(?:c)?)p)?)|int(?:(?:16|32|64|8)?)|long|nit|s(?:hort|tring)|tiny|vlong)|v(?:a(?:ddress|rray)|long|oid|short)|w(?:char|string))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(and|not|in|is|isin|or|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|\|\||&&|[-~+/*%=<>&^|.$]", OPERATOR),
        Rule::token(r"(?m)0[xX]([0-9a-fA-F_]*\.[0-9a-fA-F_]+|[0-9a-fA-F_]+)[pP][+\-]?[0-9_]+[lLfFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)[0-9_]+(\.[0-9_]+[eE][+\-]?[0-9_]+|\.[0-9_]*|[eE][+\-]?[0-9_]+)[lLfFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)\.(0|[1-9][0-9_]*)([eE][+\-]?[0-9_]+)?[lLfFdD]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0[Bb][01_]+([tTsSiIlLvV]|ll|LL|([iIuU])(8|16|32|64))?", NUMBER_BIN),
        Rule::token(r"(?m)0[0-7_]+([tTsSiIlLvV]|ll|LL|([iIuU])(8|16|32|64))?", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][0-9a-fA-F_]+([tTsSiIlLvV]|ll|LL|([iIuU])(8|16|32|64))?", NUMBER_HEX),
        Rule::token(r"(?m)(0|[1-9][0-9_]*)([tTsSiIlLvV]|ll|LL|([iIuU])(8|16|32|64))?", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)([rR][cC]?|[cC][rR])""""#, STRING, NewState::Push(vec![r"tdqs"])),
        Rule::token_to(r"(?m)([rR][cC]?|[cC][rR])'''", STRING, NewState::Push(vec![r"tsqs"])),
        Rule::token_to(r#"(?m)([rR][cC]?|[cC][rR])""#, STRING, NewState::Push(vec![r"dqs"])),
        Rule::token_to(r"(?m)([rR][cC]?|[cC][rR])'", STRING, NewState::Push(vec![r"sqs"])),
        Rule::token_to(r#"(?m)[cCfFqQwWuU]?""""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)[cCfFqQwWuU]?'''", STRING, NewState::Push(vec![r"_tmp_1"])),
        Rule::token_to(r#"(?m)[cCfFqQwWuU]?""#, STRING, NewState::Push(vec![r"_tmp_2"])),
        Rule::token_to(r"(?m)[cCfFqQwWuU]?'", STRING, NewState::Push(vec![r"_tmp_3"])),
        Rule::token(r"(?m)[\[\]{}:(),;?]", PUNCTUATION),
        Rule::token(r"(?m)[a-zA-Z_]\w*:>", NAME_LABEL),
        Rule::token(r"(?m)(_svc|while)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(root|self|this)\b", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
    ]);
    m.insert(r"operators", vec![
        Rule::token(r"(?m)(and|not|in|is|isin|or|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|\|\||&&|[-~+/*%=<>&^|.$]", OPERATOR),
    ]);
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
    m.insert(r"nl", vec![
        Rule::token(r"(?m)\n", STRING),
    ]);
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
    m.insert(r"comment2", vec![
        Rule::token(r"(?m)[^/*]", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::PushSame),
        Rule::token_to(r"(?m)[*]/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[/*]", COMMENT_MULTILINE),
    ]);
    m.insert(r"if0", vec![
        Rule::bygroups_to(r"(?m)^(\s*)(#if.*?(?<!\\))(\n)", vec![Some(WHITESPACE), Some(COMMENT), Some(WHITESPACE)], NewState::PushSame),
        Rule::bygroups_to(r"(?m)^(\s*)(#endif.*?(?<!\\))(\n)", vec![Some(WHITESPACE), Some(COMMENT), Some(WHITESPACE)], NewState::Pop(1)),
        Rule::bygroups(r"(?m)(.*?)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
    ]);
    m.insert(r"macro", vec![
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(import|include)(\s+)(<[^>]*?>)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING)], NewState::Pop(1)),
        Rule::bygroups_to(r#"(?m)(import|include)(\s+)("[^"]*?")"#, vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING)], NewState::Pop(1)),
        Rule::bygroups_to(r"(?m)(import|include)(\s+)('[^']*?')", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(STRING)], NewState::Pop(1)),
        Rule::token(r"(?m)[^/\n]+", COMMENT_PREPROC),
        Rule::token(r"(?m)/", COMMENT_PREPROC),
        Rule::token(r"(?m)(?<=\\)\n", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"funcname", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_FUNCTION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=\()", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"classname", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)[a-zA-Z_]\w*", NAME_CLASS, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=\{)", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"modulename", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::Push(vec![r"modulename2", r"tvarlist"])),
        Rule::default(NewState::Push(vec![r"modulename2"])),
    ]);
    m.insert(r"modulename2", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token_to(r"(?m)([a-zA-Z_]\w*)", NAME_NAMESPACE, NewState::Pop(2)),
    ]);
    m.insert(r"tvarlist", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)//(.*?)$", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/[*]", COMMENT_MULTILINE, NewState::Push(vec![r"comment2"])),
        Rule::bygroups_to(r"(?m)(#)(\s*)(if)(\s+)(0)", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC), Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"if0"])),
        Rule::token_to(r"(?m)#", COMMENT_PREPROC, NewState::Push(vec![r"macro"])),
        Rule::token(r"(?m)(and|not|in|is|isin|or|xor)\b", OPERATOR_WORD),
        Rule::token(r"(?m)!=|==|<<|>>|\|\||&&|[-~+/*%=<>&^|.$]", OPERATOR),
        Rule::token_to(r"(?m)\[", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\]", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m),", PUNCTUATION),
        Rule::token(r"(?m)(with|where)\b", KEYWORD),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    Table(m)
}

impl Lexer for FelixLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
