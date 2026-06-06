//! AUTO-GENERATED from `pygments.pygments.lexers.teraterm:TeraTermLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.teraterm:TeraTermLexer:teratermmacro

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: teratermmacro, teraterm, ttl
pub struct TeratermmacroLexer;

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
        Rule::token(r"(?m);[^\r\n]*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"in-comment"])),
        Rule::bygroups(r"(?m)(?i)^(\s*)(:[a-z0-9_]+)", vec![Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?m)(?i)\b(basename|beep|bplusrecv|bplussend|break|bringupbox|callmenu|changedir|checksum16|checksum16file|checksum32|checksum32file|checksum8|checksum8file|clearscreen|clipb2var|closesbox|closett|code2str|connect|continue|crc16|crc16file|crc32|crc32file|cygconnect|delpassword|delpassword2|dirname|dirnamebox|disconnect|dispstr|do|else|elseif|enablekeyb|end|endif|enduntil|endwhile|exec|execcmnd|exit|expandenv|fileclose|fileconcat|filecopy|filecreate|filedelete|filelock|filemarkptr|filenamebox|fileopen|fileread|filereadln|filerename|filesearch|fileseek|fileseekback|filestat|filestrseek|filestrseek2|filetruncate|fileunlock|filewrite|filewriteln|findclose|findfirst|findnext|flushrecv|foldercreate|folderdelete|foldersearch|for|getdate|getdir|getenv|getfileattr|gethostname|getipv4addr|getipv6addr|getmodemstatus|getpassword|getpassword2|getspecialfolder|gettime|gettitle|getttdir|getttpos|getver|if|ifdefined|include|inputbox|int2str|intdim|ispassword|ispassword2|kmtfinish|kmtget|kmtrecv|kmtsend|listbox|loadkeymap|logautoclosemode|logclose|loginfo|logopen|logpause|logrotate|logstart|logwrite|loop|makepath|messagebox|mpause|next|passwordbox|pause|quickvanrecv|quickvansend|random|recvfilerecvln|regexoption|restoresetup|return|rotateleft|rotateright|scprecv|scpsend|send|sendbinary|sendbreak|sendbroadcast|sendfile|sendkcode|sendln|sendlnbroadcast|sendlnmulticast|sendmulticast|sendtext|setbaud|setdate|setdebug|setdir|setdlgpos|setdtr|setecho|setenv|setexitcode|setfileattr|setflowctrl|setmulticastname|setpassword|setpassword2|setrts|setserialdelaycharsetserialdelaylinesetspeed|setsync|settime|settitle|show|showtt|sprintf|sprintf2|statusbox|str2code|str2int|strcompare|strconcat|strcopy|strdim|strinsert|strjoin|strlen|strmatch|strremove|strreplace|strscan|strspecial|strsplit|strtrim|testlink|then|tolower|toupper|unlink|until|uptime|var2clipb|wait|wait4all|waitevent|waitln|waitn|waitrecv|waitregex|while|xmodemrecv|xmodemsend|yesnobox|ymodemrecv|ymodemsend|zmodemrecv|zmodemsend)\b", KEYWORD),
        Rule::bygroups(r"(?m)(?i)(call|goto)([ \t]+)([a-z0-9_]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?m)(?i)(groupmatchstr1|groupmatchstr2|groupmatchstr3|groupmatchstr4|groupmatchstr5|groupmatchstr6|groupmatchstr7|groupmatchstr8|groupmatchstr9|inputstr|matchstr|mtimeout|param1|param2|param3|param4|param5|param6|param7|param8|param9|paramcnt|params|result|timeout)\b", NAME_BUILTIN),
        Rule::token(r"(?m)(?i)[a-z_][a-z0-9_]*", NAME_VARIABLE),
        Rule::token(r"(?m)and|not|or|xor", OPERATOR_WORD),
        Rule::token(r"(?m)[!%&*+<=>^~\|\/-]+", OPERATOR),
        Rule::token(r"(?m)[()]", STRING_SYMBOL),
        Rule::bygroups(r"(?m)(-?)([0-9]+)", vec![Some(OPERATOR), Some(NUMBER_INTEGER)]),
        Rule::token(r"(?m)(?i)\$[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?m)(?i)#(?:[0-9]+|\$[0-9a-f]+)", STRING_CHAR),
        Rule::token(r"(?m)'[^'\n]*'", STRING_SINGLE),
        Rule::token(r#"(?m)"[^"\n]*""#, STRING_DOUBLE),
        Rule::bygroups(r"(?m)('[^']*)(\n)", vec![Some(ERROR), Some(WHITESPACE)]),
        Rule::bygroups(r#"(?m)("[^"]*)(\n)"#, vec![Some(ERROR), Some(WHITESPACE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)\S", TEXT),
    ]);
    m.insert(r"comments", vec![
        Rule::token(r"(?m);[^\r\n]*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"in-comment"])),
    ]);
    m.insert(r"labels", vec![
        Rule::bygroups(r"(?m)(?i)^(\s*)(:[a-z0-9_]+)", vec![Some(WHITESPACE), Some(NAME_LABEL)]),
    ]);
    m.insert(r"commands", vec![
        Rule::token(r"(?m)(?i)\b(basename|beep|bplusrecv|bplussend|break|bringupbox|callmenu|changedir|checksum16|checksum16file|checksum32|checksum32file|checksum8|checksum8file|clearscreen|clipb2var|closesbox|closett|code2str|connect|continue|crc16|crc16file|crc32|crc32file|cygconnect|delpassword|delpassword2|dirname|dirnamebox|disconnect|dispstr|do|else|elseif|enablekeyb|end|endif|enduntil|endwhile|exec|execcmnd|exit|expandenv|fileclose|fileconcat|filecopy|filecreate|filedelete|filelock|filemarkptr|filenamebox|fileopen|fileread|filereadln|filerename|filesearch|fileseek|fileseekback|filestat|filestrseek|filestrseek2|filetruncate|fileunlock|filewrite|filewriteln|findclose|findfirst|findnext|flushrecv|foldercreate|folderdelete|foldersearch|for|getdate|getdir|getenv|getfileattr|gethostname|getipv4addr|getipv6addr|getmodemstatus|getpassword|getpassword2|getspecialfolder|gettime|gettitle|getttdir|getttpos|getver|if|ifdefined|include|inputbox|int2str|intdim|ispassword|ispassword2|kmtfinish|kmtget|kmtrecv|kmtsend|listbox|loadkeymap|logautoclosemode|logclose|loginfo|logopen|logpause|logrotate|logstart|logwrite|loop|makepath|messagebox|mpause|next|passwordbox|pause|quickvanrecv|quickvansend|random|recvfilerecvln|regexoption|restoresetup|return|rotateleft|rotateright|scprecv|scpsend|send|sendbinary|sendbreak|sendbroadcast|sendfile|sendkcode|sendln|sendlnbroadcast|sendlnmulticast|sendmulticast|sendtext|setbaud|setdate|setdebug|setdir|setdlgpos|setdtr|setecho|setenv|setexitcode|setfileattr|setflowctrl|setmulticastname|setpassword|setpassword2|setrts|setserialdelaycharsetserialdelaylinesetspeed|setsync|settime|settitle|show|showtt|sprintf|sprintf2|statusbox|str2code|str2int|strcompare|strconcat|strcopy|strdim|strinsert|strjoin|strlen|strmatch|strremove|strreplace|strscan|strspecial|strsplit|strtrim|testlink|then|tolower|toupper|unlink|until|uptime|var2clipb|wait|wait4all|waitevent|waitln|waitn|waitrecv|waitregex|while|xmodemrecv|xmodemsend|yesnobox|ymodemrecv|ymodemsend|zmodemrecv|zmodemsend)\b", KEYWORD),
        Rule::bygroups(r"(?m)(?i)(call|goto)([ \t]+)([a-z0-9_]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
    ]);
    m.insert(r"builtin-variables", vec![
        Rule::token(r"(?m)(?i)(groupmatchstr1|groupmatchstr2|groupmatchstr3|groupmatchstr4|groupmatchstr5|groupmatchstr6|groupmatchstr7|groupmatchstr8|groupmatchstr9|inputstr|matchstr|mtimeout|param1|param2|param3|param4|param5|param6|param7|param8|param9|paramcnt|params|result|timeout)\b", NAME_BUILTIN),
    ]);
    m.insert(r"user-variables", vec![
        Rule::token(r"(?m)(?i)[a-z_][a-z0-9_]*", NAME_VARIABLE),
    ]);
    m.insert(r"operators", vec![
        Rule::token(r"(?m)and|not|or|xor", OPERATOR_WORD),
        Rule::token(r"(?m)[!%&*+<=>^~\|\/-]+", OPERATOR),
        Rule::token(r"(?m)[()]", STRING_SYMBOL),
    ]);
    m.insert(r"numeric-literals", vec![
        Rule::bygroups(r"(?m)(-?)([0-9]+)", vec![Some(OPERATOR), Some(NUMBER_INTEGER)]),
        Rule::token(r"(?m)(?i)\$[0-9a-f]+", NUMBER_HEX),
    ]);
    m.insert(r"string-literals", vec![
        Rule::token(r"(?m)(?i)#(?:[0-9]+|\$[0-9a-f]+)", STRING_CHAR),
        Rule::token(r"(?m)'[^'\n]*'", STRING_SINGLE),
        Rule::token(r#"(?m)"[^"\n]*""#, STRING_DOUBLE),
        Rule::bygroups(r"(?m)('[^']*)(\n)", vec![Some(ERROR), Some(WHITESPACE)]),
        Rule::bygroups(r#"(?m)("[^"]*)(\n)"#, vec![Some(ERROR), Some(WHITESPACE)]),
    ]);
    m.insert(r"all-whitespace", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"in-comment", vec![
        Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
        Rule::token(r"(?m)[^*/]+", COMMENT_MULTILINE),
        Rule::token(r"(?m)[*/]", COMMENT_MULTILINE),
    ]);
    Table(m)
}

impl Lexer for TeratermmacroLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
