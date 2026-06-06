//! AUTO-GENERATED from `pygments.pygments.lexers.theorem:RocqLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.theorem:RocqLexer:coq

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: coq, rocq, rocq-prover
pub struct CoqLexer;

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
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)false|true|\(\)|\[\]", NAME_BUILTIN_PSEUDO),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?m)\b(?:[^\W\d][\w\']*\.)+[^\W\d][\w\']*\b", NAME),
        Rule::token(r"(?m)\bEquations\b\??", KEYWORD_NAMESPACE),
        Rule::bygroups(r"(?m)\b(Elpi)(\s+)(Program|Query|Accumulate|Command|Typecheck|Db|Export|Tactic)?\b", vec![Some(KEYWORD_NAMESPACE), Some(TEXT), Some(KEYWORD_NAMESPACE)]),
        Rule::token_to(r"(?m)\bUnset\b|\bSet(?=[ \t]+[A-Z][a-z][^\n]*?\.)", KEYWORD_NAMESPACE, NewState::Push(vec![r"set-options"])),
        Rule::token_to(r"(?m)\b(?:String|Number)\s+Notation", KEYWORD_NAMESPACE, NewState::Push(vec![r"sn-notation"])),
        Rule::token(r"(?m)\b(A(?:bort|dmitted|ll|rguments|xiom(?:(?:s)?))|Bind|C(?:anonical|heck|l(?:ass|ose)|o(?:Fixpoint|Inductive|ercion|mpute|ntext(?:(?:ual)?)|rollary))|De(?:clare|fin(?:ed|ition)|limit)|E(?:lpi|nd|val|x(?:ample|isting|(?:por|trac)t))|F(?:a(?:ct|il)|ixpoint|rom|unction)|G(?:lobal|oal|raph)|H(?:int|ypothes(?:(?:[ei])s))|I(?:mp(?:licit(?:(?:s)?)|ort)|n(?:(?:clud|ductiv|stanc)e))|L(?:e(?:mma|t)|ocal|tac(?:(?:2)?))|Mo(?:dule|nomorphic|rphism)|N(?:(?:ext\ Oblig|ot)ation)|Op(?:aque|en)|P(?:arameter(?:(?:s)?)|olymorphic|r(?:enex|int(?:(?:ing)?)|o(?:gram|jections|of|p(?:erty|osition))))|Qed|Re(?:cord|lation|mark|quire|s(?:erved|olve|tart)|write)|S(?:ave|c(?:(?:hem|op)e)|e(?:arch|ction)|how|tr(?:ict|ucture))|T(?:actic|heorem|ransparent|ypes)|Un(?:do|(?:ivers|shelv)e)|V(?:aria(?:ble(?:(?:s)?)|nt)|iew)|(?:in|out)side)\b", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)\b(as|cofix|e(?:lse|nd|xists(?:(?:2)?))|f(?:ix|or(?:(?:all)?)|un)|i(?:[fns])|let|match|nosimpl|of|return|struct|then|with)\b", KEYWORD),
        Rule::token(r"(?m)\b(Prop|S(?:Prop|et)|Type)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)\b(a(?:fter|pply|ssert|uto(?:(?:rewrite)?))|bool_congr|c(?:ase|bv|hange|lear|o(?:mpute|n(?:(?:g|structo)r))|ut(?:(?:rewrite)?))|destruct|e(?:a(?:pply|uto)|constructor|lim|rewrite|transitivity)|f(?:(?:ie|o)ld)|generalize|h(?:ave|nf)|in(?:duction|jection|t(?:ro(?:(?:s)?)|uition)|version)|l(?:azy|eft|oss)|move|nat(?:_(?:congr|norm)|ive_compute)|p(?:attern|ose)|r(?:e(?:d|fine|name|place|vert|write)|i(?:ght|ng))|s(?:et(?:(?:oid_rewrite)?)|impl|plit|u(?:bst|ff(?:(?:ices)?))|ymmetry)|t(?:auto|r(?:ansitivity|ivial))|u(?:n(?:fold|lock)|sing)|vm_compute|w(?:ithout|log))\b", KEYWORD),
        Rule::token(r"(?m)\b(a(?:dmit|ssumption)|by|con(?:gruence|tradiction)|d(?:(?:iscriminat|on)e)|exact|l(?:(?:[ir])a)|n(?:ia|ow|ra)|omega|psatz|r(?:eflexivity|omega)|solve|tauto)\b", KEYWORD_PSEUDO),
        Rule::token(r"(?m)\b(do|first|idtac|last|repeat|try)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\b([A-Z][\w\']*)", NAME),
        Rule::token(r"(?m)(≥|≤|≠|↔|→|∃|∀|∨|∧|¬|λ|\|\}|\{\||\\/|/\\|=>|~|\}|\|]|\||lp:\{\{|\{<|\{|`|_|]|\[\||\[>|\[<|\[|\?\?|\?|>\}|>]|>|=|<->|<-|<|;;|;|:>|:=|::|:|\.\.|\.|->|-\.|-|,|\+|\*|\)|\(|&&|&|#|!=)", OPERATOR),
        Rule::token(r"(?m)([=<>@^|&+\*/$%-]|[!?~])?[!$%&*+\./:<=>?@^|~-]", OPERATOR),
        Rule::token(r"(?m)[^\W\d][\w']*", NAME),
        Rule::token(r"(?m)\d[\d_]*", NUMBER_INTEGER),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*", NUMBER_HEX),
        Rule::token(r"(?m)0[oO][0-7][0-7_]*", NUMBER_OCT),
        Rule::token(r"(?m)0[bB][01][01_]*", NUMBER_BIN),
        Rule::token(r"(?m)-?\d[\d_]*(.[\d_]*)?([eE][+\-]?\d[\d_]*)", NUMBER_FLOAT),
        Rule::token(r#"(?m)'(?:(\\[\\\"'ntbr ])|(\\[0-9]{3})|(\\x[0-9a-fA-F]{2}))'"#, STRING_CHAR),
        Rule::token(r"(?m)'.'", STRING_CHAR),
        Rule::token(r"(?m)'", KEYWORD),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[~?][a-z][\w\']*:", NAME),
        Rule::token(r"(?m)\S", NAME_BUILTIN_PSEUDO),
    ]);
    m.insert(r"set-options", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)[A-Z]\w*", KEYWORD_NAMESPACE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\d+", NUMBER_INTEGER),
        Rule::token_to(r"(?m)\.", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"sn-notation", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\b(?:via|mapping|abstract|warning|after)\b", KEYWORD),
        Rule::token(r"(?m)=>|[()\[\]:,]", OPERATOR),
        Rule::token(r"(?m)\b[^\W\d][\w\']*(?:\.[^\W\d][\w\']*)*\b", NAME),
        Rule::token(r"(?m)\d[\d_]*", NUMBER_INTEGER),
        Rule::token(r"(?m)0[xX][\da-fA-F][\da-fA-F_]*", NUMBER_HEX),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token_to(r"(?m)\.", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"comment", vec![
        Rule::token(r"(?m)([^(*)]+|\*+(?!\)))+", COMMENT),
        Rule::token_to(r"(?m)\(\*", COMMENT, NewState::PushSame),
        Rule::token_to(r"(?m)\*\)", COMMENT, NewState::Pop(1)),
        Rule::token(r"(?m)[(*)]", COMMENT),
    ]);
    m.insert(r"string", vec![
        Rule::token(r#"(?m)[^"]+"#, STRING_DOUBLE),
        Rule::token(r#"(?m)"""#, STRING_DOUBLE),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Pop(1)),
    ]);
    m.insert(r"dotted", vec![
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r"(?m)\.", PUNCTUATION),
        Rule::token(r"(?m)[A-Z][\w\']*(?=\s*\.)", NAME_NAMESPACE),
        Rule::token_to(r"(?m)[A-Z][\w\']*", NAME_CLASS, NewState::Pop(1)),
        Rule::token_to(r"(?m)[a-z][a-z0-9_\']*", NAME, NewState::Pop(1)),
        Rule::default(NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for CoqLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
