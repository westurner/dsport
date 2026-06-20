#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.configs:SquidConfLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.configs:SquidConfLexer:squidconf

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: squidconf, squid.conf, squid
pub struct SquidconfLexer;

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
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token_to(r"(?im)#", COMMENT, NewState::Push(vec![r"comment"])),
        Rule::token(r"(?im)\b(a(?:c(?:cess_log|l)|lways_direct|n(?:nounce_(?:host|p(?:eriod|ort)|to)|onymize_headers)|ppend_domain|s_whois_server|uth(?:_param_basic|enticate_(?:children|program|ttl)))|b(?:(?:roken_post|uffered_log)s)|c(?:ache(?:_(?:a(?:ccess_log|nnounce)|d(?:ir|ns_program)|effective_(?:group|user)|host(?:(?:_(?:acl|domain))?)|log|m(?:em(?:(?:_(?:high|low))?)|gr)|peer(?:(?:_access)?)|replacement_policy|s(?:to(?:plist(?:(?:_pattern)?)|re_log)|wap(?:(?:_(?:high|lo(?:[gw])))?)))|mgr_passwd)|lient_(?:db|lifetime|netmask)|o(?:nnect_timeout|redump_dir))|d(?:e(?:ad_peer_timeout|bug_options|lay_(?:access|class|initial_bucket_level|p(?:(?:arameter|ool)s))|ny_info)|ns_(?:children|(?:defname|nameserver|testname)s))|e(?:mulate_httpd_log|rr_html_text)|f(?:ake_user_agent|irewall_ip|orward(?:_snmpd_port|ed_for)|qdncache_size|tp(?:_(?:list_width|passive|user)|get_(?:options|program)))|h(?:alf_closed_clients|eader_(?:access|replace)|i(?:erarchy_stoplist|gh_(?:(?:page_fault|response_time)_warning))|osts_file|t(?:cp_port|tp(?:_(?:a(?:ccess|nonymizer)|port|reply_access)|d_accel(?:(?:_(?:host|port|uses_host_header|with_proxy))?))))|i(?:cp_(?:access|hit_stale|(?:por|query_timeou)t)|dent_(?:lookup(?:(?:_access)?)|timeout)|n(?:coming_(?:(?:htt|ic)p_average)|side_firewall)|pcache_(?:high|low|size))|lo(?:cal_(?:domain|ip)|g(?:_(?:fqdn|(?:icp_querie|mime_hdr)s)|file_rotate))|m(?:aximum_(?:object_size|single_addr_tries)|cast_(?:groups|icp_query_timeout|miss_(?:addr|encode_key|port))|emory_(?:pools(?:(?:_limit)?)|replacement_policy)|i(?:me_table|n(?:_(?:(?:htt|ic)p_poll_cnt)|imum_(?:direct_hops|object_size|retry_timeout))|ss_access))|n(?:e(?:gative_(?:(?:(?:dns_)?)ttl)|ighbor_t(?:imeout|ype_domain)|tdb_(?:high|low|ping_(?:period|rate))|ver_direct)|o_cache)|p(?:assthrough_proxy|conn_timeout|i(?:d_filename|nger_program)|ositive_dns_ttl|r(?:efer_direct|oxy_auth(?:(?:_realm)?)))|qu(?:ery_icmp|ick_abort(?:(?:_(?:m(?:ax|in)|pct))?))|r(?:ange_offset_limit|e(?:ad_timeout|direct_(?:children|program|rewrites_host_header)|f(?:erence_age|resh_pattern)|load_into_ims|quest_(?:body_max_size|size|timeout)))|s(?:hutdown_lifetime|i(?:ngle_parent_bypass|teselect_timeout)|nmp_(?:access|incoming_address|port)|ource_ping|sl_proxy|t(?:ore_(?:avg_object_size|objects_per_bucket)|rip_query_terms)|wap_level(?:(?:[12])_dirs))|t(?:cp_(?:incoming_address|outgoing_address|recv_bufsize)|est_reachability)|u(?:dp_(?:hit_obj(?:(?:_size)?)|(?:incom|outgo)ing_address)|n(?:ique_hostname|linkd_program)|ri_whitespace|seragent_log)|visible_hostname|wais_relay(?:(?:_(?:(?:hos|por)t))?))\b", KEYWORD),
        Rule::token(r"(?im)\b(all(?:(?:ow)?)|c(?:hildren|redentialsttl)|d(?:e(?:fault|ny)|is(?:able|kd))|heap|lru|multicast\-responder|no(?:\-(?:digest|query)|ne)|o(?:ff(?:(?:line_toggle)?)|n)|p(?:arent|roxy\-only)|q(?:[12])|r(?:ealm|ound\-robin)|ttl|via|weight)\b", NAME_CONSTANT),
        Rule::token(r"(?im)\b(client_list|info|parameter|s(?:erver_list|hutdown|quid\.conf))\b", STRING),
        Rule::token(r"(?im)stats/(dns|f(?:iledescriptors|qdncache)|i(?:o|pcache)|netdb|objects|re(?:director|ply_headers)|utilization|vm_objects)\b", STRING),
        Rule::token(r"(?im)log/(clear|disable|enable|status)=", STRING),
        Rule::token(r"(?im)\b(browser|dst(?:(?:domain)?)|ident|method|p(?:ort|roto)|re(?:ferer_regex|(?:[pq])_mime_type)|s(?:nmp_community|rc)|time|u(?:rl(?:(?:(?:path)?)_regex)|ser))\b", KEYWORD),
        Rule::token(r"(?im)(((\d+|0x[0-9a-f]+)(\.(\d+|0x[0-9a-f]+)){3})|(([0-9a-f]{0,4})(:([0-9a-f]{0,4})){1,7}))(/((((\d+|0x[0-9a-f]+)(\.(\d+|0x[0-9a-f]+)){3})|(([0-9a-f]{0,4})(:([0-9a-f]{0,4})){1,7}))|\d+))?", NUMBER_FLOAT),
        Rule::token(r"(?im)(?:\b\d+\b(?:-\b\d+|%)?)", NUMBER),
        Rule::token(r"(?im)\S+", TEXT),
    ]);
    m.insert(
        r"comment",
        vec![
            Rule::token_to(r"(?im)\s*TAG:.*", STRING_ESCAPE, NewState::Pop(1)),
            Rule::token_to(r"(?im).+", COMMENT, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for SquidconfLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
