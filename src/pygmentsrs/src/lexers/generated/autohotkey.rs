//! AUTO-GENERATED from `pygments.pygments.lexers.automation:AutohotkeyLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.automation:AutohotkeyLexer:autohotkey

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: autohotkey, ahk
pub struct AutohotkeyLexer;

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
        Rule::bygroups_to(r"(?m)^(\s*)(/\*)", vec![Some(TEXT), Some(COMMENT_MULTILINE)], NewState::Push(vec![r"incomment"])),
        Rule::bygroups_to(r"(?m)^(\s*)(\()", vec![Some(TEXT), Some(GENERIC)], NewState::Push(vec![r"incontinuation"])),
        Rule::token(r"(?m)\s+;.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)^;.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m)[\]{}(),;\[]", PUNCTUATION),
        Rule::token(r"(?m)(in|is|and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)\%[a-zA-Z_#@$][\w#@$]*\%", NAME_VARIABLE),
        Rule::token(r"(?m)!=|==|:=|\.=|<<|>>|[-~+/*%=<>&^|?:!.]", OPERATOR),
        Rule::bygroups(r"(?m)(?i)^(\s*)(global|local|static|#AllowSameLineComments|#ClipboardTimeout|#CommentFlag|#ErrorStdOut|#EscapeChar|#HotkeyInterval|#HotkeyModifierTimeout|#Hotstring|#IfWinActive|#IfWinExist|#IfWinNotActive|#IfWinNotExist|#IncludeAgain|#Include|#InstallKeybdHook|#InstallMouseHook|#KeyHistory|#LTrim|#MaxHotkeysPerInterval|#MaxMem|#MaxThreads|#MaxThreadsBuffer|#MaxThreadsPerHotkey|#NoEnv|#NoTrayIcon|#Persistent|#SingleInstance|#UseHook|#WinActivateForce|AutoTrim|BlockInput|Break|Click|ClipWait|Continue|Control|ControlClick|ControlFocus|ControlGetFocus|ControlGetPos|ControlGetText|ControlGet|ControlMove|ControlSend|ControlSendRaw|ControlSetText|CoordMode|Critical|DetectHiddenText|DetectHiddenWindows|Drive|DriveGet|DriveSpaceFree|Edit|Else|EnvAdd|EnvDiv|EnvGet|EnvMult|EnvSet|EnvSub|EnvUpdate|Exit|ExitApp|FileAppend|FileCopy|FileCopyDir|FileCreateDir|FileCreateShortcut|FileDelete|FileGetAttrib|FileGetShortcut|FileGetSize|FileGetTime|FileGetVersion|FileInstall|FileMove|FileMoveDir|FileRead|FileReadLine|FileRecycle|FileRecycleEmpty|FileRemoveDir|FileSelectFile|FileSelectFolder|FileSetAttrib|FileSetTime|FormatTime|GetKeyState|Gosub|Goto|GroupActivate|GroupAdd|GroupClose|GroupDeactivate|Gui|GuiControl|GuiControlGet|Hotkey|IfEqual|IfExist|IfGreaterOrEqual|IfGreater|IfInString|IfLess|IfLessOrEqual|IfMsgBox|IfNotEqual|IfNotExist|IfNotInString|IfWinActive|IfWinExist|IfWinNotActive|IfWinNotExist|If |ImageSearch|IniDelete|IniRead|IniWrite|InputBox|Input|KeyHistory|KeyWait|ListHotkeys|ListLines|ListVars|Loop|Menu|MouseClickDrag|MouseClick|MouseGetPos|MouseMove|MsgBox|OnExit|OutputDebug|Pause|PixelGetColor|PixelSearch|PostMessage|Process|Progress|Random|RegDelete|RegRead|RegWrite|Reload|Repeat|Return|RunAs|RunWait|Run|SendEvent|SendInput|SendMessage|SendMode|SendPlay|SendRaw|Send|SetBatchLines|SetCapslockState|SetControlDelay|SetDefaultMouseSpeed|SetEnv|SetFormat|SetKeyDelay|SetMouseDelay|SetNumlockState|SetScrollLockState|SetStoreCapslockMode|SetTimer|SetTitleMatchMode|SetWinDelay|SetWorkingDir|Shutdown|Sleep|Sort|SoundBeep|SoundGet|SoundGetWaveVolume|SoundPlay|SoundSet|SoundSetWaveVolume|SplashImage|SplashTextOff|SplashTextOn|SplitPath|StatusBarGetText|StatusBarWait|StringCaseSense|StringGetPos|StringLeft|StringLen|StringLower|StringMid|StringReplace|StringRight|StringSplit|StringTrimLeft|StringTrimRight|StringUpper|Suspend|SysGet|Thread|ToolTip|Transform|TrayTip|URLDownloadToFile|While|WinActivate|WinActivateBottom|WinClose|WinGetActiveStats|WinGetActiveTitle|WinGetClass|WinGetPos|WinGetText|WinGetTitle|WinGet|WinHide|WinKill|WinMaximize|WinMenuSelectItem|WinMinimizeAllUndo|WinMinimizeAll|WinMinimize|WinMove|WinRestore|WinSetTitle|WinSet|WinShow|WinWaitActive|WinWaitClose|WinWaitNotActive|WinWait)\b", vec![Some(TEXT), Some(NAME_BUILTIN)]),
        Rule::bygroups(r#"(?m)(^\s*)([^:\s("]+?:{1,2})"#, vec![Some(TEXT), Some(NAME_LABEL)]),
        Rule::bygroups(r"(?m)(^\s*)(::[^:\s]+?::)", vec![Some(TEXT), Some(NAME_LABEL)]),
        Rule::token(r"(?m)(?i)(Abs|ACos|Asc|ASin|ATan|Ceil|Chr|Cos|DllCall|Exp|FileExist|Floor|GetKeyState|IL_Add|IL_Create|IL_Destroy|InStr|IsFunc|IsLabel|Ln|Log|LV_Add|LV_Delete|LV_DeleteCol|LV_GetCount|LV_GetNext|LV_GetText|LV_Insert|LV_InsertCol|LV_Modify|LV_ModifyCol|LV_SetImageList|Mod|NumGet|NumPut|OnMessage|RegExMatch|RegExReplace|RegisterCallback|Round|SB_SetIcon|SB_SetParts|SB_SetText|Sin|Sqrt|StrLen|SubStr|Tan|TV_Add|TV_Delete|TV_GetChild|TV_GetCount|TV_GetNext|TV_Get|TV_GetParent|TV_GetPrev|TV_GetSelection|TV_GetText|TV_Modify|VarSetCapacity|WinActive|WinExist|Object|ComObjActive|ComObjArray|ComObjEnwrap|ComObjUnwrap|ComObjParameter|ComObjType|ComObjConnect|ComObjCreate|ComObjGet|ComObjError|ComObjValue|Insert|MinIndex|MaxIndex|Remove|SetCapacity|GetCapacity|GetAddress|_NewEnum|FileOpen|Read|Write|ReadLine|WriteLine|ReadNumType|WriteNumType|RawRead|RawWrite|Seek|Tell|Close|Next|IsObject|StrPut|StrGet|Trim|LTrim|RTrim)\b", NAME_FUNCTION),
        Rule::token(r"(?m)(?i)(A_AhkPath|A_AhkVersion|A_AppData|A_AppDataCommon|A_AutoTrim|A_BatchLines|A_CaretX|A_CaretY|A_ComputerName|A_ControlDelay|A_Cursor|A_DDDD|A_DDD|A_DD|A_DefaultMouseSpeed|A_Desktop|A_DesktopCommon|A_DetectHiddenText|A_DetectHiddenWindows|A_EndChar|A_EventInfo|A_ExitReason|A_FormatFloat|A_FormatInteger|A_Gui|A_GuiEvent|A_GuiControl|A_GuiControlEvent|A_GuiHeight|A_GuiWidth|A_GuiX|A_GuiY|A_Hour|A_IconFile|A_IconHidden|A_IconNumber|A_IconTip|A_Index|A_IPAddress1|A_IPAddress2|A_IPAddress3|A_IPAddress4|A_ISAdmin|A_IsCompiled|A_IsCritical|A_IsPaused|A_IsSuspended|A_KeyDelay|A_Language|A_LastError|A_LineFile|A_LineNumber|A_LoopField|A_LoopFileAttrib|A_LoopFileDir|A_LoopFileExt|A_LoopFileFullPath|A_LoopFileLongPath|A_LoopFileName|A_LoopFileShortName|A_LoopFileShortPath|A_LoopFileSize|A_LoopFileSizeKB|A_LoopFileSizeMB|A_LoopFileTimeAccessed|A_LoopFileTimeCreated|A_LoopFileTimeModified|A_LoopReadLine|A_LoopRegKey|A_LoopRegName|A_LoopRegSubkey|A_LoopRegTimeModified|A_LoopRegType|A_MDAY|A_Min|A_MM|A_MMM|A_MMMM|A_Mon|A_MouseDelay|A_MSec|A_MyDocuments|A_Now|A_NowUTC|A_NumBatchLines|A_OSType|A_OSVersion|A_PriorHotkey|A_ProgramFiles|A_Programs|A_ProgramsCommon|A_ScreenHeight|A_ScreenWidth|A_ScriptDir|A_ScriptFullPath|A_ScriptName|A_Sec|A_Space|A_StartMenu|A_StartMenuCommon|A_Startup|A_StartupCommon|A_StringCaseSense|A_Tab|A_Temp|A_ThisFunc|A_ThisHotkey|A_ThisLabel|A_ThisMenu|A_ThisMenuItem|A_ThisMenuItemPos|A_TickCount|A_TimeIdle|A_TimeIdlePhysical|A_TimeSincePriorHotkey|A_TimeSinceThisHotkey|A_TitleMatchMode|A_TitleMatchModeSpeed|A_UserName|A_WDay|A_WinDelay|A_WinDir|A_WorkingDir|A_YDay|A_YEAR|A_YWeek|A_YYYY|Clipboard|ClipboardAll|ComSpec|ErrorLevel|ProgramFiles|True|False|A_IsUnicode|A_FileEncoding|A_OSVersion|A_PtrSize)\b", NAME_VARIABLE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0\d+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[a-zA-Z_#@$][\w#@$]*", NAME),
        Rule::token(r"(?m)\\|\'", TEXT),
        Rule::token(r"(?m)\`([,%`abfnrtv\-+;])", STRING_ESCAPE),
        Rule::token(r"(?m)[^\S\n]", TEXT),
    ]);
    m.insert(r"commands", vec![
        Rule::bygroups(r"(?m)(?i)^(\s*)(global|local|static|#AllowSameLineComments|#ClipboardTimeout|#CommentFlag|#ErrorStdOut|#EscapeChar|#HotkeyInterval|#HotkeyModifierTimeout|#Hotstring|#IfWinActive|#IfWinExist|#IfWinNotActive|#IfWinNotExist|#IncludeAgain|#Include|#InstallKeybdHook|#InstallMouseHook|#KeyHistory|#LTrim|#MaxHotkeysPerInterval|#MaxMem|#MaxThreads|#MaxThreadsBuffer|#MaxThreadsPerHotkey|#NoEnv|#NoTrayIcon|#Persistent|#SingleInstance|#UseHook|#WinActivateForce|AutoTrim|BlockInput|Break|Click|ClipWait|Continue|Control|ControlClick|ControlFocus|ControlGetFocus|ControlGetPos|ControlGetText|ControlGet|ControlMove|ControlSend|ControlSendRaw|ControlSetText|CoordMode|Critical|DetectHiddenText|DetectHiddenWindows|Drive|DriveGet|DriveSpaceFree|Edit|Else|EnvAdd|EnvDiv|EnvGet|EnvMult|EnvSet|EnvSub|EnvUpdate|Exit|ExitApp|FileAppend|FileCopy|FileCopyDir|FileCreateDir|FileCreateShortcut|FileDelete|FileGetAttrib|FileGetShortcut|FileGetSize|FileGetTime|FileGetVersion|FileInstall|FileMove|FileMoveDir|FileRead|FileReadLine|FileRecycle|FileRecycleEmpty|FileRemoveDir|FileSelectFile|FileSelectFolder|FileSetAttrib|FileSetTime|FormatTime|GetKeyState|Gosub|Goto|GroupActivate|GroupAdd|GroupClose|GroupDeactivate|Gui|GuiControl|GuiControlGet|Hotkey|IfEqual|IfExist|IfGreaterOrEqual|IfGreater|IfInString|IfLess|IfLessOrEqual|IfMsgBox|IfNotEqual|IfNotExist|IfNotInString|IfWinActive|IfWinExist|IfWinNotActive|IfWinNotExist|If |ImageSearch|IniDelete|IniRead|IniWrite|InputBox|Input|KeyHistory|KeyWait|ListHotkeys|ListLines|ListVars|Loop|Menu|MouseClickDrag|MouseClick|MouseGetPos|MouseMove|MsgBox|OnExit|OutputDebug|Pause|PixelGetColor|PixelSearch|PostMessage|Process|Progress|Random|RegDelete|RegRead|RegWrite|Reload|Repeat|Return|RunAs|RunWait|Run|SendEvent|SendInput|SendMessage|SendMode|SendPlay|SendRaw|Send|SetBatchLines|SetCapslockState|SetControlDelay|SetDefaultMouseSpeed|SetEnv|SetFormat|SetKeyDelay|SetMouseDelay|SetNumlockState|SetScrollLockState|SetStoreCapslockMode|SetTimer|SetTitleMatchMode|SetWinDelay|SetWorkingDir|Shutdown|Sleep|Sort|SoundBeep|SoundGet|SoundGetWaveVolume|SoundPlay|SoundSet|SoundSetWaveVolume|SplashImage|SplashTextOff|SplashTextOn|SplitPath|StatusBarGetText|StatusBarWait|StringCaseSense|StringGetPos|StringLeft|StringLen|StringLower|StringMid|StringReplace|StringRight|StringSplit|StringTrimLeft|StringTrimRight|StringUpper|Suspend|SysGet|Thread|ToolTip|Transform|TrayTip|URLDownloadToFile|While|WinActivate|WinActivateBottom|WinClose|WinGetActiveStats|WinGetActiveTitle|WinGetClass|WinGetPos|WinGetText|WinGetTitle|WinGet|WinHide|WinKill|WinMaximize|WinMenuSelectItem|WinMinimizeAllUndo|WinMinimizeAll|WinMinimize|WinMove|WinRestore|WinSetTitle|WinSet|WinShow|WinWaitActive|WinWaitClose|WinWaitNotActive|WinWait)\b", vec![Some(TEXT), Some(NAME_BUILTIN)]),
    ]);
    m.insert(
        r"labels",
        vec![
            Rule::bygroups(
                r#"(?m)(^\s*)([^:\s("]+?:{1,2})"#,
                vec![Some(TEXT), Some(NAME_LABEL)],
            ),
            Rule::bygroups(
                r"(?m)(^\s*)(::[^:\s]+?::)",
                vec![Some(TEXT), Some(NAME_LABEL)],
            ),
        ],
    );
    m.insert(r"builtInFunctions", vec![
        Rule::token(r"(?m)(?i)(Abs|ACos|Asc|ASin|ATan|Ceil|Chr|Cos|DllCall|Exp|FileExist|Floor|GetKeyState|IL_Add|IL_Create|IL_Destroy|InStr|IsFunc|IsLabel|Ln|Log|LV_Add|LV_Delete|LV_DeleteCol|LV_GetCount|LV_GetNext|LV_GetText|LV_Insert|LV_InsertCol|LV_Modify|LV_ModifyCol|LV_SetImageList|Mod|NumGet|NumPut|OnMessage|RegExMatch|RegExReplace|RegisterCallback|Round|SB_SetIcon|SB_SetParts|SB_SetText|Sin|Sqrt|StrLen|SubStr|Tan|TV_Add|TV_Delete|TV_GetChild|TV_GetCount|TV_GetNext|TV_Get|TV_GetParent|TV_GetPrev|TV_GetSelection|TV_GetText|TV_Modify|VarSetCapacity|WinActive|WinExist|Object|ComObjActive|ComObjArray|ComObjEnwrap|ComObjUnwrap|ComObjParameter|ComObjType|ComObjConnect|ComObjCreate|ComObjGet|ComObjError|ComObjValue|Insert|MinIndex|MaxIndex|Remove|SetCapacity|GetCapacity|GetAddress|_NewEnum|FileOpen|Read|Write|ReadLine|WriteLine|ReadNumType|WriteNumType|RawRead|RawWrite|Seek|Tell|Close|Next|IsObject|StrPut|StrGet|Trim|LTrim|RTrim)\b", NAME_FUNCTION),
    ]);
    m.insert(r"builtInVariables", vec![
        Rule::token(r"(?m)(?i)(A_AhkPath|A_AhkVersion|A_AppData|A_AppDataCommon|A_AutoTrim|A_BatchLines|A_CaretX|A_CaretY|A_ComputerName|A_ControlDelay|A_Cursor|A_DDDD|A_DDD|A_DD|A_DefaultMouseSpeed|A_Desktop|A_DesktopCommon|A_DetectHiddenText|A_DetectHiddenWindows|A_EndChar|A_EventInfo|A_ExitReason|A_FormatFloat|A_FormatInteger|A_Gui|A_GuiEvent|A_GuiControl|A_GuiControlEvent|A_GuiHeight|A_GuiWidth|A_GuiX|A_GuiY|A_Hour|A_IconFile|A_IconHidden|A_IconNumber|A_IconTip|A_Index|A_IPAddress1|A_IPAddress2|A_IPAddress3|A_IPAddress4|A_ISAdmin|A_IsCompiled|A_IsCritical|A_IsPaused|A_IsSuspended|A_KeyDelay|A_Language|A_LastError|A_LineFile|A_LineNumber|A_LoopField|A_LoopFileAttrib|A_LoopFileDir|A_LoopFileExt|A_LoopFileFullPath|A_LoopFileLongPath|A_LoopFileName|A_LoopFileShortName|A_LoopFileShortPath|A_LoopFileSize|A_LoopFileSizeKB|A_LoopFileSizeMB|A_LoopFileTimeAccessed|A_LoopFileTimeCreated|A_LoopFileTimeModified|A_LoopReadLine|A_LoopRegKey|A_LoopRegName|A_LoopRegSubkey|A_LoopRegTimeModified|A_LoopRegType|A_MDAY|A_Min|A_MM|A_MMM|A_MMMM|A_Mon|A_MouseDelay|A_MSec|A_MyDocuments|A_Now|A_NowUTC|A_NumBatchLines|A_OSType|A_OSVersion|A_PriorHotkey|A_ProgramFiles|A_Programs|A_ProgramsCommon|A_ScreenHeight|A_ScreenWidth|A_ScriptDir|A_ScriptFullPath|A_ScriptName|A_Sec|A_Space|A_StartMenu|A_StartMenuCommon|A_Startup|A_StartupCommon|A_StringCaseSense|A_Tab|A_Temp|A_ThisFunc|A_ThisHotkey|A_ThisLabel|A_ThisMenu|A_ThisMenuItem|A_ThisMenuItemPos|A_TickCount|A_TimeIdle|A_TimeIdlePhysical|A_TimeSincePriorHotkey|A_TimeSinceThisHotkey|A_TitleMatchMode|A_TitleMatchModeSpeed|A_UserName|A_WDay|A_WinDelay|A_WinDir|A_WorkingDir|A_YDay|A_YEAR|A_YWeek|A_YYYY|Clipboard|ClipboardAll|ComSpec|ErrorLevel|ProgramFiles|True|False|A_IsUnicode|A_FileEncoding|A_OSVersion|A_PtrSize)\b", NAME_VARIABLE),
    ]);
    m.insert(
        r"stringescape",
        vec![Rule::token(r#"(?m)\"\"|\`([,%`abfnrtv])"#, STRING_ESCAPE)],
    );
    m.insert(
        r"dqs",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\n]+"#, STRING),
        ],
    );
    m.insert(r"strings", vec![Rule::token(r#"(?m)[^"\n]+"#, STRING)]);
    m.insert(
        r"_tmp_0",
        vec![
            Rule::token(r#"(?m)\"\"|\`([,%`abfnrtv])"#, STRING_ESCAPE),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\n]+"#, STRING),
        ],
    );
    m.insert(
        r"numbers",
        vec![
            Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
            Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
            Rule::token(r"(?m)0\d+", NUMBER_OCT),
            Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
            Rule::token(
                r"(?m)\d+L",
                TokenType::new(&["Literal", "Number", "Integer", "Long"]),
            ),
            Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        ],
    );
    m.insert(r"garbage", vec![Rule::token(r"(?m)[^\S\n]", TEXT)]);
    m.insert(
        r"incomment",
        vec![
            Rule::token_to(r"(?m)^\s*\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[^*]+", COMMENT_MULTILINE),
            Rule::token(r"(?m)\*", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"incontinuation",
        vec![
            Rule::token_to(r"(?m)^\s*\)", GENERIC, NewState::Pop(1)),
            Rule::token(r"(?m)[^)]", GENERIC),
            Rule::token(r"(?m)[)]", GENERIC),
        ],
    );
    Table(m)
}

impl Lexer for AutohotkeyLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
