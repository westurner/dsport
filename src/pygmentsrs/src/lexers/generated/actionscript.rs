//! AUTO-GENERATED from `pygments.pygments.lexers.actionscript:ActionScriptLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.actionscript:ActionScriptLexer:actionscript

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: actionscript, as
pub struct ActionscriptLexer;

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
        Rule::token(r"(?ms)\s+", WHITESPACE),
        Rule::token(r"(?ms)//.*?\n", COMMENT_SINGLE),
        Rule::token(r"(?ms)/\*.*?\*/", COMMENT_MULTILINE),
        Rule::token(r"(?ms)/(\\\\|\\[^\\]|[^/\\\n])*/[gim]*", STRING_REGEX),
        Rule::token(r"(?ms)[~^*!%&<>|+=:;,/?\\-]+", OPERATOR),
        Rule::token(r"(?ms)[{}\[\]();.]+", PUNCTUATION),
        Rule::token(r"(?ms)(arguments|break|c(?:a(?:se|tch)|ontinue)|d(?:efault|o)|e(?:ach|lse)|for|i(?:nstanceof|[fn])|new|return|switch|t(?:h(?:is|row)|ry|ypeof)|var|w(?:hile|ith))\b", KEYWORD),
        Rule::token(r"(?ms)(c(?:lass|onst)|dynamic|extends|f(?:inal|unction)|get|i(?:mp(?:lements|ort)|nt(?:er(?:face|nal)|rinsic))|na(?:(?:mespac|tiv)e)|override|p(?:ackage|r(?:ivate|otected)|ublic)|return|s(?:et|tatic|uper))\b", KEYWORD_DECLARATION),
        Rule::token(r"(?ms)(true|false|null|NaN|Infinity|-Infinity|undefined|Void)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?ms)(A(?:VM1Movie|c(?:cessibility(?:(?:Properties)?)|ti(?:onScriptVersion|vityEvent))|ntiAliasType|pplicationDomain|rray|s(?:Broadcaster|yncErrorEvent))|B(?:evelFilter|itmap(?:(?:Data(?:(?:Channel)?)|Filter(?:(?:Quality|Type)?))?)|l(?:endMode|urFilter)|oolean|yteArray)|C(?:SMSettings|a(?:mera|p(?:abilities|sStyle))|lass|o(?:lor(?:(?:MatrixFilter|Transform)?)|n(?:textMenu(?:(?:BuiltInItems|Event|Item)?)|vultionFilter)))|D(?:at(?:aEvent|e)|e(?:finitionError|leteObjectSample)|i(?:ctionary|spla(?:cmentMapFilter(?:(?:Mode)?)|yObject(?:(?:Container)?)))|ropShadowFilter)|E(?:OFError|ndian|rror(?:(?:Event)?)|v(?:alError|ent(?:(?:Dispatcher|Phase)?))|xternalInterface)|F(?:ile(?:Filter|Reference(?:(?:List)?))|o(?:cus(?:Direction|Event)|nt(?:(?:(?:Styl|Typ)e)?))|rameLabel|u(?:llScreenEvent|nction))|G(?:lowFilter|r(?:a(?:dient(?:BevelFilter|GlowFilter|Type)|phics)|idFitType))|HTTPStatusEvent|I(?:BitmapDrawable|D(?:3Info|ata(?:(?:In|Out)put)|ynamicPropertyOutputIDynamicPropertyWriter)|E(?:ventDispatcher|xternalizable)|ME(?:(?:ConversionMode|Event)?)|OError(?:(?:Event)?)|llegalOperationError|n(?:ter(?:activeObject|polationMethod)|v(?:alidSWFError|okeEvent)))|JointStyle|Key(?:(?:Location|board(?:(?:Event)?))?)|L(?:ineScaleMode|o(?:ad(?:Vars|er(?:(?:Context|Info)?))|cal(?:Connection|e)))|M(?:at(?:h|rix)|emoryError|icrophone|o(?:rphShape|use(?:(?:Event)?)|vieClip(?:(?:Loader)?)))|N(?:amespace|e(?:t(?:Connection|St(?:atusEvent|ream))|wObjectSample)|umber)|Object(?:(?:Encoding)?)|P(?:ixelSnapping|oint|r(?:intJob(?:(?:O(?:ptions|rientation))?)|o(?:gressEvent|xy)))|QName|R(?:angeError|e(?:ctangle|ferenceError|gExp|sponder))|S(?:WFVersion|ample|c(?:ene|riptTimeoutError)|e(?:curity(?:(?:Domain|Error(?:(?:Event)?)|Panel)?)|lection)|ha(?:pe|redObject(?:(?:FlushStatus)?))|impleButton|o(?:cket|und(?:(?:Channel|LoaderContext|Mixer|Transform)?))|pr(?:eadMethod|ite)|t(?:a(?:ck(?:Frame|OverflowError)|ge(?:(?:Align|DisplayState|Quality|ScaleMode)?)|t(?:(?:icTex|usEven)t))|ring|yleSheet)|y(?:n(?:cEvent|taxError)|stem))|T(?:ext(?:ColorType|F(?:ield(?:(?:(?:AutoSiz|Typ)e)?)|ormat(?:(?:Align)?))|LineMetrics|Renderer|Snapshot)|imer(?:(?:Event)?)|ransform|ypeError)|UR(?:IError|L(?:Loader(?:(?:DataFormat)?)|Request(?:(?:Header|Method)?)|Stream|Variabeles))|V(?:erifyError|ideo)|XML(?:(?:Document|List|Node(?:(?:Type)?)|Socket|UI)?)|(?:(?:u)?)int)\b", NAME_BUILTIN),
        Rule::token(r"(?ms)(clearInterval|decodeURI(?:(?:Component)?)|e(?:ncodeURI|scape|val)|fscommand|get(?:Timer|URL|Version)|is(?:Finite|NaN|XMLName)|parse(?:(?:Floa|In)t)|setInterval|trace|u(?:nescape|pdateAfterEvent))\b", NAME_FUNCTION),
        Rule::token(r"(?ms)[$a-zA-Z_]\w*", NAME_OTHER),
        Rule::token(r"(?ms)[0-9][0-9]*\.[0-9]+([eE][0-9]+)?[fd]?", NUMBER_FLOAT),
        Rule::token(r"(?ms)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?ms)[0-9]+", NUMBER_INTEGER),
        Rule::token(r#"(?ms)"(\\\\|\\[^\\]|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?ms)'(\\\\|\\[^\\]|[^'\\])*'", STRING_SINGLE),
    ]);
    Table(m)
}

impl Lexer for ActionscriptLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
