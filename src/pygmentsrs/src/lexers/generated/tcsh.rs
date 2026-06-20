//! AUTO-GENERATED from `pygments.pygments.lexers.shell:TcshLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.shell:TcshLexer:tcsh

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tcsh, csh
pub struct TcshLexer;

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
        Rule::token(r"(?m)\b(if|endif|else|while|then|foreach|case|default|break|continue|goto|breaksw|end|switch|endsw)\s*\b", KEYWORD),
        Rule::token(r"(?m)\b(alias|alloc|bg|bindkey|builtins|bye|caller|cd|chdir|complete|dirs|echo|echotc|eval|exec|exit|fg|filetest|getxvers|glob|getspath|hashstat|history|hup|inlib|jobs|kill|limit|log|login|logout|ls-F|migrate|newgrp|nice|nohup|notify|onintr|popd|printenv|pushd|rehash|repeat|rootnode|popd|pushd|set|shift|sched|setenv|setpath|settc|setty|setxvers|shift|source|stop|suspend|source|suspend|telltc|time|umask|unalias|uncomplete|unhash|universe|unlimit|unset|unsetenv|ver|wait|warp|watchlog|where|which)\s*\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    m.insert(r"basic", vec![
        Rule::token(r"(?m)\b(if|endif|else|while|then|foreach|case|default|break|continue|goto|breaksw|end|switch|endsw)\s*\b", KEYWORD),
        Rule::token(r"(?m)\b(alias|alloc|bg|bindkey|builtins|bye|caller|cd|chdir|complete|dirs|echo|echotc|eval|exec|exit|fg|filetest|getxvers|glob|getspath|hashstat|history|hup|inlib|jobs|kill|limit|log|login|logout|ls-F|migrate|newgrp|nice|nohup|notify|onintr|popd|printenv|pushd|rehash|repeat|rootnode|popd|pushd|set|shift|sched|setenv|setpath|settc|setty|setxvers|shift|source|stop|suspend|source|suspend|telltc|time|umask|unalias|uncomplete|unhash|universe|unlimit|unset|unsetenv|ver|wait|warp|watchlog|where|which)\s*\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
    ]);
    m.insert(
        r"data",
        vec![
            Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
            Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
            Rule::token(r"(?m)\s+", TEXT),
            Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
            Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
            Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
        ],
    );
    m.insert(r"curly", vec![
        Rule::token_to(r"(?m)\}", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m):-", KEYWORD),
        Rule::token(r"(?m)\w+", NAME_VARIABLE),
        Rule::token(r#"(?m)[^}:"\'`$]+"#, PUNCTUATION),
        Rule::token(r"(?m):", PUNCTUATION),
        Rule::token(r"(?m)\b(if|endif|else|while|then|foreach|case|default|break|continue|goto|breaksw|end|switch|endsw)\s*\b", KEYWORD),
        Rule::token(r"(?m)\b(alias|alloc|bg|bindkey|builtins|bye|caller|cd|chdir|complete|dirs|echo|echotc|eval|exec|exit|fg|filetest|getxvers|glob|getspath|hashstat|history|hup|inlib|jobs|kill|limit|log|login|logout|ls-F|migrate|newgrp|nice|nohup|notify|onintr|popd|printenv|pushd|rehash|repeat|rootnode|popd|pushd|set|shift|sched|setenv|setpath|settc|setty|setxvers|shift|source|stop|suspend|source|suspend|telltc|time|umask|unalias|uncomplete|unhash|universe|unlimit|unset|unsetenv|ver|wait|warp|watchlog|where|which)\s*\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    m.insert(r"paren", vec![
        Rule::token_to(r"(?m)\)", KEYWORD, NewState::Pop(1)),
        Rule::token(r"(?m)\b(if|endif|else|while|then|foreach|case|default|break|continue|goto|breaksw|end|switch|endsw)\s*\b", KEYWORD),
        Rule::token(r"(?m)\b(alias|alloc|bg|bindkey|builtins|bye|caller|cd|chdir|complete|dirs|echo|echotc|eval|exec|exit|fg|filetest|getxvers|glob|getspath|hashstat|history|hup|inlib|jobs|kill|limit|log|login|logout|ls-F|migrate|newgrp|nice|nohup|notify|onintr|popd|printenv|pushd|rehash|repeat|rootnode|popd|pushd|set|shift|sched|setenv|setpath|settc|setty|setxvers|shift|source|stop|suspend|source|suspend|telltc|time|umask|unalias|uncomplete|unhash|universe|unlimit|unset|unsetenv|ver|wait|warp|watchlog|where|which)\s*\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    m.insert(r"backticks", vec![
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Pop(1)),
        Rule::token(r"(?m)\b(if|endif|else|while|then|foreach|case|default|break|continue|goto|breaksw|end|switch|endsw)\s*\b", KEYWORD),
        Rule::token(r"(?m)\b(alias|alloc|bg|bindkey|builtins|bye|caller|cd|chdir|complete|dirs|echo|echotc|eval|exec|exit|fg|filetest|getxvers|glob|getspath|hashstat|history|hup|inlib|jobs|kill|limit|log|login|logout|ls-F|migrate|newgrp|nice|nohup|notify|onintr|popd|printenv|pushd|rehash|repeat|rootnode|popd|pushd|set|shift|sched|setenv|setpath|settc|setty|setxvers|shift|source|stop|suspend|source|suspend|telltc|time|umask|unalias|uncomplete|unhash|universe|unlimit|unset|unsetenv|ver|wait|warp|watchlog|where|which)\s*\b", NAME_BUILTIN),
        Rule::token(r"(?m)#.*", COMMENT),
        Rule::token(r"(?m)\\[\w\W]", STRING_ESCAPE),
        Rule::bygroups(r"(?m)(\b\w+)(\s*)(=)", vec![Some(NAME_VARIABLE), Some(TEXT), Some(OPERATOR)]),
        Rule::token(r"(?m)[\[\]{}()=]+", OPERATOR),
        Rule::token(r"(?m)<<\s*(\'?)\\?(\w+)[\w\W]+?\2", STRING),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\$\(", KEYWORD, NewState::Push(vec![r"paren"])),
        Rule::token_to(r"(?m)\$\{#?", KEYWORD, NewState::Push(vec![r"curly"])),
        Rule::token_to(r"(?m)`", STRING_BACKTICK, NewState::Push(vec![r"backticks"])),
        Rule::token(r#"(?m)(?s)"(\\\\|\\[0-7]+|\\.|[^"\\])*""#, STRING_DOUBLE),
        Rule::token(r"(?m)(?s)'(\\\\|\\[0-7]+|\\.|[^'\\])*'", STRING_SINGLE),
        Rule::token(r"(?m)\s+", TEXT),
        Rule::token(r#"(?m)[^=\s\[\]{}()$"\'`\\;#]+"#, TEXT),
        Rule::token(r"(?m)\d+(?= |\Z)", NUMBER),
        Rule::token(r"(?m)\$#?(\w+|.)", NAME_VARIABLE),
    ]);
    Table(m)
}

impl Lexer for TcshLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
