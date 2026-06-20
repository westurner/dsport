//! AUTO-GENERATED from `pygments.pygments.lexers.graphics:AsymptoteLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.graphics:AsymptoteLexer:asymptote

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{GroupAction, NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: asymptote, asy
pub struct AsymptoteLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(
        r"whitespace",
        vec![
            Rule::token(r"(?m)\n", WHITESPACE),
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
            Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT),
            Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT),
        ],
    );
    m.insert(r"statements", vec![
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::bygroups_g(r"(?m)\b(case)(.+?)(:)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(TEXT))]),
        Rule::token(r"(?m)(and|controls|tension|atleast|curl|if|else|while|for|do|return|break|continue|struct|typedef|new|access|import|unravel|from|include|quote|static|public|private|restricted|this|explicit|true|false|null|cycle|newframe|operator)\b", KEYWORD),
        Rule::token(r"(?m)(Braid|FitResult|Label|Legend|TreeNode|abscissa|arc|arrowhead|binarytree|binarytreeNode|block|bool|bool3|bounds|bqe|circle|conic|coord|coordsys|cputime|ellipse|file|filltype|frame|grid3|guide|horner|hsv|hyperbola|indexedTransform|int|inversion|key|light|line|linefit|marginT|marker|mass|object|pair|parabola|path|path3|pen|picture|point|position|projection|real|revolution|scaleT|scientific|segment|side|slice|splitface|string|surface|tensionSpecifier|ticklocate|ticksgridT|tickvalues|transform|transformation|tree|triangle|trilinear|triple|vector|vertex|void)(?=\s+[a-zA-Z])", KEYWORD_TYPE),
        Rule::token(r"(?m)(Braid|FitResult|TreeNode|abscissa|arrowhead|block|bool|bool3|bounds|coord|frame|guide|horner|int|linefit|marginT|pair|pen|picture|position|real|revolution|slice|splitface|ticksgridT|tickvalues|tree|triple|vertex|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:(?!:)", NAME_LABEL),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
    ]);
    m.insert(r"root", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT),
        Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT),
        Rule::bygroups_g_to(r"(?m)((?:[\w*\s])+?(?:\s|\*))([a-zA-Z_]\w*)(\s*\([^;]*?\))((?:\s|//.*?\n|/\*.*?\*/)+)(\{)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(PUNCTUATION))], NewState::Push(vec![r"function"])),
        Rule::bygroups_g(r"(?m)((?:[\w*\s])+?(?:\s|\*))([a-zA-Z_]\w*)(\s*\([^;]*?\))((?:\s|//.*?\n|/\*.*?\*/)+)(;)", vec![Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(NAME_FUNCTION)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(PUNCTUATION))]),
        Rule::default(NewState::Push(vec![r"statement"])),
    ]);
    m.insert(r"statement", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT),
        Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::bygroups_g(r"(?m)\b(case)(.+?)(:)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(TEXT))]),
        Rule::token(r"(?m)(and|controls|tension|atleast|curl|if|else|while|for|do|return|break|continue|struct|typedef|new|access|import|unravel|from|include|quote|static|public|private|restricted|this|explicit|true|false|null|cycle|newframe|operator)\b", KEYWORD),
        Rule::token(r"(?m)(Braid|FitResult|Label|Legend|TreeNode|abscissa|arc|arrowhead|binarytree|binarytreeNode|block|bool|bool3|bounds|bqe|circle|conic|coord|coordsys|cputime|ellipse|file|filltype|frame|grid3|guide|horner|hsv|hyperbola|indexedTransform|int|inversion|key|light|line|linefit|marginT|marker|mass|object|pair|parabola|path|path3|pen|picture|point|position|projection|real|revolution|scaleT|scientific|segment|side|slice|splitface|string|surface|tensionSpecifier|ticklocate|ticksgridT|tickvalues|transform|transformation|tree|triangle|trilinear|triple|vector|vertex|void)(?=\s+[a-zA-Z])", KEYWORD_TYPE),
        Rule::token(r"(?m)(Braid|FitResult|TreeNode|abscissa|arrowhead|block|bool|bool3|bounds|coord|frame|guide|horner|int|linefit|marginT|pair|pen|picture|position|real|revolution|slice|splitface|ticksgridT|tickvalues|tree|triple|vertex|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:(?!:)", NAME_LABEL),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m)[{}]", PUNCTUATION),
        Rule::token_to(r"(?m);", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(r"function", vec![
        Rule::token(r"(?m)\n", WHITESPACE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups(r"(?m)(\\)(\n)", vec![Some(TEXT), Some(WHITESPACE)]),
        Rule::token(r"(?m)//(\n|(.|\n)*?[^\\]\n)", COMMENT),
        Rule::token(r"(?m)/(\\\n)?\*(.|\n)*?\*(\\\n)?/", COMMENT),
        Rule::token(r#"(?m)"(\\\\|\\[^\\]|[^"\\])*""#, STRING),
        Rule::token_to(r"(?m)'", STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+)[eE][+-]?\d+[lL]?", NUMBER_FLOAT),
        Rule::token(r"(?m)(\d+\.\d*|\.\d+|\d+[fF])[fF]?", NUMBER_FLOAT),
        Rule::token(r"(?m)0x[0-9a-fA-F]+[Ll]?", NUMBER_HEX),
        Rule::token(r"(?m)0[0-7]+[Ll]?", NUMBER_OCT),
        Rule::token(r"(?m)\d+[Ll]?", NUMBER_INTEGER),
        Rule::token(r"(?m)[~!%^&*+=|?:<>/-]", OPERATOR),
        Rule::token(r"(?m)[()\[\],.]", PUNCTUATION),
        Rule::bygroups_g(r"(?m)\b(case)(.+?)(:)", vec![Some(GroupAction::Token(KEYWORD)), Some(GroupAction::UsingThis { state: None }), Some(GroupAction::Token(TEXT))]),
        Rule::token(r"(?m)(and|controls|tension|atleast|curl|if|else|while|for|do|return|break|continue|struct|typedef|new|access|import|unravel|from|include|quote|static|public|private|restricted|this|explicit|true|false|null|cycle|newframe|operator)\b", KEYWORD),
        Rule::token(r"(?m)(Braid|FitResult|Label|Legend|TreeNode|abscissa|arc|arrowhead|binarytree|binarytreeNode|block|bool|bool3|bounds|bqe|circle|conic|coord|coordsys|cputime|ellipse|file|filltype|frame|grid3|guide|horner|hsv|hyperbola|indexedTransform|int|inversion|key|light|line|linefit|marginT|marker|mass|object|pair|parabola|path|path3|pen|picture|point|position|projection|real|revolution|scaleT|scientific|segment|side|slice|splitface|string|surface|tensionSpecifier|ticklocate|ticksgridT|tickvalues|transform|transformation|tree|triangle|trilinear|triple|vector|vertex|void)(?=\s+[a-zA-Z])", KEYWORD_TYPE),
        Rule::token(r"(?m)(Braid|FitResult|TreeNode|abscissa|arrowhead|block|bool|bool3|bounds|coord|frame|guide|horner|int|linefit|marginT|pair|pen|picture|position|real|revolution|slice|splitface|ticksgridT|tickvalues|tree|triple|vertex|void)\b", KEYWORD_TYPE),
        Rule::token(r"(?m)[a-zA-Z_]\w*:(?!:)", NAME_LABEL),
        Rule::token(r"(?m)[a-zA-Z_]\w*", NAME),
        Rule::token(r"(?m);", PUNCTUATION),
        Rule::token_to(r"(?m)\{", PUNCTUATION, NewState::PushSame),
        Rule::token_to(r"(?m)\}", PUNCTUATION, NewState::Pop(1)),
    ]);
    m.insert(
        r"string",
        vec![
            Rule::token_to(r"(?m)'", STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\\([\\abfnrtv"\'?]|x[a-fA-F0-9]{2,4}|[0-7]{1,3})"#,
                STRING_ESCAPE,
            ),
            Rule::token(r"(?m)\n", STRING),
            Rule::token(r"(?m)[^\\'\n]+", STRING),
            Rule::token(r"(?m)\\\n", STRING),
            Rule::token(r"(?m)\\n", STRING),
            Rule::token(r"(?m)\\", STRING),
        ],
    );
    Table(m)
}

impl Lexer for AsymptoteLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
