//! AUTO-GENERATED from `pygments.pygments.lexers.business:ABAPLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.business:ABAPLexer:abap

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: abap
pub struct AbapLexer;

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
        r"common",
        vec![
            Rule::token(r"(?im)\s+", WHITESPACE),
            Rule::token(r"(?im)^\*.*$", COMMENT_SINGLE),
            Rule::token(r#"(?im)\".*?\n"#, COMMENT_SINGLE),
            Rule::token(r"(?im)##\w+", COMMENT_SPECIAL),
        ],
    );
    m.insert(
        r"variable-names",
        vec![
            Rule::token(r"(?im)<\S+>", NAME_VARIABLE),
            Rule::token(r"(?im)\w[\w~]*(?:(\[\])|->\*)?", NAME_VARIABLE),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?im)\s+", WHITESPACE),
        Rule::token(r"(?im)^\*.*$", COMMENT_SINGLE),
        Rule::token(r#"(?im)\".*?\n"#, COMMENT_SINGLE),
        Rule::token(r"(?im)##\w+", COMMENT_SPECIAL),
        Rule::token(r"(?im)CALL\s+(?:BADI|CUSTOMER-FUNCTION|FUNCTION)", KEYWORD),
        Rule::token(r"(?im)(CALL\s+(?:DIALOG|SCREEN|SUBSCREEN|SELECTION-SCREEN|TRANSACTION|TRANSFORMATION))\b", KEYWORD),
        Rule::bygroups(r"(?im)(FORM|PERFORM)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)(PERFORM)(\s+)(\()(\w+)(\))", vec![Some(KEYWORD), Some(WHITESPACE), Some(PUNCTUATION), Some(NAME_VARIABLE), Some(PUNCTUATION)]),
        Rule::bygroups(r"(?im)(MODULE)(\s+)(\S+)(\s+)(INPUT|OUTPUT)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION), Some(WHITESPACE), Some(KEYWORD)]),
        Rule::bygroups(r"(?im)(METHOD)(\s+)([\w~]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_FUNCTION)]),
        Rule::bygroups(r"(?im)(\s+)([\w\-]+)([=\-]>)([\w\-~]+)", vec![Some(WHITESPACE), Some(NAME_VARIABLE), Some(OPERATOR), Some(NAME_FUNCTION)]),
        Rule::token(r"(?im)(?<=(=|-)>)([\w\-~]+)(?=\()", NAME_FUNCTION),
        Rule::bygroups(r"(?im)(TEXT)(-)(\d{3})", vec![Some(KEYWORD), Some(PUNCTUATION), Some(NUMBER_INTEGER)]),
        Rule::bygroups(r"(?im)(TEXT)(-)(\w{3})", vec![Some(KEYWORD), Some(PUNCTUATION), Some(NAME_VARIABLE)]),
        Rule::token(r"(?im)(ADD-CORRESPONDING|AUTHORITY-CHECK|CLASS-DATA|CLASS-EVENTS|CLASS-METHODS|CLASS-POOL|DELETE-ADJACENT|DIVIDE-CORRESPONDING|EDITOR-CALL|ENHANCEMENT-POINT|ENHANCEMENT-SECTION|EXIT-COMMAND|FIELD-GROUPS|FIELD-SYMBOLS|FIELD-SYMBOL|FUNCTION-POOL|INTERFACE-POOL|INVERTED-DATE|LOAD-OF-PROGRAM|LOG-POINT|MESSAGE-ID|MOVE-CORRESPONDING|MULTIPLY-CORRESPONDING|NEW-LINE|NEW-PAGE|NEW-SECTION|NO-EXTENSION|OUTPUT-LENGTH|PRINT-CONTROL|SELECT-OPTIONS|START-OF-SELECTION|SUBTRACT-CORRESPONDING|SYNTAX-CHECK|SYSTEM-EXCEPTIONS|TYPE-POOL|TYPE-POOLS|NO-DISPLAY)\b", KEYWORD),
        Rule::token(r"(?im)(?<![-\>])(CREATE\s+(PUBLIC|PRIVATE|DATA|OBJECT)|(PUBLIC|PRIVATE|PROTECTED)\s+SECTION|(TYPE|LIKE)\s+((LINE\s+OF|REF\s+TO|(SORTED|STANDARD|HASHED)\s+TABLE\s+OF))?|FROM\s+(DATABASE|MEMORY)|CALL\s+METHOD|(GROUP|ORDER) BY|HAVING|SEPARATED BY|GET\s+(BADI|BIT|CURSOR|DATASET|LOCALE|PARAMETER|PF-STATUS|(PROPERTY|REFERENCE)\s+OF|RUN\s+TIME|TIME\s+(STAMP)?)?|SET\s+(BIT|BLANK\s+LINES|COUNTRY|CURSOR|DATASET|EXTENDED\s+CHECK|HANDLER|HOLD\s+DATA|LANGUAGE|LEFT\s+SCROLL-BOUNDARY|LOCALE|MARGIN|PARAMETER|PF-STATUS|PROPERTY\s+OF|RUN\s+TIME\s+(ANALYZER|CLOCK\s+RESOLUTION)|SCREEN|TITLEBAR|UPADTE\s+TASK\s+LOCAL|USER-COMMAND)|CONVERT\s+((INVERTED-)?DATE|TIME|TIME\s+STAMP|TEXT)|(CLOSE|OPEN)\s+(DATASET|CURSOR)|(TO|FROM)\s+(DATA BUFFER|INTERNAL TABLE|MEMORY ID|DATABASE|SHARED\s+(MEMORY|BUFFER))|DESCRIBE\s+(DISTANCE\s+BETWEEN|FIELD|LIST|TABLE)|FREE\s(MEMORY|OBJECT)?|PROCESS\s+(BEFORE\s+OUTPUT|AFTER\s+INPUT|ON\s+(VALUE-REQUEST|HELP-REQUEST))|AT\s+(LINE-SELECTION|USER-COMMAND|END\s+OF|NEW)|AT\s+SELECTION-SCREEN(\s+(ON(\s+(BLOCK|(HELP|VALUE)-REQUEST\s+FOR|END\s+OF|RADIOBUTTON\s+GROUP))?|OUTPUT))?|SELECTION-SCREEN:?\s+((BEGIN|END)\s+OF\s+((TABBED\s+)?BLOCK|LINE|SCREEN)|COMMENT|FUNCTION\s+KEY|INCLUDE\s+BLOCKS|POSITION|PUSHBUTTON|SKIP|ULINE)|LEAVE\s+(LIST-PROCESSING|PROGRAM|SCREEN|TO LIST-PROCESSING|TO TRANSACTION)(ENDING|STARTING)\s+AT|FORMAT\s+(COLOR|INTENSIFIED|INVERSE|HOTSPOT|INPUT|FRAMES|RESET)|AS\s+(CHECKBOX|SUBSCREEN|WINDOW)|WITH\s+(((NON-)?UNIQUE)?\s+KEY|FRAME)|(BEGIN|END)\s+OF|DELETE(\s+ADJACENT\s+DUPLICATES\sFROM)?|COMPARING(\s+ALL\s+FIELDS)?|(INSERT|APPEND)(\s+INITIAL\s+LINE\s+(IN)?TO|\s+LINES\s+OF)?|IN\s+((BYTE|CHARACTER)\s+MODE|PROGRAM)|END-OF-(DEFINITION|PAGE|SELECTION)|WITH\s+FRAME(\s+TITLE)|(REPLACE|FIND)\s+((FIRST|ALL)\s+OCCURRENCES?\s+OF\s+)?(SUBSTRING|REGEX)?|MATCH\s+(LENGTH|COUNT|LINE|OFFSET)|(RESPECTING|IGNORING)\s+CASE|IN\s+UPDATE\s+TASK|(SOURCE|RESULT)\s+(XML)?|REFERENCE\s+INTO|AND\s+(MARK|RETURN)|CLIENT\s+SPECIFIED|CORRESPONDING\s+FIELDS\s+OF|IF\s+FOUND|FOR\s+EVENT|INHERITING\s+FROM|LEAVE\s+TO\s+SCREEN|LOOP\s+AT\s+(SCREEN)?|LOWER\s+CASE|MATCHCODE\s+OBJECT|MODIF\s+ID|MODIFY\s+SCREEN|NESTING\s+LEVEL|NO\s+INTERVALS|OF\s+STRUCTURE|RADIOBUTTON\s+GROUP|RANGE\s+OF|REF\s+TO|SUPPRESS DIALOG|TABLE\s+OF|UPPER\s+CASE|TRANSPORTING\s+NO\s+FIELDS|VALUE\s+CHECK|VISIBLE\s+LENGTH|HEADER\s+LINE|COMMON\s+PART)\b", KEYWORD),
        Rule::token(r"(?im)(^|(?<=(\s|\.)))(ABBREVIATED|ABSTRACT|ADD|ALIASES|ALIGN|ALPHA|ASSERT|AS|ASSIGN(ING)?|AT(\s+FIRST)?|BACK|BLOCK|BREAK-POINT|CASE|CAST|CATCH|CHANGING|CHECK|CLASS|CLEAR|COLLECT|COLOR|COMMIT|COND|CONV|CREATE|COMMUNICATION|COMPONENTS?|COMPUTE|CONCATENATE|CONDENSE|CONSTANTS|CONTEXTS|CONTINUE|CONTROLS|COUNTRY|CURRENCY|DATA|DATE|DECIMALS|DEFAULT|DEFINE|DEFINITION|DEFERRED|DEMAND|DETAIL|DIRECTORY|DIVIDE|DO|DUMMY|ELSE(IF)?|ENDAT|ENDCASE|ENDCATCH|ENDCLASS|ENDDO|ENDFORM|ENDFUNCTION|ENDIF|ENDINTERFACE|ENDLOOP|ENDMETHOD|ENDMODULE|ENDSELECT|ENDTRY|ENDWHILE|ENHANCEMENT|EVENTS|EXACT|EXCEPTIONS?|EXIT|EXPONENT|EXPORT|EXPORTING|EXTRACT|FETCH|FIELDS?|FOR|FORM|FORMAT|FREE|FROM|FUNCTION|HIDE|ID|IF|IMPORT|IMPLEMENTATION|IMPORTING|IN|INCLUDE|INCLUDING|INDEX|INFOTYPES|INITIALIZATION|INTERFACE|INTERFACES|INTO|LANGUAGE|LEAVE|LENGTH|LINES|LOAD|LOCAL|JOIN|KEY|NEW|NEXT|MAXIMUM|MESSAGE|METHOD[S]?|MINIMUM|MODULE|MODIFIER|MODIFY|MOVE|MULTIPLY|NODES|NUMBER|OBLIGATORY|OBJECT|OF|OFF|ON|OTHERS|OVERLAY|PACK|PAD|PARAMETERS|PERCENTAGE|POSITION|PROGRAM|PROVIDE|PUBLIC|PUT|PF\d\d|RAISE|RAISING|RANGES?|READ|RECEIVE|REDEFINITION|REFRESH|REJECT|REPORT|RESERVE|REF|RESUME|RETRY|RETURN|RETURNING|RIGHT|ROLLBACK|REPLACE|SCROLL|SEARCH|SELECT|SHIFT|SIGN|SINGLE|SIZE|SKIP|SORT|SPLIT|STATICS|STOP|STYLE|SUBMATCHES|SUBMIT|SUBTRACT|SUM(?!\()|SUMMARY|SUMMING|SUPPLY|SWITCH|TABLE|TABLES|TIMESTAMP|TIMES?|TIMEZONE|TITLE|\??TO|TOP-OF-PAGE|TRANSFER|TRANSLATE|TRY|TYPES|ULINE|UNDER|UNPACK|UPDATE|USING|VALUE|VALUES|VIA|VARYING|VARY|WAIT|WHEN|WHERE|WIDTH|WHILE|WITH|WINDOW|WRITE|XSD|ZERO)\b", KEYWORD),
        Rule::bygroups(r"(?im)(abs|acos|asin|atan|boolc|boolx|bit_set|char_off|charlen|ceil|cmax|cmin|condense|contains|contains_any_of|contains_any_not_of|concat_lines_of|cos|cosh|count|count_any_of|count_any_not_of|dbmaxlen|distance|escape|exp|find|find_end|find_any_of|find_any_not_of|floor|frac|from_mixed|insert|lines|log|log10|match|matches|nmax|nmin|numofchar|repeat|replace|rescale|reverse|round|segment|shift_left|shift_right|sign|sin|sinh|sqrt|strlen|substring|substring_after|substring_from|substring_before|substring_to|tan|tanh|to_upper|to_lower|to_mixed|translate|trunc|xstrlen)(\()\b", vec![Some(NAME_BUILTIN), Some(PUNCTUATION)]),
        Rule::token(r"(?im)&[0-9]", NAME),
        Rule::token(r"(?im)[0-9]+", NUMBER_INTEGER),
        Rule::token(r"(?im)(?<=(\s|.))(AND|OR|EQ|NE|GT|LT|GE|LE|CO|CN|CA|NA|CS|NOT|NS|CP|NP|BYTE-CO|BYTE-CN|BYTE-CA|BYTE-NA|BYTE-CS|BYTE-NS|IS\s+(NOT\s+)?(INITIAL|ASSIGNED|REQUESTED|BOUND))\b", OPERATOR_WORD),
        Rule::token(r"(?im)<\S+>", NAME_VARIABLE),
        Rule::token(r"(?im)\w[\w~]*(?:(\[\])|->\*)?", NAME_VARIABLE),
        Rule::token(r"(?im)[?*<>=\-+&]", OPERATOR),
        Rule::token(r"(?im)'(''|[^'])*'", STRING_SINGLE),
        Rule::token(r"(?im)`([^`])*`", STRING_SINGLE),
        Rule::bygroups(r"(?im)([|}])([^{}|]*?)([|{])", vec![Some(PUNCTUATION), Some(STRING_SINGLE), Some(PUNCTUATION)]),
        Rule::token(r"(?im)[/;:()\[\],.]", PUNCTUATION),
        Rule::bygroups(r"(?im)(!)(\w+)", vec![Some(OPERATOR), Some(NAME)]),
    ]);
    Table(m)
}

impl Lexer for AbapLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
