#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.ldap:LdaprcLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.ldap:LdaprcLexer:ldapconf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: ldapconf, ldaprc
pub struct LdapconfLexer;

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
        Rule::token(r"(?im)#.*", COMMENT_SINGLE),
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::bygroups(r"(?im)(GSSAPI_(?:ALLOW_REMOTE_PRINCIPAL|ENCRYPT|SIGN)|REFERRALS|SASL_NOCANON)(\s+)(on|true|yes|off|false|no)$", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(KEEPALIVE_(?:IDLE|PROBES|INTERVAL)|NETWORK_TIMEOUT|PORT|SIZELIMIT|TIMELIMIT|TIMEOUT)(\s+)(\d+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?im)(VERSION)(\s+)(2|3)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?im)(DEREF)(\s+)(never|searching|finding|always)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(SASL_SECPROPS)(\s+)((?:none|noanonymous|noplain|noactive|nodict|forwardsec|passcred|(?:minssf|maxssf|maxbufsize)=\d+)(?:,none|noanonymous|noplain|noactive|nodict|forwardsec|passcred|(?:minssf|maxssf|maxbufsize)=\d+)*)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(SASL_CBINDING)(\s+)(none|tls-unique|tls-endpoint)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(TLS_REQ(?:CERT|SAN))(\s+)(allow|demand|hard|never|try)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(TLS_CRLCHECK)(\s+)(none|peer|all)", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD_CONSTANT)]),
        Rule::bygroups(r"(?im)(BASE|BINDDN)(\s+)(\S+)$", vec![Some(KEYWORD), Some(WHITESPACE), Some(LITERAL)]),
        Rule::bygroups(r"(?im)(HOST)(\s+)([a-z0-9]+)((?::(\d+))?)", vec![Some(KEYWORD), Some(WHITESPACE), Some(LITERAL), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?im)((?:URI|SOCKET_BIND_ADDRESSES|SASL_(?:MECH|REALM|AUTHCID|AUTHZID|CBINDING)|TLS_(?:CACERT|CACERTDIR|CERT|ECNAME|KEY|CIPHER_SUITE|PROTOCOL_MIN|RANDFILE|CRLFILE)))(\s+)(\S+)$", vec![Some(KEYWORD), Some(WHITESPACE), Some(LITERAL)]),
    ]);
    Table(m)
}

impl Lexer for LdapconfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
