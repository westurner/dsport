#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.sql:PostgresExplainLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.sql:PostgresExplainLexer:postgres_explain

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: postgres-explain
pub struct PostgresExplainLexer;

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
        Rule::token(r"(?m)(:|\(|\)|ms|kB|->|\.\.|\,|\/|=|%)", PUNCTUATION),
        Rule::token(r"(?m)(\s+)", WHITESPACE),
        Rule::bygroups_to(r"(?m)(cost)(=?)", vec![Some(NAME_CLASS), Some(PUNCTUATION)], NewState::Push(vec![r"instrumentation"])),
        Rule::bygroups_to(r"(?m)(actual)( )(=?)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"instrumentation"])),
        Rule::token(r"(?m)(B(?:(?:atche|ucket)s)|Disk\ (?:(?:Maximum\ Stor|Us)age)|E(?:(?:stimate|viction)s)|Hits|Index\ Searches|M(?:emory(?:(?:\ Usage)?)|isses)|Overflows|Planned\ Partitions|Storage|actual|capacity|distinct\ keys|hit\ percent|lookups|originally|row(?:(?:s)?))\b", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(hit|read|dirtied|written|write|time|calls)(=)", vec![Some(COMMENT_SINGLE), Some(OPERATOR)]),
        Rule::token(r"(?m)(shared|temp|local)", KEYWORD_PSEUDO),
        Rule::bygroups_to(r"(?m)(Sort Method)(: )", vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)], NewState::Push(vec![r"sort"])),
        Rule::bygroups_to(r"(?m)(Sort Key|Group Key|Presorted Key|Hash Key)(:)( )", vec![Some(COMMENT_PREPROC), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"object_name"])),
        Rule::bygroups_to(r"(?m)(Cache Key|Cache Mode)(:)( )", vec![Some(COMMENT), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"object_name"])),
        Rule::token_to(r"(?m)(Disabled|F(?:ilter|unction\ Call)|H(?:ash\ Cond|eap\ Blocks)|In(?:dex\ Cond|ner\ Unique)|Join\ Filter|Merge\ Cond|O(?:ne\-Time\ Filter|rder\ By|utput)|Params\ Evaluated|R(?:e(?:check\ Cond|lations|mote\ SQL)|un\ Condition)|S(?:ampling|ingle\ Copy|ubplans\ Removed)|T(?:ID\ Cond|able\ Function\ Call))\b", COMMENT_PREPROC, NewState::Push(vec![r"predicate"])),
        Rule::token_to(r"(?m)Conflict ", COMMENT_PREPROC, NewState::Push(vec![r"conflict"])),
        Rule::bygroups_to(r"(?m)(InitPlan|SubPlan)( )(\d+)( )", vec![Some(KEYWORD), Some(WHITESPACE), Some(NUMBER_INTEGER), Some(WHITESPACE)], NewState::Push(vec![r"init_plan"])),
        Rule::token(r"(?m)(Buffers|Execution\ (?:(?:[Tt])ime)|Full\-sort\ Groups|Join\ Filter|P(?:lanning(?:(?:\ (?:(?:[Tt])ime))?)|re\-sorted\ Groups)|Query\ Identifier|Sort\ Method|Time|Worker(?:(?:s\ (?:(?:Launch|Plann)ed))?))\b", COMMENT_PREPROC),
        Rule::token(r"(?m)(Heap\ Fetches|Rows\ Removed\ by\ (?:Filter|Index\ Recheck|Join\ Filter)|never\ executed)\b", NAME_EXCEPTION),
        Rule::bygroups(r"(?m)(I/O Timings)(:)( )", vec![Some(NAME_EXCEPTION), Some(PUNCTUATION), Some(WHITESPACE)]),
        Rule::token(r"(?m)(A(?:ggregate|ppend)|Bitmap(?:\ (?:(?:Heap|Index)\ Scan)|And|Or)|C(?:(?:TE|ustom)\ Scan)|Delete|F(?:(?:oreig|unctio)n\ Scan)|G(?:ather(?:(?:\ Merge)?)|roup(?:(?:Aggregate)?))|Hash(?:(?:\ Join|Aggregate)?)|In(?:cremental\ Sort|dex\ (?:(?:(?:Only\ )?)Scan)|sert)|L(?:imit|ockRows)|M(?:aterialize|e(?:moize|rge(?:(?:\ (?:Append|Join))?))|ixedAggregate)|N(?:amed\ Tuplestore\ Scan|ested\ Loop)|ProjectSet|Re(?:cursive\ Union|sult)|S(?:ample\ Scan|e(?:q\ Scan|tOp)|ort|ub(?:(?:Pl|query\ Sc)an))|T(?:(?:able\ Function|id(?:(?:\ Range)?))\ Scan)|U(?:(?:niqu|pdat)e)|Values\ Scan|W(?:indowAgg|orkTable\ Scan))\b", KEYWORD),
        Rule::token(r"(?m)((Right|Left|Full|Semi|Anti) Join)", KEYWORD_TYPE),
        Rule::token(r"(?m)(Parallel |Async |Finalize |Partial )", COMMENT_PREPROC),
        Rule::token(r"(?m)Backward", COMMENT_PREPROC),
        Rule::token(r"(?m)(Intersect|Except|Hash)", COMMENT_PREPROC),
        Rule::bygroups(r"(?m)(CTE)( )(\w*)?", vec![Some(COMMENT), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::token_to(r"(?m)(on|using)", PUNCTUATION, NewState::Push(vec![r"object_name"])),
        Rule::token(r"(?m)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r"(?m)-?\d+\.\d+", NUMBER_FLOAT),
        Rule::token(r"(?m)(-?\d+)", NUMBER_INTEGER),
        Rule::token(r"(?m)(true|false)", NAME_CONSTANT),
        Rule::token(r"(?m)\s*QUERY PLAN\s*\n\s*-+", COMMENT_SINGLE),
        Rule::bygroups_to(r"(?m)(Settings)(:)( )", vec![Some(COMMENT_PREPROC), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"setting"])),
        Rule::bygroups(r"(?m)(JIT|Functions|Options|Timing)(:)", vec![Some(COMMENT_PREPROC), Some(PUNCTUATION)]),
        Rule::token(r"(?m)(Inlining|Optimization|Expressions|Deforming|Generation|Emission|Total)", KEYWORD_PSEUDO),
        Rule::bygroups(r"(?m)(Trigger)( )(\S*)(:)( )", vec![Some(COMMENT_PREPROC), Some(WHITESPACE), Some(NAME_VARIABLE), Some(PUNCTUATION), Some(WHITESPACE)]),
    ]);
    m.insert(
        r"expression",
        vec![
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::PushSame),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)(never executed)", NAME_EXCEPTION),
            Rule::token(r"(?m)[^)(]+", COMMENT),
        ],
    );
    m.insert(
        r"object_name",
        vec![
            Rule::bygroups_to(
                r"(?m)(\(cost)(=?)",
                vec![Some(NAME_CLASS), Some(PUNCTUATION)],
                NewState::Push(vec![r"instrumentation"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\(actual)( )(=?)",
                vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"instrumentation"]),
            ),
            Rule::token_to(r"(?m)\(", PUNCTUATION, NewState::Push(vec![r"expression"])),
            Rule::token(r"(?m)(on)", PUNCTUATION),
            Rule::token(r"(?m)\w+(\.\w+)*( USING \S+| \w+ USING \S+)", NAME_VARIABLE),
            Rule::token(r#"(?m)\"?\w+\"?(?:\.\"?\w+\"?)?"#, NAME_VARIABLE),
            Rule::token(r"(?m)\'\S*\'", NAME_VARIABLE),
            Rule::token_to(
                r"(?m),\n",
                PUNCTUATION,
                NewState::Push(vec![r"object_name"]),
            ),
            Rule::token_to(r"(?m),", PUNCTUATION, NewState::Push(vec![r"object_name"])),
            Rule::token(r#"(?m)"\*SELECT\*( \d+)?"(.\w+)?"#, NAME_VARIABLE),
            Rule::token(r#"(?m)"\*VALUES\*(_\d+)?"(.\w+)?"#, NAME_VARIABLE),
            Rule::token(r#"(?m)"ANY_subquery""#, NAME_VARIABLE),
            Rule::token(r"(?m)\$\d+", NAME_VARIABLE),
            Rule::token(r"(?m)::\w+", NAME_VARIABLE),
            Rule::token(r"(?m) +", WHITESPACE),
            Rule::token(r#"(?m)""#, PUNCTUATION),
            Rule::token(r"(?m)\[\.\.\.\]", PUNCTUATION),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"predicate",
        vec![
            Rule::bygroups_to(
                r"(?m)(\()([^\n]*)(\))",
                vec![Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)],
                NewState::Pop(1),
            ),
            Rule::token_to(r"(?m)[^\n]*", NAME_VARIABLE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"instrumentation",
        vec![
            Rule::token(r"(?m)=|\.\.", PUNCTUATION),
            Rule::token(r"(?m) +", WHITESPACE),
            Rule::token(r"(?m)(rows|width|time|loops)", NAME_CLASS),
            Rule::token(r"(?m)\d+\.\d+", NUMBER_FLOAT),
            Rule::token(r"(?m)(\d+)", NUMBER_INTEGER),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"conflict",
        vec![
            Rule::bygroups(
                r"(?m)(Resolution: )(\w+)",
                vec![Some(COMMENT_PREPROC), Some(NAME_VARIABLE)],
            ),
            Rule::token_to(
                r"(?m)(Arbiter \w+:)",
                COMMENT_PREPROC,
                NewState::Push(vec![r"object_name"]),
            ),
            Rule::token_to(
                r"(?m)(Filter: )",
                COMMENT_PREPROC,
                NewState::Push(vec![r"predicate"]),
            ),
        ],
    );
    m.insert(
        r"setting",
        vec![
            Rule::bygroups(
                r"(?m)([a-z_]*?)(\s*)(=)(\s*)(\'.*?\')",
                vec![
                    Some(NAME_ATTRIBUTE),
                    Some(WHITESPACE),
                    Some(OPERATOR),
                    Some(WHITESPACE),
                    Some(STRING),
                ],
            ),
            Rule::token(r"(?m)\, ", PUNCTUATION),
        ],
    );
    m.insert(
        r"init_plan",
        vec![
            Rule::token(r"(?m)\(", PUNCTUATION),
            Rule::token(r"(?m)returns \$\d+(,\$\d+)?", NAME_VARIABLE),
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"sort",
        vec![
            Rule::token(r"(?m):|kB", PUNCTUATION),
            Rule::token(
                r"(?m)(quicksort|top-N|heapsort|Average|Memory|Peak)",
                TokenType::new(&["Comment", "Prepoc"]),
            ),
            Rule::token(r"(?m)(external|merge|Disk|sort)", NAME_EXCEPTION),
            Rule::token(r"(?m)(\d+)", NUMBER_INTEGER),
            Rule::token(r"(?m) +", WHITESPACE),
        ],
    );
    Table(m)
}

impl Lexer for PostgresExplainLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
