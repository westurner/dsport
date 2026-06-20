#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sql:MySqlLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:MySqlLexer:mysql

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: mysql
pub struct MysqlLexer;

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
        Rule::token(r"(?im)(?:#|--\s+).*", COMMENT_SINGLE),
        Rule::token_to(r"(?im)/\*\+", COMMENT_SPECIAL, NewState::Push(vec![r"optimizer-hints"])),
        Rule::token_to(r"(?im)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"multiline-comment"])),
        Rule::token(r"(?im)x'([0-9a-f]{2})+'", NUMBER_HEX),
        Rule::token(r"(?im)0x[0-9a-f]+", NUMBER_HEX),
        Rule::token(r"(?im)b'[01]+'", NUMBER_BIN),
        Rule::token(r"(?im)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?im)[0-9]+\.[0-9]*(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]*\.[0-9]+(e[+-]?[0-9]+)?", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+e[+-]?[0-9]+", NUMBER_FLOAT),
        Rule::token(r"(?im)[0-9]+(?=[^0-9a-z$_\u0080-\uffff])", NUMBER_INTEGER),
        Rule::token(r#"(?im)\{\s*d\s*(?P<quote>['\"])\s*\d{2}(\d{2})?.?\d{2}.?\d{2}\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token(r#"(?im)\{\s*t\s*(?P<quote>['\"])\s*(?:\d+\s+)?\d{1,2}.?\d{1,2}.?\d{1,2}(\.\d*)?\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token(r#"(?im)\{\s*ts\s*(?P<quote>['\"])\s*\d{2}(?:\d{2})?.?\d{2}.?\d{2}\s+\d{1,2}.?\d{1,2}.?\d{1,2}(\.\d*)?\s*(?P=quote)\s*\}"#, LITERAL_DATE),
        Rule::token_to(r"(?im)'", STRING_SINGLE, NewState::Push(vec![r"single-quoted-string"])),
        Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Push(vec![r"double-quoted-string"])),
        Rule::token(r"(?im)@@(?:global\.|persist\.|persist_only\.|session\.)?[a-z_]+", NAME_VARIABLE),
        Rule::token(r"(?im)@[a-z0-9_$.]+", NAME_VARIABLE),
        Rule::token_to(r"(?im)@'", NAME_VARIABLE, NewState::Push(vec![r"single-quoted-variable"])),
        Rule::token_to(r#"(?im)@""#, NAME_VARIABLE, NewState::Push(vec![r"double-quoted-variable"])),
        Rule::token_to(r"(?im)@`", NAME_VARIABLE, NewState::Push(vec![r"backtick-quoted-variable"])),
        Rule::token(r"(?im)\?", NAME_VARIABLE),
        Rule::token(r"(?im)[!%&*+/:<=>^|~-]+", OPERATOR),
        Rule::token(r"(?im)\b(set)(?!\s*\()", KEYWORD),
        Rule::bygroups(r"(?im)\b(character)(\s+)(set)\b", vec![Some(KEYWORD), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::token(r"(?im)\b(false|null|true|unknown)\b", NAME_CONSTANT),
        Rule::token(r"(?im)\b(b(?:i(?:gint|nary|t)|lob|ool(?:(?:ean)?)|yte)|char|d(?:ate(?:(?:time)?)|ec(?:(?:imal)?)|ouble)|enum|f(?:ixed|loat(?:(?:[48])?))|geometry(?:(?:collection)?)|int(?:(?:eger|[12348])?)|json|l(?:inestring|ong(?:(?:blob|text)?))|m(?:edium(?:blob|(?:in|tex)t)|iddleint|ulti(?:linestring|po(?:int|lygon)))|n(?:ational|char|umeric|varchar)|p(?:o(?:int|lygon)|recision)|real|s(?:e(?:rial|t)|mallint)|t(?:ext|i(?:me(?:(?:stamp)?)|ny(?:blob|(?:in|tex)t)))|var(?:binary|char(?:(?:acter)?)|ying)|year)\b", KEYWORD_TYPE),
        Rule::token(r"(?im)\b(a(?:bsent|c(?:c(?:essible|ount)|ti(?:on|ve))|d(?:d|min)|fter|g(?:ainst|gregate)|l(?:gorithm|l(?:(?:ow_missing_files)?)|ter|ways)|n(?:alyze|[dy])|rray|s(?:c(?:(?:ii)?)|ensitive|sign_gtids_to_anonymous_transactions)|ttribute|ut(?:hentication|o(?:(?:_(?:increment|refresh(?:(?:_source)?))|extend_size)?))|vg(?:(?:_row_length)?)|[st])|b(?:ackup|e(?:fore|gin|rnoulli|tween)|inlog|lock|oth|tree|u(?:ckets|lk)|y)|c(?:a(?:che|ll|s(?:cade(?:(?:d)?)|e)|talog_name)|h(?:a(?:in|llenge_response|n(?:ge(?:(?:d)?)|nel)|r(?:acter|set))|eck(?:(?:sum)?))|ipher|l(?:ass_origin|ient|o(?:(?:[ns])e))|o(?:alesce|de|l(?:lat(?:e|ion)|umn(?:(?:_(?:format|name)|s)?))|m(?:m(?:ent|it(?:(?:ted)?))|p(?:act|letion|onent|ress(?:ed|ion)))|n(?:current|dition|nection|s(?:istent|traint(?:(?:_(?:catalog|name|schema))?))|t(?:ains|ext|inue)|vert))|pu|r(?:eate|oss)|u(?:be|me_dist|r(?:rent(?:(?:_(?:date|time(?:(?:stamp)?)|user))?)|sor(?:(?:_name)?))))|d(?:a(?:ta(?:(?:base(?:(?:s)?)|file)?)|y(?:(?:_(?:hour|mi(?:crosecond|nute)|second))?))|e(?:allocate|clare|f(?:ault(?:(?:_auth)?)|in(?:er|ition))|l(?:ay(?:_key_write|ed)|ete)|nse_rank|sc(?:(?:ri(?:be|ption))?)|terministic)|i(?:agnostics|rectory|s(?:able|card|k|tinct(?:(?:row)?))|v)|o|rop|u(?:al(?:(?:ity)?)|(?:mpfil|plicat)e)|ynamic)|e(?:ach|lse(?:(?:if)?)|mpty|n(?:able|c(?:losed|ryption)|d(?:(?:s)?)|forced|gine(?:(?:_attribute|s)?))|rror(?:(?:s)?)|scape(?:(?:d)?)|ve(?:nt(?:(?:s)?)|ry)|x(?:c(?:ept|(?:hang|lud)e)|ecute|i(?:sts|t)|p(?:ansion|ire|lain|ort)|te(?:n(?:ded|t_size)|rnal(?:(?:_format)?))))|f(?:a(?:ctor|iled_login_attempts|lse|st|ults)|etch|i(?:elds|l(?:e(?:(?:_(?:block_size|format|name|p(?:attern|refix))|s)?)|ter)|nish|rst(?:(?:_value)?))|lush|o(?:llow(?:ing|s)|r(?:(?:ce|eign|mat)?)|und)|rom|u(?:ll(?:(?:text)?)|nction))|g(?:e(?:nera(?:l|te(?:(?:d)?))|omcollection|t(?:(?:_(?:format|(?:master|source)_public_key))?))|lobal|r(?:ant(?:(?:s)?)|oup(?:(?:_replication|ing|s)?))|tid(?:_only|s))|h(?:a(?:ndler|sh|ving)|e(?:ader|lp)|i(?:gh_priority|sto(?:gram|ry))|o(?:st(?:(?:s)?)|ur(?:(?:_(?:mi(?:crosecond|nute)|second))?)))|i(?:dentified|gnore(?:(?:_server_ids)?)|mport|n(?:active|dex(?:(?:es)?)|file|itia(?:l(?:(?:_size)?)|te)|ner|out|s(?:e(?:nsitive|rt(?:(?:_method)?))|ta(?:ll|nce))|t(?:er(?:sect|val)|o)|v(?:isible|oker))|o_(?:after_gtids|before_gtids|thread)|pc|s(?:olation|suer)|terate|[fnos])|j(?:oin|son_(?:(?:tabl|valu)e))|k(?:ey(?:(?:_block_size|ring|s)?)|ill)|l(?:a(?:g|nguage|st(?:(?:_value)?)|teral)|e(?:a(?:d(?:(?:ing)?)|ve(?:(?:s)?))|ft|ss|vel)|i(?:brary|ke|mit|ne(?:ar|s)|st)|o(?:ad|c(?:al(?:(?:time(?:(?:stamp)?))?)|k(?:(?:ed|s)?))|g(?:(?:file|s)?)|op|w_priority))|m(?:a(?:nual|ster(?:(?:_(?:auto_position|bind|co(?:mpression_algorithms|nnect_retry)|delay|h(?:eartbeat_period|ost)|log_(?:file|pos)|p(?:assword|ort|ublic_key_path)|retry_count|ssl(?:(?:_(?:c(?:a(?:(?:path)?)|ert|ipher|rl(?:(?:path)?))|key|verify_server_cert))?)|tls_(?:ciphersuites|version)|user|zstd_compression_level))?)|tch|x(?:_(?:connections_per_hour|queries_per_hour|rows|size|u(?:pdates_per_hour|ser_connections))|value))|e(?:dium|m(?:ber|ory)|rge|ssage_text)|i(?:crosecond|grate|n(?:_rows|ute(?:(?:_(?:(?:(?:micro)?)second))?)))|o(?:d(?:(?:e|if(?:ies|y))?)|nth)|utex|ysql_errno)|n(?:a(?:me(?:(?:s)?)|tural)|db(?:(?:cluster)?)|e(?:sted|twork_namespace|ver|w|xt)|o(?:(?:_w(?:ait|rite_to_binlog)|degroup|ne|(?:(?:wai)?)t)?)|t(?:(?:h_valu|il)e)|u(?:ll(?:(?:s)?)|mber))|o(?:ff(?:(?:set)?)|ld|n(?:e|ly)|p(?:en|ti(?:mize(?:(?:r_costs)?)|on(?:(?:al(?:(?:ly)?)|s)?)))|r(?:d(?:er|inality)|ganization)|thers|ut(?:(?:er|file)?)|(?:v|wn)er|[fjnr])|p(?:a(?:ck_keys|ge|r(?:a(?:llel|meters)|se(?:_tree|r)|ti(?:al|tion(?:(?:ing|s)?)))|ssword(?:(?:_lock_time)?)|th)|er(?:cent_rank|sist(?:(?:_only)?))|hase|lugin(?:(?:_dir|s)?)|ort|r(?:e(?:ced(?:es|ing)|pare|serve|v)|i(?:mary|vilege(?:_checks_user|s))|o(?:ce(?:dure|ss(?:(?:list)?))|file(?:(?:s)?)|xy))|urge)|qu(?:a(?:lify|rter)|ery|ick)|r(?:an(?:dom|ge|k)|e(?:ad(?:(?:_(?:only|write)|s)?)|build|c(?:over|ursive)|d(?:o_buffer_size|undant)|ference(?:(?:s)?)|g(?:exp|istration)|l(?:a(?:tional|y(?:(?:_(?:log_(?:file|pos)|thread)|log)?))|ease|oad)|move|name|organize|p(?:air|eat(?:(?:able)?)|l(?:ace|ica(?:(?:s|t(?:e_(?:do_(?:db|table)|ignore_(?:db|table)|rewrite_db|wild_(?:(?:do|ignore)_table))|ion))?)))|quire(?:(?:_(?:row_format|table_primary_key_check))?)|s(?:et|ignal|ource|pect|t(?:art|ore|rict)|ume)|t(?:ain|urn(?:(?:ed_sqlstate|ing|s)?))|(?:us|v(?:ers|ok))e)|ight|like|o(?:l(?:e|l(?:back|up))|tate|utine|w(?:(?:_(?:count|format|number)|s)?))|tree)|s(?:3|avepoint|che(?:dule|ma(?:(?:_name|s)?))|e(?:c(?:ond(?:(?:_microsecond|ary(?:(?:_(?:engine(?:(?:_attribute)?)|(?:(?:un)?)load))?))?)|urity)|lect|nsitive|parator|r(?:ializable|ver)|ssion)|h(?:are|ow|utdown)|i(?:gn(?:al|ed)|mple)|kip|l(?:ave|ow)|napshot|o(?:cket|me|name|u(?:nds|rce(?:(?:_(?:auto_position|bind|co(?:mpression_algorithms|nnect(?:_retry|ion_auto_failover))|delay|h(?:eartbeat_period|ost)|log_(?:file|pos)|p(?:assword|ort|ublic_key_path)|retry_count|ssl(?:(?:_(?:c(?:a(?:(?:path)?)|ert|ipher|rl(?:(?:path)?))|key|verify_server_cert))?)|tls_(?:ciphersuites|version)|user|zstd_compression_level))?)))|p(?:atial|ecific)|ql(?:(?:_(?:after_(?:(?:gtid|mts_gap)s)|b(?:efore_gtids|(?:ig|uffer)_result)|calc_found_rows|no_cache|small_result|t(?:hread|si_(?:day|hour|m(?:inute|onth)|quarter|second|week|year)))|exception|state|warning)?)|rid|sl|t(?:a(?:cked|rt(?:(?:ing|s)?)|t(?:s_(?:auto_recalc|persistent|sample_pages)|us))|o(?:p|r(?:age|ed))|r(?:aight_join|eam|i(?:ct_load|ng)))|u(?:b(?:class_origin|ject|partition(?:(?:s)?))|per|spend)|w(?:(?:ap|itche)s)|ystem)|t(?:able(?:(?:_(?:checksum|name)|s(?:(?:(?:ampl|pac)e)?))?)|e(?:mp(?:orary|table)|rminated)|h(?:an|en|read_priority)|i(?:es|mestamp(?:add|diff))|ls|o|r(?:a(?:iling|nsaction)|igger(?:(?:s)?)|u(?:(?:(?:ncat)?)e))|ype(?:(?:s)?))|u(?:n(?:bounded|committed|d(?:efined|o(?:(?:(?:_buffer_siz|fil)e)?))|i(?:code|nstall|on|que)|known|lock|register|signed|til)|p(?:(?:dat|grad)e)|r(?:[il])|s(?:age|e(?:(?:_frm|r(?:(?:_resources)?))?)|ing)|tc_(?:date|time(?:(?:stamp)?)))|v(?:a(?:l(?:idation|ue(?:(?:s)?))|riables)|cpu|e(?:ctor|rify_key_constraints)|i(?:ew|rtual|sible))|w(?:a(?:it|rnings)|e(?:ek|ight_string)|h(?:e(?:n|re)|ile)|i(?:ndow|th(?:(?:out)?))|ork|r(?:apper|ite))|x(?:509|a|id|ml|or)|year_month|z(?:erofill|one))\b", KEYWORD),
        Rule::bygroups(r"(?im)\b(a(?:bs|cos|dd(?:(?:dat|tim)e)|es_(?:(?:de|en)crypt)|ny_value|sin|tan(?:(?:2)?))|b(?:enchmark|i(?:n(?:(?:_to_uuid)?)|t_(?:and|count|length|(?:(?:x)?)or)))|c(?:a(?:n_access_(?:column|database|event|r(?:esource_group|outine)|t(?:able|rigger)|user|view)|st)|eil(?:(?:ing)?)|har(?:(?:(?:acter)?)_length)|o(?:ercibility|mpress|n(?:cat(?:(?:_ws)?)|nection_id|v(?:(?:ert_(?:cpu_id_mask|interval_to_user_interval|tz))?))|unt|[st])|rc32|ur(?:(?:dat|rent_rol|tim)e))|d(?:a(?:te(?:_(?:add|format|sub)|diff)|y(?:name|of(?:month|week|year)))|egrees)|e(?:lt|tag|x(?:p(?:(?:ort_set)?)|tract(?:(?:value)?)))|f(?:i(?:eld|nd_in_set)|loor|o(?:rmat_(?:bytes|pico_time)|und_rows)|rom_(?:base64|days|unixtime|vector))|g(?:et_(?:dd_(?:c(?:(?:olumn_privilege|reate_option)s)|index_(?:private_data|sub_part_length)|property_key_value|schema_options|tablespace_private_data)|jdv_property_key_value|lock)|(?:r(?:eates|oup_conca)|tid_sub(?:se|trac))t)|hex|i(?:cu_version|fnull|n(?:et(?:6_(?:aton|ntoa)|_(?:aton|ntoa))|str|ternal_(?:a(?:uto_increment|vg_row_length)|check(?:_time|sum)|d(?:ata_(?:free|length)|d_char_length)|get_(?:comment_or_error|dd_column_extra|enabled_role_json|hostname|mandatory_roles_json|partition_nodegroup|username|view_warning_or_error)|i(?:ndex_(?:column_cardinality|length)|s_(?:(?:enabled|mandatory)_role))|keys_disabled|max_data_length|table(?:_rows|space_(?:autoextend_size|data_free|ext(?:ent_size|ra)|free_extents|i(?:d|nitial_size)|logfile_group_n(?:ame|umber)|maximum_size|row_format|status|t(?:otal_extents|ype)|version))|u(?:pdate_time|se_terminology_previous)))|s(?:_(?:free_lock|ipv(?:4_(?:compat|mapped)|[46])|u(?:sed_lock|uid)|visible_dd_object)|null))|json_(?:array(?:(?:_(?:append|insert)|agg)?)|contains(?:(?:_path)?)|d(?:epth|uality_object)|extract|insert|keys|length|merge(?:(?:_p(?:atch|reserve))?)|o(?:bject(?:(?:agg)?)|verlaps)|pretty|quote|re(?:(?:mov|plac)e)|s(?:chema_valid(?:(?:ation_report)?)|e(?:arch|t)|torage_(?:(?:fre|siz)e))|type|unquote|valid)|l(?:ast_(?:day|insert_id)|case|e(?:ast|ngth)|ike_range_m(?:ax|in)|n|o(?:ad_file|cate|g(?:(?:10|2)?)|wer)|pad|trim)|m(?:a(?:ke(?:_set|(?:dat|tim)e)|ster_pos_wait|x)|br(?:co(?:ntains|ver(?:edby|s))|disjoint|equals|intersects|overlaps|touches|within)|d5|i(?:[dn])|onthname)|n(?:ame_const|ow|ullif)|o(?:ct(?:(?:et_length)?)|rd)|p(?:eriod_(?:add|diff)|i|o(?:sition|w(?:(?:er)?))|s_(?:(?:(?:current_)?)thread_id))|quote|r(?:a(?:dians|nd(?:(?:om_bytes)?))|e(?:gexp_(?:instr|like|replace|substr)|lease_(?:all_locks|lock)|move_dd_property_key|verse)|o(?:les_graphml|und)|pad|trim)|s(?:e(?:c_to_time|ssion_user)|ha(?:(?:[12])?)|i(?:(?:(?:g)?)n)|leep|ou(?:ndex|rce_pos_wait)|pace|qrt|t(?:_(?:a(?:rea|s(?:binary|geojson|text|wk(?:[bt])))|buffer(?:(?:_strategy)?)|c(?:entroid|o(?:llect|n(?:tains|vexhull))|rosses)|di(?:fference|mension|s(?:joint|tance(?:(?:_sphere)?)))|e(?:n(?:dpoint|velope)|quals|xteriorring)|frechetdistance|geo(?:hash|m(?:collfrom(?:t(?:(?:(?:e)?)xt)|wkb)|etry(?:collectionfrom(?:text|wkb)|from(?:text|wkb)|n|type)|from(?:geojson|text|wkb)))|hausdorffdistance|i(?:nter(?:iorringn|sect(?:ion|s))|s(?:closed|empty|simple|valid))|l(?:at(?:fromgeohash|itude)|ength|ine(?:from(?:text|wkb)|interpolatepoint(?:(?:s)?)|stringfrom(?:text|wkb))|ong(?:fromgeohash|itude))|m(?:akeenvelope|linefrom(?:text|wkb)|po(?:intfrom(?:text|wkb)|lyfrom(?:text|wkb))|ulti(?:linestringfrom(?:text|wkb)|po(?:intfrom(?:text|wkb)|lygonfrom(?:text|wkb))))|num(?:geometries|interiorring(?:(?:s)?)|points)|overlaps|po(?:int(?:atdistance|from(?:geohash|text|wkb)|n)|ly(?:from(?:text|wkb)|gonfrom(?:text|wkb)))|s(?:implify|rid|tartpoint|wapxy|ymdifference)|t(?:ouches|ransform)|union|validate|within|[xy])|atement_digest(?:(?:_text)?)|d(?:(?:dev(?:(?:_(?:(?:po|sam)p))?))?)|r(?:_to_date|cmp|ing_to_vector))|u(?:b(?:date|str(?:(?:ing(?:(?:_index)?))?)|time)|m)|ys(?:date|tem_user))|t(?:an|ime(?:_(?:format|to_sec)|diff)|o_(?:base64|days|seconds|vector)|rim)|u(?:case|n(?:compress(?:(?:ed_length)?)|hex|ix_timestamp)|p(?:datexml|per)|uid(?:(?:_(?:short|to_bin))?))|v(?:a(?:lidate_password_strength|r(?:_(?:(?:po|sam)p)|iance))|e(?:ctor_(?:dim|to_string)|rsion))|w(?:ait_(?:for_executed_gtid_set|until_sql_thread_after_gtids)|eek(?:day|ofyear))|yearweek)\b(\s*)(\()", vec![Some(NAME_FUNCTION), Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::token(r"(?im)[0-9a-z$_-￿]+", NAME),
        Rule::token_to(r"(?im)`", TokenType::new(&["Name", "Quoted"]), NewState::Push(vec![r"schema-object-name"])),
        Rule::token(r"(?im)[(),.;]", PUNCTUATION),
    ]);
    m.insert(r"optimizer-hints", vec![
        Rule::token(r"(?im)[^*a-z]+", COMMENT_SPECIAL),
        Rule::token_to(r"(?im)\*/", COMMENT_SPECIAL, NewState::Pop(1)),
        Rule::token(r"(?im)(b(?:ka|nl)|d(?:erived_condition_pushdown|upsweedout)|firstmatch|group_index|hash_join|in(?:dex(?:(?:_merge)?)|toexists)|join_(?:fixed_order|index|order|(?:pre|suf)fix)|loosescan|m(?:a(?:terialization|x_execution_time)|erge|rr)|no_(?:b(?:ka|nl)|derived_condition_pushdown|group_index|hash_join|i(?:cp|ndex(?:(?:_merge)?))|join_index|m(?:erge|rr)|order_index|(?:range_optimizatio|s(?:emijoi|kip_sca))n)|order_index|qb_name|resource_group|s(?:e(?:mijoin|t_var)|kip_scan|ubquery))\b", COMMENT_PREPROC),
        Rule::token(r"(?im)[a-z]+", COMMENT_SPECIAL),
        Rule::token(r"(?im)\*", COMMENT_SPECIAL),
    ]);
    m.insert(
        r"multiline-comment",
        vec![
            Rule::token(r"(?im)[^*]+", COMMENT_MULTILINE),
            Rule::token_to(r"(?im)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?im)\*", COMMENT_MULTILINE),
        ],
    );
    m.insert(
        r"single-quoted-string",
        vec![
            Rule::token(r"(?im)[^'\\]+", STRING_SINGLE),
            Rule::token(r"(?im)''", STRING_ESCAPE),
            Rule::token(r#"(?im)\\[0'"bnrtZ\\%_]"#, STRING_ESCAPE),
            Rule::token_to(r"(?im)'", STRING_SINGLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"double-quoted-string",
        vec![
            Rule::token(r#"(?im)[^"\\]+"#, STRING_DOUBLE),
            Rule::token(r#"(?im)"""#, STRING_ESCAPE),
            Rule::token(r#"(?im)\\[0'"bnrtZ\\%_]"#, STRING_ESCAPE),
            Rule::token_to(r#"(?im)""#, STRING_DOUBLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"single-quoted-variable",
        vec![
            Rule::token(r"(?im)[^']+", NAME_VARIABLE),
            Rule::token(r"(?im)''", NAME_VARIABLE),
            Rule::token_to(r"(?im)'", NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"double-quoted-variable",
        vec![
            Rule::token(r#"(?im)[^"]+"#, NAME_VARIABLE),
            Rule::token(r#"(?im)"""#, NAME_VARIABLE),
            Rule::token_to(r#"(?im)""#, NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"backtick-quoted-variable",
        vec![
            Rule::token(r"(?im)[^`]+", NAME_VARIABLE),
            Rule::token(r"(?im)``", NAME_VARIABLE),
            Rule::token_to(r"(?im)`", NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"schema-object-name",
        vec![
            Rule::token(r"(?im)[^`]+", TokenType::new(&["Name", "Quoted"])),
            Rule::token(r"(?im)``", TokenType::new(&["Name", "Quoted", "Escape"])),
            Rule::token_to(
                r"(?im)`",
                TokenType::new(&["Name", "Quoted"]),
                NewState::Pop(1),
            ),
        ],
    );
    Table(m)
}

impl Lexer for MysqlLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
