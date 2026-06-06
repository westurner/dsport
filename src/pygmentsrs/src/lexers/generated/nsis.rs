//! AUTO-GENERATED from `pygments.pygments.lexers.installers:NSISLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.installers:NSISLexer:nsis

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: nsis, nsi, nsh
pub struct NsisLexer;

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
        Rule::bygroups(r"(?im)([;#].*)(\n)", vec![Some(COMMENT), Some(WHITESPACE)]),
        Rule::token(r"(?im)'.*?'", STRING_SINGLE),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"str_double"])),
        Rule::token_to(r"(?im)`", STRING_BACKTICK, NewState::Push(vec![r"str_backtick"])),
        Rule::token(r"(?im)\!(addincludedir(?:dir)?|addplugindir|appendfile|cd|define|delfilefile|echo(?:message)?|else|endif|error|execute|if(?:macro)?n?(?:def)?|include|insertmacro|macro(?:end)?|packhdr|search(?:parse|replace)|system|tempfilesymbol|undef|verbose|warning)\b", COMMENT_PREPROC),
        Rule::token(r"(?im)\$(R?[0-9])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?im)\$(ADMINTOOLS|APPDATA|CDBURN_AREA|COOKIES|COMMONFILES(?:32|64)|DESKTOP|DOCUMENTS|EXE(?:DIR|FILE|PATH)|FAVORITES|FONTS|HISTORY|HWNDPARENT|INTERNET_CACHE|LOCALAPPDATA|MUSIC|NETHOOD|PICTURES|PLUGINSDIR|PRINTHOOD|PROFILE|PROGRAMFILES(?:32|64)|QUICKLAUNCH|RECENT|RESOURCES(?:_LOCALIZED)?|SENDTO|SM(?:PROGRAMS|STARTUP)|STARTMENU|SYSDIR|TEMP(?:LATES)?|VIDEOS|WINDIR|\{NSISDIR\})", NAME_BUILTIN),
        Rule::token(r"(?im)\$(CMDLINE|INSTDIR|OUTDIR|LANGUAGE)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)\$[a-z_]\w*", NAME_VARIABLE),
        Rule::bygroups(r"(?im)(\n)(Function)(\s+)([._a-z][.\w]*)\b", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)\b([_a-z]\w*)(::)([a-z][a-z0-9]*)\b", vec![Some(KEYWORD_NAMESPACE), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)\b([_a-z]\w*)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?im)(\b[ULS]|\B)([!<>=]?=|\<\>?|\>)\B", OPERATOR),
        Rule::token(r"(?im)[|+-]", OPERATOR),
        Rule::token(r"(?im)\\", PUNCTUATION),
        Rule::token(r"(?im)\b(Abort|Add(?:BrandingImage|Size)|Allow(?:RootDirInstall|SkipFiles)|AutoCloseWindow|BG(?:Font|Gradient)|BrandingText|BringToFront|Call(?:InstDLL)?|(?:Sub)?Caption|ChangeUI|CheckBitmap|ClearErrors|CompletedText|ComponentText|CopyFiles|CRCCheck|Create(?:Directory|Font|Shortcut)|Delete(?:INI(?:Sec|Str)|Reg(?:Key|Value))?|DetailPrint|DetailsButtonText|Dir(?:Show|Text|Var|Verify)|(?:Disabled|Enabled)Bitmap|EnableWindow|EnumReg(?:Key|Value)|Exch|Exec(?:Shell|Wait)?|ExpandEnvStrings|File(?:BufSize|Close|ErrorText|Open|Read(?:Byte)?|Seek|Write(?:Byte)?)?|Find(?:Close|First|Next|Window)|FlushINI|Function(?:End)?|Get(?:CurInstType|CurrentAddress|DlgItem|DLLVersion(?:Local)?|ErrorLevel|FileTime(?:Local)?|FullPathName|FunctionAddress|InstDirError|LabelAddress|TempFileName)|Goto|HideWindow|Icon|If(?:Abort|Errors|FileExists|RebootFlag|Silent)|InitPluginsDir|Install(?:ButtonText|Colors|Dir(?:RegKey)?)|Inst(?:ProgressFlags|Type(?:[GS]etText)?)|Int(?:CmpU?|Fmt|Op)|IsWindow|LangString(?:UP)?|License(?:BkColor|Data|ForceSelection|LangString|Text)|LoadLanguageFile|LockWindow|Log(?:Set|Text)|MessageBox|MiscButtonText|Name|Nop|OutFile|(?:Uninst)?Page(?:Ex(?:End)?)?|PluginDir|Pop|Push|Quit|Read(?:(?:Env|INI|Reg)Str|RegDWORD)|Reboot|(?:Un)?RegDLL|Rename|RequestExecutionLevel|ReserveFile|Return|RMDir|SearchPath|Section(?:Divider|End|(?:(?:Get|Set)(?:Flags|InstTypes|Size|Text))|Group(?:End)?|In)?|SendMessage|Set(?:AutoClose|BrandingImage|Compress(?:ionLevel|or(?:DictSize)?)?|CtlColors|CurInstType|DatablockOptimize|DateSave|Details(?:Print|View)|Error(?:s|Level)|FileAttributes|Font|OutPath|Overwrite|PluginUnload|RebootFlag|ShellVarContext|Silent|StaticBkColor)|Show(?:(?:I|Uni)nstDetails|Window)|Silent(?:Un)?Install|Sleep|SpaceTexts|Str(?:CmpS?|Cpy|Len)|SubSection(?:End)?|Uninstall(?:ButtonText|(?:Sub)?Caption|EXEName|Icon|Text)|UninstPage|Var|VI(?:AddVersionKey|ProductVersion)|WindowIcon|Write(?:INIStr|Reg(:?Bin|DWORD|(?:Expand)?Str)|Uninstaller)|XPStyle)\b", KEYWORD),
        Rule::token(r"(?im)\b(CUR|END|(?:FILE_ATTRIBUTE_)?(?:ARCHIVE|HIDDEN|NORMAL|OFFLINE|READONLY|SYSTEM|TEMPORARY)|HK(CC|CR|CU|DD|LM|PD|U)|HKEY_(?:CLASSES_ROOT|CURRENT_(?:CONFIG|USER)|DYN_DATA|LOCAL_MACHINE|PERFORMANCE_DATA|USERS)|ID(?:ABORT|CANCEL|IGNORE|NO|OK|RETRY|YES)|MB_(?:ABORTRETRYIGNORE|DEFBUTTON[1-4]|ICON(?:EXCLAMATION|INFORMATION|QUESTION|STOP)|OK(?:CANCEL)?|RETRYCANCEL|RIGHT|SETFOREGROUND|TOPMOST|USERICON|YESNO(?:CANCEL)?)|SET|SHCTX|SW_(?:HIDE|SHOW(?:MAXIMIZED|MINIMIZED|NORMAL))|admin|all|auto|both|bottom|bzip2|checkbox|colored|current|false|force|hide|highest|if(?:diff|newer)|lastused|leave|left|listonly|lzma|nevershow|none|normal|off|on|pop|push|radiobuttons|right|show|silent|silentlog|smooth|textonly|top|true|try|user|zlib)\b", NAME_CONSTANT),
        Rule::token(r"(?im)\$\{[a-z_|][\w|]*\}", KEYWORD_PSEUDO),
        Rule::token(r"(?im)/[a-z_]\w*", NAME_ATTRIBUTE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)[\w.]+", TEXT),
    ]);
    m.insert(r"macro", vec![
        Rule::token(r"(?im)\!(addincludedir(?:dir)?|addplugindir|appendfile|cd|define|delfilefile|echo(?:message)?|else|endif|error|execute|if(?:macro)?n?(?:def)?|include|insertmacro|macro(?:end)?|packhdr|search(?:parse|replace)|system|tempfilesymbol|undef|verbose|warning)\b", COMMENT_PREPROC),
    ]);
    m.insert(r"interpol", vec![
        Rule::token(r"(?im)\$(R?[0-9])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?im)\$(ADMINTOOLS|APPDATA|CDBURN_AREA|COOKIES|COMMONFILES(?:32|64)|DESKTOP|DOCUMENTS|EXE(?:DIR|FILE|PATH)|FAVORITES|FONTS|HISTORY|HWNDPARENT|INTERNET_CACHE|LOCALAPPDATA|MUSIC|NETHOOD|PICTURES|PLUGINSDIR|PRINTHOOD|PROFILE|PROGRAMFILES(?:32|64)|QUICKLAUNCH|RECENT|RESOURCES(?:_LOCALIZED)?|SENDTO|SM(?:PROGRAMS|STARTUP)|STARTMENU|SYSDIR|TEMP(?:LATES)?|VIDEOS|WINDIR|\{NSISDIR\})", NAME_BUILTIN),
        Rule::token(r"(?im)\$(CMDLINE|INSTDIR|OUTDIR|LANGUAGE)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)\$[a-z_]\w*", NAME_VARIABLE),
    ]);
    m.insert(r"basic", vec![
        Rule::bygroups(r"(?im)(\n)(Function)(\s+)([._a-z][.\w]*)\b", vec![Some(WHITESPACE), Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)\b([_a-z]\w*)(::)([a-z][a-z0-9]*)\b", vec![Some(KEYWORD_NAMESPACE), Some(PUNCTUATION), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)\b([_a-z]\w*)(:)", vec![Some(NAME_LABEL), Some(PUNCTUATION)]),
        Rule::token(r"(?im)(\b[ULS]|\B)([!<>=]?=|\<\>?|\>)\B", OPERATOR),
        Rule::token(r"(?im)[|+-]", OPERATOR),
        Rule::token(r"(?im)\\", PUNCTUATION),
        Rule::token(r"(?im)\b(Abort|Add(?:BrandingImage|Size)|Allow(?:RootDirInstall|SkipFiles)|AutoCloseWindow|BG(?:Font|Gradient)|BrandingText|BringToFront|Call(?:InstDLL)?|(?:Sub)?Caption|ChangeUI|CheckBitmap|ClearErrors|CompletedText|ComponentText|CopyFiles|CRCCheck|Create(?:Directory|Font|Shortcut)|Delete(?:INI(?:Sec|Str)|Reg(?:Key|Value))?|DetailPrint|DetailsButtonText|Dir(?:Show|Text|Var|Verify)|(?:Disabled|Enabled)Bitmap|EnableWindow|EnumReg(?:Key|Value)|Exch|Exec(?:Shell|Wait)?|ExpandEnvStrings|File(?:BufSize|Close|ErrorText|Open|Read(?:Byte)?|Seek|Write(?:Byte)?)?|Find(?:Close|First|Next|Window)|FlushINI|Function(?:End)?|Get(?:CurInstType|CurrentAddress|DlgItem|DLLVersion(?:Local)?|ErrorLevel|FileTime(?:Local)?|FullPathName|FunctionAddress|InstDirError|LabelAddress|TempFileName)|Goto|HideWindow|Icon|If(?:Abort|Errors|FileExists|RebootFlag|Silent)|InitPluginsDir|Install(?:ButtonText|Colors|Dir(?:RegKey)?)|Inst(?:ProgressFlags|Type(?:[GS]etText)?)|Int(?:CmpU?|Fmt|Op)|IsWindow|LangString(?:UP)?|License(?:BkColor|Data|ForceSelection|LangString|Text)|LoadLanguageFile|LockWindow|Log(?:Set|Text)|MessageBox|MiscButtonText|Name|Nop|OutFile|(?:Uninst)?Page(?:Ex(?:End)?)?|PluginDir|Pop|Push|Quit|Read(?:(?:Env|INI|Reg)Str|RegDWORD)|Reboot|(?:Un)?RegDLL|Rename|RequestExecutionLevel|ReserveFile|Return|RMDir|SearchPath|Section(?:Divider|End|(?:(?:Get|Set)(?:Flags|InstTypes|Size|Text))|Group(?:End)?|In)?|SendMessage|Set(?:AutoClose|BrandingImage|Compress(?:ionLevel|or(?:DictSize)?)?|CtlColors|CurInstType|DatablockOptimize|DateSave|Details(?:Print|View)|Error(?:s|Level)|FileAttributes|Font|OutPath|Overwrite|PluginUnload|RebootFlag|ShellVarContext|Silent|StaticBkColor)|Show(?:(?:I|Uni)nstDetails|Window)|Silent(?:Un)?Install|Sleep|SpaceTexts|Str(?:CmpS?|Cpy|Len)|SubSection(?:End)?|Uninstall(?:ButtonText|(?:Sub)?Caption|EXEName|Icon|Text)|UninstPage|Var|VI(?:AddVersionKey|ProductVersion)|WindowIcon|Write(?:INIStr|Reg(:?Bin|DWORD|(?:Expand)?Str)|Uninstaller)|XPStyle)\b", KEYWORD),
        Rule::token(r"(?im)\b(CUR|END|(?:FILE_ATTRIBUTE_)?(?:ARCHIVE|HIDDEN|NORMAL|OFFLINE|READONLY|SYSTEM|TEMPORARY)|HK(CC|CR|CU|DD|LM|PD|U)|HKEY_(?:CLASSES_ROOT|CURRENT_(?:CONFIG|USER)|DYN_DATA|LOCAL_MACHINE|PERFORMANCE_DATA|USERS)|ID(?:ABORT|CANCEL|IGNORE|NO|OK|RETRY|YES)|MB_(?:ABORTRETRYIGNORE|DEFBUTTON[1-4]|ICON(?:EXCLAMATION|INFORMATION|QUESTION|STOP)|OK(?:CANCEL)?|RETRYCANCEL|RIGHT|SETFOREGROUND|TOPMOST|USERICON|YESNO(?:CANCEL)?)|SET|SHCTX|SW_(?:HIDE|SHOW(?:MAXIMIZED|MINIMIZED|NORMAL))|admin|all|auto|both|bottom|bzip2|checkbox|colored|current|false|force|hide|highest|if(?:diff|newer)|lastused|leave|left|listonly|lzma|nevershow|none|normal|off|on|pop|push|radiobuttons|right|show|silent|silentlog|smooth|textonly|top|true|try|user|zlib)\b", NAME_CONSTANT),
    ]);
    m.insert(r"str_double", vec![
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?im)\$(\\[nrt"]|\$)"#, STRING_ESCAPE),
        Rule::token(r"(?im)\$(R?[0-9])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?im)\$(ADMINTOOLS|APPDATA|CDBURN_AREA|COOKIES|COMMONFILES(?:32|64)|DESKTOP|DOCUMENTS|EXE(?:DIR|FILE|PATH)|FAVORITES|FONTS|HISTORY|HWNDPARENT|INTERNET_CACHE|LOCALAPPDATA|MUSIC|NETHOOD|PICTURES|PLUGINSDIR|PRINTHOOD|PROFILE|PROGRAMFILES(?:32|64)|QUICKLAUNCH|RECENT|RESOURCES(?:_LOCALIZED)?|SENDTO|SM(?:PROGRAMS|STARTUP)|STARTMENU|SYSDIR|TEMP(?:LATES)?|VIDEOS|WINDIR|\{NSISDIR\})", NAME_BUILTIN),
        Rule::token(r"(?im)\$(CMDLINE|INSTDIR|OUTDIR|LANGUAGE)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)\$[a-z_]\w*", NAME_VARIABLE),
        Rule::token(r#"(?im)[^"]+"#, STRING_DOUBLE),
    ]);
    m.insert(r"str_backtick", vec![
        Rule::token_to(r"(?im)`", STRING_DOUBLE, NewState::Pop(1)),
        Rule::token(r#"(?im)\$(\\[nrt"]|\$)"#, STRING_ESCAPE),
        Rule::token(r"(?im)\$(R?[0-9])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?im)\$(ADMINTOOLS|APPDATA|CDBURN_AREA|COOKIES|COMMONFILES(?:32|64)|DESKTOP|DOCUMENTS|EXE(?:DIR|FILE|PATH)|FAVORITES|FONTS|HISTORY|HWNDPARENT|INTERNET_CACHE|LOCALAPPDATA|MUSIC|NETHOOD|PICTURES|PLUGINSDIR|PRINTHOOD|PROFILE|PROGRAMFILES(?:32|64)|QUICKLAUNCH|RECENT|RESOURCES(?:_LOCALIZED)?|SENDTO|SM(?:PROGRAMS|STARTUP)|STARTMENU|SYSDIR|TEMP(?:LATES)?|VIDEOS|WINDIR|\{NSISDIR\})", NAME_BUILTIN),
        Rule::token(r"(?im)\$(CMDLINE|INSTDIR|OUTDIR|LANGUAGE)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?im)\$[a-z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?im)[^`]+", STRING_DOUBLE),
    ]);
    Table(m)
}

impl Lexer for NsisLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
