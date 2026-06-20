//! AUTO-GENERATED from `pygments.pygments.lexers.automation:AutoItLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.automation:AutoItLexer:autoit

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: autoit
pub struct AutoitLexer;

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
        Rule::token(r"(?m);.*\n", COMMENT_SINGLE),
        Rule::token(r"(?m)(#comments-start|#cs)(.|\n)*?(#comments-end|#ce)", COMMENT_MULTILINE),
        Rule::token(r"(?m)[\[\]{}(),;]", PUNCTUATION),
        Rule::token(r"(?m)(and|or|not)\b", OPERATOR_WORD),
        Rule::token(r"(?m)[$|@][a-zA-Z_]\w*", NAME_VARIABLE),
        Rule::token(r"(?m)!=|==|:=|\.=|<<|>>|[-~+/*%=<>&^|?:!.]", OPERATOR),
        Rule::bygroups(r"(?m)(?i)(\s*)(#include-once|#include|#endregion|#forcedef|#forceref|#region|and|byref|case|continueloop|dim|do|else|elseif|endfunc|endif|endselect|exit|exitloop|for|func|global|if|local|next|not|or|return|select|step|then|to|until|wend|while|exit)\b", vec![Some(TEXT), Some(NAME_BUILTIN)]),
        Rule::bygroups(r"(?m)(^\s*)(\{\S+?\})", vec![Some(TEXT), Some(NAME_LABEL)]),
        Rule::token(r"(?m)(?i)(abs|acos|adlibregister|adlibunregister|asc|ascw|asin|assign|atan|autoitsetoption|autoitwingettitle|autoitwinsettitle|beep|binary|binarylen|binarymid|binarytostring|bitand|bitnot|bitor|bitrotate|bitshift|bitxor|blockinput|break|call|cdtray|ceiling|chr|chrw|clipget|clipput|consoleread|consolewrite|consolewriteerror|controlclick|controlcommand|controldisable|controlenable|controlfocus|controlgetfocus|controlgethandle|controlgetpos|controlgettext|controlhide|controllistview|controlmove|controlsend|controlsettext|controlshow|controltreeview|cos|dec|dircopy|dircreate|dirgetsize|dirmove|dirremove|dllcall|dllcalladdress|dllcallbackfree|dllcallbackgetptr|dllcallbackregister|dllclose|dllopen|dllstructcreate|dllstructgetdata|dllstructgetptr|dllstructgetsize|dllstructsetdata|drivegetdrive|drivegetfilesystem|drivegetlabel|drivegetserial|drivegettype|drivemapadd|drivemapdel|drivemapget|drivesetlabel|drivespacefree|drivespacetotal|drivestatus|envget|envset|envupdate|eval|execute|exp|filechangedir|fileclose|filecopy|filecreatentfslink|filecreateshortcut|filedelete|fileexists|filefindfirstfile|filefindnextfile|fileflush|filegetattrib|filegetencoding|filegetlongname|filegetpos|filegetshortcut|filegetshortname|filegetsize|filegettime|filegetversion|fileinstall|filemove|fileopen|fileopendialog|fileread|filereadline|filerecycle|filerecycleempty|filesavedialog|fileselectfolder|filesetattrib|filesetpos|filesettime|filewrite|filewriteline|floor|ftpsetproxy|guicreate|guictrlcreateavi|guictrlcreatebutton|guictrlcreatecheckbox|guictrlcreatecombo|guictrlcreatecontextmenu|guictrlcreatedate|guictrlcreatedummy|guictrlcreateedit|guictrlcreategraphic|guictrlcreategroup|guictrlcreateicon|guictrlcreateinput|guictrlcreatelabel|guictrlcreatelist|guictrlcreatelistview|guictrlcreatelistviewitem|guictrlcreatemenu|guictrlcreatemenuitem|guictrlcreatemonthcal|guictrlcreateobj|guictrlcreatepic|guictrlcreateprogress|guictrlcreateradio|guictrlcreateslider|guictrlcreatetab|guictrlcreatetabitem|guictrlcreatetreeview|guictrlcreatetreeviewitem|guictrlcreateupdown|guictrldelete|guictrlgethandle|guictrlgetstate|guictrlread|guictrlrecvmsg|guictrlregisterlistviewsort|guictrlsendmsg|guictrlsendtodummy|guictrlsetbkcolor|guictrlsetcolor|guictrlsetcursor|guictrlsetdata|guictrlsetdefbkcolor|guictrlsetdefcolor|guictrlsetfont|guictrlsetgraphic|guictrlsetimage|guictrlsetlimit|guictrlsetonevent|guictrlsetpos|guictrlsetresizing|guictrlsetstate|guictrlsetstyle|guictrlsettip|guidelete|guigetcursorinfo|guigetmsg|guigetstyle|guiregistermsg|guisetaccelerators|guisetbkcolor|guisetcoord|guisetcursor|guisetfont|guisethelp|guiseticon|guisetonevent|guisetstate|guisetstyle|guistartgroup|guiswitch|hex|hotkeyset|httpsetproxy|httpsetuseragent|hwnd|inetclose|inetget|inetgetinfo|inetgetsize|inetread|inidelete|iniread|inireadsection|inireadsectionnames|inirenamesection|iniwrite|iniwritesection|inputbox|int|isadmin|isarray|isbinary|isbool|isdeclared|isdllstruct|isfloat|ishwnd|isint|iskeyword|isnumber|isobj|isptr|isstring|log|memgetstats|mod|mouseclick|mouseclickdrag|mousedown|mousegetcursor|mousegetpos|mousemove|mouseup|mousewheel|msgbox|number|objcreate|objcreateinterface|objevent|objevent|objget|objname|onautoitexitregister|onautoitexitunregister|opt|ping|pixelchecksum|pixelgetcolor|pixelsearch|pluginclose|pluginopen|processclose|processexists|processgetstats|processlist|processsetpriority|processwait|processwaitclose|progressoff|progresson|progressset|ptr|random|regdelete|regenumkey|regenumval|regread|regwrite|round|run|runas|runaswait|runwait|send|sendkeepactive|seterror|setextended|shellexecute|shellexecutewait|shutdown|sin|sleep|soundplay|soundsetwavevolume|splashimageon|splashoff|splashtexton|sqrt|srandom|statusbargettext|stderrread|stdinwrite|stdioclose|stdoutread|string|stringaddcr|stringcompare|stringformat|stringfromasciiarray|stringinstr|stringisalnum|stringisalpha|stringisascii|stringisdigit|stringisfloat|stringisint|stringislower|stringisspace|stringisupper|stringisxdigit|stringleft|stringlen|stringlower|stringmid|stringregexp|stringregexpreplace|stringreplace|stringright|stringsplit|stringstripcr|stringstripws|stringtoasciiarray|stringtobinary|stringtrimleft|stringtrimright|stringupper|tan|tcpaccept|tcpclosesocket|tcpconnect|tcplisten|tcpnametoip|tcprecv|tcpsend|tcpshutdown|tcpstartup|timerdiff|timerinit|tooltip|traycreateitem|traycreatemenu|traygetmsg|trayitemdelete|trayitemgethandle|trayitemgetstate|trayitemgettext|trayitemsetonevent|trayitemsetstate|trayitemsettext|traysetclick|trayseticon|traysetonevent|traysetpauseicon|traysetstate|traysettooltip|traytip|ubound|udpbind|udpclosesocket|udpopen|udprecv|udpsend|udpshutdown|udpstartup|vargettype|winactivate|winactive|winclose|winexists|winflash|wingetcaretpos|wingetclasslist|wingetclientsize|wingethandle|wingetpos|wingetprocess|wingetstate|wingettext|wingettitle|winkill|winlist|winmenuselectitem|winminimizeall|winminimizeallundo|winmove|winsetontop|winsetstate|winsettitle|winsettrans|winwait|winwaitactive|winwaitclose|winwaitnotactive)\b", NAME_FUNCTION),
        Rule::token(r"(?m)(?i)(@appdatacommondir|@appdatadir|@autoitexe|@autoitpid|@autoitversion|@autoitx64|@com_eventobj|@commonfilesdir|@compiled|@computername|@comspec|@cpuarch|@cr|@crlf|@desktopcommondir|@desktopdepth|@desktopdir|@desktopheight|@desktoprefresh|@desktopwidth|@documentscommondir|@error|@exitcode|@exitmethod|@extended|@favoritescommondir|@favoritesdir|@gui_ctrlhandle|@gui_ctrlid|@gui_dragfile|@gui_dragid|@gui_dropid|@gui_winhandle|@homedrive|@homepath|@homeshare|@hotkeypressed|@hour|@ipaddress1|@ipaddress2|@ipaddress3|@ipaddress4|@kblayout|@lf|@logondnsdomain|@logondomain|@logonserver|@mday|@min|@mon|@msec|@muilang|@mydocumentsdir|@numparams|@osarch|@osbuild|@oslang|@osservicepack|@ostype|@osversion|@programfilesdir|@programscommondir|@programsdir|@scriptdir|@scriptfullpath|@scriptlinenumber|@scriptname|@sec|@startmenucommondir|@startmenudir|@startupcommondir|@startupdir|@sw_disable|@sw_enable|@sw_hide|@sw_lock|@sw_maximize|@sw_minimize|@sw_restore|@sw_show|@sw_showdefault|@sw_showmaximized|@sw_showminimized|@sw_showminnoactive|@sw_showna|@sw_shownoactivate|@sw_shownormal|@sw_unlock|@systemdir|@tab|@tempdir|@tray_id|@trayiconflashing|@trayiconvisible|@username|@userprofiledir|@wday|@windowsdir|@workingdir|@yday|@year)\b", NAME_VARIABLE_GLOBAL),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"_tmp_0"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqs"])),
        Rule::token(r"(?m)(\d+\.\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+[eE][+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?m)0\d+", NUMBER_OCT),
        Rule::token(r"(?m)0[xX][a-fA-F0-9]+", NUMBER_HEX),
        Rule::token(r"(?m)\d+L", TokenType::new(&["Literal", "Number", "Integer", "Long"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[a-zA-Z_#@$][\w#@$]*", NAME),
        Rule::token(r"(?m)\\|\'", TEXT),
        Rule::token(r"(?m)\`([,%`abfnrtv\-+;])", STRING_ESCAPE),
        Rule::token(r"(?m)_\n", TEXT),
        Rule::token(r"(?m)[^\S\n]", TEXT),
    ]);
    m.insert(r"commands", vec![
        Rule::bygroups(r"(?m)(?i)(\s*)(#include-once|#include|#endregion|#forcedef|#forceref|#region|and|byref|case|continueloop|dim|do|else|elseif|endfunc|endif|endselect|exit|exitloop|for|func|global|if|local|next|not|or|return|select|step|then|to|until|wend|while|exit)\b", vec![Some(TEXT), Some(NAME_BUILTIN)]),
    ]);
    m.insert(
        r"labels",
        vec![Rule::bygroups(
            r"(?m)(^\s*)(\{\S+?\})",
            vec![Some(TEXT), Some(NAME_LABEL)],
        )],
    );
    m.insert(r"builtInFunctions", vec![
        Rule::token(r"(?m)(?i)(abs|acos|adlibregister|adlibunregister|asc|ascw|asin|assign|atan|autoitsetoption|autoitwingettitle|autoitwinsettitle|beep|binary|binarylen|binarymid|binarytostring|bitand|bitnot|bitor|bitrotate|bitshift|bitxor|blockinput|break|call|cdtray|ceiling|chr|chrw|clipget|clipput|consoleread|consolewrite|consolewriteerror|controlclick|controlcommand|controldisable|controlenable|controlfocus|controlgetfocus|controlgethandle|controlgetpos|controlgettext|controlhide|controllistview|controlmove|controlsend|controlsettext|controlshow|controltreeview|cos|dec|dircopy|dircreate|dirgetsize|dirmove|dirremove|dllcall|dllcalladdress|dllcallbackfree|dllcallbackgetptr|dllcallbackregister|dllclose|dllopen|dllstructcreate|dllstructgetdata|dllstructgetptr|dllstructgetsize|dllstructsetdata|drivegetdrive|drivegetfilesystem|drivegetlabel|drivegetserial|drivegettype|drivemapadd|drivemapdel|drivemapget|drivesetlabel|drivespacefree|drivespacetotal|drivestatus|envget|envset|envupdate|eval|execute|exp|filechangedir|fileclose|filecopy|filecreatentfslink|filecreateshortcut|filedelete|fileexists|filefindfirstfile|filefindnextfile|fileflush|filegetattrib|filegetencoding|filegetlongname|filegetpos|filegetshortcut|filegetshortname|filegetsize|filegettime|filegetversion|fileinstall|filemove|fileopen|fileopendialog|fileread|filereadline|filerecycle|filerecycleempty|filesavedialog|fileselectfolder|filesetattrib|filesetpos|filesettime|filewrite|filewriteline|floor|ftpsetproxy|guicreate|guictrlcreateavi|guictrlcreatebutton|guictrlcreatecheckbox|guictrlcreatecombo|guictrlcreatecontextmenu|guictrlcreatedate|guictrlcreatedummy|guictrlcreateedit|guictrlcreategraphic|guictrlcreategroup|guictrlcreateicon|guictrlcreateinput|guictrlcreatelabel|guictrlcreatelist|guictrlcreatelistview|guictrlcreatelistviewitem|guictrlcreatemenu|guictrlcreatemenuitem|guictrlcreatemonthcal|guictrlcreateobj|guictrlcreatepic|guictrlcreateprogress|guictrlcreateradio|guictrlcreateslider|guictrlcreatetab|guictrlcreatetabitem|guictrlcreatetreeview|guictrlcreatetreeviewitem|guictrlcreateupdown|guictrldelete|guictrlgethandle|guictrlgetstate|guictrlread|guictrlrecvmsg|guictrlregisterlistviewsort|guictrlsendmsg|guictrlsendtodummy|guictrlsetbkcolor|guictrlsetcolor|guictrlsetcursor|guictrlsetdata|guictrlsetdefbkcolor|guictrlsetdefcolor|guictrlsetfont|guictrlsetgraphic|guictrlsetimage|guictrlsetlimit|guictrlsetonevent|guictrlsetpos|guictrlsetresizing|guictrlsetstate|guictrlsetstyle|guictrlsettip|guidelete|guigetcursorinfo|guigetmsg|guigetstyle|guiregistermsg|guisetaccelerators|guisetbkcolor|guisetcoord|guisetcursor|guisetfont|guisethelp|guiseticon|guisetonevent|guisetstate|guisetstyle|guistartgroup|guiswitch|hex|hotkeyset|httpsetproxy|httpsetuseragent|hwnd|inetclose|inetget|inetgetinfo|inetgetsize|inetread|inidelete|iniread|inireadsection|inireadsectionnames|inirenamesection|iniwrite|iniwritesection|inputbox|int|isadmin|isarray|isbinary|isbool|isdeclared|isdllstruct|isfloat|ishwnd|isint|iskeyword|isnumber|isobj|isptr|isstring|log|memgetstats|mod|mouseclick|mouseclickdrag|mousedown|mousegetcursor|mousegetpos|mousemove|mouseup|mousewheel|msgbox|number|objcreate|objcreateinterface|objevent|objevent|objget|objname|onautoitexitregister|onautoitexitunregister|opt|ping|pixelchecksum|pixelgetcolor|pixelsearch|pluginclose|pluginopen|processclose|processexists|processgetstats|processlist|processsetpriority|processwait|processwaitclose|progressoff|progresson|progressset|ptr|random|regdelete|regenumkey|regenumval|regread|regwrite|round|run|runas|runaswait|runwait|send|sendkeepactive|seterror|setextended|shellexecute|shellexecutewait|shutdown|sin|sleep|soundplay|soundsetwavevolume|splashimageon|splashoff|splashtexton|sqrt|srandom|statusbargettext|stderrread|stdinwrite|stdioclose|stdoutread|string|stringaddcr|stringcompare|stringformat|stringfromasciiarray|stringinstr|stringisalnum|stringisalpha|stringisascii|stringisdigit|stringisfloat|stringisint|stringislower|stringisspace|stringisupper|stringisxdigit|stringleft|stringlen|stringlower|stringmid|stringregexp|stringregexpreplace|stringreplace|stringright|stringsplit|stringstripcr|stringstripws|stringtoasciiarray|stringtobinary|stringtrimleft|stringtrimright|stringupper|tan|tcpaccept|tcpclosesocket|tcpconnect|tcplisten|tcpnametoip|tcprecv|tcpsend|tcpshutdown|tcpstartup|timerdiff|timerinit|tooltip|traycreateitem|traycreatemenu|traygetmsg|trayitemdelete|trayitemgethandle|trayitemgetstate|trayitemgettext|trayitemsetonevent|trayitemsetstate|trayitemsettext|traysetclick|trayseticon|traysetonevent|traysetpauseicon|traysetstate|traysettooltip|traytip|ubound|udpbind|udpclosesocket|udpopen|udprecv|udpsend|udpshutdown|udpstartup|vargettype|winactivate|winactive|winclose|winexists|winflash|wingetcaretpos|wingetclasslist|wingetclientsize|wingethandle|wingetpos|wingetprocess|wingetstate|wingettext|wingettitle|winkill|winlist|winmenuselectitem|winminimizeall|winminimizeallundo|winmove|winsetontop|winsetstate|winsettitle|winsettrans|winwait|winwaitactive|winwaitclose|winwaitnotactive)\b", NAME_FUNCTION),
    ]);
    m.insert(r"builtInMarcros", vec![
        Rule::token(r"(?m)(?i)(@appdatacommondir|@appdatadir|@autoitexe|@autoitpid|@autoitversion|@autoitx64|@com_eventobj|@commonfilesdir|@compiled|@computername|@comspec|@cpuarch|@cr|@crlf|@desktopcommondir|@desktopdepth|@desktopdir|@desktopheight|@desktoprefresh|@desktopwidth|@documentscommondir|@error|@exitcode|@exitmethod|@extended|@favoritescommondir|@favoritesdir|@gui_ctrlhandle|@gui_ctrlid|@gui_dragfile|@gui_dragid|@gui_dropid|@gui_winhandle|@homedrive|@homepath|@homeshare|@hotkeypressed|@hour|@ipaddress1|@ipaddress2|@ipaddress3|@ipaddress4|@kblayout|@lf|@logondnsdomain|@logondomain|@logonserver|@mday|@min|@mon|@msec|@muilang|@mydocumentsdir|@numparams|@osarch|@osbuild|@oslang|@osservicepack|@ostype|@osversion|@programfilesdir|@programscommondir|@programsdir|@scriptdir|@scriptfullpath|@scriptlinenumber|@scriptname|@sec|@startmenucommondir|@startmenudir|@startupcommondir|@startupdir|@sw_disable|@sw_enable|@sw_hide|@sw_lock|@sw_maximize|@sw_minimize|@sw_restore|@sw_show|@sw_showdefault|@sw_showmaximized|@sw_showminimized|@sw_showminnoactive|@sw_showna|@sw_shownoactivate|@sw_shownormal|@sw_unlock|@systemdir|@tab|@tempdir|@tray_id|@trayiconflashing|@trayiconvisible|@username|@userprofiledir|@wday|@windowsdir|@workingdir|@yday|@year)\b", NAME_VARIABLE_GLOBAL),
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
        r"sqs",
        vec![
            Rule::token(r"(?m)\'\'|\`([,%`abfnrtv])", STRING_ESCAPE),
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
            Rule::token(r"(?m)[^'\n]+", STRING),
        ],
    );
    Table(m)
}

impl Lexer for AutoitLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
