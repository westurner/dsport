#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.jvm:IokeLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.jvm:IokeLexer:ioke

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ioke, ik
pub struct IokeLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"interpolatableText", vec![
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
    ]);
    m.insert(r"text", vec![
        Rule::token_to(r#"(?m)(?<!\\)""#, STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
        Rule::token(r#"(?m)[^"]"#, STRING),
    ]);
    m.insert(r"documentation", vec![
        Rule::token_to(r#"(?m)(?<!\\)""#, STRING_DOC, NewState::Pop(1)),
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
        Rule::token(r#"(?m)[^"]"#, STRING_DOC),
    ]);
    m.insert(r"textInterpolationRoot", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m);(.*?)\n", COMMENT),
        Rule::token(r"(?m)\A#!(.*?)\n", COMMENT),
        Rule::token_to(r"(?m)#/", STRING_REGEX, NewState::Push(vec![r"slashRegexp"])),
        Rule::token_to(r"(?m)#r\[", STRING_REGEX, NewState::Push(vec![r"squareRegexp"])),
        Rule::token(r"(?m):[\w!:?]+", STRING_SYMBOL),
        Rule::token(r"(?m)[\w!:?]+:(?![\w!?])", STRING_OTHER),
        Rule::token(r#"(?m):"(\\\\|\\[^\\]|[^"\\])*""#, STRING_SYMBOL),
        Rule::token_to(r#"(?m)((?<=fn\()|(?<=fnx\()|(?<=method\()|(?<=macro\()|(?<=lecro\()|(?<=syntax\()|(?<=dmacro\()|(?<=dlecro\()|(?<=dlecrox\()|(?<=dsyntax\())(\s*)""#, STRING_DOC, NewState::Push(vec![r"documentation"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"text"])),
        Rule::token_to(r"(?m)#\[", STRING, NewState::Push(vec![r"squareText"])),
        Rule::token(r"(?m)\w[\w!:?]+(?=\s*=.*mimic\s)", NAME_ENTITY),
        Rule::token(r"(?m)[a-zA-Z_][\w!:?]*(?=[\s]*[+*/-]?=[^=].*($|\.))", NAME_VARIABLE),
        Rule::token(r"(?m)(break|cond|continue|do|ensure|for|for:dict|for:set|if|let|loop|p:for|p:for:dict|p:for:set|return|unless|until|while|with)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(eval|mimic|print|println)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(cell\?|cellNames|cellOwner\?|cellOwner|cells|cell|documentation|hash|identity|mimic|removeCell\!|undefineCell\!)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(stackTraceAsText)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(dict|list|message|set)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(case|case:and|case:else|case:nand|case:nor|case:not|case:or|case:otherwise|case:xor)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(asText|become\!|derive|freeze\!|frozen\?|in\?|is\?|kind\?|mimic\!|mimics|mimics\?|prependMimic\!|removeAllMimics\!|removeMimic\!|same\?|send|thaw\!|uniqueHexId)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(after|around|before)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(kind|cellDescriptionDict|cellSummary|genSym|inspect|notice)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(use|destructuring)", KEYWORD_RESERVED),
        Rule::token(r"(?m)(cell\?|cellOwner\?|cellOwner|cellNames|cells|cell|documentation|identity|removeCell!|undefineCell)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(internal:compositeRegexp|internal:concatenateText|internal:createDecimal|internal:createNumber|internal:createRegexp|internal:createText)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(availableRestarts|bind|error\!|findRestart|handle|invokeRestart|rescue|restart|signal\!|warn\!)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(nil|false|true)(?![\w!:?])", NAME_CONSTANT),
        Rule::token(r"(?m)(Arity|Base|Call|Condition|DateTime|Aspects|Pointcut|Assignment|BaseBehavior|Boolean|Case|AndCombiner|Else|NAndCombiner|NOrCombiner|NotCombiner|OrCombiner|XOrCombiner|Conditions|Definitions|FlowControl|Internal|Literals|Reflection|DefaultMacro|DefaultMethod|DefaultSyntax|Dict|FileSystem|Ground|Handler|Hook|IO|IokeGround|Struct|LexicalBlock|LexicalMacro|List|Message|Method|Mixins|NativeMethod|Number|Origin|Pair|Range|Reflector|Regexp Match|Regexp|Rescue|Restart|Runtime|Sequence|Set|Symbol|System|Text|Tuple)(?![\w!:?])", NAME_BUILTIN),
        Rule::token(r"(?m)(generateMatchMethod|aliasMethod|λ|ʎ|fnx|fn|method|dmacro|dlecro|syntax|macro|dlecrox|lecrox|lecro|syntax)(?![\w!:?])", NAME_FUNCTION),
        Rule::token(r"(?m)-?0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)-?(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)#\(", PUNCTUATION),
        Rule::token(r"(?m)(&&>>|\|\|>>|\*\*>>|:::|::|\.\.\.|===|\*\*>|\*\*=|&&>|&&=|\|\|>|\|\|=|\->>|\+>>|!>>|<>>>|<>>|&>>|%>>|#>>|@>>|/>>|\*>>|\?>>|\|>>|\^>>|~>>|\$>>|=>>|<<=|>>=|<=>|<\->|=~|!~|=>|\+\+|\-\-|<=|>=|==|!=|&&|\.\.|\+=|\-=|\*=|\/=|%=|&=|\^=|\|=|<\-|\+>|!>|<>|&>|%>|#>|\@>|\/>|\*>|\?>|\|>|\^>|~>|\$>|<\->|\->|<<|>>|\*\*|\?\||\?&|\|\||>|<|\*|\/|%|\+|\-|&|\^|\||=|\$|!|~|\?|#|\u2260|\u2218|\u2208|\u2209)", OPERATOR),
        Rule::token(r"(?m)(and|nand|or|xor|nor|return|import)(?![\w!?])", OPERATOR),
        Rule::token(r"(?m)(\`\`|\`|\'\'|\'|\.|\,|@@|@|\[|\]|\(|\)|\{|\})", PUNCTUATION),
        Rule::token(r"(?m)[A-Z][\w!:?]*", NAME_CLASS),
        Rule::token(r"(?m)[a-z_][\w!:?]*", NAME),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m);(.*?)\n", COMMENT),
        Rule::token(r"(?m)\A#!(.*?)\n", COMMENT),
        Rule::token_to(r"(?m)#/", STRING_REGEX, NewState::Push(vec![r"slashRegexp"])),
        Rule::token_to(r"(?m)#r\[", STRING_REGEX, NewState::Push(vec![r"squareRegexp"])),
        Rule::token(r"(?m):[\w!:?]+", STRING_SYMBOL),
        Rule::token(r"(?m)[\w!:?]+:(?![\w!?])", STRING_OTHER),
        Rule::token(r#"(?m):"(\\\\|\\[^\\]|[^"\\])*""#, STRING_SYMBOL),
        Rule::token_to(r#"(?m)((?<=fn\()|(?<=fnx\()|(?<=method\()|(?<=macro\()|(?<=lecro\()|(?<=syntax\()|(?<=dmacro\()|(?<=dlecro\()|(?<=dlecrox\()|(?<=dsyntax\())(\s*)""#, STRING_DOC, NewState::Push(vec![r"documentation"])),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"text"])),
        Rule::token_to(r"(?m)#\[", STRING, NewState::Push(vec![r"squareText"])),
        Rule::token(r"(?m)\w[\w!:?]+(?=\s*=.*mimic\s)", NAME_ENTITY),
        Rule::token(r"(?m)[a-zA-Z_][\w!:?]*(?=[\s]*[+*/-]?=[^=].*($|\.))", NAME_VARIABLE),
        Rule::token(r"(?m)(break|cond|continue|do|ensure|for|for:dict|for:set|if|let|loop|p:for|p:for:dict|p:for:set|return|unless|until|while|with)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(eval|mimic|print|println)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(cell\?|cellNames|cellOwner\?|cellOwner|cells|cell|documentation|hash|identity|mimic|removeCell\!|undefineCell\!)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(stackTraceAsText)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(dict|list|message|set)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(case|case:and|case:else|case:nand|case:nor|case:not|case:or|case:otherwise|case:xor)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(asText|become\!|derive|freeze\!|frozen\?|in\?|is\?|kind\?|mimic\!|mimics|mimics\?|prependMimic\!|removeAllMimics\!|removeMimic\!|same\?|send|thaw\!|uniqueHexId)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(after|around|before)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(kind|cellDescriptionDict|cellSummary|genSym|inspect|notice)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(use|destructuring)", KEYWORD_RESERVED),
        Rule::token(r"(?m)(cell\?|cellOwner\?|cellOwner|cellNames|cells|cell|documentation|identity|removeCell!|undefineCell)(?![\w!:?])", KEYWORD),
        Rule::token(r"(?m)(internal:compositeRegexp|internal:concatenateText|internal:createDecimal|internal:createNumber|internal:createRegexp|internal:createText)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(availableRestarts|bind|error\!|findRestart|handle|invokeRestart|rescue|restart|signal\!|warn\!)(?![\w!:?])", KEYWORD_RESERVED),
        Rule::token(r"(?m)(nil|false|true)(?![\w!:?])", NAME_CONSTANT),
        Rule::token(r"(?m)(Arity|Base|Call|Condition|DateTime|Aspects|Pointcut|Assignment|BaseBehavior|Boolean|Case|AndCombiner|Else|NAndCombiner|NOrCombiner|NotCombiner|OrCombiner|XOrCombiner|Conditions|Definitions|FlowControl|Internal|Literals|Reflection|DefaultMacro|DefaultMethod|DefaultSyntax|Dict|FileSystem|Ground|Handler|Hook|IO|IokeGround|Struct|LexicalBlock|LexicalMacro|List|Message|Method|Mixins|NativeMethod|Number|Origin|Pair|Range|Reflector|Regexp Match|Regexp|Rescue|Restart|Runtime|Sequence|Set|Symbol|System|Text|Tuple)(?![\w!:?])", NAME_BUILTIN),
        Rule::token(r"(?m)(generateMatchMethod|aliasMethod|λ|ʎ|fnx|fn|method|dmacro|dlecro|syntax|macro|dlecrox|lecrox|lecro|syntax)(?![\w!:?])", NAME_FUNCTION),
        Rule::token(r"(?m)-?0[xX][0-9a-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)-?(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?m)-?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)#\(", PUNCTUATION),
        Rule::token(r"(?m)(&&>>|\|\|>>|\*\*>>|:::|::|\.\.\.|===|\*\*>|\*\*=|&&>|&&=|\|\|>|\|\|=|\->>|\+>>|!>>|<>>>|<>>|&>>|%>>|#>>|@>>|/>>|\*>>|\?>>|\|>>|\^>>|~>>|\$>>|=>>|<<=|>>=|<=>|<\->|=~|!~|=>|\+\+|\-\-|<=|>=|==|!=|&&|\.\.|\+=|\-=|\*=|\/=|%=|&=|\^=|\|=|<\-|\+>|!>|<>|&>|%>|#>|\@>|\/>|\*>|\?>|\|>|\^>|~>|\$>|<\->|\->|<<|>>|\*\*|\?\||\?&|\|\||>|<|\*|\/|%|\+|\-|&|\^|\||=|\$|!|~|\?|#|\u2260|\u2218|\u2208|\u2209)", OPERATOR),
        Rule::token(r"(?m)(and|nand|or|xor|nor|return|import)(?![\w!?])", OPERATOR),
        Rule::token(r"(?m)(\`\`|\`|\'\'|\'|\.|\,|@@|@|\[|\]|\(|\)|\{|\})", PUNCTUATION),
        Rule::token(r"(?m)[A-Z][\w!:?]*", NAME_CLASS),
        Rule::token(r"(?m)[a-z_][\w!:?]*", NAME),
    ]);
    m.insert(r"slashRegexp", vec![
        Rule::token_to(r"(?m)(?<!\\)/[im-psux]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
        Rule::token(r"(?m)\\/", STRING_REGEX),
        Rule::token(r"(?m)[^/]", STRING_REGEX),
    ]);
    m.insert(r"squareRegexp", vec![
        Rule::token_to(r"(?m)(?<!\\)][im-psux]*", STRING_REGEX, NewState::Pop(1)),
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
        Rule::token(r"(?m)\\]", STRING_REGEX),
        Rule::token(r"(?m)[^\]]", STRING_REGEX),
    ]);
    m.insert(r"squareText", vec![
        Rule::token_to(r"(?m)(?<!\\)]", STRING, NewState::Pop(1)),
        Rule::token(r#"(?m)(\\b|\\e|\\t|\\n|\\f|\\r|\\"|\\\\|\\#|\\\Z|\\u[0-9a-fA-F]{1,4}|\\[0-3]?[0-7]?[0-7])"#, STRING_ESCAPE),
        Rule::token_to(r"(?m)#\{", PUNCTUATION, NewState::Push(vec![r"textInterpolationRoot"])),
        Rule::token(r"(?m)[^\]]", STRING),
    ]);
    Table(m)
}

impl Lexer for IokeLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
