//! AUTO-GENERATED from `pygments.pygments.lexers.perl:PerlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.perl:PerlLexer:perl

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: perl, pl
pub struct PerlLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"balanced-regex", vec![
        Rule::token_to(r"(?ms)/(\\\\|\\[^\\]|[^\\/])*/[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)!(\\\\|\\[^\\]|[^\\!])*![egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\\(\\\\|[^\\])*\\[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\{(\\\\|\\[^\\]|[^\\}])*\}[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)<(\\\\|\\[^\\]|[^\\>])*>[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\[(\\\\|\\[^\\]|[^\\\]])*\][egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\((\\\\|\\[^\\]|[^\\)])*\)[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)@(\\\\|\\[^\\]|[^\\@])*@[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)%(\\\\|\\[^\\]|[^\\%])*%[egimosx]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\$(\\\\|\\[^\\]|[^\\$])*\$[egimosx]*", STRING_REGEX, NewState::Pop(1)),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?ms)\A\#!.+?$", COMMENT_HASHBANG),
        Rule::token(r"(?ms)\#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?ms)^=[a-zA-Z0-9]+\s+.*?\n=cut", COMMENT_MULTILINE),
        Rule::token(r"(?ms)(BEGIN|CHECK|END|INIT|c(?:(?:as|ontinu)e)|do|els(?:e|if)|for(?:(?:each)?)|if|last|my|ne(?:w|xt)|our|print|re(?:do|set|turn)|then|un(?:less|til)|while)\b", KEYWORD),
        Rule::bygroups_to(r"(?ms)(format)(\s+)(\w+)(\s*)(=)(\s*\n)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"format"])),
        Rule::token(r"(?ms)(eq|lt|gt|le|ge|ne|not|and|or|cmp)\b", OPERATOR_WORD),
        Rule::token(r"(?ms)s/(\\\\|\\[^\\]|[^\\/])*/(\\\\|\\[^\\]|[^\\/])*/[egimosx]*", STRING_REGEX),
        Rule::token(r"(?ms)s!(\\\\|\\!|[^!])*!(\\\\|\\!|[^!])*![egimosx]*", STRING_REGEX),
        Rule::token(r"(?ms)s\\(\\\\|[^\\])*\\(\\\\|[^\\])*\\[egimosx]*", STRING_REGEX),
        Rule::token(r"(?ms)s@(\\\\|\\[^\\]|[^\\@])*@(\\\\|\\[^\\]|[^\\@])*@[egimosx]*", STRING_REGEX),
        Rule::token(r"(?ms)s%(\\\\|\\[^\\]|[^\\%])*%(\\\\|\\[^\\]|[^\\%])*%[egimosx]*", STRING_REGEX),
        Rule::token_to(r"(?ms)s\{(\\\\|\\[^\\]|[^\\}])*\}\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?ms)s<(\\\\|\\[^\\]|[^\\>])*>\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?ms)s\[(\\\\|\\[^\\]|[^\\\]])*\]\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token_to(r"(?ms)s\((\\\\|\\[^\\]|[^\\)])*\)\s*", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token(r"(?ms)m?/(\\\\|\\[^\\]|[^\\/\n])*/[gcimosx]*", STRING_REGEX),
        Rule::token_to(r"(?ms)m(?=[/!\\{<\[(@%$])", STRING_REGEX, NewState::Push(vec![r"balanced-regex"])),
        Rule::token(r"(?ms)((?<==~)|(?<=\())\s*/(\\\\|\\[^\\]|[^\\/])*/[gcimosx]*", STRING_REGEX),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)(a(?:bs|ccept|larm|tan2)|b(?:in(?:d|mode)|less)|c(?:aller|h(?:dir|mod|o(?:mp|p|wn)|r(?:(?:oot)?))|lose(?:(?:dir)?)|o(?:n(?:nect|tinue)|s)|rypt)|d(?:bm(?:close|open)|e(?:fined|lete)|ie|ump)|e(?:ach|nd(?:(?:gr|host|net|p(?:roto|w)|serv)ent)|of|val|x(?:ec|i(?:sts|t)|p))|f(?:cntl|ileno|lock|or(?:k|m(?:at|line)))|g(?:et(?:c|gr(?:ent|gid|nam)|host(?:by(?:addr|name)|ent)|login|net(?:by(?:addr|name)|ent)|p(?:eername|grp|pid|r(?:iority|oto(?:byn(?:ame|umber)|ent))|w(?:ent|nam|uid))|s(?:erv(?:by(?:name|port)|ent)|ock(?:name|opt)))|lob|mtime|oto|rep)|hex|i(?:mport|n(?:dex|t)|octl)|join|k(?:eys|ill)|l(?:ast|c(?:(?:first)?)|ength|i(?:nk|sten)|o(?:cal(?:(?:time)?)|g)|stat)|m(?:ap|kdir|sg(?:ctl|get|rcv|snd)|y)|next|o(?:ct|pen(?:(?:dir)?)|rd|ur)|p(?:ack|ipe|o(?:[ps])|r(?:intf|ototype)|ush)|quotemeta|r(?:and|e(?:ad(?:(?:dir|lin(?:[ek])|pipe)?)|cv|do|f|name|verse|winddir)|index|mdir)|s(?:calar|e(?:ek(?:(?:dir)?)|lect|m(?:ctl|get|op)|nd|t(?:grent|hostent|netent|p(?:grp|r(?:iority|otoent)|went)|s(?:(?:erven|ockop)t)))|h(?:ift|m(?:ctl|get|read|write)|utdown)|in|leep|o(?:cket(?:(?:pair)?)|rt)|p(?:li(?:ce|t)|rintf)|qrt|rand|t(?:at|udy)|ubstr|y(?:mlink|s(?:call|open|read|seek|tem|write)))|t(?:ell(?:(?:dir)?)|i(?:e(?:(?:d)?)|me(?:(?:s)?))|r(?:(?:uncate)?))|u(?:c(?:(?:first)?)|mask|n(?:def|link|pack|shift|tie)|time)|v(?:alues|ec)|w(?:a(?:it(?:(?:pid)?)|ntarray|rn)|rite))\b", NAME_BUILTIN),
        Rule::token(r"(?ms)((__(DATA|DIE|WARN)__)|(STD(IN|OUT|ERR)))\b", NAME_BUILTIN_PSEUDO),
        Rule::bygroups(r#"(?ms)(<<)([\'"]?)([a-zA-Z_]\w*)(\2;?\n.*?\n)(\3)(\n)"#, vec![Some(STRING), Some(STRING), Some(STRING_DELIMITER), Some(STRING), Some(STRING_DELIMITER), Some(WHITESPACE)]),
        Rule::token_to(r"(?ms)__END__", COMMENT_PREPROC, NewState::Push(vec![r"end-part"])),
        Rule::token(r"(?ms)\$\^[ADEFHILMOPSTWX]", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?ms)\$[\\\"\[\]'&`+*.,;=%~?@$!<>(^|/-](?!\w)"#, NAME_VARIABLE_GLOBAL),
        Rule::token_to(r"(?ms)[$@%#]+", NAME_VARIABLE, NewState::Push(vec![r"varname"])),
        Rule::token(r"(?ms)0_?[0-7]+(_[0-7]+)*", NUMBER_OCT),
        Rule::token(r"(?ms)0x[0-9A-Fa-f]+(_[0-9A-Fa-f]+)*", NUMBER_HEX),
        Rule::token(r"(?ms)0b[01]+(_[01]+)*", NUMBER_BIN),
        Rule::token(r"(?ms)(?i)(\d*(_\d*)*\.\d+(_\d*)*|\d+(_\d*)*\.\d+(_\d*)*)(e[+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?ms)(?i)\d+(_\d*)*e[+-]?\d+(_\d*)*", NUMBER_FLOAT),
        Rule::token(r"(?ms)\d+(_\d+)*", NUMBER_INTEGER),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?ms)`(\\\\|\\[^\\]|[^`\\])*`", STRING_BACKTICK),
        Rule::token(r"(?ms)<([^\s>]+)>", STRING_REGEX),
        Rule::token_to(r"(?ms)(q|qq|qw|qr|qx)\{", STRING_OTHER, NewState::Push(vec![r"cb-string"])),
        Rule::token_to(r"(?ms)(q|qq|qw|qr|qx)\(", STRING_OTHER, NewState::Push(vec![r"rb-string"])),
        Rule::token_to(r"(?ms)(q|qq|qw|qr|qx)\[", STRING_OTHER, NewState::Push(vec![r"sb-string"])),
        Rule::token_to(r"(?ms)(q|qq|qw|qr|qx)\<", STRING_OTHER, NewState::Push(vec![r"lt-string"])),
        Rule::token(r"(?ms)(q|qq|qw|qr|qx)([\W_])(.|\n)*?\2", STRING_OTHER),
        Rule::bygroups(r"(?ms)(package)(\s+)([a-zA-Z_]\w*(?:::[a-zA-Z_]\w*)*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?ms)(use|require|no)(\s+)([a-zA-Z_]\w*(?:::[a-zA-Z_]\w*)*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups_to(r"(?ms)(sub)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"funcname"])),
        Rule::token(r"(?ms)(no|(?:packag|requir|us)e)\b", KEYWORD),
        Rule::token(r"(?ms)(\[\]|\*\*|::|<<|>>|>=|<=>|<=|={3}|!=|=~|!~|&&?|\|\||\.{1,3})", OPERATOR),
        Rule::token(r"(?ms)[-+/*%=<>&^|!\\~]=?", OPERATOR),
        Rule::token(r"(?ms)[()\[\]:;,<>/?{}]", PUNCTUATION),
        Rule::token_to(r"(?ms)(?=\w)", NAME, NewState::Push(vec![r"name"])),
    ]);
    m.insert(r"format", vec![
        Rule::token_to(r"(?ms)\.\n", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?ms)[^\n]*\n", STRING_INTERPOL),
    ]);
    m.insert(r"varname", vec![
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token_to(r"(?ms)\{", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?ms)\)|,", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?ms)\w+::", NAME_NAMESPACE),
        Rule::token_to(r"(?ms)[\w:]+", NAME_VARIABLE, NewState::Pop(1)),
    ]);
    m.insert(r"name", vec![
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*(::[a-zA-Z_]\w*)*(::)?(?=\s*->)", NAME_NAMESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[a-zA-Z_]\w*(::[a-zA-Z_]\w*)*::", NAME_NAMESPACE, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[\w:]+", NAME, NewState::Pop(1)),
        Rule::token_to(r"(?ms)[A-Z_]+(?=\W)", NAME_CONSTANT, NewState::Pop(1)),
        Rule::token_to(r"(?ms)(?=\W)", TEXT, NewState::Pop(1)),
    ]);
    m.insert(r"funcname", vec![
        Rule::token(r"(?ms)[a-zA-Z_]\w*[!?]?", NAME_FUNCTION),
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::bygroups(r"(?ms)(\([$@%]*\))(\s*)", vec![Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::token_to(r"(?ms);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?ms).*?\{", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"cb-string", vec![
        Rule::token(r"(?ms)\\[{}\\]", STRING_OTHER),
        Rule::token(r"(?ms)\\", STRING_OTHER),
        Rule::token_to(r"(?ms)\{", STRING_OTHER, NewState::Push(vec![r"cb-string"])),
        Rule::token_to(r"(?ms)\}", STRING_OTHER, NewState::Pop(1)),
        Rule::token(r"(?ms)[^{}\\]+", STRING_OTHER),
    ]);
    m.insert(r"rb-string", vec![
        Rule::token(r"(?ms)\\[()\\]", STRING_OTHER),
        Rule::token(r"(?ms)\\", STRING_OTHER),
        Rule::token_to(r"(?ms)\(", STRING_OTHER, NewState::Push(vec![r"rb-string"])),
        Rule::token_to(r"(?ms)\)", STRING_OTHER, NewState::Pop(1)),
        Rule::token(r"(?ms)[^()]+", STRING_OTHER),
    ]);
    m.insert(r"sb-string", vec![
        Rule::token(r"(?ms)\\[\[\]\\]", STRING_OTHER),
        Rule::token(r"(?ms)\\", STRING_OTHER),
        Rule::token_to(r"(?ms)\[", STRING_OTHER, NewState::Push(vec![r"sb-string"])),
        Rule::token_to(r"(?ms)\]", STRING_OTHER, NewState::Pop(1)),
        Rule::token(r"(?ms)[^\[\]]+", STRING_OTHER),
    ]);
    m.insert(r"lt-string", vec![
        Rule::token(r"(?ms)\\[<>\\]", STRING_OTHER),
        Rule::token(r"(?ms)\\", STRING_OTHER),
        Rule::token_to(r"(?ms)\<", STRING_OTHER, NewState::Push(vec![r"lt-string"])),
        Rule::token_to(r"(?ms)\>", STRING_OTHER, NewState::Pop(1)),
        Rule::token(r"(?ms)[^<>]+", STRING_OTHER),
    ]);
    m.insert(r"end-part", vec![
        Rule::token_to(r"(?ms).+", COMMENT_PREPROC, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for PerlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
