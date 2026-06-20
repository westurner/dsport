#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.factor:FactorLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.factor:FactorLexer:factor

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: factor
pub struct FactorLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"root",
        vec![
            Rule::token(r"(?m)#!.*$", COMMENT_PREPROC),
            Rule::default(NewState::Push(vec![r"base"])),
        ],
    );
    m.insert(r"base", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)((?:MACRO|MEMO|TYPED)?:[:]?)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(M:[:]?)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(C:)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(GENERIC:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(HOOK:|GENERIC#)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(\()(\s)", vec![Some(NAME_FUNCTION), Some(WHITESPACE)], NewState::Push(vec![r"stackeffect"])),
        Rule::bygroups(r"(?m)(;)(\s)", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)(USING:)(\s+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"vocabs"])),
        Rule::bygroups(r"(?m)(USE:|UNUSE:|IN:|QUALIFIED:)(\s+)(\S+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups(r"(?m)(QUALIFIED-WITH:)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE)]),
        Rule::bygroups_to(r"(?m)(FROM:|EXCLUDE:)(\s+)(\S+)(\s+=>\s)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"words"])),
        Rule::bygroups(r"(?m)(RENAME:)(\s+)(\S+)(\s+)(\S+)(\s+)(=>)(\s+)(\S+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_NAMESPACE), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(ALIAS:|TYPEDEF:)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(DEFER:|FORGET:|POSTPONE:)(\s+)(\S+)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(TUPLE:|ERROR:)(\s+)(\S+)(\s+)(<)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"slots"])),
        Rule::bygroups_to(r"(?m)(TUPLE:|ERROR:|BUILTIN:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)], NewState::Push(vec![r"slots"])),
        Rule::bygroups(r"(?m)(MIXIN:|UNION:|INTERSECTION:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(PREDICATE:)(\s+)(\S+)(\s+)(<)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(C:)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(INSTANCE:)(\s+)(\S+)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(SLOT:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?m)(SINGLETON:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token_to(r"(?m)SINGLETONS:", KEYWORD, NewState::Push(vec![r"classes"])),
        Rule::bygroups(r"(?m)(CONSTANT:|SYMBOL:|MAIN:|HELP:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups_to(r"(?m)(SYMBOLS:)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"words"])),
        Rule::bygroups(r"(?m)(SYNTAX:)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(ALIEN:)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(STRUCT:)(\s+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(FUNCTION:)(\s+)(\S+)(\s+)(\S+)(\s+)(\()(\s+)([^)]+)(\))(\s)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(TEXT), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(FUNCTION-ALIAS:)(\s+)(\S+)(\s+)(\S+)(\s+)(\S+)(\s+)(\()(\s+)([^)]+)(\))(\s)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(TEXT), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(<PRIVATE|PRIVATE>)(\s)", vec![Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::token(r#"(?m)"""\s(?:.|\n)*?\s""""#, STRING),
        Rule::token(r#"(?m)"(?:\\\\|\\"|[^"])*""#, STRING),
        Rule::bygroups(r#"(?m)(\S+")(\s+)((?:\\\\|\\"|[^"])*")"#, vec![Some(STRING), Some(WHITESPACE), Some(STRING)]),
        Rule::bygroups(r"(?m)(CHAR:)(\s+)(\\[\\abfnrstv]|[^\\]\S*)(\s)", vec![Some(STRING_CHAR), Some(WHITESPACE), Some(STRING_CHAR), Some(WHITESPACE)]),
        Rule::token(r"(?m)!\s+.*$", COMMENT),
        Rule::token(r"(?m)#!\s+.*$", COMMENT),
        Rule::token(r"(?m)/\*\s+(?:.|\n)*?\s\*/", COMMENT),
        Rule::token(r"(?m)[tf]\b", NAME_CONSTANT),
        Rule::token(r"(?m)[\\$]\s+\S+", NAME_CONSTANT),
        Rule::token(r"(?m)M\\\s+\S+\s+\S+", NAME_CONSTANT),
        Rule::token(r"(?m)[+-]?(?:[\d,]*\d)?\.(?:\d([\d,]*\d)?)?(?:[eE][+-]?\d+)?\s", NUMBER),
        Rule::token(r"(?m)[+-]?\d(?:[\d,]*\d)?(?:[eE][+-]?\d+)?\s", NUMBER),
        Rule::token(r"(?m)0x[a-fA-F\d](?:[a-fA-F\d,]*[a-fA-F\d])?(?:p\d([\d,]*\d)?)?\s", NUMBER),
        Rule::token(r"(?m)NAN:\s+[a-fA-F\d](?:[a-fA-F\d,]*[a-fA-F\d])?(?:p\d([\d,]*\d)?)?\s", NUMBER),
        Rule::token(r"(?m)0b[01]+\s", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+\s", NUMBER_OCT),
        Rule::token(r"(?m)(?:\d([\d,]*\d)?)?\+\d(?:[\d,]*\d)?/\d(?:[\d,]*\d)?\s", NUMBER),
        Rule::token(r"(?m)(?:\-\d([\d,]*\d)?)?\-\d(?:[\d,]*\d)?/\d(?:[\d,]*\d)?\s", NUMBER),
        Rule::token(r"(?m)(?:deprecated|final|foldable|flushable|inline|recursive)\s", KEYWORD),
        Rule::bygroups(r"(?m)(\(clone\)|\-rot|2(?:bi(?:(?:[*@])?)|curry|d(?:(?:ro|[iu])p)|keep|nip|over|tri(?:(?:[*@])?))|3(?:bi|curry|d(?:(?:ro|[iu])p)|keep|tri)|4(?:(?:d(?:ro|[iu])|kee)p)|<wrapper>|>boolean|\?(?:execute|if)|a(?:nd|ssert(?:(?:[=?])?))|b(?:i(?:(?:\-curry(?:(?:[*@])?)|[*@])?)|o(?:a|olean(?:(?:\?)?)|th\?)|uild)|c(?:all(?:(?:stack(?:(?:>array|\?)?))?)|l(?:ear|one)|ompose(?:(?:\?)?)|urry(?:(?:\?)?))|d(?:atastack|i(?:[ep])|o|rop|up(?:(?:d)?))|e(?:ither\?|q(?:(?:(?:ual)?)\?)|xecute)|hashcode(?:(?:\*)?)|i(?:dentity\-(?:hashcode|tuple(?:(?:\?)?))|f(?:(?:\*)?))|keep|loop|most|n(?:ew|ip|ot|ull)|o(?:bject|(?:(?:ve)?)r)|p(?:ick|repose)|r(?:etainstack|ot)|s(?:ame\?|wap(?:(?:d)?))|t(?:hrow|ri(?:(?:\-curry(?:(?:[*@])?)|[*@])?)|uple(?:(?:\?)?))|un(?:less(?:(?:\*)?)|til)|w(?:h(?:en(?:(?:\*)?)|ile)|ith|rapper(?:(?:\?)?))|xor|[=?])(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(2cache|<enum>|>alist|\?(?:at|of)|a(?:ssoc(?:(?:\-(?:a(?:(?:ll|ny)\?)|c(?:(?:lone\-lik|ombin)e)|diff(?:(?:!|er)?)|e(?:ach|mpty\?)|fi(?:lter(?:(?:!|\-as)?)|nd)|hashcode|intersect|like|map(?:(?:\-as)?)|partition|refine|s(?:ize|tack|ubset\?)|union(?:(?:!)?))|>map|[=?])?)|t(?:(?:[*+])?))|c(?:ache|hange\-at|lear\-assoc)|delete\-at(?:(?:\*)?)|e(?:num(?:(?:\?)?)|xtract\-keys)|inc\-at|key(?:[?s])|ma(?:p>assoc|ybe\-set\-at)|new\-assoc|of|push\-at|rename\-at|s(?:et\-at|ift\-(?:(?:key|value)s)|ubstitute)|unzip|value(?:\-at(?:(?:\*)?)|[?s])|zip)(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(2cleave(?:(?:>quot)?)|3cleave(?:(?:>quot)?)|4cleave(?:(?:>quot)?)|alist>quot|c(?:a(?:ll\-effect|se(?:(?:\-find|>quot)?))|leave(?:(?:>quot)?)|ond(?:(?:>quot)?))|deep\-spread>quot|execute\-effect|linear\-case\-quot|no\-c(?:ase(?:(?:\?)?)|ond(?:(?:\?)?))|recursive\-hashcode|s(?:hallow\-spread>quot|pread)|to\-fixed\-point|wrong\-values(?:(?:\?)?))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\((?:(?:all\-integers\?|(?:each|find)\-integer)\))|/(?:mod|[fi])|2(?:[/\^])|<(?:=|fp\-nan>)|>(?:=|bignum|f(?:ixnum|loat)|integer)|\?1\+|a(?:bs|l(?:ign|l\-integers\?))|bi(?:gnum(?:(?:\?)?)|t(?:\?|and|not|or|s>(?:double|float)|xor))|complex(?:(?:\?)?)|d(?:enominator|ouble>bits)|e(?:ach\-integer|ven\?)|f(?:i(?:nd\-(?:(?:(?:last\-)?)integer)|xnum(?:(?:\?)?))|loat(?:(?:>bits|\?)?)|p\-(?:bitwise=|infinity\?|nan(?:\-payload|\?)|qnan\?|s(?:ign|(?:nan|pecial)\?)))|i(?:f\-zero|maginary\-part|nteger(?:(?:>fixnum(?:(?:\-strict)?)|\?)?))|log2(?:(?:\-expects\-positive(?:(?:\?)?))?)|mod|n(?:e(?:g(?:(?:\?)?)|xt\-(?:float|power\-of\-2))|um(?:ber(?:(?:[=?])?)|erator))|o(?:dd\?|ut\-of\-fixnum\-range(?:(?:\?)?))|p(?:ower\-of\-2\?|rev\-float)|r(?:atio(?:(?:\?|nal(?:(?:\?)?))?)|e(?:al(?:(?:\-part|\?)?)|cip|m))|s(?:gn|hift|q)|times|u(?:<=|>=|n(?:less\-zero|ordered\?)|[<>])|when\-zero|zero\?|[*+\-/<>])(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(1sequence|2(?:all\?|each|map(?:(?:\-(?:as|reduce))?)|reduce|se(?:lector|quence))|3(?:append(?:(?:\-as)?)|each|map(?:(?:\-as)?)|sequence)|4sequence|<(?:(?:re(?:petition|versed)|slice)>)|\?(?:first|last|nth|se(?:cond|t\-nth))|a(?:ccumulate(?:(?:!|\-as)?)|ll\?|ny\?|ppend(?:(?:!|\-as)?)|ssert\-sequence(?:(?:[=?])?))|b(?:inary\-reduce|ounds\-(?:check(?:(?:\?)?)|error(?:(?:\?)?))|ut\-last(?:(?:\-slice)?))|c(?:artesian\-(?:each|map|product)|h(?:ange\-nth|eck\-slice(?:(?:\-error)?))|lone\-like|o(?:ll(?:apse\-slice|ector(?:(?:\-for)?))|ncat(?:(?:\-as)?)|py|unt)|ut(?:(?:\*|\-slice)?))|d(?:elete\-(?:all|slice)|rop\-prefix)|e(?:ach(?:(?:\-(?:from|index))?)|mpty\?|xchange)|f(?:i(?:lter(?:(?:!|\-as)?)|nd(?:(?:\-(?:from|index(?:(?:\-from)?)|last(?:(?:\-from)?)))?)|rst(?:(?:[234])?))|lip|o(?:llow|urth))|glue|h(?:a(?:lves|rvest)|ead(?:(?:\-slice(?:(?:\*)?)|[*?])?))|i(?:f\-empty|mmutable(?:(?:\-sequence(?:(?:\?)?)|\?)?)|n(?:d(?:ex(?:(?:\-from)?)|ices)|fimum(?:(?:\-by)?)|sert\-nth|terleave)|ota(?:(?:\-tuple(?:(?:\?)?))?))|join(?:(?:\-as)?)|l(?:ast(?:(?:\-index(?:(?:\-from)?))?)|ength(?:(?:en)?)|ike|onge(?:r(?:(?:\?)?)|st))|m(?:a(?:p(?:(?:!|\-(?:as|find(?:(?:\-last)?)|in(?:dex|tegers)|reduce|sum))?)|x\-length)|ember(?:(?:(?:\-eq)?)\?)|i(?:dpoint@|(?:n\-lengt|smatc)h)|ove)|n(?:ew\-(?:(?:lik|resizabl|sequenc)e)|on\-negative\-integer\-expected(?:(?:\?)?)|th(?:(?:s)?))|p(?:a(?:d(?:\-(?:head|tail)|ding)|rtition)|op(?:(?:\*)?)|r(?:e(?:fix|pend(?:(?:\-as)?))|oduc(?:e\-as|[et]))|ush(?:(?:\-(?:all|either|if))?))|re(?:duce(?:(?:\-index)?)|move(?:(?:!|\-(?:eq(?:(?:!)?)|nth(?:(?:!)?)))?)|p(?:etition(?:(?:\?)?)|l(?:ace\-slice|icate(?:(?:\-as)?)))|st(?:(?:\-slice)?)|verse(?:(?:d\?|[!d])?))|s(?:e(?:cond|lector(?:(?:\-for)?)|quence(?:(?:\-hashcode|[=?])?)|t\-(?:f(?:irst|ourth)|l(?:ast|ength)|nth|(?:secon|thir)d))|hort(?:(?:e(?:r\?|st|[nr]))?)|ift|lice(?:(?:\-error(?:(?:\?)?)|\?)?)|nip(?:(?:\-slice)?)|tart(?:(?:\*)?)|u(?:bseq(?:(?:\?)?)|ffix(?:(?:!)?)|m(?:(?:\-lengths)?)|premum(?:(?:\-by)?)|rround))|t(?:ail(?:(?:\-slice(?:(?:\*)?)|[*?])?)|hird|rim(?:(?:\-(?:head(?:(?:\-slice)?)|slice|tail(?:(?:\-slice)?)))?))|un(?:clip(?:(?:\-(?:last(?:(?:\-slice)?)|slice))?)|less\-empty)|virtual(?:\-(?:exemplar|sequence(?:(?:\?)?))|@)|when\-empty)(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\+@|c(?:hange(?:(?:\-global)?)|ounter)|dec|g(?:et(?:(?:\-global)?)|lobal)|i(?:n(?:c|it(?:\-namespaces|ialize))|s\-global)|make\-assoc|names(?:pace|tack)|o(?:ff|n)|set(?:(?:\-(?:global|namestack))?)|toggle|with\-(?:global|scope|variable(?:(?:s)?)))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(1array|2array|3array|4array|<array>|>array|array(?:(?:\?)?)|pair(?:(?:\?)?)|resize\-array)(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(\((?:(?:each\-stream\-block(?:(?:\-slice)?)|stream\-contents\-by\-(?:block|element|length(?:(?:\-or\-block)?)))\))|\+(?:(?:byte|character)\+)|b(?:ad\-seek\-type(?:(?:\?)?)|l)|contents|e(?:ach\-(?:block(?:(?:\-s(?:(?:iz|lic)e))?)|line|morsel|stream\-(?:block(?:(?:\-slice)?)|line))|rror\-stream)|flush|in(?:put\-stream(?:(?:\?)?)|valid\-read\-buffer(?:(?:\?)?))|lines|nl|output\-stream(?:(?:\?)?)|print|read(?:(?:\-(?:into|partial(?:(?:\-into)?)|until)|1|ln)?)|s(?:eek\-(?:absolute(?:(?:\?)?)|end(?:(?:\?)?)|input|output|relative(?:(?:\?)?))|tream\-(?:bl|co(?:ntents(?:(?:\*)?)|py(?:(?:\*)?))|element\-type|flush|l(?:ength|ines)|nl|print|read(?:(?:\-(?:into|partial(?:(?:\-(?:into|unsafe))?)|un(?:safe|til))|1|ln)?)|seek(?:(?:able\?)?)|tell|write(?:(?:1)?)))|tell\-(?:(?:in|out)put)|w(?:ith\-(?:error(?:\-stream(?:(?:\*)?)|>output)|input\-(?:output\+error\-streams(?:(?:\*)?)|stream(?:(?:\*)?))|output(?:\+error\-stream(?:(?:\*)?)|\-stream(?:(?:\*)?)|>error)|streams(?:(?:\*)?))|rite(?:(?:1)?)))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(1string|<string>|>string|resize\-string|string(?:(?:\?)?))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(1vector|<vector>|>vector|\?push|vector(?:(?:\?)?))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::bygroups(r"(?m)(<(?:(?:con(?:(?:di|tinua)tion)|restart)>)|attempt\-all(?:(?:\-error(?:(?:\?)?))?)|c(?:all(?:back\-error\-hook|cc(?:[01]))|leanup|o(?:mpute\-restarts|n(?:dition(?:(?:\?)?)|tinu(?:ation(?:(?:\?)?)|e(?:(?:\-(?:restart|with))?))))|urrent\-continuation)|error(?:(?:\-(?:continuation|(?:(?:in\-)?)thread))?)|i(?:fcc|gnore\-errors|n\-callback\?)|original\-error|re(?:cover|start(?:(?:[?s])?)|t(?:hrow(?:(?:\-restarts)?)|urn(?:(?:\-continuation)?)))|thr(?:ead\-error\-hook|ow\-(?:continue|restarts))|with\-(?:datastack|return))(\s+)", vec![Some(NAME_BUILTIN), Some(WHITESPACE)]),
        Rule::token(r"(?m)\S+", TEXT),
    ]);
    m.insert(
        r"stackeffect",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(\()(\s+)",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
                NewState::Push(vec![r"stackeffect"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\))(\s+)",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups(
                r"(?m)(--)(\s+)",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
            ),
            Rule::token(r"(?m)\S+", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"slots",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(;)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups(
                r"(?m)(\{)(\s+)(\S+)(\s+)([^}]+)(\s+)(\})(\s+)",
                vec![
                    Some(TEXT),
                    Some(WHITESPACE),
                    Some(NAME_VARIABLE),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                    Some(TEXT),
                    Some(WHITESPACE),
                ],
            ),
            Rule::token(r"(?m)\S+", NAME_VARIABLE),
        ],
    );
    m.insert(
        r"vocabs",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(;)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\S+", NAME_NAMESPACE),
        ],
    );
    m.insert(
        r"classes",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(;)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\S+", NAME_CLASS),
        ],
    );
    m.insert(
        r"words",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)(;)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\S+", NAME_FUNCTION),
        ],
    );
    Table(m)
}

impl Lexer for FactorLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
