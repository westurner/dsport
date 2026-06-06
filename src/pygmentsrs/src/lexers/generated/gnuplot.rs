//! AUTO-GENERATED from `pygments.pygments.lexers.graphics:GnuplotLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphics:GnuplotLexer:gnuplot

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gnuplot
pub struct GnuplotLexer;

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
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m)bind\b|bin\b|bi\b", KEYWORD, NewState::Push(vec![r"bind"])),
        Rule::token_to(r"(?m)exit\b|exi\b|ex\b|quit\b|qui\b|qu\b|q\b", KEYWORD, NewState::Push(vec![r"quit"])),
        Rule::token_to(r"(?m)fit\b|fi\b|f\b", KEYWORD, NewState::Push(vec![r"fit"])),
        Rule::bygroups_to(r"(?m)(if)(\s*)(\()", vec![Some(KEYWORD), Some(TEXT), Some(PUNCTUATION)], NewState::Push(vec![r"if"])),
        Rule::token(r"(?m)else\b", KEYWORD),
        Rule::token_to(r"(?m)pause\b|paus\b|pau\b|pa\b", KEYWORD, NewState::Push(vec![r"pause"])),
        Rule::token_to(r"(?m)plot\b|plo\b|pl\b|p\b|replot\b|replo\b|repl\b|rep\b|splot\b|splo\b|spl\b|sp\b", KEYWORD, NewState::Push(vec![r"plot"])),
        Rule::token_to(r"(?m)save\b|sav\b|sa\b", KEYWORD, NewState::Push(vec![r"save"])),
        Rule::token_to(r"(?m)set\b|se\b", KEYWORD, NewState::Push(vec![r"genericargs", r"optionarg"])),
        Rule::token_to(r"(?m)show\b|sho\b|sh\b|unset\b|unse\b|uns\b", KEYWORD, NewState::Push(vec![r"noargs", r"optionarg"])),
        Rule::token_to(r"(?m)lower\b|lowe\b|low\b|raise\b|rais\b|rai\b|ra\b|call\b|cal\b|ca\b|cd\b|clear\b|clea\b|cle\b|cl\b|help\b|hel\b|he\b|h\b|\?\b|history\b|histor\b|histo\b|hist\b|his\b|hi\b|load\b|loa\b|lo\b|l\b|print\b|prin\b|pri\b|pr\b|pwd\b|reread\b|rerea\b|rere\b|rer\b|re\b|reset\b|rese\b|res\b|screendump\b|screendum\b|screendu\b|screend\b|screen\b|scree\b|scre\b|scr\b|shell\b|shel\b|she\b|system\b|syste\b|syst\b|sys\b|sy\b|update\b|updat\b|upda\b|upd\b|up\b", KEYWORD, NewState::Push(vec![r"genericargs"])),
        Rule::token_to(r"(?m)pwd\b|reread\b|rerea\b|rere\b|rer\b|re\b|reset\b|rese\b|res\b|screendump\b|screendum\b|screendu\b|screend\b|screen\b|scree\b|scre\b|scr\b|shell\b|shel\b|she\b|test\b", KEYWORD, NewState::Push(vec![r"noargs"])),
        Rule::bygroups_to(r"(?m)([a-zA-Z_]\w*)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(WHITESPACE), Some(OPERATOR)], NewState::Push(vec![r"genericargs"])),
        Rule::bygroups_to(r"(?m)([a-zA-Z_]\w*)(\s*)(\()(.*?)(\))(\s*)(=)", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION), Some(TEXT), Some(PUNCTUATION), Some(WHITESPACE), Some(OPERATOR)], NewState::Push(vec![r"genericargs"])),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::token(r"(?m);", KEYWORD),
    ]);
    m.insert(r"whitespace", vec![
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)[^\\\n]+", COMMENT),
        Rule::token(r"(?m)\\\n", COMMENT),
        Rule::token(r"(?m)\\", COMMENT),
        Rule::default(NewState::Pop(1)),
    ]);
    m.insert(r"noargs", vec![
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"dqstring", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"sqstring", vec![
        Rule::token(r"(?m)''", STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
        Rule::token(r"(?m)[^\\'\n]+", STRING),
        Rule::token(r"(?m)\\\n", STRING),
        Rule::token(r"(?m)\\", STRING),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"genericargs", vec![
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"optionarg", vec![
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m)all\b|al\b|a\b|angles\b|angle\b|angl\b|ang\b|an\b|arrow\b|arro\b|arr\b|ar\b|autoscale\b|autoscal\b|autosca\b|autosc\b|autos\b|auto\b|aut\b|au\b|bars\b|bar\b|ba\b|b\b|border\b|borde\b|bord\b|bor\b|boxwidth\b|boxwidt\b|boxwid\b|boxwi\b|boxw\b|box\b|clabel\b|clabe\b|clab\b|cla\b|cl\b|clip\b|cli\b|cl\b|c\b|cntrparam\b|cntrpara\b|cntrpar\b|cntrpa\b|cntrp\b|cntr\b|cnt\b|cn\b|contour\b|contou\b|conto\b|cont\b|con\b|co\b|data\b|dat\b|da\b|datafile\b|datafil\b|datafi\b|dataf\b|data\b|dgrid3d\b|dgrid3\b|dgrid\b|dgri\b|dgr\b|dg\b|dummy\b|dumm\b|dum\b|du\b|encoding\b|encodin\b|encodi\b|encod\b|enco\b|enc\b|decimalsign\b|decimalsig\b|decimalsi\b|decimals\b|decimal\b|decima\b|decim\b|deci\b|dec\b|fit\b|fontpath\b|fontpat\b|fontpa\b|fontp\b|font\b|format\b|forma\b|form\b|for\b|fo\b|function\b|functio\b|functi\b|funct\b|func\b|fun\b|fu\b|functions\b|function\b|functio\b|functi\b|funct\b|func\b|fun\b|fu\b|grid\b|gri\b|gr\b|g\b|hidden3d\b|hidden3\b|hidden\b|hidde\b|hidd\b|hid\b|historysize\b|historysiz\b|historysi\b|historys\b|history\b|histor\b|histo\b|hist\b|his\b|isosamples\b|isosample\b|isosampl\b|isosamp\b|isosam\b|isosa\b|isos\b|iso\b|is\b|key\b|ke\b|k\b|keytitle\b|keytitl\b|keytit\b|keyti\b|keyt\b|label\b|labe\b|lab\b|la\b|linestyle\b|linestyl\b|linesty\b|linest\b|lines\b|line\b|lin\b|li\b|ls\b|loadpath\b|loadpat\b|loadpa\b|loadp\b|load\b|loa\b|locale\b|local\b|loca\b|loc\b|logscale\b|logscal\b|logsca\b|logsc\b|logs\b|log\b|macros\b|macro\b|macr\b|mac\b|mapping\b|mappin\b|mappi\b|mapp\b|map\b|mapping3d\b|mapping3\b|mapping\b|mappin\b|mappi\b|mapp\b|map\b|margin\b|margi\b|marg\b|mar\b|lmargin\b|lmargi\b|lmarg\b|lmar\b|rmargin\b|rmargi\b|rmarg\b|rmar\b|tmargin\b|tmargi\b|tmarg\b|tmar\b|bmargin\b|bmargi\b|bmarg\b|bmar\b|mouse\b|mous\b|mou\b|mo\b|multiplot\b|multiplo\b|multipl\b|multip\b|multi\b|mxtics\b|mxtic\b|mxti\b|mxt\b|nomxtics\b|nomxtic\b|nomxti\b|nomxt\b|mx2tics\b|mx2tic\b|mx2ti\b|mx2t\b|nomx2tics\b|nomx2tic\b|nomx2ti\b|nomx2t\b|mytics\b|mytic\b|myti\b|myt\b|nomytics\b|nomytic\b|nomyti\b|nomyt\b|my2tics\b|my2tic\b|my2ti\b|my2t\b|nomy2tics\b|nomy2tic\b|nomy2ti\b|nomy2t\b|mztics\b|mztic\b|mzti\b|mzt\b|nomztics\b|nomztic\b|nomzti\b|nomzt\b|mcbtics\b|mcbtic\b|mcbti\b|mcbt\b|nomcbtics\b|nomcbtic\b|nomcbti\b|nomcbt\b|offsets\b|offset\b|offse\b|offs\b|off\b|of\b|origin\b|origi\b|orig\b|ori\b|or\b|output\b|outpu\b|outp\b|out\b|ou\b|o\b|parametric\b|parametri\b|parametr\b|paramet\b|parame\b|param\b|para\b|par\b|pa\b|pm3d\b|pm3\b|pm\b|palette\b|palett\b|palet\b|pale\b|pal\b|colorbox\b|colorbo\b|colorb\b|plot\b|plo\b|pl\b|p\b|pointsize\b|pointsiz\b|pointsi\b|points\b|point\b|poin\b|poi\b|polar\b|pola\b|pol\b|print\b|prin\b|pri\b|pr\b|object\b|objec\b|obje\b|obj\b|samples\b|sample\b|sampl\b|samp\b|sam\b|sa\b|size\b|siz\b|si\b|style\b|styl\b|sty\b|st\b|surface\b|surfac\b|surfa\b|surf\b|sur\b|su\b|table\b|terminal\b|termina\b|termin\b|termi\b|term\b|ter\b|te\b|t\b|termoptions\b|termoption\b|termoptio\b|termopti\b|termopt\b|termop\b|termo\b|tics\b|tic\b|ti\b|ticscale\b|ticscal\b|ticsca\b|ticsc\b|ticslevel\b|ticsleve\b|ticslev\b|ticsle\b|ticsl\b|timefmt\b|timefm\b|timef\b|timestamp\b|timestam\b|timesta\b|timest\b|times\b|time\b|tim\b|title\b|titl\b|tit\b|variables\b|variable\b|variabl\b|variab\b|varia\b|vari\b|var\b|va\b|v\b|version\b|versio\b|versi\b|vers\b|ver\b|ve\b|view\b|vie\b|vi\b|xyplane\b|xyplan\b|xypla\b|xypl\b|xyp\b|xdata\b|xdat\b|xda\b|x2data\b|x2dat\b|x2da\b|ydata\b|ydat\b|yda\b|y2data\b|y2dat\b|y2da\b|zdata\b|zdat\b|zda\b|cbdata\b|cbdat\b|cbda\b|xlabel\b|xlabe\b|xlab\b|xla\b|xl\b|x2label\b|x2labe\b|x2lab\b|x2la\b|x2l\b|ylabel\b|ylabe\b|ylab\b|yla\b|yl\b|y2label\b|y2labe\b|y2lab\b|y2la\b|y2l\b|zlabel\b|zlabe\b|zlab\b|zla\b|zl\b|cblabel\b|cblabe\b|cblab\b|cbla\b|cbl\b|xtics\b|xtic\b|xti\b|noxtics\b|noxtic\b|noxti\b|x2tics\b|x2tic\b|x2ti\b|nox2tics\b|nox2tic\b|nox2ti\b|ytics\b|ytic\b|yti\b|noytics\b|noytic\b|noyti\b|y2tics\b|y2tic\b|y2ti\b|noy2tics\b|noy2tic\b|noy2ti\b|ztics\b|ztic\b|zti\b|noztics\b|noztic\b|nozti\b|cbtics\b|cbtic\b|cbti\b|nocbtics\b|nocbtic\b|nocbti\b|xdtics\b|xdtic\b|xdti\b|noxdtics\b|noxdtic\b|noxdti\b|x2dtics\b|x2dtic\b|x2dti\b|nox2dtics\b|nox2dtic\b|nox2dti\b|ydtics\b|ydtic\b|ydti\b|noydtics\b|noydtic\b|noydti\b|y2dtics\b|y2dtic\b|y2dti\b|noy2dtics\b|noy2dtic\b|noy2dti\b|zdtics\b|zdtic\b|zdti\b|nozdtics\b|nozdtic\b|nozdti\b|cbdtics\b|cbdtic\b|cbdti\b|nocbdtics\b|nocbdtic\b|nocbdti\b|xmtics\b|xmtic\b|xmti\b|noxmtics\b|noxmtic\b|noxmti\b|x2mtics\b|x2mtic\b|x2mti\b|nox2mtics\b|nox2mtic\b|nox2mti\b|ymtics\b|ymtic\b|ymti\b|noymtics\b|noymtic\b|noymti\b|y2mtics\b|y2mtic\b|y2mti\b|noy2mtics\b|noy2mtic\b|noy2mti\b|zmtics\b|zmtic\b|zmti\b|nozmtics\b|nozmtic\b|nozmti\b|cbmtics\b|cbmtic\b|cbmti\b|nocbmtics\b|nocbmtic\b|nocbmti\b|xrange\b|xrang\b|xran\b|xra\b|xr\b|x2range\b|x2rang\b|x2ran\b|x2ra\b|x2r\b|yrange\b|yrang\b|yran\b|yra\b|yr\b|y2range\b|y2rang\b|y2ran\b|y2ra\b|y2r\b|zrange\b|zrang\b|zran\b|zra\b|zr\b|cbrange\b|cbrang\b|cbran\b|cbra\b|cbr\b|rrange\b|rrang\b|rran\b|rra\b|rr\b|trange\b|trang\b|tran\b|tra\b|tr\b|urange\b|urang\b|uran\b|ura\b|ur\b|vrange\b|vrang\b|vran\b|vra\b|vr\b|xzeroaxis\b|xzeroaxi\b|xzeroax\b|xzeroa\b|x2zeroaxis\b|x2zeroaxi\b|x2zeroax\b|x2zeroa\b|yzeroaxis\b|yzeroaxi\b|yzeroax\b|yzeroa\b|y2zeroaxis\b|y2zeroaxi\b|y2zeroax\b|y2zeroa\b|zzeroaxis\b|zzeroaxi\b|zzeroax\b|zzeroa\b|zeroaxis\b|zeroaxi\b|zeroax\b|zeroa\b|zero\b|zer\b|ze\b|z\b", NAME_BUILTIN, NewState::Pop(1)),
    ]);
    m.insert(r"bind", vec![
        Rule::token_to(r"(?m)!", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)allwindows\b|allwindow\b|allwindo\b|allwind\b|allwin\b|allwi\b|allw\b|all\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"quit", vec![
        Rule::token(r"(?m)gnuplot\b", KEYWORD),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"fit", vec![
        Rule::token(r"(?m)via\b", NAME_BUILTIN),
        Rule::token(r"(?m)axes\b|axe\b|ax\b|axis\b|axi\b|binary\b|binar\b|bina\b|bin\b|every\b|ever\b|eve\b|ev\b|index\b|inde\b|ind\b|in\b|i\b|matrix\b|matri\b|matr\b|mat\b|smooth\b|smoot\b|smoo\b|smo\b|sm\b|s\b|thru\b|title\b|titl\b|tit\b|ti\b|t\b|notitle\b|notitl\b|notit\b|noti\b|not\b|using\b|usin\b|usi\b|us\b|u\b|with\b|wit\b|wi\b|w\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"plot", vec![
        Rule::token(r"(?m)axes\b|axe\b|ax\b|axis\b|axi\b|binary\b|binar\b|bina\b|bin\b|every\b|ever\b|eve\b|ev\b|index\b|inde\b|ind\b|in\b|i\b|matrix\b|matri\b|matr\b|mat\b|smooth\b|smoot\b|smoo\b|smo\b|sm\b|s\b|thru\b|title\b|titl\b|tit\b|ti\b|t\b|notitle\b|notitl\b|notit\b|noti\b|not\b|using\b|usin\b|usi\b|us\b|u\b|with\b|wit\b|wi\b|w\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"if", vec![
        Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"pause", vec![
        Rule::token(r"(?m)(mouse|any|button1|button2|button3)\b", NAME_BUILTIN),
        Rule::token(r"(?m)keypress\b|keypres\b|keypre\b|keypr\b|keyp\b|key\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    m.insert(r"save", vec![
        Rule::token(r"(?m)functions\b|function\b|functio\b|functi\b|funct\b|func\b|fun\b|fu\b|f\b|set\b|se\b|s\b|terminal\b|termina\b|termin\b|termi\b|term\b|ter\b|te\b|t\b|variables\b|variable\b|variabl\b|variab\b|varia\b|vari\b|var\b|va\b|v\b", NAME_BUILTIN),
        Rule::token_to(r"(?m)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)[ \t\v\f]+", WHITESPACE),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"dqstring"])),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"sqstring"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+)", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[,.~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[{}()\[\]]", PUNCTUATION),
        Rule::token(r"(?m)(eq|ne)\b", OPERATOR_WORD),
        Rule::bygroups(r"(?m)([a-zA-Z_]\w*)(\s*)(\()", vec![Some(NAME_FUNCTION), Some(TEXT), Some(PUNCTUATION)]),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)@[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
    ]);
    Table(m)
}

impl Lexer for GnuplotLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
