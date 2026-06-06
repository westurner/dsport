//! AUTO-GENERATED from `pygments.pygments.lexers.hdl:SystemVerilogLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.hdl:SystemVerilogLexer:systemverilog

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: systemverilog, sv
pub struct SystemverilogLexer;

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
        Rule::bygroups_to(r"(?m)^(\s*)(`define)", vec![Some(WHITESPACE), Some(COMMENT_PREPROC)], NewState::Push(vec![r"macro"])),
        Rule::bygroups(r"(?m)^(\s*)(package)(\s+)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(WHITESPACE)]),
        Rule::bygroups_to(r"(?m)^(\s*)(import)(\s+)", vec![Some(WHITESPACE), Some(KEYWORD_NAMESPACE), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?m)/(\\\n)?/(\n|(.|\n)*?[^\\]\n)", COMMENT_SINGLE),
        Rule::token(r"(?m)/(\\\n)?[*](.|\n)*?[*](\\\n)?/", COMMENT_MULTILINE),
        Rule::token(r"(?m)[{}#@]", PUNCTUATION),
        Rule::token_to(r#"(?m)L?""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)L?'(\\.|\\[0-7]{1,3}|\\x[a-fA-F0-9]{1,2}|[^\\\'\n])'", STRING_CHAR),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)([1-9][_0-9]*)?\s*\'[sS]?[bB]\s*[xXzZ?01][_xXzZ?01]*", NUMBER_BIN),
        Rule::token(r"(?m)([1-9][_0-9]*)?\s*\'[sS]?[oO]\s*[xXzZ?0-7][_xXzZ?0-7]*", NUMBER_OCT),
        Rule::token(r"(?m)([1-9][_0-9]*)?\s*\'[sS]?[dD]\s*[xXzZ?0-9][_xXzZ?0-9]*", NUMBER_INTEGER),
        Rule::token(r"(?m)([1-9][_0-9]*)?\s*\'[sS]?[hH]\s*[xXzZ?0-9a-fA-F][_xXzZ?0-9a-fA-F]*", NUMBER_HEX),
        Rule::token(r"(?m)\'[01xXzZ]", NUMBER),
        Rule::token(r"(?m)[0-9][_0-9]*", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)(dist|inside)\b", OPERATOR_WORD),
        Rule::token(r"(?m)[()\[\],.;\'$]", PUNCTUATION),
        Rule::token(r"(?m)`[a-zA-Z_]\w*", NAME_CONSTANT),
        Rule::token(r"(?m)(a(?:ccept_on|l(?:ias|ways(?:(?:_(?:comb|ff|latch))?))|nd|ss(?:ert|ign|ume)|utomatic)|b(?:e(?:fore|gin)|in(?:sof|[ds])|reak|uf(?:(?:if(?:[01]))?))|c(?:ase(?:(?:[xz])?)|ell|hecker|locking|mos|o(?:n(?:fig|straint|t(?:ext|inue))|ver(?:(?:group|point)?))|ross)|d(?:e(?:assign|f(?:ault|param)|sign)|isable|o)|e(?:dge|lse|n(?:d(?:(?:c(?:ase|hecker|(?:lockin|onfi)g)|function|g(?:enerate|roup)|interface|module|p(?:ackage|r(?:imitive|o(?:gram|perty)))|s(?:equence|pecify)|ta(?:ble|sk))?)|um)|ventually|x(?:p(?:(?:ec|or)t)|tern))|f(?:i(?:nal|rst_match)|or(?:(?:ce|e(?:ach|ver)|k(?:(?:join)?))?)|unction)|g(?:en(?:erate|var)|lobal)|highz(?:[01])|i(?:f(?:(?:f|none)?)|gnore_bins|llegal_bins|mp(?:l(?:(?:ement|ie)s)|ort)|n(?:c(?:dir|lude)|itial|out|put|stance|ter(?:connect|face|sect)))|join(?:(?:_(?:any|none))?)|l(?:arge|et|ib(?:list|rary)|ocal(?:(?:param)?))|m(?:a(?:cromodule|tches)|edium|od(?:port|ule))|n(?:and|e(?:gedge|ttype|w|xttime)|mos|o(?:showcancelled|tif(?:[01])|[rt])|ull)|o(?:r|utput)|p(?:a(?:ck(?:age|ed)|rameter)|mos|osedge|r(?:i(?:mitive|ority)|o(?:gram|perty|tected))|u(?:l(?:l(?:down|up|[01])|sestyle_on(?:(?:detec|even)t))|re))|r(?:and(?:(?:c(?:(?:ase)?)|sequence)?)|cmos|e(?:f|ject_on|lease|peat|strict|turn)|nmos|pmos|tran(?:(?:if(?:[01]))?))|s(?:_(?:always|eventually|nexttime|until(?:(?:_with)?))|calared|equence|howcancelled|mall|o(?:ft|lve)|pec(?:ify|param)|t(?:atic|r(?:ong(?:(?:[01])?)|uct))|uper|ync_(?:(?:accep|rejec)t_on))|t(?:a(?:ble|gged|sk)|h(?:is|roughout)|ime(?:precision|unit)|ran(?:(?:if(?:[01]))?)|ypedef)|u(?:n(?:i(?:on|que(?:(?:0)?))|t(?:il(?:(?:_with)?)|yped))|se)|v(?:ectored|irtual)|w(?:ait(?:(?:_order)?)|eak(?:(?:[01])?)|hile|i(?:ldcard|th(?:(?:in)?)))|x(?:(?:(?:n)?)or))\b", KEYWORD),
        Rule::bygroups(r"(?m)(class)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(extends)(\s+)([a-zA-Z_]\w*)", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::bygroups(r"(?m)(endclass\b)(?:(\s*)(:)(\s*)([a-zA-Z_]\w*))?", vec![Some(KEYWORD_DECLARATION), Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE), Some(NAME_CLASS)]),
        Rule::token(r"(?m)(b(?:it|yte)|c(?:handle|onst)|event|int(?:(?:eger)?)|lo(?:gic|ngint)|re(?:al(?:(?:time)?)|g)|s(?:hort(?:int|real)|igned|tring|upply(?:[01]))|t(?:ime|ri(?:(?:and|or|reg|[01])?)|ype)|u(?:nsigned|wire)|v(?:ar|oid)|w(?:and|ire|or))\b", KEYWORD_TYPE),
        Rule::token(r"(?m)(`(?:__(?:(?:FIL|LIN)E__)|begin_keywords|celldefine|def(?:(?:ault_nettyp|in)e)|e(?:ls(?:e|if)|nd(?:_keywords|celldefine|if))|i(?:f(?:(?:(?:n)?)def)|nclude)|line|nounconnected_drive|pragma|resetall|timescale|un(?:connected_drive|def(?:(?:ineall)?))))\b", COMMENT_PREPROC),
        Rule::token(r"(?m)(\$(?:a(?:cos(?:(?:h)?)|s(?:in(?:(?:h)?)|sert(?:control|failo(?:ff|n)|kill|nonvacuouson|o(?:ff|n)|passo(?:ff|n)|vacuousoff)|ync\$(?:and\$(?:array|plane)|n(?:and\$(?:array|plane)|or\$(?:array|plane))|or\$(?:array|plane)))|tan(?:(?:[2h])?))|bits(?:(?:to(?:(?:(?:short)?)real))?)|c(?:ast|eil|hang(?:ed(?:(?:_gclk)?)|ing_gclk)|log2|o(?:s(?:(?:h)?)|unt(?:(?:bit|one)s)|verage_(?:control|get(?:(?:_max)?)|(?:merg|sav)e)))|d(?:i(?:mensions|s(?:play(?:(?:[bho])?)|t_(?:chi_square|e(?:rlang|xponential)|normal|poisson|t|uniform)))|ump(?:all|f(?:ile|lush)|limit|o(?:ff|n)|ports(?:(?:all|flush|limit|o(?:ff|n))?)|vars))|e(?:rror|x(?:it|p))|f(?:a(?:lling_gclk|tal)|close|display(?:(?:[bho])?)|e(?:ll(?:(?:_gclk)?)|of|rror)|flush|get(?:[cs])|inish|loor|monitor(?:(?:[bho])?)|open|read|s(?:canf|eek|trobe(?:(?:[bho])?))|tell|uture_gclk|write(?:(?:[bho])?))|get_coverage|h(?:igh|ypot)|i(?:n(?:crement|fo)|sun(?:bounded|known)|tor)|l(?:eft|n|o(?:ad_coverage_db|g10|w))|monitor(?:(?:o(?:ff|n)|[bho])?)|onehot(?:(?:0)?)|p(?:ast(?:(?:_gclk)?)|ow|rinttimescale)|q_(?:add|exam|full|(?:initializ|remov)e)|r(?:andom|e(?:a(?:dmem(?:[bh])|lt(?:ime|obits))|wind)|i(?:ght|sing_gclk)|ose(?:(?:_gclk)?)|toi)|s(?:ampled|et_coverage_db_name|format(?:(?:f)?)|hortrealtobits|i(?:gned|n(?:(?:h)?)|ze)|qrt|scanf|t(?:able(?:(?:_gclk)?)|eady_gclk|ime|op|robe(?:(?:[bho])?))|write(?:(?:[bho])?)|y(?:nc\$(?:and\$(?:array|plane)|n(?:and\$(?:array|plane)|or\$(?:array|plane))|or\$(?:array|plane))|stem))|t(?:an(?:(?:h)?)|est\$plusargs|ime(?:(?:format)?)|ypename)|un(?:getc|packed_dimensions|signed)|value\$plusargs|w(?:arning|rite(?:(?:mem(?:[bh])|[bho])?))))\b", NAME_BUILTIN),
        Rule::token(r"(?m)[a-zA-Z_]\w*:(?!:)", NAME_LABEL),
        Rule::token(r"(?m)\$?[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)\\(\S+)", NAME),
    ]);
    m.insert(r"string", vec![
        Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)\\([\\abfnrtv"\']|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#, STRING_ESCAPE),
        Rule::token(r#"(?m)[^\\"\n]+"#, STRING),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(STRING_ESCAPE), Some(WHITESPACE)]),
        Rule::token(r"(?m)\\", STRING),
    ]);
    m.insert(r"macro", vec![
        Rule::token(r"(?m)[^/\n]+", COMMENT_PREPROC),
        Rule::token(r"(?m)/[*](.|\n)*?[*]/", COMMENT_MULTILINE),
        Rule::token_to(r"(?m)//.*?$", COMMENT_SINGLE, NewState::Pop(1)),
        Rule::token(r"(?m)/", COMMENT_PREPROC),
        Rule::token(r"(?m)(?<=\\)\n", COMMENT_PREPROC),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
    ]);
    m.insert(r"import", vec![
        Rule::token_to(r"(?m)[\w:]+\*?", NAME_NAMESPACE, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for SystemverilogLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
