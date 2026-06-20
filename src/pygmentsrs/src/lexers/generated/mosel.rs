//! AUTO-GENERATED from `pygments.pygments.lexers.mosel:MoselLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.mosel:MoselLexer:mosel

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mosel
pub struct MoselLexer;

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
        Rule::token(r"(?m)\n", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)!.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?m)\(!(.|\n)*?!\)", COMMENT_MULTILINE),
        Rule::token(r"(?m)\b(a(?:nd|s)|break|c(?:ase|ount)|d(?:eclarations|o|ynamic)|e(?:l(?:if|se)|nd(?:(?:\-)?)|valuation)|f(?:alse|or(?:all|ward)|rom|unction)|hashmap|i(?:f|mports|n(?:clude|itiali(?:(?:[sz])ations)|ter))|m(?:ax|in|odel)|n(?:amespace|ext|ot|s(?:group|search))|o(?:ptions|[fr])|p(?:a(?:ckage|rameters)|ro(?:cedure|d)|ublic)|re(?:cord|peat|quirements|turn)|sum|t(?:hen|o|rue)|u(?:n(?:ion|til)|ses)|version|w(?:hile|ith))\b", TokenType::new(&["Keyword", "Builtin"])),
        Rule::token(r"(?m)\b(Mo(?:(?:[ds])el)|array|boolean|counter|date(?:(?:time)?)|i(?:nteger|s_(?:binary|continuous|free|integer|partint|s(?:em(?:(?:co|i)nt)|os(?:[12]))))|li(?:nctr|st)|mp(?:problem|var)|nlctr|r(?:ange|e(?:al|turned))|s(?:et|tring)|t(?:ext|ime)|xmldoc)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(\+|\-|\*|/|=|<=|>=|\||\^|<|>|<>|\.\.|\.|:=|::|:|in|mod|div)", OPERATOR),
        Rule::token(r"(?m)[()\[\]{},;]+", PUNCTUATION),
        Rule::token(r"(?m)\b(_|a(?:bs|dd(?:cut(?:(?:s)?)|m(?:ipsol|onths))|rctan|s(?:proc|sert))|b(?:asisstability|it(?:flip|neg|s(?:(?:e|hif)t)|test|val))|c(?:a(?:lcsolinfo|nceltimer)|eil|lear(?:aliases|m(?:ipdir|odcut))|o(?:m(?:mand|pile)|nnect|py(?:(?:soltoini|tex)t)|s)|r(?:eate|ossoverlpsol)|u(?:rrent(?:(?:dat|tim)e)|t(?:elt|first|head|last|t(?:ail|ext))))|d(?:atablock|e(?:f(?:(?:delayedrow|securevec)s)|l(?:c(?:ell|uts)|text)|tach)|isconnect|rop(?:cuts|nextevent))|e(?:ndswith|rase|stimatemarginals|x(?:i(?:sts|t)|p(?:(?:andpath|ortprob)?)))|f(?:c(?:lose|opy)|delete|flush|i(?:n(?:alize|d(?:fi(?:les|rst)|last|text|xsrvs))|xglobal)|l(?:oor|ushmsgq)|move|o(?:pen|rmattext)|s(?:elect|kipline)|write(?:(?:_|ln(?:(?:_)?))?))|get(?:a(?:ct|liases|nn(?:(?:ident|otation)s)|snumber)|b(?:anner|stat)|c(?:har|lass|nlist|oeff(?:(?:s)?)|plist|wd)|d(?:a(?:te|y(?:(?:num|s)?))|irsep|sop(?:aram|rop(?:(?:num)?))|ual(?:(?:ray)?))|e(?:lt|n(?:dparse|v)|xitcode)|f(?:i(?:d|rst)|name|rom(?:(?:(?:[gu])?)id)|s(?:ize|tat)|time)|gid|h(?:ead|o(?:stalias|ur))|i(?:d|is(?:(?:(?:sens|typ)e)?)|nf(?:cause|eas))|l(?:ast|b|ct|eft|oaded(?:(?:linct|mpva)rs))|m(?:inute|o(?:dprop(?:(?:num)?)|nth)|sec)|n(?:ame|extevent|ode)|o(?:bjval|serr(?:msg|or))|p(?:a(?:ram|thsep)|r(?:imalray|obstat))|qtype|r(?:ange|cost|e(?:adcnt|verse)|ight|mtid)|s(?:e(?:cond|nsrng|pchar)|ize(?:(?:(?:)?)?)|lack|ol(?:(?:)?)|ta(?:rt|tus)|ucc|ys(?:info(?:(?:)?)|stat))|t(?:ail|ime(?:(?:r)?)|mpdir|rim|ype(?:(?:)?))|u(?:b|id)|va(?:lue|rs(?:(?:)?))|weekday|y(?:(?:ear)?))|hasfeature|i(?:mplies|n(?:dicator|itglobal|serttext)|s(?:dynamic|eof|finite|hidden(?:(?:)?)|i(?:isvalid|n(?:f|tegral))|nan|odd|queueempty|valid))|jointext|l(?:n|o(?:ad(?:(?:basis|cuts|lpsol|mipsol|prob)?)|calsetparam|g))|m(?:a(?:ke(?:dir|path|sos(?:[12]))|x(?:imi(?:(?:[sz])e)|list))|emoryuse|in(?:imi(?:(?:[sz])e)|list))|n(?:e(?:w(?:muid|tar|zip)|xtfield)|ullevent)|openpipe|p(?:a(?:rse(?:extn|int|real|text)|stetext|th(?:match|split))|eeknextevent|ostsolve|ublish)|q(?:sort|uote)|r(?:andom|e(?:ad(?:(?:basis|dirs|ln|sol|textline)?)|finemipsol|g(?:match|replace)|jectintsol|move(?:dir|files)|pairinfeas(?:(?:_deprec)?)|s(?:et(?:(?:basis|iis|modpar|sol)?)|toreparam)|verse)|ound|un)|s(?:ave(?:basis|mipsol|s(?:ol|tate))|e(?:lectsol|nd|t(?:archconsistency|bstat|c(?:allback|bcutoff|har|o(?:eff|ntrol))|d(?:a(?:te|y)|(?:efstre|sopar)am)|en(?:dparse|v)|g(?:id|ndata)|h(?:idden(?:(?:)?)|o(?:stalias|ur))|ioerr|lb|m(?:atherr|i(?:nute|pdir)|o(?:d(?:cut|par)|nth)|sec)|name|oserror|param|qtype|ran(?:dseed|ge)|s(?:e(?:cond|pchar)|ol|tart|ucc)|t(?:ime(?:(?:r)?)|rim|ype)|u(?:b|cbdata|id)|(?:workdi|yea)r))|in|leep|plit(?:head|t(?:ail|ext))|qrt|t(?:artswith|o(?:p(?:(?:optimi(?:(?:[sz])e))?)|recut(?:(?:s)?))|rfmt)|ubstr|ystem)|t(?:arlist|extfmt|imestamp|o(?:(?:low|upp)er)|rim)|u(?:n(?:load(?:(?:prob)?)|publish|tar|zip)|selastbarsol)|version(?:num|str)|w(?:ait(?:(?:expired|for(?:(?:end)?))?)|rite(?:(?:_|basis|dirs|ln(?:(?:_)?)|prob|sol)?))|x(?:or|prs_add(?:ctr|indic))|ziplist)\b", NAME_FUNCTION),
        Rule::token(r"(?m)(\d+\.(?!\.)\d*|\.(?!.)\d+)([eE][+-]?\d+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d+([eE][+-]?\d+)?", NUMBER_INTEGER),
        Rule::token(r"(?m)[+-]?Infinity", NUMBER_INTEGER),
        Rule::token(r"(?m)0[xX][0-9a-fA-F]+", NUMBER),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"double_quote"])),
        Rule::token_to(r"(?m)\'", STRING_SINGLE, NewState::Push(vec![r"single_quote"])),
        Rule::token(r"(?m)(\w+|(\.(?!\.)))", TEXT),
    ]);
    m.insert(
        r"single_quote",
        vec![
            Rule::token_to(r"(?m)\'", STRING_SINGLE, NewState::Pop(1)),
            Rule::token(r"(?m)[^\']+", STRING_SINGLE),
        ],
    );
    m.insert(
        r"double_quote",
        vec![
            Rule::token(
                r#"(?m)(\\"|\\[0-7]{1,3}\D|\\[abfnrtv]|\\\\)"#,
                STRING_ESCAPE,
            ),
            Rule::token_to(r#"(?m)\""#, STRING_DOUBLE, NewState::Pop(1)),
            Rule::token(r#"(?m)[^"\\]+"#, STRING_DOUBLE),
        ],
    );
    Table(m)
}

impl Lexer for MoselLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
