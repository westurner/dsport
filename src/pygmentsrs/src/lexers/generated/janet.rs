//! AUTO-GENERATED from `pygments.pygments.lexers.lisp:JanetLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.lisp:JanetLexer:janet

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: janet
pub struct JanetLexer;

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
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)(?x)
                  [+-]? [0-9]{1,2} r [0-9a-zA-Z][0-9a-zA-Z_]* \. ([0-9a-zA-Z][0-9a-zA-Z_]*)?
                  (&[+-]?[0-9a-zA-Z]+)?
               ", NUMBER),
        Rule::token(r"(?m)(?x)
                  [+-]? [0-9]{1,2} r (\.)? [0-9a-zA-Z][0-9a-zA-Z_]*
                  (&[+-]?[0-9a-zA-Z]+)?
               ", NUMBER),
        Rule::token(r"(?m)(?x) [+-]? 0x [0-9a-fA-F][0-9a-fA-F_]* \. ([0-9a-fA-F][0-9a-fA-F_]*)?", NUMBER_HEX),
        Rule::token(r"(?m)(?x) [+-]? 0x (\.)? [0-9a-fA-F][0-9a-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)(?x) [+-]? [0-9][0-9_]* \. ([0-9][0-9_]*)? ([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)(?x) [+-]? (\.)? [0-9][0-9_]* ([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token_to(r#"(?m)@?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)@?(`+)(.|\n)+?\1", STRING),
        Rule::token(r"(?m)['~,;|]", OPERATOR),
        Rule::token(r"(?m)@?[(\[{]|[)\]}]", PUNCTUATION),
        Rule::token(r"(?m)(false|nil|true)(?=\s|#|[)\]]|$)", TokenType::new(&["Keyword", "Constants"])),
        Rule::token(r"(?m)(:(([0-9:]|[a-zA-Z!$%&*+\-./<=>?@^_]))+|:)", NAME_CONSTANT),
        Rule::token(r"(?m)(de(?:bugger\-env|fault\-peg\-grammar)|janet/(?:build|config\-bits|version)|load\-image\-dict|m(?:a(?:ke\-image\-dict|th/(?:\-inf|e|in(?:f|t(?:\-m(?:ax|in)|32\-m(?:ax|in)))|nan|pi))|odule/(?:cache|load(?:ers|ing)|paths))|root\-env|std(?:err|in|out))(?=\s|#|[)\]]|$)", NAME_VARIABLE_GLOBAL),
        Rule::token(r"(?m)(?<=\()(break|d(?:ef|o)|fn|if|qu(?:(?:(?:asiqu)?)ote)|s(?:et|plice)|u(?:(?:nquot|pscop)e)|var|while)(?=\s|#|[)\]]|$)", KEYWORD_RESERVED),
        Rule::token(r"(?m)(?<=\()(%=|\*=|\+(?:[+=])|\-(?:(?:\?>|[>?])>|[\-=>])|/=|a(?:nd|s(?:\-(?:>|macro)|\?\->|sert))|c(?:a(?:se|tseq)|hr|o(?:m(?:ment|p(?:if|time|when))|nd|ro))|d(?:e(?:f(?:ault|dyn|er|macro(?:(?:\-)?)|n\-|[\-n])|lay)|oc)|e(?:ach(?:(?:[kpy])?)|defer|v/(?:do\-thread|gather|spawn(?:(?:\-thread)?)|with\-deadline))|f(?:fi/defbind|iber\-fn|or(?:(?:ever|v)?))|generate|i(?:f\-(?:let|not|with)|mport)|juxt|l(?:abel|et|oop)|match|or|pro(?:(?:mp|tec)t)|repeat|s(?:eq|hort\-fn)|t(?:abseq|oggle|r(?:acev|y))|u(?:nless|se)|var(?:\-|fn)|w(?:hen(?:(?:\-(?:let|with))?)|ith(?:(?:\-(?:(?:dyn|sym|var)s))?)))(?=\s|#|[)\]]|$)", NAME_BUILTIN),
        Rule::token(r"(?m)(?<=\()(<=|>=|a(?:bstract\?|ccumulate(?:(?:2)?)|ll(?:(?:\-(?:(?:binding|dynamic)s))?)|ny\?|pply|rray(?:(?:/(?:c(?:lear|oncat)|ensure|fill|insert|new(?:(?:\-filled)?)|p(?:eek|op|ush)|remove|slice|trim|weak)|\?)?)|sm)|b(?:a(?:d\-(?:(?:compil|pars)e)|nd)|lshift|not|o(?:olean\?|r)|r(?:(?:(?:u)?)shift)|uffer(?:(?:/(?:b(?:it(?:(?:\-(?:clear|set|toggle))?)|lit)|clear|f(?:ill|ormat|rom\-bytes)|new(?:(?:\-filled)?)|p(?:opn|ush(?:(?:\-(?:at|byte|string|word))?))|slice|trim)|\?)?)|xor|ytes\?)|c(?:ancel|function\?|li\-main|mp|o(?:mp(?:(?:are(?:(?:(?:[<>])=|[<=>])?)|ile|lement)?)|unt)|urenv)|d(?:e(?:bug(?:(?:/(?:arg\-stack|break|fbreak|lineage|st(?:ack(?:(?:trace)?)|ep)|un(?:(?:(?:f)?)break))|ger(?:(?:\-on\-status)?))?)|c|ep(?:(?:(?:\-not)?)=)|fglobal|scribe)|i(?:ctionary\?|s(?:asm|tinct)|v)|o(?:c(?:\*|\-(?:format|of))|file)|rop(?:(?:\-(?:until|while))?)|yn)|e(?:flush|mpty\?|nv\-lookup|prin(?:(?:tf|[ft])?)|rror(?:(?:f)?)|v(?:/(?:a(?:cquire\-(?:(?:(?:[rw])?)lock)|ll\-tasks)|c(?:a(?:ll|ncel|pacity)|h(?:an(?:(?:\-close)?)|unk)|lose|ount)|deadline|full|g(?:ive(?:(?:\-supervisor)?)|o)|lock|r(?:e(?:ad|lease\-(?:(?:(?:[rw])?)lock))|select|wlock)|s(?:elect|leep)|t(?:ake|hread(?:(?:\-chan)?))|write)|al(?:(?:\-string)?)|e(?:(?:n|ry)\?))|xtreme)|f(?:alse\?|fi/(?:align|c(?:all(?:(?:ing\-conventions)?)|lose|ontext)|free|jitfn|lookup|malloc|native|pointer\-(?:buffer|cfunction)|read|s(?:i(?:(?:gnatur|z)e)|truct)|(?:trampolin|writ)e)|i(?:ber(?:/(?:c(?:an\-resume\?|urrent)|getenv|last\-value|maxstack|new|root|s(?:et(?:env|maxstack)|tatus))|\?)|l(?:e/(?:close|flush|lines|open|read|seek|te(?:ll|mp)|write)|ter)|nd(?:(?:\-index)?)|rst)|l(?:atten(?:(?:\-into)?)|ush|ycheck)|r(?:e(?:eze|quencies)|om\-pairs)|unction\?)|g(?:c(?:collect|(?:(?:set)?)interval)|e(?:nsym|t(?:(?:\-in|line|proto)?))|roup\-by)|has(?:\-(?:(?:key|value)\?)|h)|i(?:de(?:mpotent\?|ntity)|mport\*|n(?:(?:c|dex(?:\-of|ed\?)|t(?:/(?:s64|to\-(?:bytes|number)|u64)|\?|er(?:(?:leav|pos)e))|vert)?))|juxt\*|k(?:e(?:ep(?:(?:\-syntax(?:(?:!)?))?)|y(?:s|word(?:(?:/slice|\?)?)))|vs)|l(?:ast|ength(?:(?:able\?)?)|oad\-image)|m(?:a(?:c(?:ex(?:(?:1)?)|lintf)|ke\-(?:env|image)|pcat|rshal|th/(?:a(?:bs|cos(?:(?:h)?)|sin(?:(?:h)?)|tan(?:(?:[2h])?))|c(?:brt|eil|os(?:(?:h)?))|e(?:rf(?:(?:c)?)|xp(?:(?:2|m1)?))|floor|g(?:amma|cd)|hypot|l(?:cm|og(?:(?:\-gamma|1(?:[0p])|2)?))|next|pow|r(?:andom|ng(?:(?:\-(?:buffer|int|uniform))?)|ound)|s(?:eedrandom|in(?:(?:h)?)|qrt)|t(?:an(?:(?:h)?)|runc))|x\-of|[px])|e(?:an|mcmp|rge(?:(?:\-(?:into|module))?))|in(?:(?:\-of)?)|od(?:(?:ule/(?:add\-paths|expand\-path|find|value))?))|n(?:a(?:n\?|t(?:\?|ive))|e(?:g\?|t/(?:a(?:ccept(?:(?:\-loop)?)|ddress(?:(?:\-unpack)?))|c(?:hunk|lose|onnect)|flush|l(?:isten|ocalname)|peername|re(?:ad|cv\-from)|s(?:e(?:nd\-to|rver|tsockopt)|hutdown)|write)|xt)|il\?|ot(?:(?:=)?)|umber\?)|o(?:dd\?|ne\?|s/(?:arch|c(?:d|hmod|lock|ompiler|pu\-count|(?:ryptoran|w)d)|d(?:ate|ir)|e(?:nviron|x(?:ecute|it))|getenv|isatty|l(?:ink|stat)|mk(?:dir|time)|open|p(?:erm\-(?:int|string)|ipe|osix\-(?:exec|fork)|roc\-(?:close|kill|wait))|r(?:e(?:a(?:dlink|lpath)|name)|m(?:(?:dir)?))|s(?:etenv|hell|igaction|leep|pawn|t(?:at|rftime)|ymlink)|t(?:ime|ouch)|umask|which))|p(?:a(?:irs|r(?:se(?:(?:\-all|r/(?:byte|c(?:(?:lon|onsum)e)|e(?:of|rror)|flush|has\-more|insert|new|produce|stat(?:e|us)|where))?)|ti(?:al|tion(?:(?:\-by)?))))|eg/(?:compile|find(?:(?:\-all)?)|match|replace(?:(?:\-all)?))|os(?:\?|twalk)|p|r(?:ewalk|in(?:(?:tf|[ft])?)|o(?:duct|pagate))|ut(?:(?:\-in)?))|quit|r(?:ange|e(?:duce(?:(?:2)?)|pl|quire|sume|turn|verse(?:(?:!)?))|un\-context)|s(?:andbox|can\-number|etdyn|ignal|l(?:ice|urp)|o(?:me|rt(?:(?:\-by|ed(?:(?:\-by)?))?))|pit|tr(?:ing(?:(?:/(?:ascii\-(?:(?:low|upp)er)|bytes|check\-set|f(?:ind(?:(?:\-all)?)|ormat|rom\-bytes)|has\-(?:(?:pre|suf)fix\?)|join|re(?:p(?:eat|lace(?:(?:\-all)?))|verse)|s(?:lice|plit)|trim(?:(?:[lr])?))|\?)?)|uct(?:(?:/(?:getproto|proto\-flatten|to\-table|with\-proto)|\?)?))|um|ymbol(?:(?:/slice|\?)?))|t(?:a(?:ble(?:(?:/(?:cl(?:ear|one)|getproto|new|proto\-flatten|rawget|setproto|to\-struct|weak(?:(?:\-(?:(?:key|value)s))?))|\?)?)|ke(?:(?:\-(?:until|while))?)|rray/(?:buffer|copy\-bytes|length|new|properties|s(?:lice|wap\-bytes)))|h(?:aw|read/(?:c(?:lose|urrent)|exit|new|receive|send))|r(?:ace|u(?:(?:e|thy)\?))|uple(?:(?:/(?:brackets|s(?:etmap|lice|ourcemap)|type)|\?)?)|ype)|u(?:n(?:marshal|trace)|pdate(?:(?:\-in)?))|va(?:lues|rglobal)|wa(?:lk|rn\-compile)|xprin(?:(?:tf|[ft])?)|yield|z(?:ero\?|ipcoll)|[%*+\-/<=>])(?=\s|#|[)\]]|$)", NAME_FUNCTION),
        Rule::token(r"(?m)[a-zA-Z!$%&*+\-./<=>?@^_](([0-9:]|[a-zA-Z!$%&*+\-./<=>?@^_]))*", NAME_VARIABLE),
    ]);
    m.insert(r"string", vec![
        Rule::token(r"(?m)\\(u[0-9a-fA-F]{4}|U[0-9a-fA-F]{6})", STRING_ESCAPE),
        Rule::token(r"(?m)\\x[0-9a-fA-F]{2}", STRING_ESCAPE),
        Rule::token(r"(?m)\\.", STRING_ESCAPE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)[^\\"]+"#, STRING),
    ]);
    Table(m)
}

impl Lexer for JanetLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
