#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ride:RideLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ride:RideLexer:ride

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ride
pub struct RideLexer;

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
        Rule::token(r"(?m)#.*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"doublequote"])),
        Rule::token_to(r"(?m)utf8\'", STRING, NewState::Push(vec![r"utf8quote"])),
        Rule::token_to(r"(?m)base(58|64|16)\'", STRING, NewState::Push(vec![r"singlequote"])),
        Rule::token(r"(?m)(@(?:Callable|Verifier)|case|else|func|if|let|match|then)\b", KEYWORD_RESERVED),
        Rule::token(r"(?m)\{-#.*?#-\}", KEYWORD_RESERVED),
        Rule::token(r"(?m)FOLD<\d+>", KEYWORD_RESERVED),
        Rule::token(r"(?m)(A(?:ddress|lias|sset(?:(?:Pair)?)|ttachedPayment)|B(?:inaryEntry|lockInfo|oolean(?:(?:Entry)?)|urn(?:(?:Transaction)?)|yteVector)|C(?:eiling|reateAliasTransaction)|D(?:ata(?:Entry|Transaction)|own)|ExchangeTransaction|Floor|GenesisTransaction|Half(?:Down|Even|Up)|I(?:n(?:t(?:(?:egerEntry)?)|vo(?:(?:ca|keScriptTransac)tion))|ssue(?:(?:Transaction)?))|L(?:ease(?:(?:(?:Cancel)?)Transaction)|ist)|M(?:assTransferTransaction|d5)|NoAlg|Order|PaymentTransaction|Reissue(?:(?:Transaction)?)|S(?:cript(?:Result|Transfer)|et(?:(?:(?:Asset)?)ScriptTransaction)|ha(?:1|2(?:24|56)|3(?:2(?:24|56)|384|512|84)|512)|ponsorFeeTransaction|tring(?:(?:Entry)?))|Trans(?:action|fer(?:(?:Set|Transaction)?))|U(?:nit|p)|WriteSet)", KEYWORD_TYPE),
        Rule::token(r"(?m)\((!=|\&\&|\+\+|:(?:[+:])|<=|=(?:[=>])|>=|\|\||[!%*+\-./:<=>|])\)", NAME_FUNCTION),
        Rule::token(r"(?m)(!=|\&\&|\+\+|:(?:[+:])|<=|=(?:[=>])|>=|\|\||[!%*+\-./:<=>|])", NAME_FUNCTION),
        Rule::token(r"(?m)(Buy|CEILING|DOWN|FLOOR|HALF(?:DOWN|EVEN|UP)|MD5|NOALG|S(?:HA(?:1|2(?:24|56)|3(?:2(?:24|56)|384|512|84)|512)|ell)|UP|height|lastBlock|nil|this|unit)", NAME_FUNCTION),
        Rule::token(r"(?m)(a(?:ddressFrom(?:PublicKey|Recipient|String(?:(?:Value)?))|sset(?:Balance|Info))|bl(?:ake2b256|ockInfoByHeight)|c(?:heckMerkleProof|on(?:(?:(?:tain)?)s))|drop(?:(?:Right)?)|extract|fr(?:action|omBase(?:(?:16|58|64)String))|get(?:B(?:inary(?:(?:Value)?)|oolean(?:(?:Value)?))|Element|Integer(?:(?:Value)?)|String(?:(?:Value)?))|i(?:ndexOf|sDefined)|keccak256|l(?:astIndexOf|og)|median|p(?:arseInt(?:(?:Value)?)|ow)|rsaVerify|s(?:ha256|i(?:gVerify|ze)|plit)|t(?:ake(?:(?:Right)?)|hrow|o(?:B(?:ase(?:(?:16|58|64)String)|ytes)|Int|(?:(?:Utf8)?)String)|rans(?:(?:action(?:(?:Height)?)|ferTransaction)ById))|(?:valu(?:(?:eOrE(?:ls|rrorMessag))?)|wavesBalanc)e)", NAME_FUNCTION),
        Rule::token(r"(?m)_?\d+", NUMBER_INTEGER),
        Rule::token(r"(?m)[a-zA-Z_][a-zA-Z0-9_\']*", NAME_VARIABLE),
        Rule::token(r"(?m)[,()\[\]{}]", PUNCTUATION),
    ]);
    m.insert(r"numbers", vec![Rule::token(r"(?m)_?\d+", NUMBER_INTEGER)]);
    m.insert(
        r"doublequote",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r#"(?m)\\[nrfvb\\"]"#, STRING_ESCAPE),
            Rule::token(r#"(?m)[^"]"#, STRING),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"utf8quote",
        vec![
            Rule::token(r"(?m)\\u[0-9a-fA-F]{4}", STRING_ESCAPE),
            Rule::token(r"(?m)\\[nrfvb\\\']", STRING_ESCAPE),
            Rule::token(r"(?m)[^\']", STRING),
            Rule::token_to(r"(?m)\'", STRING, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"singlequote",
        vec![
            Rule::token(r"(?m)[^\']", STRING),
            Rule::token_to(r"(?m)\'", STRING, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for RideLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
