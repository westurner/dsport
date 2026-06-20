#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:ClojureLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:ClojureLexer:clojure

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: clojure, clj
pub struct ClojureLexer;

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
        Rule::token(r"(?m);.*$", COMMENT_SINGLE),
        Rule::token(r"(?m),+", TEXT),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+/\d+", NUMBER),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)0x-?[abcdef\d]+", NUMBER_HEX),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token(r"(?m)'(?!#)[\w!$%*+<=>?/.#|-]+", STRING_SYMBOL),
        Rule::token(r"(?m)\\(.|[a-z]+)", STRING_CHAR),
        Rule::token(r"(?m)::?#?(?!#)[\w!$%*+<=>?/.#|-]+", STRING_SYMBOL),
        Rule::token(r"(?m)~@|[`\'#^~&@]", OPERATOR),
        Rule::token(r"(?m)(\.|d(?:ef|o)|fn|if|l(?:et|oop)|new|quote|var) ", KEYWORD),
        Rule::token(r"(?m)(de(?:clare|f(?:in(?:(?:lin|terfac)e)|m(?:acro|ethod|ulti)|n\-|once|pro(?:ject|tocol)|record|struct|type|[\-n]))|ns) ", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(\->|\.\.|<=|==|>=|a(?:ccessor|ge(?:nt(?:(?:\-errors)?)|t)|l(?:ength|l\-ns|ter)|nd|pp(?:end\-child|ly)|rray\-map|s(?:et(?:(?:\-(?:b(?:oolean|yte)|char|double|float|int|long|short))?)|s(?:ert|oc))|wait(?:(?:\-for)?))|b(?:ean|i(?:nding|t\-(?:and|not|or|shift\-(?:(?:lef|righ)t)|xor))|oolean|ranch\?|utlast|yte)|c(?:ast|h(?:ar|ildren)|l(?:(?:as|ear\-agent\-error)s)|o(?:m(?:m(?:ent|ute)|p(?:(?:arator|lement)?))|n(?:cat|st(?:(?:antl|ruct\-prox)y)|tains\?|[djs])|unt)|reate\-(?:ns|struct)|ycle)|d(?:e(?:c|ref)|i(?:fference|s(?:j|soc|tinct))|o(?:all|c|run|s(?:eq|ync)|t(?:imes|o)|uble|wn)|rop(?:(?:\-while)?))|e(?:dit|n(?:d\?|sure)|v(?:al|ery\?))|f(?:alse\?|first|i(?:l(?:e\-seq|ter)|nd(?:(?:\-(?:doc|ns|var))?)|rst)|l(?:oat|ush)|nseq|or|rest)|ge(?:nsym|t(?:(?:\-proxy\-class)?))|hash\-(?:map|set)|i(?:denti(?:cal\?|ty)|f\-(?:(?:le|no)t)|mport|n(?:\-ns|dex|s(?:ert\-(?:child|(?:lef|righ)t)|pect\-t(?:(?:abl|re)e)|tance\?)|t(?:er(?:leave|section)|o(?:(?:\-array)?))|[ct])|terate)|join|key(?:(?:s|word(?:(?:\?)?))?)|l(?:a(?:st|zy\-c(?:at|ons))|eft(?:(?:s)?)|i(?:ne\-seq|st(?:(?:\*)?))|o(?:ad(?:(?:\-file)?)|cking|ng|op))|m(?:a(?:croexpand(?:(?:\-1)?)|ke\-(?:array|node)|p(?:\-invert|\?|cat)|x\-key|[px])|e(?:mfn|rge(?:(?:\-with)?)|ta)|in(?:(?:\-key)?))|n(?:ame(?:(?:space)?)|e(?:g\?|w(?:(?:line)?)|xt)|il\?|o(?:de|t(?:(?:\-(?:(?:an|ever)y\?)|=)?))|s\-(?:i(?:(?:mport|ntern)s)|map|name|publics|re(?:fers|solve)|unmap)|th(?:(?:rest)?))|or|p(?:a(?:r(?:se|tial)|th)|eek|o(?:p|s\?)|r(?:(?:\-str|int(?:(?:\-str|ln(?:(?:\-str)?))?)|n(?:(?:\-str)?)|o(?:ject|xy(?:(?:\-mappings)?)))?))|quot|r(?:an(?:d(?:(?:\-int)?)|ge)|e(?:\-(?:find|groups|matche(?:[rs])|pattern|seq)|ad(?:(?:\-line)?)|duce|f(?:\-set|er)|move(?:(?:\-(?:method|ns))?)|name(?:(?:\-keys)?)|p(?:eat|l(?:(?:ac|icat)e))|s(?:olve|t|ultset\-seq)|verse|[fm])|first|ight(?:(?:s)?)|oot|rest|seq)|s(?:e(?:cond|lect(?:(?:\-keys)?)|nd(?:(?:\-off)?)|q(?:\-zip|\?)|[qt])|hort|lurp|o(?:me|rt(?:(?:\-by|ed\-(?:map(?:(?:\-by)?)|set))?))|p(?:ecial\-symbol\?|lit\-(?:at|with))|tr(?:(?:ing\?|uct(?:(?:\-map)?))?)|ub(?:s|vec)|y(?:mbol(?:(?:\?)?)|nc))|t(?:ake(?:(?:\-(?:nth|while))?)|est|ime|o\-array(?:(?:\-2d)?)|r(?:ee\-seq|ue\?))|u(?:nion|p(?:(?:date\-proxy)?))|v(?:a(?:l(?:(?:s)?)|r(?:\-(?:(?:[gs])et)|\?))|ector(?:(?:\-zip|\?)?))|w(?:hen(?:(?:\-(?:(?:firs|le|no)t))?)|ith\-(?:local\-vars|meta|o(?:pen|ut\-str)))|xml\-(?:seq|zip)|z(?:ero\?|ip(?:map|per))|[*+\-/<=>]) ", NAME_BUILTIN),
        Rule::token(r"(?m)(?<=\()(?!#)[\w!$%*+<=>?/.#|-]+", NAME_FUNCTION),
        Rule::token(r"(?m)(?!#)[\w!$%*+<=>?/.#|-]+", NAME_VARIABLE),
        Rule::token(r"(?m)(\[|\])", PUNCTUATION),
        Rule::token(r"(?m)(\{|\})", PUNCTUATION),
        Rule::token(r"(?m)(\(|\))", PUNCTUATION),
    ]);
    Table(m)
}

impl Lexer for ClojureLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
