//! AUTO-GENERATED from `pygments.pygments.lexers.testing:GherkinLexer`.
//!
//! Do not edit by hand. Regenerate with:
//!   python tools/gen_lexer.py pygments.lexers.testing:GherkinLexer:gherkin

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::lexer::Lexer;
use crate::lexer::engine::{NewState, Rule, StateTable, tokenize};
use crate::token::*;

/// Aliases: gherkin, cucumber
pub struct GherkinLexer;

struct Table(HashMap<&'static str, Vec<Rule>>);

impl StateTable for Table {
    fn state(&self, name: &str) -> Option<&[Rule]> {
        self.0.get(name).map(Vec::as_slice)
    }
}

static TABLE: OnceLock<Table> = OnceLock::new();

fn build_table() -> Table {
    let mut m: HashMap<&'static str, Vec<Rule>> = HashMap::new();
    m.insert(r"comments", vec![Rule::token(r"(?m)^\s*#.*$", COMMENT)]);
    m.insert(r"feature_elements", vec![
        Rule::token_to(r"(?m)^(\s*)(하지만|조건|먼저|만일|만약|단|그리고|그러면|那麼|那么|而且|當|当|前提|假設|假设|假如|假定|但是|但し|並且|并且|同時|同时|もし|ならば|ただし|しかし|かつ|و |متى |لكن |عندما |ثم |بفرض |اذاً |כאשר |וגם |בהינתן |אזי |אז |אבל |Якщо |Унда |То |Припустимо, що |Припустимо |Онда |Но |Нехай |Лекин |Когато |Када |Кад |К тому же |И |Задато |Задати |Задате |Если |Допустим |Дадено |Ва |Бирок |Аммо |Али |Але |Агар |А |І |Și |És |Zatati |Zakładając |Zadato |Zadate |Zadano |Zadani |Zadan |Youse know when youse got |Youse know like when |Yna |Ya know how |Ya gotta |Y |Wun |Wtedy |When y'all |When |Wenn |WEN |Và |Ve |Und |Un |Thì |Then y'all |Then |Tapi |Tak |Tada |Tad |Så |Stel |Soit |Siis |Si |Sed |Se |Quando |Quand |Quan |Pryd |Pokud |Pokiaľ |Però |Pero |Pak |Oraz |Onda |Ond |Oletetaan |Og |Och |O zaman |Når |När |Niin |Nhưng |N |Mutta |Men |Mas |Maka |Majd |Mais |Maar |Ma |Lorsque |Lorsqu'|Kun |Kuid |Kui |Khi |Keď |Ketika |Když |Kaj |Kai |Kada |Kad |Jeżeli |Ja |Ir |I CAN HAZ |I |Ha |Givun |Givet |Given y'all |Given |Gitt |Gegeven |Gegeben sei |Fakat |Eğer ki |Etant donné |Et |Então |Entonces |Entao |En |Eeldades |E |Duota |Dun |Donitaĵo |Donat |Donada |Do |Diyelim ki |Dengan |Den youse gotta |De |Dato |Dar |Dann |Dan |Dado |Dacă |Daca |DEN |Când |Cuando |Cho |Cept |Cand |Cal |But y'all |But |Buh |Biết |Bet |BUT |Atès |Atunci |Atesa |Anrhegedig a |Angenommen |And y'all |And |An |Ama |Als |Alors |Allora |Ali |Aleshores |Ale |Akkor |Aber |AN |A také |A |\* )", KEYWORD, NewState::Push(vec![r"step_content_stack"])),
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
        Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
    ]);
    m.insert(r"feature_elements_on_stack", vec![
        Rule::token_to(r"(?m)^(\s*)(하지만|조건|먼저|만일|만약|단|그리고|그러면|那麼|那么|而且|當|当|前提|假設|假设|假如|假定|但是|但し|並且|并且|同時|同时|もし|ならば|ただし|しかし|かつ|و |متى |لكن |عندما |ثم |بفرض |اذاً |כאשר |וגם |בהינתן |אזי |אז |אבל |Якщо |Унда |То |Припустимо, що |Припустимо |Онда |Но |Нехай |Лекин |Когато |Када |Кад |К тому же |И |Задато |Задати |Задате |Если |Допустим |Дадено |Ва |Бирок |Аммо |Али |Але |Агар |А |І |Și |És |Zatati |Zakładając |Zadato |Zadate |Zadano |Zadani |Zadan |Youse know when youse got |Youse know like when |Yna |Ya know how |Ya gotta |Y |Wun |Wtedy |When y'all |When |Wenn |WEN |Và |Ve |Und |Un |Thì |Then y'all |Then |Tapi |Tak |Tada |Tad |Så |Stel |Soit |Siis |Si |Sed |Se |Quando |Quand |Quan |Pryd |Pokud |Pokiaľ |Però |Pero |Pak |Oraz |Onda |Ond |Oletetaan |Og |Och |O zaman |Når |När |Niin |Nhưng |N |Mutta |Men |Mas |Maka |Majd |Mais |Maar |Ma |Lorsque |Lorsqu'|Kun |Kuid |Kui |Khi |Keď |Ketika |Když |Kaj |Kai |Kada |Kad |Jeżeli |Ja |Ir |I CAN HAZ |I |Ha |Givun |Givet |Given y'all |Given |Gitt |Gegeven |Gegeben sei |Fakat |Eğer ki |Etant donné |Et |Então |Entonces |Entao |En |Eeldades |E |Duota |Dun |Donitaĵo |Donat |Donada |Do |Diyelim ki |Dengan |Den youse gotta |De |Dato |Dar |Dann |Dan |Dado |Dacă |Daca |DEN |Când |Cuando |Cho |Cept |Cand |Cal |But y'all |But |Buh |Biết |Bet |BUT |Atès |Atunci |Atesa |Anrhegedig a |Angenommen |And y'all |And |An |Ama |Als |Alors |Allora |Ali |Aleshores |Ale |Akkor |Aber |AN |A také |A |\* )", KEYWORD, NewState::Pop(2)),
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
        Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
    ]);
    m.insert(
        r"examples_table",
        vec![
            Rule::token_to(
                r"(?m)\s+\|",
                KEYWORD,
                NewState::Push(vec![r"examples_table_header"]),
            ),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
        ],
    );
    m.insert(
        r"examples_table_header",
        vec![
            Rule::token_to(r"(?m)\s+\|\s*$", KEYWORD, NewState::Pop(2)),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)\\\|", NAME_VARIABLE),
            Rule::token(r"(?m)\s*\|", KEYWORD),
            Rule::token(r"(?m)[^|]", NAME_VARIABLE),
        ],
    );
    m.insert(r"scenario_sections_on_stack", vec![
        Rule::bygroups_to(r"(?m)^(\s*)(시나리오 개요|시나리오|배경|背景|場景大綱|場景|场景大纲|场景|劇本大綱|劇本|剧本大纲|剧本|テンプレ|シナリオテンプレート|シナリオテンプレ|シナリオアウトライン|シナリオ|سيناريو مخطط|سيناريو|الخلفية|תרחיש|תבנית תרחיש|רקע|Тарих|Сценарій|Сценарио|Сценарий структураси|Сценарий|Структура сценарію|Структура сценарија|Структура сценария|Скица|Рамка на сценарий|Пример|Предыстория|Предистория|Позадина|Передумова|Основа|Концепт|Контекст|Założenia|Wharrimean is|Tình huống|The thing of it is|Tausta|Taust|Tapausaihio|Tapaus|Szenariogrundriss|Szenario|Szablon scenariusza|Stsenaarium|Struktura scenarija|Skica|Skenario konsep|Skenario|Situācija|Senaryo taslağı|Senaryo|Scénář|Scénario|Schema dello scenario|Scenārijs pēc parauga|Scenārijs|Scenár|Scenaro|Scenariusz|Scenariul de şablon|Scenariul de sablon|Scenariu|Scenario Outline|Scenario Amlinellol|Scenario|Scenarijus|Scenarijaus šablonas|Scenarij|Scenarie|Rerefons|Raamstsenaarium|Primer|Pozadí|Pozadina|Pozadie|Plan du scénario|Plan du Scénario|Osnova scénáře|Osnova|Náčrt Scénáře|Náčrt Scenáru|Mate|MISHUN SRSLY|MISHUN|Kịch bản|Konturo de la scenaro|Kontext|Konteksts|Kontekstas|Kontekst|Koncept|Khung tình huống|Khung kịch bản|Háttér|Grundlage|Geçmiş|Forgatókönyv vázlat|Forgatókönyv|Fono|Esquema do Cenário|Esquema do Cenario|Esquema del escenario|Esquema de l'escenari|Escenario|Escenari|Dis is what went down|Dasar|Contexto|Contexte|Contesto|Condiţii|Conditii|Cenário|Cenario|Cefndir|Bối cảnh|Blokes|Bakgrunn|Bakgrund|Baggrund|Background|B4|Antecedents|Antecedentes|All y'all|Achtergrond|Abstrakt Scenario|Abstract Scenario)(:)(.*)$", vec![Some(NAME_FUNCTION), Some(KEYWORD), Some(KEYWORD), Some(NAME_FUNCTION)], NewState::Push(vec![r"feature_elements_on_stack"])),
    ]);
    m.insert(r"narrative", vec![
        Rule::bygroups_to(r"(?m)^(\s*)(시나리오 개요|시나리오|배경|背景|場景大綱|場景|场景大纲|场景|劇本大綱|劇本|剧本大纲|剧本|テンプレ|シナリオテンプレート|シナリオテンプレ|シナリオアウトライン|シナリオ|سيناريو مخطط|سيناريو|الخلفية|תרחיש|תבנית תרחיש|רקע|Тарих|Сценарій|Сценарио|Сценарий структураси|Сценарий|Структура сценарію|Структура сценарија|Структура сценария|Скица|Рамка на сценарий|Пример|Предыстория|Предистория|Позадина|Передумова|Основа|Концепт|Контекст|Założenia|Wharrimean is|Tình huống|The thing of it is|Tausta|Taust|Tapausaihio|Tapaus|Szenariogrundriss|Szenario|Szablon scenariusza|Stsenaarium|Struktura scenarija|Skica|Skenario konsep|Skenario|Situācija|Senaryo taslağı|Senaryo|Scénář|Scénario|Schema dello scenario|Scenārijs pēc parauga|Scenārijs|Scenár|Scenaro|Scenariusz|Scenariul de şablon|Scenariul de sablon|Scenariu|Scenario Outline|Scenario Amlinellol|Scenario|Scenarijus|Scenarijaus šablonas|Scenarij|Scenarie|Rerefons|Raamstsenaarium|Primer|Pozadí|Pozadina|Pozadie|Plan du scénario|Plan du Scénario|Osnova scénáře|Osnova|Náčrt Scénáře|Náčrt Scenáru|Mate|MISHUN SRSLY|MISHUN|Kịch bản|Konturo de la scenaro|Kontext|Konteksts|Kontekstas|Kontekst|Koncept|Khung tình huống|Khung kịch bản|Háttér|Grundlage|Geçmiş|Forgatókönyv vázlat|Forgatókönyv|Fono|Esquema do Cenário|Esquema do Cenario|Esquema del escenario|Esquema de l'escenari|Escenario|Escenari|Dis is what went down|Dasar|Contexto|Contexte|Contesto|Condiţii|Conditii|Cenário|Cenario|Cefndir|Bối cảnh|Blokes|Bakgrunn|Bakgrund|Baggrund|Background|B4|Antecedents|Antecedentes|All y'all|Achtergrond|Abstrakt Scenario|Abstract Scenario)(:)(.*)$", vec![Some(NAME_FUNCTION), Some(KEYWORD), Some(KEYWORD), Some(NAME_FUNCTION)], NewState::Push(vec![r"feature_elements_on_stack"])),
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
        Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
    ]);
    m.insert(
        r"table_vars",
        vec![Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE)],
    );
    m.insert(
        r"numbers",
        vec![Rule::token(
            r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?",
            STRING,
        )],
    );
    m.insert(
        r"string",
        vec![
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\s|.)", STRING),
        ],
    );
    m.insert(
        r"py_string",
        vec![
            Rule::token_to(r#"(?m)""""#, KEYWORD, NewState::Pop(1)),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\s|.)", STRING),
        ],
    );
    m.insert(
        r"step_content_root",
        vec![
            Rule::token_to(r"(?m)$", KEYWORD, NewState::Pop(1)),
            Rule::token_to(
                r#"(?m)""#,
                NAME_FUNCTION,
                NewState::Push(vec![r"double_string"]),
            ),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", STRING),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
        ],
    );
    m.insert(
        r"step_content",
        vec![
            Rule::token_to(
                r#"(?m)""#,
                NAME_FUNCTION,
                NewState::Push(vec![r"double_string"]),
            ),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", STRING),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
        ],
    );
    m.insert(
        r"step_content_stack",
        vec![
            Rule::token_to(r"(?m)$", KEYWORD, NewState::Pop(2)),
            Rule::token_to(
                r#"(?m)""#,
                NAME_FUNCTION,
                NewState::Push(vec![r"double_string"]),
            ),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", STRING),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
        ],
    );
    m.insert(
        r"table_content",
        vec![
            Rule::token_to(r"(?m)\s+\|\s*$", KEYWORD, NewState::Pop(1)),
            Rule::token(r"(?m)^\s*#.*$", COMMENT),
            Rule::token(r"(?m)\\\|", STRING),
            Rule::token(r"(?m)\s*\|", KEYWORD),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\s|.)", STRING),
        ],
    );
    m.insert(
        r"double_string",
        vec![
            Rule::token_to(r#"(?m)""#, NAME_FUNCTION, NewState::Pop(1)),
            Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
            Rule::token(r"(?m)(\s|.)", STRING),
        ],
    );
    m.insert(r"root", vec![
        Rule::token(r"(?m)\n", NAME_FUNCTION),
        Rule::token(r"(?m)^\s*#.*$", COMMENT),
        Rule::token_to(r#"(?m)""""#, KEYWORD, NewState::Push(vec![r"py_string"])),
        Rule::token_to(r"(?m)\s+\|", KEYWORD, NewState::Push(vec![r"table_content"])),
        Rule::token_to(r#"(?m)""#, NAME_FUNCTION, NewState::Push(vec![r"double_string"])),
        Rule::token(r"(?m)(<[^>]+>)", NAME_VARIABLE),
        Rule::token(r"(?m)(\d+\.?\d*|\d*\.\d+)([eE][+-]?[0-9]+)?", STRING),
        Rule::bygroups(r"(?m)(\s*)(@[^@\r\n\t ]+)", vec![Some(NAME_FUNCTION), Some(NAME_TAG)]),
        Rule::bygroups_to(r"(?m)^(\s*)(하지만|조건|먼저|만일|만약|단|그리고|그러면|那麼|那么|而且|當|当|前提|假設|假设|假如|假定|但是|但し|並且|并且|同時|同时|もし|ならば|ただし|しかし|かつ|و |متى |لكن |عندما |ثم |بفرض |اذاً |כאשר |וגם |בהינתן |אזי |אז |אבל |Якщо |Унда |То |Припустимо, що |Припустимо |Онда |Но |Нехай |Лекин |Когато |Када |Кад |К тому же |И |Задато |Задати |Задате |Если |Допустим |Дадено |Ва |Бирок |Аммо |Али |Але |Агар |А |І |Și |És |Zatati |Zakładając |Zadato |Zadate |Zadano |Zadani |Zadan |Youse know when youse got |Youse know like when |Yna |Ya know how |Ya gotta |Y |Wun |Wtedy |When y'all |When |Wenn |WEN |Và |Ve |Und |Un |Thì |Then y'all |Then |Tapi |Tak |Tada |Tad |Så |Stel |Soit |Siis |Si |Sed |Se |Quando |Quand |Quan |Pryd |Pokud |Pokiaľ |Però |Pero |Pak |Oraz |Onda |Ond |Oletetaan |Og |Och |O zaman |Når |När |Niin |Nhưng |N |Mutta |Men |Mas |Maka |Majd |Mais |Maar |Ma |Lorsque |Lorsqu'|Kun |Kuid |Kui |Khi |Keď |Ketika |Když |Kaj |Kai |Kada |Kad |Jeżeli |Ja |Ir |I CAN HAZ |I |Ha |Givun |Givet |Given y'all |Given |Gitt |Gegeven |Gegeben sei |Fakat |Eğer ki |Etant donné |Et |Então |Entonces |Entao |En |Eeldades |E |Duota |Dun |Donitaĵo |Donat |Donada |Do |Diyelim ki |Dengan |Den youse gotta |De |Dato |Dar |Dann |Dan |Dado |Dacă |Daca |DEN |Când |Cuando |Cho |Cept |Cand |Cal |But y'all |But |Buh |Biết |Bet |BUT |Atès |Atunci |Atesa |Anrhegedig a |Angenommen |And y'all |And |An |Ama |Als |Alors |Allora |Ali |Aleshores |Ale |Akkor |Aber |AN |A také |A |\* )", vec![Some(NAME_FUNCTION), Some(KEYWORD)], NewState::Push(vec![r"step_content_root"])),
        Rule::bygroups_to(r"(?m)^(기능|機能|功能|フィーチャ|خاصية|תכונה|Функціонал|Функционалност|Функционал|Фича|Особина|Могућност|Özellik|Właściwość|Tính năng|Trajto|Savybė|Požiadavka|Požadavek|Osobina|Ominaisuus|Omadus|OH HAI|Mogućnost|Mogucnost|Jellemző|Fīča|Funzionalità|Funktionalität|Funkcionalnost|Funkcionalitāte|Funcționalitate|Functionaliteit|Functionalitate|Funcionalitat|Funcionalidade|Fonctionnalité|Fitur|Feature|Egenskap|Egenskab|Crikey|Característica|Arwedd)(:)(.*)$", vec![Some(KEYWORD), Some(KEYWORD), Some(NAME_FUNCTION)], NewState::Push(vec![r"narrative"])),
        Rule::bygroups_to(r"(?m)^(\s*)(시나리오 개요|시나리오|배경|背景|場景大綱|場景|场景大纲|场景|劇本大綱|劇本|剧本大纲|剧本|テンプレ|シナリオテンプレート|シナリオテンプレ|シナリオアウトライン|シナリオ|سيناريو مخطط|سيناريو|الخلفية|תרחיש|תבנית תרחיש|רקע|Тарих|Сценарій|Сценарио|Сценарий структураси|Сценарий|Структура сценарію|Структура сценарија|Структура сценария|Скица|Рамка на сценарий|Пример|Предыстория|Предистория|Позадина|Передумова|Основа|Концепт|Контекст|Założenia|Wharrimean is|Tình huống|The thing of it is|Tausta|Taust|Tapausaihio|Tapaus|Szenariogrundriss|Szenario|Szablon scenariusza|Stsenaarium|Struktura scenarija|Skica|Skenario konsep|Skenario|Situācija|Senaryo taslağı|Senaryo|Scénář|Scénario|Schema dello scenario|Scenārijs pēc parauga|Scenārijs|Scenár|Scenaro|Scenariusz|Scenariul de şablon|Scenariul de sablon|Scenariu|Scenario Outline|Scenario Amlinellol|Scenario|Scenarijus|Scenarijaus šablonas|Scenarij|Scenarie|Rerefons|Raamstsenaarium|Primer|Pozadí|Pozadina|Pozadie|Plan du scénario|Plan du Scénario|Osnova scénáře|Osnova|Náčrt Scénáře|Náčrt Scenáru|Mate|MISHUN SRSLY|MISHUN|Kịch bản|Konturo de la scenaro|Kontext|Konteksts|Kontekstas|Kontekst|Koncept|Khung tình huống|Khung kịch bản|Háttér|Grundlage|Geçmiş|Forgatókönyv vázlat|Forgatókönyv|Fono|Esquema do Cenário|Esquema do Cenario|Esquema del escenario|Esquema de l'escenari|Escenario|Escenari|Dis is what went down|Dasar|Contexto|Contexte|Contesto|Condiţii|Conditii|Cenário|Cenario|Cefndir|Bối cảnh|Blokes|Bakgrunn|Bakgrund|Baggrund|Background|B4|Antecedents|Antecedentes|All y'all|Achtergrond|Abstrakt Scenario|Abstract Scenario)(:)(.*)$", vec![Some(NAME_FUNCTION), Some(KEYWORD), Some(KEYWORD), Some(NAME_FUNCTION)], NewState::Push(vec![r"feature_elements"])),
        Rule::bygroups_to(r"(?m)^(\s*)(예|例子|例|サンプル|امثلة|דוגמאות|Сценарији|Примери|Приклади|Мисоллар|Значения|Örnekler|Voorbeelden|Variantai|Tapaukset|Scenarios|Scenariji|Scenarijai|Příklady|Példák|Príklady|Przykłady|Primjeri|Primeri|Piemēri|Pavyzdžiai|Paraugs|Juhtumid|Exemplos|Exemples|Exemplele|Exempel|Examples|Esempi|Enghreifftiau|Ekzemploj|Eksempler|Ejemplos|EXAMPLZ|Dữ liệu|Contoh|Cobber|Beispiele)(:)(.*)$", vec![Some(NAME_FUNCTION), Some(KEYWORD), Some(KEYWORD), Some(NAME_FUNCTION)], NewState::Push(vec![r"examples_table"])),
        Rule::token(r"(?m)(\s|.)", NAME_FUNCTION),
    ]);
    Table(m)
}

impl Lexer for GherkinLexer {
    fn get_tokens(&self, code: &str) -> Vec<(TokenType, String)> {
        let table = TABLE.get_or_init(build_table);
        tokenize(table, code)
    }
}
