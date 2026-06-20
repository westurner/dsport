//! AUTO-GENERATED from `pygments.pygments.lexers.tact:TactLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.tact:TactLexer:tact

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: tact
pub struct TactLexer;

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
        Rule::token(r"(?m)[.;(),\[\]{}]", PUNCTUATION),
        Rule::token(r"(?m)\?|!!", OPERATOR),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)((?<=\.\.\.)|(?<![.$]))\b(import)\b(\s*)", vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"import"])),
        Rule::bygroups_to(r"(?m)((?<=\.\.\.)|(?<![.$]))\b(struct|message)\b", vec![Some(PUNCTUATION), Some(KEYWORD)], NewState::Push(vec![r"struct"])),
        Rule::token_to(r"(?m)((?<=\.\.\.)|(?<![.$]))\b(contract|trait)\b", KEYWORD, NewState::Push(vec![r"contract-or-trait"])),
        Rule::bygroups_to(r"(?m)(@)(\w+)(\()", vec![Some(KEYWORD_PSEUDO), Some(KEYWORD), Some(PUNCTUATION)], NewState::Push(vec![r"annotation"])),
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*fun\b)", KEYWORD, NewState::Push(vec![r"fun-declaration"])),
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*const\b)", KEYWORD, NewState::Push(vec![r"const-declaration"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block-declaration"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
    ]);
    m.insert(
        r"comments",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
        ],
    );
    m.insert(
        r"import-in",
        vec![Rule::bygroups_to(
            r"(?m)((?<=\.\.\.)|(?<![.$]))\b(import)\b(\s*)",
            vec![Some(PUNCTUATION), Some(KEYWORD), Some(WHITESPACE)],
            NewState::Push(vec![r"import"]),
        )],
    );
    m.insert(
        r"struct-in",
        vec![Rule::bygroups_to(
            r"(?m)((?<=\.\.\.)|(?<![.$]))\b(struct|message)\b",
            vec![Some(PUNCTUATION), Some(KEYWORD)],
            NewState::Push(vec![r"struct"]),
        )],
    );
    m.insert(
        r"contract-or-trait-in",
        vec![Rule::token_to(
            r"(?m)((?<=\.\.\.)|(?<![.$]))\b(contract|trait)\b",
            KEYWORD,
            NewState::Push(vec![r"contract-or-trait"]),
        )],
    );
    m.insert(
        r"annotation-in",
        vec![Rule::bygroups_to(
            r"(?m)(@)(\w+)(\()",
            vec![Some(KEYWORD_PSEUDO), Some(KEYWORD), Some(PUNCTUATION)],
            NewState::Push(vec![r"annotation"]),
        )],
    );
    m.insert(r"fun-declaration-in", vec![
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*fun\b)", KEYWORD, NewState::Push(vec![r"fun-declaration"])),
    ]);
    m.insert(r"const-declaration-in", vec![
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*const\b)", KEYWORD, NewState::Push(vec![r"const-declaration"])),
    ]);
    m.insert(r"statements", vec![
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block-declaration"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
    ]);
    m.insert(
        r"block-declaration-in",
        vec![Rule::token_to(
            r"(?m)\{",
            PUNCTUATION,
            NewState::Push(vec![r"block-declaration"]),
        )],
    );
    m.insert(r"expressions", vec![
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
    ]);
    m.insert(
        r"type-annotation-in",
        vec![Rule::bygroups_to(
            r"(?m)(:)(\s+)",
            vec![Some(PUNCTUATION), Some(WHITESPACE)],
            NewState::Push(vec![r"type-annotation"]),
        )],
    );
    m.insert(r"keywords", vec![
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
    ]);
    m.insert(
        r"numeric",
        vec![
            Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
            Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        ],
    );
    m.insert(
        r"string-in",
        vec![Rule::token_to(
            r#"(?m)""#,
            STRING,
            NewState::Push(vec![r"string"]),
        )],
    );
    m.insert(
        r"variable",
        vec![Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE)],
    );
    m.insert(
        r"function-call",
        vec![Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION)],
    );
    m.insert(
        r"struct-init-in",
        vec![Rule::bygroups_to(
            r"(?m)(\b\w+)(\s*)(\{)",
            vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)],
            NewState::Push(vec![r"struct-init"]),
        )],
    );
    m.insert(
        r"import",
        vec![
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"struct",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token(r"(?m)\b\w+", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\()((?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b)(\))",
                vec![Some(PUNCTUATION), Some(NUMBER_HEX), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)(\()((?:\b[0-9]+\b))(\))",
                vec![Some(PUNCTUATION), Some(NUMBER_INTEGER), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"struct-body"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"struct-header",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token(r"(?m)\b\w+", NAME_CLASS),
            Rule::bygroups(
                r"(?m)(\()((?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b)(\))",
                vec![Some(PUNCTUATION), Some(NUMBER_HEX), Some(PUNCTUATION)],
            ),
            Rule::bygroups(
                r"(?m)(\()((?:\b[0-9]+\b))(\))",
                vec![Some(PUNCTUATION), Some(NUMBER_INTEGER), Some(PUNCTUATION)],
            ),
        ],
    );
    m.insert(
        r"struct-body-in",
        vec![Rule::token_to(
            r"(?m)\{",
            PUNCTUATION,
            NewState::Push(vec![r"struct-body"]),
        )],
    );
    m.insert(
        r"struct-body",
        vec![
            Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\b\w+",
                NAME_PROPERTY,
                NewState::Push(vec![r"field-declaration"]),
            ),
        ],
    );
    m.insert(
        r"field-declaration-in",
        vec![Rule::token_to(
            r"(?m)\b\w+",
            NAME_PROPERTY,
            NewState::Push(vec![r"field-declaration"]),
        )],
    );
    m.insert(
        r"contract-or-trait",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token(r"(?m)with", KEYWORD),
            Rule::token(r"(?m)\b\w+", NAME_CLASS),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"contract-or-trait-body"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"contract-or-trait-body-in",
        vec![Rule::token_to(
            r"(?m)\{",
            PUNCTUATION,
            NewState::Push(vec![r"contract-or-trait-body"]),
        )],
    );
    m.insert(r"contract-or-trait-body", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(2)),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token_to(r"(?m)(init)", KEYWORD, NewState::Push(vec![r"init-declaration"])),
        Rule::token_to(r"(?m)(receive|exernal)", KEYWORD, NewState::Push(vec![r"receive-declaration"])),
        Rule::token_to(r"(?m)(bounced)", KEYWORD, NewState::Push(vec![r"bounce-declaration"])),
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*fun\b)", KEYWORD, NewState::Push(vec![r"fun-declaration"])),
        Rule::token_to(r"(?m)(?=\b(?:(?:get|native|extends|mutates|virtual|override|inline|abstract)\s*)*const\b)", KEYWORD, NewState::Push(vec![r"const-declaration"])),
        Rule::token_to(r"(?m)\b\w+", NAME_PROPERTY, NewState::Push(vec![r"field-declaration"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"init-declaration-in",
        vec![Rule::token_to(
            r"(?m)(init)",
            KEYWORD,
            NewState::Push(vec![r"init-declaration"]),
        )],
    );
    m.insert(
        r"receive-declaration-in",
        vec![Rule::token_to(
            r"(?m)(receive|exernal)",
            KEYWORD,
            NewState::Push(vec![r"receive-declaration"]),
        )],
    );
    m.insert(
        r"bounce-declaration-in",
        vec![Rule::token_to(
            r"(?m)(bounced)",
            KEYWORD,
            NewState::Push(vec![r"bounce-declaration"]),
        )],
    );
    m.insert(
        r"field-declaration",
        vec![
            Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::bygroups_to(
                r"(?m)(:)(\s+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"type-annotation"]),
            ),
            Rule::token_to(r"(?m)(=)", OPERATOR, NewState::Push(vec![r"variable-init"])),
        ],
    );
    m.insert(
        r"variable-init-in",
        vec![Rule::token_to(
            r"(?m)(=)",
            OPERATOR,
            NewState::Push(vec![r"variable-init"]),
        )],
    );
    m.insert(
        r"const-declaration",
        vec![
            Rule::token_to(r"(?m)(;)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)const", KEYWORD),
            Rule::token(
                r"(?m)(abstract|extends|get|inline|mutates|native|override|virtual)\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)\b\w+\b", NAME_CONSTANT),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::bygroups_to(
                r"(?m)(:)(\s+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"type-annotation"]),
            ),
            Rule::token_to(r"(?m)(=)", OPERATOR, NewState::Push(vec![r"variable-init"])),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"init-declaration",
        vec![
            Rule::token_to(r"(?m)(?<=\})", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"fun-arguments"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"block-declaration"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"fun-arguments-in",
        vec![Rule::token_to(
            r"(?m)\(",
            PUNCTUATION,
            NewState::Push(vec![r"fun-arguments"]),
        )],
    );
    m.insert(
        r"receive-declaration",
        vec![
            Rule::token_to(r"(?m)(?<=\})", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"fun-arguments"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"block-declaration"]),
            ),
        ],
    );
    m.insert(
        r"bounce-declaration",
        vec![
            Rule::token_to(r"(?m)(?<=\})", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"fun-arguments"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"block-declaration"]),
            ),
        ],
    );
    m.insert(
        r"fun-declaration",
        vec![
            Rule::token_to(r"(?m)(?<=\}|\;)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)fun", KEYWORD),
            Rule::token(
                r"(?m)\b(get|native|extends|mutates|virtual|override|inline|abstract)\b",
                KEYWORD,
            ),
            Rule::token(r"(?m)\b[\w]+", NAME_FUNCTION),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"fun-arguments"]),
            ),
            Rule::bygroups_to(
                r"(?m)(:)(\s+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"type-annotation"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"block-declaration"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m)[,;]", PUNCTUATION),
        ],
    );
    m.insert(
        r"fun-declaration-body",
        vec![
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(
                r"(?m)\(",
                PUNCTUATION,
                NewState::Push(vec![r"fun-arguments"]),
            ),
            Rule::bygroups_to(
                r"(?m)(:)(\s+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"type-annotation"]),
            ),
            Rule::token_to(
                r"(?m)\{",
                PUNCTUATION,
                NewState::Push(vec![r"block-declaration"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"fun-arguments",
        vec![
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
            Rule::bygroups_to(
                r"(?m)(:)(\s+)",
                vec![Some(PUNCTUATION), Some(WHITESPACE)],
                NewState::Push(vec![r"type-annotation"]),
            ),
            Rule::bygroups(
                r"(?m)(self)|(\b[\w]+\b)",
                vec![Some(NAME_VARIABLE_INSTANCE), Some(NAME_VARIABLE)],
            ),
            Rule::token(r"(?m),", PUNCTUATION),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(r"block-declaration", vec![
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::Push(vec![r"block-declaration"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
    ]);
    m.insert(
        r"annotation",
        vec![
            Rule::token_to(r"(?m)\)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)\w+", NAME_FUNCTION_MAGIC),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"annotation-argument",
        vec![Rule::token(r"(?m)\w+", NAME_FUNCTION_MAGIC)],
    );
    m.insert(
        r"struct-init",
        vec![
            Rule::token_to(r"(?m)(\})", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::bygroups_to(
                r"(?m)(\b[\w]+)(\s*)(:)",
                vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)],
                NewState::Push(vec![r"struct-property"]),
            ),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"struct-property-in",
        vec![Rule::bygroups_to(
            r"(?m)(\b[\w]+)(\s*)(:)",
            vec![Some(NAME_PROPERTY), Some(WHITESPACE), Some(PUNCTUATION)],
            NewState::Push(vec![r"struct-property"]),
        )],
    );
    m.insert(r"struct-property", vec![
        Rule::token_to(r"(?m)(?=\}|\,)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(r"variable-init", vec![
        Rule::token_to(r"(?m)(?=\}|\{|\,|\;)", PUNCTUATION, NewState::Pop(1)),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::token(r"(?m)//.*", COMMENT_SINGLE),
        Rule::token_to(r"(?m)/\*", COMMENT_MULTILINE, NewState::Push(vec![r"comments-multiline"])),
        Rule::bygroups_to(r"(?m)(:)(\s+)", vec![Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"type-annotation"])),
        Rule::token(r"(?m)\b(as|bounced|const|do|e(?:lse|xtends)|fun|get|i(?:n(?:itOf|line)|[fs])|let|m(?:ap|utates)|native|override|re(?:peat|turn)|self|until|virtual|while)\b", KEYWORD),
        Rule::token(r"(?m)(<=>|>=|<=|!=|==|\^>>|~>>|>>|<<|\/%|\^%|~%|\^\/|~\/|\+=|-=|\*=|\/=|~\/=|\^\/=|%=|\^%=|<<=|>>=|~>>=|\^>>=|&=|\|=|\^=|\^|=|~|\/|%|-|\*|\+|>|<|&|\||:|\?)", OPERATOR),
        Rule::token(r"(?m)\b((?:fals|tru)e)\b", KEYWORD_CONSTANT),
        Rule::token(r"(?m)(?:\b0[xX])[0-9a-fA-F][0-9a-fA-F_]*\b", NUMBER_HEX),
        Rule::token(r"(?m)(?:\b[0-9]+\b)", NUMBER_INTEGER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b\w+\b(?!\s*\()(?!\s*\{)", NAME_VARIABLE),
        Rule::token(r"(?m)\b\w+\b(?=\s*\()(?!\s*\{)", NAME_FUNCTION),
        Rule::bygroups_to(r"(?m)(\b\w+)(\s*)(\{)", vec![Some(NAME_CLASS), Some(WHITESPACE), Some(PUNCTUATION)], NewState::Push(vec![r"struct-init"])),
        Rule::token(r"(?m)\s+", WHITESPACE),
    ]);
    m.insert(
        r"type-annotation",
        vec![
            Rule::token_to(r"(?m)(?=\{|\;|\=|\,|\))", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::bygroups_to(
                r"(?m)\b(as)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"type-as"]),
            ),
            Rule::token_to(r"(?m)<", PUNCTUATION, NewState::Push(vec![r"type-generic"])),
            Rule::token(r"(?m)\?", OPERATOR),
            Rule::token(r"(?m)\b\w+", KEYWORD_TYPE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"type-as-in",
        vec![Rule::bygroups_to(
            r"(?m)\b(as)(\s+)",
            vec![Some(KEYWORD), Some(WHITESPACE)],
            NewState::Push(vec![r"type-as"]),
        )],
    );
    m.insert(
        r"type-generic-in",
        vec![Rule::token_to(
            r"(?m)<",
            PUNCTUATION,
            NewState::Push(vec![r"type-generic"]),
        )],
    );
    m.insert(
        r"type-generic",
        vec![
            Rule::token_to(r"(?m)>", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::bygroups_to(
                r"(?m)\b(as)(\s+)",
                vec![Some(KEYWORD), Some(WHITESPACE)],
                NewState::Push(vec![r"type-as"]),
            ),
            Rule::token(r"(?m)\b\w+", KEYWORD_TYPE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token(r"(?m),", PUNCTUATION),
        ],
    );
    m.insert(
        r"type-as",
        vec![
            Rule::token_to(r"(?m)(?=\{|\;|\=|\,|\)|\>)", PUNCTUATION, NewState::Pop(1)),
            Rule::token(r"(?m)//.*", COMMENT_SINGLE),
            Rule::token_to(
                r"(?m)/\*",
                COMMENT_MULTILINE,
                NewState::Push(vec![r"comments-multiline"]),
            ),
            Rule::token(r"(?m)\b\w+", KEYWORD_TYPE),
            Rule::token(r"(?m)\s+", WHITESPACE),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(r"(?m)\\.", STRING_ESCAPE),
            Rule::token(r#"(?m)[^\\"]+"#, STRING_DOUBLE),
        ],
    );
    m.insert(
        r"comments-multiline",
        vec![
            Rule::token_to(r"(?m)\*/", COMMENT_MULTILINE, NewState::Pop(1)),
            Rule::token(r"(?m)[^*]+", COMMENT_MULTILINE),
            Rule::token(r"(?m)[*]", COMMENT_MULTILINE),
        ],
    );
    Table(m)
}

impl Lexer for TactLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
