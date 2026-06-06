//! AUTO-GENERATED from `pygments.pygments.lexers.basic:QBasicLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.basic:QBasicLexer:qbasic

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: qbasic, basic
pub struct QbasicLexer;

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
        Rule::token(r"(?m)\n+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)^(\s*)(\d*)(\s*)(REM .*)$", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::bygroups(r"(?m)^(\s*)(\d+)(\s*)", vec![Some(WHITESPACE), Some(NAME_LABEL), Some(WHITESPACE)]),
        Rule::token(r"(?m)(?=[\s]*)(\w+)(?=[\s]*=)", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)(?=[^"]*)\'.*$"#, COMMENT_SINGLE),
        Rule::token(r#"(?m)"[^\n"]*""#, STRING_DOUBLE),
        Rule::bygroups(r"(?m)(END)(\s+)(FUNCTION|IF|SELECT|SUB)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(KEYWORD_RESERVED)]),
        Rule::bygroups(r"(?m)(DECLARE)(\s+)([A-Z]+)(\s+)(\S+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(NAME)]),
        Rule::bygroups(r"(?m)(DIM)(\s+)(SHARED)(\s+)([^\s(]+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE), Some(WHITESPACE), Some(NAME_VARIABLE_GLOBAL)]),
        Rule::bygroups(r"(?m)(DIM)(\s+)([^\s(]+)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_VARIABLE_GLOBAL)]),
        Rule::bygroups(r"(?m)^(\s*)([a-zA-Z_]+)(\s*)(\=)", vec![Some(WHITESPACE), Some(NAME_VARIABLE_GLOBAL), Some(WHITESPACE), Some(OPERATOR)]),
        Rule::bygroups(r"(?m)(GOTO|GOSUB)(\s+)(\w+\:?)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::bygroups(r"(?m)(SUB)(\s+)(\w+\:?)", vec![Some(KEYWORD_RESERVED), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?m)\b(DATA|LET)(?=\(|\b)", KEYWORD_DECLARATION),
        Rule::token(r"(?m)\b(ABS|ASC|ATN|CDBL|CHR\$|CINT|CLNG|COMMAND\$|COS|CSNG|CSRLIN|CVD|CVDMBF|CVI|CVL|CVS|CVSMBF|DATE\$|ENVIRON\$|EOF|ERDEV|ERDEV\$|ERL|ERR|EXP|FILEATTR|FIX|FRE|FREEFILE|HEX\$|INKEY\$|INP|INPUT\$|INSTR|INT|IOCTL\$|LBOUND|LCASE\$|LEFT\$|LEN|LOC|LOF|LOG|LPOS|LTRIM\$|MID\$|MKD\$|MKDMBF\$|MKI\$|MKL\$|MKS\$|MKSMBF\$|OCT\$|PEEK|PEN|PLAY|PMAP|POINT|POS|RIGHT\$|RND|RTRIM\$|SADD|SCREEN|SEEK|SETMEM|SGN|SIN|SPACE\$|SPC|SQR|STICK|STR\$|STRIG|STRING\$|TAB|TAN|TIME\$|TIMER|UBOUND|UCASE\$|VAL|VARPTR|VARPTR\$|VARSEG)(?=\(|\b)", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(\$DYNAMIC|\$INCLUDE|\$STATIC)(?=\(|\b)", KEYWORD_CONSTANT),
        Rule::token(r"(?m)\b(AND|EQV|IMP|NOT|OR|XOR)(?=\(|\b)", OPERATOR_WORD),
        Rule::token(r"(?m)\b(BEEP|BLOAD|BSAVE|CALL|CALL\ ABSOLUTE|CALL\ INTERRUPT|CALLS|CHAIN|CHDIR|CIRCLE|CLEAR|CLOSE|CLS|COLOR|COM|COMMON|CONST|DATA|DATE\$|DECLARE|DEF\ FN|DEF\ SEG|DEFDBL|DEFINT|DEFLNG|DEFSNG|DEFSTR|DEF|DIM|DO|LOOP|DRAW|END|ENVIRON|ERASE|ERROR|EXIT|FIELD|FILES|FOR|NEXT|FUNCTION|GET|GOSUB|GOTO|IF|THEN|INPUT|INPUT\ \#|IOCTL|KEY|KEY|KILL|LET|LINE|LINE\ INPUT|LINE\ INPUT\ \#|LOCATE|LOCK|UNLOCK|LPRINT|LSET|MID\$|MKDIR|NAME|ON\ COM|ON\ ERROR|ON\ KEY|ON\ PEN|ON\ PLAY|ON\ STRIG|ON\ TIMER|ON\ UEVENT|ON|OPEN|OPEN\ COM|OPTION\ BASE|OUT|PAINT|PALETTE|PCOPY|PEN|PLAY|POKE|PRESET|PRINT|PRINT\ \#|PRINT\ USING|PSET|PUT|PUT|RANDOMIZE|READ|REDIM|REM|RESET|RESTORE|RESUME|RETURN|RMDIR|RSET|RUN|SCREEN|SEEK|SELECT\ CASE|SHARED|SHELL|SLEEP|SOUND|STATIC|STOP|STRIG|SUB|SWAP|SYSTEM|TIME\$|TIMER|TROFF|TRON|TYPE|UEVENT|UNLOCK|VIEW|WAIT|WHILE|WEND|WIDTH|WINDOW|WRITE)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b(ACCESS|ALIAS|ANY|APPEND|AS|BASE|BINARY|BYVAL|CASE|CDECL|DOUBLE|ELSE|ELSEIF|ENDIF|INTEGER|IS|LIST|LOCAL|LONG|LOOP|MOD|NEXT|OFF|ON|OUTPUT|RANDOM|SIGNAL|SINGLE|STEP|STRING|THEN|TO|UNTIL|USING|WEND)\b", KEYWORD),
        Rule::token(r"(?m)[a-zA-Z_]\w*[$@#&!]", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)[a-zA-Z_]\w*\:", NAME_LABEL),
        Rule::token(r"(?m)\-?\d*\.\d+[@|#]?", NUMBER_FLOAT),
        Rule::token(r"(?m)\-?\d+[@|#]", NUMBER_FLOAT),
        Rule::token(r"(?m)\-?\d+#?", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\-?\d+#?", NUMBER_INTEGER),
        Rule::token(r"(?m)!=|==|:=|\.=|<<|>>|[-~+/\\*%=<>&^|?:!.]", OPERATOR),
        Rule::token(r"(?m)[\[\]{}(),;]", PUNCTUATION),
        Rule::token(r"(?m)[\w]+", NAME_VARIABLE_GLOBAL),
    ]);
    m.insert(r"declarations", vec![
        Rule::token(r"(?m)\b(DATA|LET)(?=\(|\b)", KEYWORD_DECLARATION),
    ]);
    m.insert(r"functions", vec![
        Rule::token(r"(?m)\b(ABS|ASC|ATN|CDBL|CHR\$|CINT|CLNG|COMMAND\$|COS|CSNG|CSRLIN|CVD|CVDMBF|CVI|CVL|CVS|CVSMBF|DATE\$|ENVIRON\$|EOF|ERDEV|ERDEV\$|ERL|ERR|EXP|FILEATTR|FIX|FRE|FREEFILE|HEX\$|INKEY\$|INP|INPUT\$|INSTR|INT|IOCTL\$|LBOUND|LCASE\$|LEFT\$|LEN|LOC|LOF|LOG|LPOS|LTRIM\$|MID\$|MKD\$|MKDMBF\$|MKI\$|MKL\$|MKS\$|MKSMBF\$|OCT\$|PEEK|PEN|PLAY|PMAP|POINT|POS|RIGHT\$|RND|RTRIM\$|SADD|SCREEN|SEEK|SETMEM|SGN|SIN|SPACE\$|SPC|SQR|STICK|STR\$|STRIG|STRING\$|TAB|TAN|TIME\$|TIMER|UBOUND|UCASE\$|VAL|VARPTR|VARPTR\$|VARSEG)(?=\(|\b)", KEYWORD_RESERVED),
    ]);
    m.insert(r"metacommands", vec![
        Rule::token(r"(?m)\b(\$DYNAMIC|\$INCLUDE|\$STATIC)(?=\(|\b)", KEYWORD_CONSTANT),
    ]);
    m.insert(r"operators", vec![
        Rule::token(r"(?m)\b(AND|EQV|IMP|NOT|OR|XOR)(?=\(|\b)", OPERATOR_WORD),
    ]);
    m.insert(r"statements", vec![
        Rule::token(r"(?m)\b(BEEP|BLOAD|BSAVE|CALL|CALL\ ABSOLUTE|CALL\ INTERRUPT|CALLS|CHAIN|CHDIR|CIRCLE|CLEAR|CLOSE|CLS|COLOR|COM|COMMON|CONST|DATA|DATE\$|DECLARE|DEF\ FN|DEF\ SEG|DEFDBL|DEFINT|DEFLNG|DEFSNG|DEFSTR|DEF|DIM|DO|LOOP|DRAW|END|ENVIRON|ERASE|ERROR|EXIT|FIELD|FILES|FOR|NEXT|FUNCTION|GET|GOSUB|GOTO|IF|THEN|INPUT|INPUT\ \#|IOCTL|KEY|KEY|KILL|LET|LINE|LINE\ INPUT|LINE\ INPUT\ \#|LOCATE|LOCK|UNLOCK|LPRINT|LSET|MID\$|MKDIR|NAME|ON\ COM|ON\ ERROR|ON\ KEY|ON\ PEN|ON\ PLAY|ON\ STRIG|ON\ TIMER|ON\ UEVENT|ON|OPEN|OPEN\ COM|OPTION\ BASE|OUT|PAINT|PALETTE|PCOPY|PEN|PLAY|POKE|PRESET|PRINT|PRINT\ \#|PRINT\ USING|PSET|PUT|PUT|RANDOMIZE|READ|REDIM|REM|RESET|RESTORE|RESUME|RETURN|RMDIR|RSET|RUN|SCREEN|SEEK|SELECT\ CASE|SHARED|SHELL|SLEEP|SOUND|STATIC|STOP|STRIG|SUB|SWAP|SYSTEM|TIME\$|TIMER|TROFF|TRON|TYPE|UEVENT|UNLOCK|VIEW|WAIT|WHILE|WEND|WIDTH|WINDOW|WRITE)\b", KEYWORD_RESERVED),
    ]);
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)\b(ACCESS|ALIAS|ANY|APPEND|AS|BASE|BINARY|BYVAL|CASE|CDECL|DOUBLE|ELSE|ELSEIF|ENDIF|INTEGER|IS|LIST|LOCAL|LONG|LOOP|MOD|NEXT|OFF|ON|OUTPUT|RANDOM|SIGNAL|SINGLE|STEP|STRING|THEN|TO|UNTIL|USING|WEND)\b", KEYWORD),
    ]);
    Table(m)
}

impl Lexer for QbasicLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
