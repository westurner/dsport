#![allow(clippy::invisible_characters)]

//! AUTO-GENERATED from `pygments.pygments.lexers.erlang:ElixirLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.erlang:ElixirLexer:elixir

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: elixir, ex, exs
pub struct ElixirLexer;

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
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\?)(\\x\{)([\da-fA-F]+)(\})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\x[\da-fA-F]{1,2})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\[abdefnrstv])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::token(r"(?m)\?\\?.", STRING_CHAR),
        Rule::token(r"(?m):::", STRING_SYMBOL),
        Rule::token(r"(?m)::", OPERATOR),
        Rule::token(r"(?m):(?:\.\.\.|<<>>|%\{\}|%|\{\})", STRING_SYMBOL),
        Rule::token(r"(?m):(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&))", STRING_SYMBOL),
        Rule::token_to(r#"(?m):""#, STRING_SYMBOL, NewState::Push(vec![r"string_double_atom"])),
        Rule::token_to(r"(?m):'", STRING_SYMBOL, NewState::Push(vec![r"string_single_atom"])),
        Rule::bygroups(r"(?m)((?:\.\.\.|<<>>|%\{\}|%|\{\})|(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&)))(:)(?=\s|\n)", vec![Some(STRING_SYMBOL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)@(?:\.\.\.|[a-z_]\w*[!?]?)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:after|catch|do|else|end|fn|rescue)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:and|in|not|or|when)(?![a-zA-Z0-9_!?])", OPERATOR_WORD),
        Rule::token(r"(?m)(?:case|cond|for|if|quote|raise|receive|super|throw|try|unless|unquote|unquote_splicing)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:def|defcallback|defdelegate|defexception|defimpl|defmacro|defmacrop|defmodule|defp|defprotocol|defstruct)(?![a-zA-Z0-9_!?])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?:alias|import|require|use)(?![a-zA-Z0-9_!?])", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:false|nil|true)(?![a-zA-Z0-9_!?])", NAME_CONSTANT),
        Rule::token(r"(?m)(?:_|__CALLER__|__DIR__|__ENV__|__MODULE__)(?![a-zA-Z0-9_!?])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?:\.\.\.|[a-z_]\w*[!?]?)", NAME),
        Rule::bygroups(r"(?m)(%?)([A-Z]\w*(?:\.[A-Z]\w*)*)", vec![Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::token(r"(?m)<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~", OPERATOR),
        Rule::token(r"(?m)\\\\|<<|>>|=>|\(|\)|:|;|,|\[|\]", PUNCTUATION),
        Rule::token(r"(?m)&\d", NAME_ENTITY),
        Rule::token(r"(?m)<|>|\+|\-|\*|/|!|\^|\&", OPERATOR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d(_?\d)*\.\d(_?\d)*([eE][-+]?\d(_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_?\d)*", NUMBER_INTEGER),
        Rule::bygroups_to(r#"(?m)(""")(\s*)"#, vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_double"])),
        Rule::bygroups_to(r"(?m)(''')(\s*)$", vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::bygroups_to(r#"(?m)(~[a-z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-intp"])),
        Rule::bygroups_to(r#"(?m)(~[A-Z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-no-intp"])),
        Rule::bygroups_to(r"(?m)(~[a-z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-intp"])),
        Rule::bygroups_to(r"(?m)(~[A-Z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\{", STRING_OTHER, NewState::Push(vec![r"cb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\{", STRING_OTHER, NewState::Push(vec![r"cb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\[", STRING_OTHER, NewState::Push(vec![r"sb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\[", STRING_OTHER, NewState::Push(vec![r"sb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\(", STRING_OTHER, NewState::Push(vec![r"pa-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\(", STRING_OTHER, NewState::Push(vec![r"pa-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]<", STRING_OTHER, NewState::Push(vec![r"ab-intp"])),
        Rule::token_to(r"(?m)~[A-Z]<", STRING_OTHER, NewState::Push(vec![r"ab-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]/", STRING_OTHER, NewState::Push(vec![r"slas-intp"])),
        Rule::token_to(r"(?m)~[A-Z]/", STRING_OTHER, NewState::Push(vec![r"slas-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-no-intp"])),
        Rule::token_to(r#"(?m)~[a-z]""#, STRING_OTHER, NewState::Push(vec![r"quot-intp"])),
        Rule::token_to(r#"(?m)~[A-Z]""#, STRING_OTHER, NewState::Push(vec![r"quot-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]'", STRING_OTHER, NewState::Push(vec![r"apos-intp"])),
        Rule::token_to(r"(?m)~[A-Z]'", STRING_OTHER, NewState::Push(vec![r"apos-no-intp"])),
        Rule::token_to(r"(?m)%\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"tuple"])),
    ]);
    m.insert(
        r"sigils",
        vec![
            Rule::bygroups_to(
                r#"(?m)(~[a-z])(""")"#,
                vec![Some(STRING_OTHER), Some(STRING_HEREDOC)],
                NewState::Push(vec![r"triquot-end", r"triquot-intp"]),
            ),
            Rule::bygroups_to(
                r#"(?m)(~[A-Z])(""")"#,
                vec![Some(STRING_OTHER), Some(STRING_HEREDOC)],
                NewState::Push(vec![r"triquot-end", r"triquot-no-intp"]),
            ),
            Rule::bygroups_to(
                r"(?m)(~[a-z])(''')",
                vec![Some(STRING_OTHER), Some(STRING_HEREDOC)],
                NewState::Push(vec![r"triapos-end", r"triapos-intp"]),
            ),
            Rule::bygroups_to(
                r"(?m)(~[A-Z])(''')",
                vec![Some(STRING_OTHER), Some(STRING_HEREDOC)],
                NewState::Push(vec![r"triapos-end", r"triapos-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]\{",
                STRING_OTHER,
                NewState::Push(vec![r"cb-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]\{",
                STRING_OTHER,
                NewState::Push(vec![r"cb-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]\[",
                STRING_OTHER,
                NewState::Push(vec![r"sb-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]\[",
                STRING_OTHER,
                NewState::Push(vec![r"sb-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]\(",
                STRING_OTHER,
                NewState::Push(vec![r"pa-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]\(",
                STRING_OTHER,
                NewState::Push(vec![r"pa-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]<",
                STRING_OTHER,
                NewState::Push(vec![r"ab-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]<",
                STRING_OTHER,
                NewState::Push(vec![r"ab-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]/",
                STRING_OTHER,
                NewState::Push(vec![r"slas-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]/",
                STRING_OTHER,
                NewState::Push(vec![r"slas-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]\|",
                STRING_OTHER,
                NewState::Push(vec![r"pipe-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]\|",
                STRING_OTHER,
                NewState::Push(vec![r"pipe-no-intp"]),
            ),
            Rule::token_to(
                r#"(?m)~[a-z]""#,
                STRING_OTHER,
                NewState::Push(vec![r"quot-intp"]),
            ),
            Rule::token_to(
                r#"(?m)~[A-Z]""#,
                STRING_OTHER,
                NewState::Push(vec![r"quot-no-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[a-z]'",
                STRING_OTHER,
                NewState::Push(vec![r"apos-intp"]),
            ),
            Rule::token_to(
                r"(?m)~[A-Z]'",
                STRING_OTHER,
                NewState::Push(vec![r"apos-no-intp"]),
            ),
        ],
    );
    m.insert(
        r"heredoc_double",
        vec![
            Rule::bygroups_to(
                r#"(?m)^(\s*)(""")"#,
                vec![Some(WHITESPACE), Some(STRING_HEREDOC)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[^#\\\n]+", STRING_HEREDOC),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", STRING_HEREDOC),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"heredoc_interpol",
        vec![
            Rule::token(r"(?m)[^#\\\n]+", STRING_HEREDOC),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", STRING_HEREDOC),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"escapes",
        vec![
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
        ],
    );
    m.insert(
        r"interpol",
        vec![Rule::token_to(
            r"(?m)#\{",
            STRING_INTERPOL,
            NewState::Push(vec![r"interpol_string"]),
        )],
    );
    m.insert(
        r"heredoc_single",
        vec![
            Rule::token_to(r"(?m)^\s*'''", STRING_HEREDOC, NewState::Pop(1)),
            Rule::token(r"(?m)[^#\\\n]+", STRING_HEREDOC),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", STRING_HEREDOC),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"heredoc_no_interpol",
        vec![
            Rule::token(r"(?m)[^\\\n]+", STRING_HEREDOC),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", WHITESPACE),
        ],
    );
    m.insert(r"interpol_string", vec![
        Rule::token_to(r"(?m)\}", STRING_INTERPOL, NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\?)(\\x\{)([\da-fA-F]+)(\})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\x[\da-fA-F]{1,2})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\[abdefnrstv])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::token(r"(?m)\?\\?.", STRING_CHAR),
        Rule::token(r"(?m):::", STRING_SYMBOL),
        Rule::token(r"(?m)::", OPERATOR),
        Rule::token(r"(?m):(?:\.\.\.|<<>>|%\{\}|%|\{\})", STRING_SYMBOL),
        Rule::token(r"(?m):(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&))", STRING_SYMBOL),
        Rule::token_to(r#"(?m):""#, STRING_SYMBOL, NewState::Push(vec![r"string_double_atom"])),
        Rule::token_to(r"(?m):'", STRING_SYMBOL, NewState::Push(vec![r"string_single_atom"])),
        Rule::bygroups(r"(?m)((?:\.\.\.|<<>>|%\{\}|%|\{\})|(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&)))(:)(?=\s|\n)", vec![Some(STRING_SYMBOL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)@(?:\.\.\.|[a-z_]\w*[!?]?)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:after|catch|do|else|end|fn|rescue)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:and|in|not|or|when)(?![a-zA-Z0-9_!?])", OPERATOR_WORD),
        Rule::token(r"(?m)(?:case|cond|for|if|quote|raise|receive|super|throw|try|unless|unquote|unquote_splicing)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:def|defcallback|defdelegate|defexception|defimpl|defmacro|defmacrop|defmodule|defp|defprotocol|defstruct)(?![a-zA-Z0-9_!?])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?:alias|import|require|use)(?![a-zA-Z0-9_!?])", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:false|nil|true)(?![a-zA-Z0-9_!?])", NAME_CONSTANT),
        Rule::token(r"(?m)(?:_|__CALLER__|__DIR__|__ENV__|__MODULE__)(?![a-zA-Z0-9_!?])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?:\.\.\.|[a-z_]\w*[!?]?)", NAME),
        Rule::bygroups(r"(?m)(%?)([A-Z]\w*(?:\.[A-Z]\w*)*)", vec![Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::token(r"(?m)<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~", OPERATOR),
        Rule::token(r"(?m)\\\\|<<|>>|=>|\(|\)|:|;|,|\[|\]", PUNCTUATION),
        Rule::token(r"(?m)&\d", NAME_ENTITY),
        Rule::token(r"(?m)<|>|\+|\-|\*|/|!|\^|\&", OPERATOR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d(_?\d)*\.\d(_?\d)*([eE][-+]?\d(_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_?\d)*", NUMBER_INTEGER),
        Rule::bygroups_to(r#"(?m)(""")(\s*)"#, vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_double"])),
        Rule::bygroups_to(r"(?m)(''')(\s*)$", vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::bygroups_to(r#"(?m)(~[a-z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-intp"])),
        Rule::bygroups_to(r#"(?m)(~[A-Z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-no-intp"])),
        Rule::bygroups_to(r"(?m)(~[a-z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-intp"])),
        Rule::bygroups_to(r"(?m)(~[A-Z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\{", STRING_OTHER, NewState::Push(vec![r"cb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\{", STRING_OTHER, NewState::Push(vec![r"cb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\[", STRING_OTHER, NewState::Push(vec![r"sb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\[", STRING_OTHER, NewState::Push(vec![r"sb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\(", STRING_OTHER, NewState::Push(vec![r"pa-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\(", STRING_OTHER, NewState::Push(vec![r"pa-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]<", STRING_OTHER, NewState::Push(vec![r"ab-intp"])),
        Rule::token_to(r"(?m)~[A-Z]<", STRING_OTHER, NewState::Push(vec![r"ab-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]/", STRING_OTHER, NewState::Push(vec![r"slas-intp"])),
        Rule::token_to(r"(?m)~[A-Z]/", STRING_OTHER, NewState::Push(vec![r"slas-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-no-intp"])),
        Rule::token_to(r#"(?m)~[a-z]""#, STRING_OTHER, NewState::Push(vec![r"quot-intp"])),
        Rule::token_to(r#"(?m)~[A-Z]""#, STRING_OTHER, NewState::Push(vec![r"quot-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]'", STRING_OTHER, NewState::Push(vec![r"apos-intp"])),
        Rule::token_to(r"(?m)~[A-Z]'", STRING_OTHER, NewState::Push(vec![r"apos-no-intp"])),
        Rule::token_to(r"(?m)%\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"tuple"])),
    ]);
    m.insert(r"map_key", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\?)(\\x\{)([\da-fA-F]+)(\})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\x[\da-fA-F]{1,2})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\[abdefnrstv])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::token(r"(?m)\?\\?.", STRING_CHAR),
        Rule::token(r"(?m):::", STRING_SYMBOL),
        Rule::token(r"(?m)::", OPERATOR),
        Rule::token(r"(?m):(?:\.\.\.|<<>>|%\{\}|%|\{\})", STRING_SYMBOL),
        Rule::token(r"(?m):(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&))", STRING_SYMBOL),
        Rule::token_to(r#"(?m):""#, STRING_SYMBOL, NewState::Push(vec![r"string_double_atom"])),
        Rule::token_to(r"(?m):'", STRING_SYMBOL, NewState::Push(vec![r"string_single_atom"])),
        Rule::bygroups(r"(?m)((?:\.\.\.|<<>>|%\{\}|%|\{\})|(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&)))(:)(?=\s|\n)", vec![Some(STRING_SYMBOL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)@(?:\.\.\.|[a-z_]\w*[!?]?)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:after|catch|do|else|end|fn|rescue)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:and|in|not|or|when)(?![a-zA-Z0-9_!?])", OPERATOR_WORD),
        Rule::token(r"(?m)(?:case|cond|for|if|quote|raise|receive|super|throw|try|unless|unquote|unquote_splicing)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:def|defcallback|defdelegate|defexception|defimpl|defmacro|defmacrop|defmodule|defp|defprotocol|defstruct)(?![a-zA-Z0-9_!?])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?:alias|import|require|use)(?![a-zA-Z0-9_!?])", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:false|nil|true)(?![a-zA-Z0-9_!?])", NAME_CONSTANT),
        Rule::token(r"(?m)(?:_|__CALLER__|__DIR__|__ENV__|__MODULE__)(?![a-zA-Z0-9_!?])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?:\.\.\.|[a-z_]\w*[!?]?)", NAME),
        Rule::bygroups(r"(?m)(%?)([A-Z]\w*(?:\.[A-Z]\w*)*)", vec![Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::token(r"(?m)<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~", OPERATOR),
        Rule::token(r"(?m)\\\\|<<|>>|=>|\(|\)|:|;|,|\[|\]", PUNCTUATION),
        Rule::token(r"(?m)&\d", NAME_ENTITY),
        Rule::token(r"(?m)<|>|\+|\-|\*|/|!|\^|\&", OPERATOR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d(_?\d)*\.\d(_?\d)*([eE][-+]?\d(_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_?\d)*", NUMBER_INTEGER),
        Rule::bygroups_to(r#"(?m)(""")(\s*)"#, vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_double"])),
        Rule::bygroups_to(r"(?m)(''')(\s*)$", vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::bygroups_to(r#"(?m)(~[a-z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-intp"])),
        Rule::bygroups_to(r#"(?m)(~[A-Z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-no-intp"])),
        Rule::bygroups_to(r"(?m)(~[a-z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-intp"])),
        Rule::bygroups_to(r"(?m)(~[A-Z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\{", STRING_OTHER, NewState::Push(vec![r"cb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\{", STRING_OTHER, NewState::Push(vec![r"cb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\[", STRING_OTHER, NewState::Push(vec![r"sb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\[", STRING_OTHER, NewState::Push(vec![r"sb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\(", STRING_OTHER, NewState::Push(vec![r"pa-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\(", STRING_OTHER, NewState::Push(vec![r"pa-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]<", STRING_OTHER, NewState::Push(vec![r"ab-intp"])),
        Rule::token_to(r"(?m)~[A-Z]<", STRING_OTHER, NewState::Push(vec![r"ab-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]/", STRING_OTHER, NewState::Push(vec![r"slas-intp"])),
        Rule::token_to(r"(?m)~[A-Z]/", STRING_OTHER, NewState::Push(vec![r"slas-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-no-intp"])),
        Rule::token_to(r#"(?m)~[a-z]""#, STRING_OTHER, NewState::Push(vec![r"quot-intp"])),
        Rule::token_to(r#"(?m)~[A-Z]""#, STRING_OTHER, NewState::Push(vec![r"quot-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]'", STRING_OTHER, NewState::Push(vec![r"apos-intp"])),
        Rule::token_to(r"(?m)~[A-Z]'", STRING_OTHER, NewState::Push(vec![r"apos-no-intp"])),
        Rule::token_to(r"(?m)%\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"tuple"])),
        Rule::token_to(r"(?m):", PUNCTUATION, NewState::Push(vec![r"map_val"])),
        Rule::token_to(r"(?m)=>", PUNCTUATION, NewState::Push(vec![r"map_val"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"map_val", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\?)(\\x\{)([\da-fA-F]+)(\})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\x[\da-fA-F]{1,2})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\[abdefnrstv])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::token(r"(?m)\?\\?.", STRING_CHAR),
        Rule::token(r"(?m):::", STRING_SYMBOL),
        Rule::token(r"(?m)::", OPERATOR),
        Rule::token(r"(?m):(?:\.\.\.|<<>>|%\{\}|%|\{\})", STRING_SYMBOL),
        Rule::token(r"(?m):(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&))", STRING_SYMBOL),
        Rule::token_to(r#"(?m):""#, STRING_SYMBOL, NewState::Push(vec![r"string_double_atom"])),
        Rule::token_to(r"(?m):'", STRING_SYMBOL, NewState::Push(vec![r"string_single_atom"])),
        Rule::bygroups(r"(?m)((?:\.\.\.|<<>>|%\{\}|%|\{\})|(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&)))(:)(?=\s|\n)", vec![Some(STRING_SYMBOL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)@(?:\.\.\.|[a-z_]\w*[!?]?)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:after|catch|do|else|end|fn|rescue)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:and|in|not|or|when)(?![a-zA-Z0-9_!?])", OPERATOR_WORD),
        Rule::token(r"(?m)(?:case|cond|for|if|quote|raise|receive|super|throw|try|unless|unquote|unquote_splicing)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:def|defcallback|defdelegate|defexception|defimpl|defmacro|defmacrop|defmodule|defp|defprotocol|defstruct)(?![a-zA-Z0-9_!?])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?:alias|import|require|use)(?![a-zA-Z0-9_!?])", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:false|nil|true)(?![a-zA-Z0-9_!?])", NAME_CONSTANT),
        Rule::token(r"(?m)(?:_|__CALLER__|__DIR__|__ENV__|__MODULE__)(?![a-zA-Z0-9_!?])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?:\.\.\.|[a-z_]\w*[!?]?)", NAME),
        Rule::bygroups(r"(?m)(%?)([A-Z]\w*(?:\.[A-Z]\w*)*)", vec![Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::token(r"(?m)<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~", OPERATOR),
        Rule::token(r"(?m)\\\\|<<|>>|=>|\(|\)|:|;|,|\[|\]", PUNCTUATION),
        Rule::token(r"(?m)&\d", NAME_ENTITY),
        Rule::token(r"(?m)<|>|\+|\-|\*|/|!|\^|\&", OPERATOR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d(_?\d)*\.\d(_?\d)*([eE][-+]?\d(_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_?\d)*", NUMBER_INTEGER),
        Rule::bygroups_to(r#"(?m)(""")(\s*)"#, vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_double"])),
        Rule::bygroups_to(r"(?m)(''')(\s*)$", vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::bygroups_to(r#"(?m)(~[a-z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-intp"])),
        Rule::bygroups_to(r#"(?m)(~[A-Z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-no-intp"])),
        Rule::bygroups_to(r"(?m)(~[a-z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-intp"])),
        Rule::bygroups_to(r"(?m)(~[A-Z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\{", STRING_OTHER, NewState::Push(vec![r"cb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\{", STRING_OTHER, NewState::Push(vec![r"cb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\[", STRING_OTHER, NewState::Push(vec![r"sb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\[", STRING_OTHER, NewState::Push(vec![r"sb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\(", STRING_OTHER, NewState::Push(vec![r"pa-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\(", STRING_OTHER, NewState::Push(vec![r"pa-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]<", STRING_OTHER, NewState::Push(vec![r"ab-intp"])),
        Rule::token_to(r"(?m)~[A-Z]<", STRING_OTHER, NewState::Push(vec![r"ab-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]/", STRING_OTHER, NewState::Push(vec![r"slas-intp"])),
        Rule::token_to(r"(?m)~[A-Z]/", STRING_OTHER, NewState::Push(vec![r"slas-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-no-intp"])),
        Rule::token_to(r#"(?m)~[a-z]""#, STRING_OTHER, NewState::Push(vec![r"quot-intp"])),
        Rule::token_to(r#"(?m)~[A-Z]""#, STRING_OTHER, NewState::Push(vec![r"quot-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]'", STRING_OTHER, NewState::Push(vec![r"apos-intp"])),
        Rule::token_to(r"(?m)~[A-Z]'", STRING_OTHER, NewState::Push(vec![r"apos-no-intp"])),
        Rule::token_to(r"(?m)%\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"tuple"])),
        Rule::token_to(r"(?m),", PUNCTUATION, NewState::Pop(1)),
        Rule::token_to(r"(?m)(?=\})", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"tuple", vec![
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token(r"(?m)#.*$", COMMENT_SINGLE),
        Rule::bygroups(r"(?m)(\?)(\\x\{)([\da-fA-F]+)(\})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\x[\da-fA-F]{1,2})", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::bygroups(r"(?m)(\?)(\\[abdefnrstv])", vec![Some(STRING_CHAR), Some(STRING_ESCAPE)]),
        Rule::token(r"(?m)\?\\?.", STRING_CHAR),
        Rule::token(r"(?m):::", STRING_SYMBOL),
        Rule::token(r"(?m)::", OPERATOR),
        Rule::token(r"(?m):(?:\.\.\.|<<>>|%\{\}|%|\{\})", STRING_SYMBOL),
        Rule::token(r"(?m):(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&))", STRING_SYMBOL),
        Rule::token_to(r#"(?m):""#, STRING_SYMBOL, NewState::Push(vec![r"string_double_atom"])),
        Rule::token_to(r"(?m):'", STRING_SYMBOL, NewState::Push(vec![r"string_single_atom"])),
        Rule::bygroups(r"(?m)((?:\.\.\.|<<>>|%\{\}|%|\{\})|(?:(?:\.\.\.|[a-z_]\w*[!?]?)|[A-Z]\w*(?:\.[A-Z]\w*)*|(?:<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>|==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~|<|>|\+|\-|\*|/|!|\^|\&)))(:)(?=\s|\n)", vec![Some(STRING_SYMBOL), Some(PUNCTUATION)]),
        Rule::token(r"(?m)@(?:\.\.\.|[a-z_]\w*[!?]?)", NAME_ATTRIBUTE),
        Rule::token(r"(?m)(?:after|catch|do|else|end|fn|rescue)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:and|in|not|or|when)(?![a-zA-Z0-9_!?])", OPERATOR_WORD),
        Rule::token(r"(?m)(?:case|cond|for|if|quote|raise|receive|super|throw|try|unless|unquote|unquote_splicing)(?![a-zA-Z0-9_!?])", KEYWORD),
        Rule::token(r"(?m)(?:def|defcallback|defdelegate|defexception|defimpl|defmacro|defmacrop|defmodule|defp|defprotocol|defstruct)(?![a-zA-Z0-9_!?])", KEYWORD_DECLARATION),
        Rule::token(r"(?m)(?:alias|import|require|use)(?![a-zA-Z0-9_!?])", KEYWORD_NAMESPACE),
        Rule::token(r"(?m)(?:false|nil|true)(?![a-zA-Z0-9_!?])", NAME_CONSTANT),
        Rule::token(r"(?m)(?:_|__CALLER__|__DIR__|__ENV__|__MODULE__)(?![a-zA-Z0-9_!?])", NAME_BUILTIN_PSEUDO),
        Rule::token(r"(?m)(?:\.\.\.|[a-z_]\w*[!?]?)", NAME),
        Rule::bygroups(r"(?m)(%?)([A-Z]\w*(?:\.[A-Z]\w*)*)", vec![Some(PUNCTUATION), Some(NAME_CLASS)]),
        Rule::token(r"(?m)<<<|>>>|\|\|\||\&\&\&|\^\^\^|\~\~\~|===|!==|\~>>|<\~>|\|\~>|<\|>", OPERATOR),
        Rule::token(r"(?m)==|!=|<=|>=|\&\&|\|\||<>|\+\+|\-\-|\|>|=\~|\->|<\-|\||\.|=|\~>|<\~", OPERATOR),
        Rule::token(r"(?m)\\\\|<<|>>|=>|\(|\)|:|;|,|\[|\]", PUNCTUATION),
        Rule::token(r"(?m)&\d", NAME_ENTITY),
        Rule::token(r"(?m)<|>|\+|\-|\*|/|!|\^|\&", OPERATOR),
        Rule::token(r"(?m)0b[01]+", NUMBER_BIN),
        Rule::token(r"(?m)0o[0-7]+", NUMBER_OCT),
        Rule::token(r"(?m)0x[\da-fA-F]+", NUMBER_HEX),
        Rule::token(r"(?m)\d(_?\d)*\.\d(_?\d)*([eE][-+]?\d(_?\d)*)?", NUMBER_FLOAT),
        Rule::token(r"(?m)\d(_?\d)*", NUMBER_INTEGER),
        Rule::bygroups_to(r#"(?m)(""")(\s*)"#, vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_double"])),
        Rule::bygroups_to(r"(?m)(''')(\s*)$", vec![Some(STRING_HEREDOC), Some(WHITESPACE)], NewState::Push(vec![r"heredoc_single"])),
        Rule::token_to(r#"(?m)""#, STRING_DOUBLE, NewState::Push(vec![r"string_double"])),
        Rule::token_to(r"(?m)'", STRING_SINGLE, NewState::Push(vec![r"string_single"])),
        Rule::bygroups_to(r#"(?m)(~[a-z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-intp"])),
        Rule::bygroups_to(r#"(?m)(~[A-Z])(""")"#, vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triquot-end", r"triquot-no-intp"])),
        Rule::bygroups_to(r"(?m)(~[a-z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-intp"])),
        Rule::bygroups_to(r"(?m)(~[A-Z])(''')", vec![Some(STRING_OTHER), Some(STRING_HEREDOC)], NewState::Push(vec![r"triapos-end", r"triapos-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\{", STRING_OTHER, NewState::Push(vec![r"cb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\{", STRING_OTHER, NewState::Push(vec![r"cb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\[", STRING_OTHER, NewState::Push(vec![r"sb-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\[", STRING_OTHER, NewState::Push(vec![r"sb-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\(", STRING_OTHER, NewState::Push(vec![r"pa-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\(", STRING_OTHER, NewState::Push(vec![r"pa-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]<", STRING_OTHER, NewState::Push(vec![r"ab-intp"])),
        Rule::token_to(r"(?m)~[A-Z]<", STRING_OTHER, NewState::Push(vec![r"ab-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]/", STRING_OTHER, NewState::Push(vec![r"slas-intp"])),
        Rule::token_to(r"(?m)~[A-Z]/", STRING_OTHER, NewState::Push(vec![r"slas-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-intp"])),
        Rule::token_to(r"(?m)~[A-Z]\|", STRING_OTHER, NewState::Push(vec![r"pipe-no-intp"])),
        Rule::token_to(r#"(?m)~[a-z]""#, STRING_OTHER, NewState::Push(vec![r"quot-intp"])),
        Rule::token_to(r#"(?m)~[A-Z]""#, STRING_OTHER, NewState::Push(vec![r"quot-no-intp"])),
        Rule::token_to(r"(?m)~[a-z]'", STRING_OTHER, NewState::Push(vec![r"apos-intp"])),
        Rule::token_to(r"(?m)~[A-Z]'", STRING_OTHER, NewState::Push(vec![r"apos-no-intp"])),
        Rule::token_to(r"(?m)%\{", PUNCTUATION, NewState::Push(vec![r"map_key"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"tuple"])),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(
        r"string_double",
        vec![
            Rule::token(r#"(?m)[^#"\\]+"#, STRING_DOUBLE),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_DOUBLE),
            Rule::bygroups_to(r#"(?m)(")"#, vec![Some(STRING_DOUBLE)], NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"string_single",
        vec![
            Rule::token(r"(?m)[^#'\\]+", STRING_SINGLE),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_SINGLE),
            Rule::bygroups_to(r"(?m)(')", vec![Some(STRING_SINGLE)], NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"string_double_atom",
        vec![
            Rule::token(r#"(?m)[^#"\\]+"#, STRING_SYMBOL),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_SYMBOL),
            Rule::bygroups_to(r#"(?m)(")"#, vec![Some(STRING_SYMBOL)], NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"string_single_atom",
        vec![
            Rule::token(r"(?m)[^#'\\]+", STRING_SYMBOL),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_SYMBOL),
            Rule::bygroups_to(r"(?m)(')", vec![Some(STRING_SYMBOL)], NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"triquot-end",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z]+", STRING_OTHER, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"triquot-intp",
        vec![
            Rule::bygroups_to(
                r#"(?m)^(\s*)(""")"#,
                vec![Some(WHITESPACE), Some(STRING_HEREDOC)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[^#\\\n]+", STRING_HEREDOC),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", STRING_HEREDOC),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"triquot-no-intp",
        vec![
            Rule::bygroups_to(
                r#"(?m)^(\s*)(""")"#,
                vec![Some(WHITESPACE), Some(STRING_HEREDOC)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[^\\\n]+", STRING_HEREDOC),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", WHITESPACE),
        ],
    );
    m.insert(
        r"triapos-end",
        vec![
            Rule::token_to(r"(?m)[a-zA-Z]+", STRING_OTHER, NewState::Pop(1)),
            Rule::default(NewState::Pop(1)),
        ],
    );
    m.insert(
        r"triapos-intp",
        vec![
            Rule::bygroups_to(
                r"(?m)^(\s*)(''')",
                vec![Some(WHITESPACE), Some(STRING_HEREDOC)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[^#\\\n]+", STRING_HEREDOC),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", STRING_HEREDOC),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"triapos-no-intp",
        vec![
            Rule::bygroups_to(
                r"(?m)^(\s*)(''')",
                vec![Some(WHITESPACE), Some(STRING_HEREDOC)],
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)[^\\\n]+", STRING_HEREDOC),
            Rule::token(r"(?m)\\.", STRING_HEREDOC),
            Rule::token(r"(?m)\n+", WHITESPACE),
        ],
    );
    m.insert(
        r"cb-intp",
        vec![
            Rule::token(r"(?m)[^#}\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\}[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"cb-no-intp",
        vec![
            Rule::token(r"(?m)[^}\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\}[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"sb-intp",
        vec![
            Rule::token(r"(?m)[^#\]\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\][a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"sb-no-intp",
        vec![
            Rule::token(r"(?m)[^\]\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\][a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"pa-intp",
        vec![
            Rule::token(r"(?m)[^#)\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\)[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"pa-no-intp",
        vec![
            Rule::token(r"(?m)[^)\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\)[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"ab-intp",
        vec![
            Rule::token(r"(?m)[^#>\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)>[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"ab-no-intp",
        vec![
            Rule::token(r"(?m)[^>\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)>[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"slas-intp",
        vec![
            Rule::token(r"(?m)[^#/\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)/[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"slas-no-intp",
        vec![
            Rule::token(r"(?m)[^/\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)/[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"pipe-intp",
        vec![
            Rule::token(r"(?m)[^#|\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\|[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"pipe-no-intp",
        vec![
            Rule::token(r"(?m)[^|\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)\|[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"quot-intp",
        vec![
            Rule::token(r#"(?m)[^#"\\]+"#, STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r#"(?m)"[a-zA-Z]*"#, STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"quot-no-intp",
        vec![
            Rule::token(r#"(?m)[^"\\]+"#, STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r#"(?m)"[a-zA-Z]*"#, STRING_OTHER, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"apos-intp",
        vec![
            Rule::token(r"(?m)[^#'\\]+", STRING_OTHER),
            Rule::bygroups(
                r"(?m)(\\x\{)([\da-fA-F]+)(\})",
                vec![Some(STRING_ESCAPE), Some(NUMBER_HEX), Some(STRING_ESCAPE)],
            ),
            Rule::token(r"(?m)(\\x[\da-fA-F]{1,2})", STRING_ESCAPE),
            Rule::token(r"(?m)(\\[abdefnrstv])", STRING_ESCAPE),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)'[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
            Rule::token_to(
                r"(?m)#\{",
                STRING_INTERPOL,
                NewState::Push(vec![r"interpol_string"]),
            ),
        ],
    );
    m.insert(
        r"apos-no-intp",
        vec![
            Rule::token(r"(?m)[^'\\]+", STRING_OTHER),
            Rule::token(r"(?m)\\.", STRING_OTHER),
            Rule::token_to(r"(?m)'[a-zA-Z]*", STRING_OTHER, NewState::Pop(1)),
        ],
    );
    Table(m)
}

impl Lexer for ElixirLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
