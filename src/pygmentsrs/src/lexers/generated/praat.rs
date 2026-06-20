//! AUTO-GENERATED from `pygments.pygments.lexers.praat:PraatLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.praat:PraatLexer:praat

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: praat
pub struct PraatLexer;

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
        Rule::bygroups(r"(?m)(\s+)(#.*?$)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)^#.*?$", COMMENT_SINGLE),
        Rule::token(r"(?m);[^\n]*", COMMENT_SINGLE),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r"(?m)\bprocedure\b", KEYWORD, NewState::Push(vec![r"procedure_definition"])),
        Rule::token_to(r"(?m)\bcall\b", KEYWORD, NewState::Push(vec![r"procedure_call"])),
        Rule::token_to(r"(?m)@", NAME_FUNCTION, NewState::Push(vec![r"procedure_call"])),
        Rule::token_to(r"(?m)(backslashTrigraphsToUnicode|choose(?:Directory|(?:Read|Write)File)|d(?:ate|emoKey|o)|e(?:nvironment|xtract(?:Line|Word))|fixed|info|left|mid|percent|r(?:e(?:adFile|place(?:(?:_regex)?))|ight)|s(?:elected|tring)|unicodeToBackslashTrigraphs)\$(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(linear|random(?:Gauss|Integer|Uniform)|zero)#(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(a(?:bs|ppend(?:File(?:(?:Line)?)|Info(?:(?:Line)?))|rc(?:cos(?:(?:h)?)|sin(?:(?:h)?)|tan(?:(?:[2h])?)))|b(?:arkToHertz|e(?:gin(?:Pause|SendPraat)|ssel(?:[IK])|ta(?:(?:2)?))|inomial(?:[PQ])|oolean)|c(?:eiling|h(?:iSquare(?:[PQ])|oice)|o(?:mment|s(?:(?:h)?))|reateDirectory)|d(?:e(?:leteFile|mo(?:C(?:licked(?:(?:In)?)|ommandKeyPressed)|ExtraControlKeyPressed|Input|KeyPressed|OptionKeyPressed|Sh(?:iftKeyPressed|ow)|W(?:aitForInput|indowTitle)|[XY]))|ifferenceLimensToPhon|o)|e(?:ditor|nd(?:Pause|SendPraat|sWith)|r(?:bToHertz|fc|[bf])|x(?:itScript|p|tractNumber))|f(?:i(?:leReadable|sher(?:[PQ]))|loor)|gauss(?:[PQ])|hertzTo(?:Bark|Erb|Mel|Semitones)|i(?:m(?:ax|in)|n(?:complete(?:Beta|GammaP)|dex(?:(?:_regex)?)|teger|v(?:Binomial(?:[PQ])|ChiSquareQ|FisherQ|GaussQ|S(?:igmoid|tudentQ))))|l(?:ength|n(?:(?:(?:Bet|Gamm)a)?)|og(?:10|2))|m(?:ax|elToHertz|in(?:(?:usObject)?))|n(?:atural|umber(?:(?:Of(?:Columns|Rows|Selected))?))|o(?:bjectsAreIdentical|ption(?:(?:Menu)?))|p(?:auseScript|honToDifferenceLimens|lusObject|ositive)|r(?:andom(?:Binomial|Gauss|Integer|Poisson|Uniform)|e(?:a(?:dFile|l)|moveObject)|index(?:(?:_regex)?)|ound|unS(?:cript|ystem(?:(?:_nocheck)?)))|s(?:e(?:lect(?:Object|ed)|mitonesToHertz|ntence(?:(?:text)?))|i(?:gmoid|n(?:(?:cpi|[ch])?))|oundPressureToPhon|qrt|t(?:artsWith|udent(?:[PQ])))|t(?:an(?:(?:h)?)|ext)|variableExists|w(?:ord|rite(?:File(?:(?:Line)?)|Info(?:(?:Line)?))))(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token(r"(?m)(assert|clearinfo|demo|e(?:ditor|l(?:if|s(?:e|if))|nd(?:editor|for|if|proc|while))|f(?:i|or|rom)|if|minus|no(?:check|progress|warn)|plus|repeat|s(?:elect|topwatch)|t(?:hen|o)|until|while)\b", KEYWORD),
        Rule::bygroups_to(r"(?m)(\bform\b)(\s+)([^\n]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(STRING)], NewState::Push(vec![r"old_form"])),
        Rule::bygroups_to(r"(?m)(print(?:line|tab)?|echo|exit|asserterror|pause|send(?:praat|socket)|include|execute|system(?:_nocheck)?)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"string_unquoted"])),
        Rule::bygroups(r"(?m)(goto|label)(\s+)(\w+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_LABEL)]),
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token(r"(?m)(defaultDirectory|homeDirectory|newline|pr(?:aatVersion|eferencesDirectory)|shellDirectory|t(?:ab|emporaryDirectory))\$", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)(e|macintosh|p(?:i|raatVersion)|un(?:defined|ix)|windows)(?=[^a-zA-Z0-9_."\'$#\[:(]|\s|^|$)"#, NAME_VARIABLE_GLOBAL),
        Rule::bygroups_to(r"(?m)\b(A(?:ctivation|ffineTransform|mplitudeTier|rt(?:(?:word)?)|utosegment)|Bark(?:Filter|Spectrogram)|C(?:CA|ategories|epstr(?:ogram|um(?:(?:c)?))|hebyshevSeries|lassificationTable|o(?:chleagram|llection|mplexSpectrogram|n(?:f(?:(?:igurat|us)ion)|tingencyTable)|r(?:pus|relation)|variance)|rossCorrelationTable(?:(?:s)?))|D(?:TW|ataModeler|i(?:agonalizer|s(?:criminant|similarity|t(?:ance|ributions)))|urationTier)|E(?:EG|RP(?:(?:Tier)?)|dit(?:(?:Costs|Distance)Table)|igen|x(?:citation(?:(?:s)?)|perimentMFC))|F(?:FNet|eatureWeights|ile(?:(?:(?:s)?)InMemory)|ormant(?:(?:Filter|Grid|Modeler|Point|Tier)?))|GaussianMixture|H(?:MM(?:(?:_(?:Observation(?:(?:Sequence)?)|State(?:(?:Sequence)?)))?)|armonicity)|I(?:Spline|n(?:dex|te(?:nsity(?:(?:Tier)?)|rvalTier)))|K(?:NN|latt(?:Grid|Table))|L(?:FCC|PC|abel|egendreSeries|inearRegression|o(?:gisticRegression|ngSound)|tas)|M(?:FCC|Spline|a(?:n(?:Pages|ipulation)|trix)|el(?:Filter|Spectrogram)|ixingMatrix|ovie)|Network|O(?:T(?:Grammar|History|Multi)|bject)|P(?:CA|a(?:irDistribution|ramCurve|ttern)|ermutation|hoto|itch(?:(?:(?:Model|Ti)er)?)|o(?:intProcess|ly(?:gon|nomial)|werCepstr(?:(?:ogra|u)m))|rocrustes)|R(?:e(?:al(?:Point|Tier)|sultsMFC)|oots)|S(?:PINET|SCP|VD|alience|calarProduct|im(?:ilarity|pleString)|o(?:rtedSetOfString|und)|pe(?:aker|ctr(?:ogram|um(?:(?:Tier)?))|(?:echSynthesiz|llingCheck)er)|trings(?:(?:Index)?))|T(?:able(?:(?:OfReal)?)|ext(?:Grid|Interval|Point|Tier)|ier|ransition)|VocalTract(?:(?:Tier)?)|W(?:(?:eigh|ordLis)t))(_)", vec![Some(NAME_BUILTIN), Some(NAME_BUILTIN)], NewState::Push(vec![r"object_reference"])),
        Rule::token(r"(?m)\.?_?[a-z][\w.]*(\$|#)?", TEXT),
        Rule::token_to(r"(?m)[\[\]]", PUNCTUATION, NewState::Push(vec![r"comma_list"])),
        Rule::token(r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#, STRING_INTERPOL),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token_to(r"(?m)(A(?:ctivation|ffineTransform|mplitudeTier|rt(?:(?:word)?)|utosegment)|Bark(?:Filter|Spectrogram)|C(?:CA|ategories|epstr(?:ogram|um(?:(?:c)?))|hebyshevSeries|lassificationTable|o(?:chleagram|llection|mplexSpectrogram|n(?:f(?:(?:igurat|us)ion)|tingencyTable)|r(?:pus|relation)|variance)|rossCorrelationTable(?:(?:s)?))|D(?:TW|ataModeler|i(?:agonalizer|s(?:criminant|similarity|t(?:ance|ributions)))|urationTier)|E(?:EG|RP(?:(?:Tier)?)|dit(?:(?:Costs|Distance)Table)|igen|x(?:citation(?:(?:s)?)|perimentMFC))|F(?:FNet|eatureWeights|ile(?:(?:(?:s)?)InMemory)|ormant(?:(?:Filter|Grid|Modeler|Point|Tier)?))|GaussianMixture|H(?:MM(?:(?:_(?:Observation(?:(?:Sequence)?)|State(?:(?:Sequence)?)))?)|armonicity)|I(?:Spline|n(?:dex|te(?:nsity(?:(?:Tier)?)|rvalTier)))|K(?:NN|latt(?:Grid|Table))|L(?:FCC|PC|abel|egendreSeries|inearRegression|o(?:gisticRegression|ngSound)|tas)|M(?:FCC|Spline|a(?:n(?:Pages|ipulation)|trix)|el(?:Filter|Spectrogram)|ixingMatrix|ovie)|Network|O(?:T(?:Grammar|History|Multi)|bject)|P(?:CA|a(?:irDistribution|ramCurve|ttern)|ermutation|hoto|itch(?:(?:(?:Model|Ti)er)?)|o(?:intProcess|ly(?:gon|nomial)|werCepstr(?:(?:ogra|u)m))|rocrustes)|R(?:e(?:al(?:Point|Tier)|sultsMFC)|oots)|S(?:PINET|SCP|VD|alience|calarProduct|im(?:ilarity|pleString)|o(?:rtedSetOfString|und)|pe(?:aker|ctr(?:ogram|um(?:(?:Tier)?))|(?:echSynthesiz|llingCheck)er)|trings(?:(?:Index)?))|T(?:able(?:(?:OfReal)?)|ext(?:Grid|Interval|Point|Tier)|ier|ransition)|VocalTract(?:(?:Tier)?)|W(?:(?:eigh|ordLis)t))(?=\s+\S+\n)", NAME_CLASS, NewState::Push(vec![r"string_unquoted"])),
        Rule::token_to(r"(?m)\b[A-Z]", KEYWORD, NewState::Push(vec![r"command"])),
        Rule::token(r"(?m)(\.{3}|[)(,])", PUNCTUATION),
    ]);
    m.insert(r"function_call", vec![
        Rule::token_to(r"(?m)(backslashTrigraphsToUnicode|choose(?:Directory|(?:Read|Write)File)|d(?:ate|emoKey|o)|e(?:nvironment|xtract(?:Line|Word))|fixed|info|left|mid|percent|r(?:e(?:adFile|place(?:(?:_regex)?))|ight)|s(?:elected|tring)|unicodeToBackslashTrigraphs)\$(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(linear|random(?:Gauss|Integer|Uniform)|zero)#(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(a(?:bs|ppend(?:File(?:(?:Line)?)|Info(?:(?:Line)?))|rc(?:cos(?:(?:h)?)|sin(?:(?:h)?)|tan(?:(?:[2h])?)))|b(?:arkToHertz|e(?:gin(?:Pause|SendPraat)|ssel(?:[IK])|ta(?:(?:2)?))|inomial(?:[PQ])|oolean)|c(?:eiling|h(?:iSquare(?:[PQ])|oice)|o(?:mment|s(?:(?:h)?))|reateDirectory)|d(?:e(?:leteFile|mo(?:C(?:licked(?:(?:In)?)|ommandKeyPressed)|ExtraControlKeyPressed|Input|KeyPressed|OptionKeyPressed|Sh(?:iftKeyPressed|ow)|W(?:aitForInput|indowTitle)|[XY]))|ifferenceLimensToPhon|o)|e(?:ditor|nd(?:Pause|SendPraat|sWith)|r(?:bToHertz|fc|[bf])|x(?:itScript|p|tractNumber))|f(?:i(?:leReadable|sher(?:[PQ]))|loor)|gauss(?:[PQ])|hertzTo(?:Bark|Erb|Mel|Semitones)|i(?:m(?:ax|in)|n(?:complete(?:Beta|GammaP)|dex(?:(?:_regex)?)|teger|v(?:Binomial(?:[PQ])|ChiSquareQ|FisherQ|GaussQ|S(?:igmoid|tudentQ))))|l(?:ength|n(?:(?:(?:Bet|Gamm)a)?)|og(?:10|2))|m(?:ax|elToHertz|in(?:(?:usObject)?))|n(?:atural|umber(?:(?:Of(?:Columns|Rows|Selected))?))|o(?:bjectsAreIdentical|ption(?:(?:Menu)?))|p(?:auseScript|honToDifferenceLimens|lusObject|ositive)|r(?:andom(?:Binomial|Gauss|Integer|Poisson|Uniform)|e(?:a(?:dFile|l)|moveObject)|index(?:(?:_regex)?)|ound|unS(?:cript|ystem(?:(?:_nocheck)?)))|s(?:e(?:lect(?:Object|ed)|mitonesToHertz|ntence(?:(?:text)?))|i(?:gmoid|n(?:(?:cpi|[ch])?))|oundPressureToPhon|qrt|t(?:artsWith|udent(?:[PQ])))|t(?:an(?:(?:h)?)|ext)|variableExists|w(?:ord|rite(?:File(?:(?:Line)?)|Info(?:(?:Line)?))))(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
    ]);
    m.insert(r"variable_name", vec![
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token(r"(?m)(defaultDirectory|homeDirectory|newline|pr(?:aatVersion|eferencesDirectory)|shellDirectory|t(?:ab|emporaryDirectory))\$", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)(e|macintosh|p(?:i|raatVersion)|un(?:defined|ix)|windows)(?=[^a-zA-Z0-9_."\'$#\[:(]|\s|^|$)"#, NAME_VARIABLE_GLOBAL),
        Rule::bygroups_to(r"(?m)\b(A(?:ctivation|ffineTransform|mplitudeTier|rt(?:(?:word)?)|utosegment)|Bark(?:Filter|Spectrogram)|C(?:CA|ategories|epstr(?:ogram|um(?:(?:c)?))|hebyshevSeries|lassificationTable|o(?:chleagram|llection|mplexSpectrogram|n(?:f(?:(?:igurat|us)ion)|tingencyTable)|r(?:pus|relation)|variance)|rossCorrelationTable(?:(?:s)?))|D(?:TW|ataModeler|i(?:agonalizer|s(?:criminant|similarity|t(?:ance|ributions)))|urationTier)|E(?:EG|RP(?:(?:Tier)?)|dit(?:(?:Costs|Distance)Table)|igen|x(?:citation(?:(?:s)?)|perimentMFC))|F(?:FNet|eatureWeights|ile(?:(?:(?:s)?)InMemory)|ormant(?:(?:Filter|Grid|Modeler|Point|Tier)?))|GaussianMixture|H(?:MM(?:(?:_(?:Observation(?:(?:Sequence)?)|State(?:(?:Sequence)?)))?)|armonicity)|I(?:Spline|n(?:dex|te(?:nsity(?:(?:Tier)?)|rvalTier)))|K(?:NN|latt(?:Grid|Table))|L(?:FCC|PC|abel|egendreSeries|inearRegression|o(?:gisticRegression|ngSound)|tas)|M(?:FCC|Spline|a(?:n(?:Pages|ipulation)|trix)|el(?:Filter|Spectrogram)|ixingMatrix|ovie)|Network|O(?:T(?:Grammar|History|Multi)|bject)|P(?:CA|a(?:irDistribution|ramCurve|ttern)|ermutation|hoto|itch(?:(?:(?:Model|Ti)er)?)|o(?:intProcess|ly(?:gon|nomial)|werCepstr(?:(?:ogra|u)m))|rocrustes)|R(?:e(?:al(?:Point|Tier)|sultsMFC)|oots)|S(?:PINET|SCP|VD|alience|calarProduct|im(?:ilarity|pleString)|o(?:rtedSetOfString|und)|pe(?:aker|ctr(?:ogram|um(?:(?:Tier)?))|(?:echSynthesiz|llingCheck)er)|trings(?:(?:Index)?))|T(?:able(?:(?:OfReal)?)|ext(?:Grid|Interval|Point|Tier)|ier|ransition)|VocalTract(?:(?:Tier)?)|W(?:(?:eigh|ordLis)t))(_)", vec![Some(NAME_BUILTIN), Some(NAME_BUILTIN)], NewState::Push(vec![r"object_reference"])),
        Rule::token(r"(?m)\.?_?[a-z][\w.]*(\$|#)?", TEXT),
        Rule::token_to(r"(?m)[\[\]]", PUNCTUATION, NewState::Push(vec![r"comma_list"])),
        Rule::token(r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#, STRING_INTERPOL),
    ]);
    m.insert(
        r"operator",
        vec![
            Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
            Rule::token(
                r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])",
                OPERATOR_WORD,
            ),
        ],
    );
    m.insert(
        r"number",
        vec![
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        ],
    );
    m.insert(
        r"string_interpolated",
        vec![Rule::token(
            r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#,
            STRING_INTERPOL,
        )],
    );
    m.insert(
        r"command",
        vec![
            Rule::token(r"(?m)( ?[\w()-]+ ?)", KEYWORD),
            Rule::token(
                r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#,
                STRING_INTERPOL,
            ),
            Rule::token_to(
                r"(?m)\.{3}",
                KEYWORD,
                NewState::Push(vec![r"#pop", r"old_arguments"]),
            ),
            Rule::token_to(
                r"(?m):",
                KEYWORD,
                NewState::Push(vec![r"#pop", r"comma_list"]),
            ),
            Rule::token_to(r"(?m)\s", WHITESPACE, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"procedure_call",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)([\w.]+)(?:(:)|(?:(\s*)(\()))",
                vec![
                    Some(NAME_FUNCTION),
                    Some(PUNCTUATION),
                    Some(WHITESPACE),
                    Some(PUNCTUATION),
                ],
                NewState::Pop(1),
            ),
            Rule::token_to(
                r"(?m)([\w.]+)",
                NAME_FUNCTION,
                NewState::Push(vec![r"#pop", r"old_arguments"]),
            ),
        ],
    );
    m.insert(
        r"procedure_definition",
        vec![
            Rule::token(r"(?m)\s", WHITESPACE),
            Rule::bygroups_to(
                r"(?m)([\w.]+)(\s*?[(:])",
                vec![Some(NAME_FUNCTION), Some(WHITESPACE)],
                NewState::Pop(1),
            ),
            Rule::bygroups_to(
                r"(?m)([\w.]+)([^\n]*)",
                vec![Some(NAME_FUNCTION), Some(TEXT)],
                NewState::Pop(1),
            ),
        ],
    );
    m.insert(
        r"function",
        vec![
            Rule::token(r"(?m)\s+", WHITESPACE),
            Rule::token_to(
                r"(?m):",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"comma_list"]),
            ),
            Rule::token_to(
                r"(?m)\s*\(",
                PUNCTUATION,
                NewState::Push(vec![r"#pop", r"comma_list"]),
            ),
        ],
    );
    m.insert(r"comma_list", vec![
        Rule::bygroups(r"(?m)(\s*\n\s*)(\.{3})", vec![Some(WHITESPACE), Some(PUNCTUATION)]),
        Rule::bygroups_to(r"(?m)(\s*)(?:([)\]])|(\n))", vec![Some(WHITESPACE), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Pop(1)),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)\b(if|then|else|fi|endif)\b", KEYWORD),
        Rule::token_to(r"(?m)(backslashTrigraphsToUnicode|choose(?:Directory|(?:Read|Write)File)|d(?:ate|emoKey|o)|e(?:nvironment|xtract(?:Line|Word))|fixed|info|left|mid|percent|r(?:e(?:adFile|place(?:(?:_regex)?))|ight)|s(?:elected|tring)|unicodeToBackslashTrigraphs)\$(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(linear|random(?:Gauss|Integer|Uniform)|zero)#(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token_to(r"(?m)(a(?:bs|ppend(?:File(?:(?:Line)?)|Info(?:(?:Line)?))|rc(?:cos(?:(?:h)?)|sin(?:(?:h)?)|tan(?:(?:[2h])?)))|b(?:arkToHertz|e(?:gin(?:Pause|SendPraat)|ssel(?:[IK])|ta(?:(?:2)?))|inomial(?:[PQ])|oolean)|c(?:eiling|h(?:iSquare(?:[PQ])|oice)|o(?:mment|s(?:(?:h)?))|reateDirectory)|d(?:e(?:leteFile|mo(?:C(?:licked(?:(?:In)?)|ommandKeyPressed)|ExtraControlKeyPressed|Input|KeyPressed|OptionKeyPressed|Sh(?:iftKeyPressed|ow)|W(?:aitForInput|indowTitle)|[XY]))|ifferenceLimensToPhon|o)|e(?:ditor|nd(?:Pause|SendPraat|sWith)|r(?:bToHertz|fc|[bf])|x(?:itScript|p|tractNumber))|f(?:i(?:leReadable|sher(?:[PQ]))|loor)|gauss(?:[PQ])|hertzTo(?:Bark|Erb|Mel|Semitones)|i(?:m(?:ax|in)|n(?:complete(?:Beta|GammaP)|dex(?:(?:_regex)?)|teger|v(?:Binomial(?:[PQ])|ChiSquareQ|FisherQ|GaussQ|S(?:igmoid|tudentQ))))|l(?:ength|n(?:(?:(?:Bet|Gamm)a)?)|og(?:10|2))|m(?:ax|elToHertz|in(?:(?:usObject)?))|n(?:atural|umber(?:(?:Of(?:Columns|Rows|Selected))?))|o(?:bjectsAreIdentical|ption(?:(?:Menu)?))|p(?:auseScript|honToDifferenceLimens|lusObject|ositive)|r(?:andom(?:Binomial|Gauss|Integer|Poisson|Uniform)|e(?:a(?:dFile|l)|moveObject)|index(?:(?:_regex)?)|ound|unS(?:cript|ystem(?:(?:_nocheck)?)))|s(?:e(?:lect(?:Object|ed)|mitonesToHertz|ntence(?:(?:text)?))|i(?:gmoid|n(?:(?:cpi|[ch])?))|oundPressureToPhon|qrt|t(?:artsWith|udent(?:[PQ])))|t(?:an(?:(?:h)?)|ext)|variableExists|w(?:ord|rite(?:File(?:(?:Line)?)|Info(?:(?:Line)?))))(?=\s*[:(])", NAME_FUNCTION, NewState::Push(vec![r"function"])),
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token(r"(?m)(defaultDirectory|homeDirectory|newline|pr(?:aatVersion|eferencesDirectory)|shellDirectory|t(?:ab|emporaryDirectory))\$", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)(e|macintosh|p(?:i|raatVersion)|un(?:defined|ix)|windows)(?=[^a-zA-Z0-9_."\'$#\[:(]|\s|^|$)"#, NAME_VARIABLE_GLOBAL),
        Rule::bygroups_to(r"(?m)\b(A(?:ctivation|ffineTransform|mplitudeTier|rt(?:(?:word)?)|utosegment)|Bark(?:Filter|Spectrogram)|C(?:CA|ategories|epstr(?:ogram|um(?:(?:c)?))|hebyshevSeries|lassificationTable|o(?:chleagram|llection|mplexSpectrogram|n(?:f(?:(?:igurat|us)ion)|tingencyTable)|r(?:pus|relation)|variance)|rossCorrelationTable(?:(?:s)?))|D(?:TW|ataModeler|i(?:agonalizer|s(?:criminant|similarity|t(?:ance|ributions)))|urationTier)|E(?:EG|RP(?:(?:Tier)?)|dit(?:(?:Costs|Distance)Table)|igen|x(?:citation(?:(?:s)?)|perimentMFC))|F(?:FNet|eatureWeights|ile(?:(?:(?:s)?)InMemory)|ormant(?:(?:Filter|Grid|Modeler|Point|Tier)?))|GaussianMixture|H(?:MM(?:(?:_(?:Observation(?:(?:Sequence)?)|State(?:(?:Sequence)?)))?)|armonicity)|I(?:Spline|n(?:dex|te(?:nsity(?:(?:Tier)?)|rvalTier)))|K(?:NN|latt(?:Grid|Table))|L(?:FCC|PC|abel|egendreSeries|inearRegression|o(?:gisticRegression|ngSound)|tas)|M(?:FCC|Spline|a(?:n(?:Pages|ipulation)|trix)|el(?:Filter|Spectrogram)|ixingMatrix|ovie)|Network|O(?:T(?:Grammar|History|Multi)|bject)|P(?:CA|a(?:irDistribution|ramCurve|ttern)|ermutation|hoto|itch(?:(?:(?:Model|Ti)er)?)|o(?:intProcess|ly(?:gon|nomial)|werCepstr(?:(?:ogra|u)m))|rocrustes)|R(?:e(?:al(?:Point|Tier)|sultsMFC)|oots)|S(?:PINET|SCP|VD|alience|calarProduct|im(?:ilarity|pleString)|o(?:rtedSetOfString|und)|pe(?:aker|ctr(?:ogram|um(?:(?:Tier)?))|(?:echSynthesiz|llingCheck)er)|trings(?:(?:Index)?))|T(?:able(?:(?:OfReal)?)|ext(?:Grid|Interval|Point|Tier)|ier|ransition)|VocalTract(?:(?:Tier)?)|W(?:(?:eigh|ordLis)t))(_)", vec![Some(NAME_BUILTIN), Some(NAME_BUILTIN)], NewState::Push(vec![r"object_reference"])),
        Rule::token(r"(?m)\.?_?[a-z][\w.]*(\$|#)?", TEXT),
        Rule::token_to(r"(?m)[\[\]]", PUNCTUATION, NewState::Push(vec![r"comma_list"])),
        Rule::token(r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#, STRING_INTERPOL),
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token(r"(?m)[()]", TEXT),
        Rule::token(r"(?m),", PUNCTUATION),
    ]);
    m.insert(r"old_arguments", vec![
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token(r"(?m)(defaultDirectory|homeDirectory|newline|pr(?:aatVersion|eferencesDirectory)|shellDirectory|t(?:ab|emporaryDirectory))\$", NAME_VARIABLE_GLOBAL),
        Rule::token(r#"(?m)(e|macintosh|p(?:i|raatVersion)|un(?:defined|ix)|windows)(?=[^a-zA-Z0-9_."\'$#\[:(]|\s|^|$)"#, NAME_VARIABLE_GLOBAL),
        Rule::bygroups_to(r"(?m)\b(A(?:ctivation|ffineTransform|mplitudeTier|rt(?:(?:word)?)|utosegment)|Bark(?:Filter|Spectrogram)|C(?:CA|ategories|epstr(?:ogram|um(?:(?:c)?))|hebyshevSeries|lassificationTable|o(?:chleagram|llection|mplexSpectrogram|n(?:f(?:(?:igurat|us)ion)|tingencyTable)|r(?:pus|relation)|variance)|rossCorrelationTable(?:(?:s)?))|D(?:TW|ataModeler|i(?:agonalizer|s(?:criminant|similarity|t(?:ance|ributions)))|urationTier)|E(?:EG|RP(?:(?:Tier)?)|dit(?:(?:Costs|Distance)Table)|igen|x(?:citation(?:(?:s)?)|perimentMFC))|F(?:FNet|eatureWeights|ile(?:(?:(?:s)?)InMemory)|ormant(?:(?:Filter|Grid|Modeler|Point|Tier)?))|GaussianMixture|H(?:MM(?:(?:_(?:Observation(?:(?:Sequence)?)|State(?:(?:Sequence)?)))?)|armonicity)|I(?:Spline|n(?:dex|te(?:nsity(?:(?:Tier)?)|rvalTier)))|K(?:NN|latt(?:Grid|Table))|L(?:FCC|PC|abel|egendreSeries|inearRegression|o(?:gisticRegression|ngSound)|tas)|M(?:FCC|Spline|a(?:n(?:Pages|ipulation)|trix)|el(?:Filter|Spectrogram)|ixingMatrix|ovie)|Network|O(?:T(?:Grammar|History|Multi)|bject)|P(?:CA|a(?:irDistribution|ramCurve|ttern)|ermutation|hoto|itch(?:(?:(?:Model|Ti)er)?)|o(?:intProcess|ly(?:gon|nomial)|werCepstr(?:(?:ogra|u)m))|rocrustes)|R(?:e(?:al(?:Point|Tier)|sultsMFC)|oots)|S(?:PINET|SCP|VD|alience|calarProduct|im(?:ilarity|pleString)|o(?:rtedSetOfString|und)|pe(?:aker|ctr(?:ogram|um(?:(?:Tier)?))|(?:echSynthesiz|llingCheck)er)|trings(?:(?:Index)?))|T(?:able(?:(?:OfReal)?)|ext(?:Grid|Interval|Point|Tier)|ier|ransition)|VocalTract(?:(?:Tier)?)|W(?:(?:eigh|ordLis)t))(_)", vec![Some(NAME_BUILTIN), Some(NAME_BUILTIN)], NewState::Push(vec![r"object_reference"])),
        Rule::token(r"(?m)\.?_?[a-z][\w.]*(\$|#)?", TEXT),
        Rule::token_to(r"(?m)[\[\]]", PUNCTUATION, NewState::Push(vec![r"comma_list"])),
        Rule::token(r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#, STRING_INTERPOL),
        Rule::token(r"(?m)([+\/*<>=!-]=?|[&*|][&*|]?|\^|<>)", OPERATOR),
        Rule::token(r"(?m)(?<![\w.])(and|or|not|div|mod)(?![\w.])", OPERATOR_WORD),
        Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
        Rule::token(r"(?m)\b\d+(\.\d*)?([eE][-+]?\d+)?%?", NUMBER),
        Rule::token_to(r#"(?m)""#, STRING, NewState::Push(vec![r"string"])),
        Rule::token(r"(?m)[^\n]", TEXT),
    ]);
    m.insert(
        r"object_reference",
        vec![
            Rule::token(
                r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#,
                STRING_INTERPOL,
            ),
            Rule::token(r"(?m)([a-z][a-zA-Z0-9_]*|\d+)", NAME_BUILTIN),
            Rule::token_to(
                r"(?m)\.(d(?:[xy])|n(?:col|row|[xy])|xm(?:ax|in)|ym(?:ax|in))",
                NAME_BUILTIN,
                NewState::Pop(1),
            ),
            Rule::token(r"(?m)\$", NAME_BUILTIN),
            Rule::token_to(r"(?m)\[", TEXT, NewState::Pop(1)),
        ],
    );
    m.insert(
        r"string_unquoted",
        vec![
            Rule::bygroups(
                r"(?m)(\n\s*)(\.{3})",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::token_to(r"(?m)\n", WHITESPACE, NewState::Pop(1)),
            Rule::token(r"(?m)\s", WHITESPACE),
            Rule::token(
                r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#,
                STRING_INTERPOL,
            ),
            Rule::token(r"(?m)'", STRING),
            Rule::token(r"(?m)[^'\n]+", STRING),
        ],
    );
    m.insert(
        r"string",
        vec![
            Rule::bygroups(
                r"(?m)(\n\s*)(\.{3})",
                vec![Some(WHITESPACE), Some(PUNCTUATION)],
            ),
            Rule::token_to(r#"(?m)""#, STRING, NewState::Pop(1)),
            Rule::token(
                r#"(?m)\'[_a-z][^\[\]\'":]*(\[([\d,]+|"[\w,]+")\])?(:[0-9]+)?\'"#,
                STRING_INTERPOL,
            ),
            Rule::token(r"(?m)'", STRING),
            Rule::token(r#"(?m)[^\'"\n]+"#, STRING),
        ],
    );
    m.insert(r"old_form", vec![
        Rule::bygroups(r"(?m)(\s+)(#.*?$)", vec![Some(WHITESPACE), Some(COMMENT_SINGLE)]),
        Rule::token(r"(?m)\s+", WHITESPACE),
        Rule::bygroups_to(r"(?m)(optionmenu|choice)([ \t]+)(\S+)(:)([ \t]+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(TEXT), Some(PUNCTUATION), Some(WHITESPACE)], NewState::Push(vec![r"number"])),
        Rule::bygroups_to(r"(?m)(option|button)([ \t]+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"string_unquoted"])),
        Rule::bygroups_to(r"(?m)(sentence|text)([ \t]+)(\S+)", vec![Some(KEYWORD), Some(WHITESPACE), Some(STRING)], NewState::Push(vec![r"string_unquoted"])),
        Rule::bygroups(r"(?m)(word)([ \t]+)(\S+)([ \t]*)(\S+)?(?:([ \t]+)(.*))?", vec![Some(KEYWORD), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(TEXT), Some(WHITESPACE), Some(TEXT)]),
        Rule::bygroups(r#"(?m)(boolean)(\s+\S+\s*)(0|1|"?(?:yes|no)"?)"#, vec![Some(KEYWORD), Some(WHITESPACE), Some(NAME_VARIABLE)]),
        Rule::bygroups(r"(?m)(real|natural|positive|integer)([ \t]+\S+[ \t]*)([+-]?)(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?%?)", vec![Some(KEYWORD), Some(WHITESPACE), Some(OPERATOR), Some(NUMBER)]),
        Rule::bygroups_to(r"(?m)(comment)(\s+)", vec![Some(KEYWORD), Some(WHITESPACE)], NewState::Push(vec![r"string_unquoted"])),
        Rule::token_to(r"(?m)\bendform\b", KEYWORD, NewState::Pop(1)),
    ]);
    Table(m)
}

impl Lexer for PraatLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
