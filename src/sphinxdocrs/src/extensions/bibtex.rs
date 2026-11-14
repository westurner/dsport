//! BibTeX parsing and normalized bibliography records.

use std::collections::{BTreeMap, HashMap};
#[cfg(feature = "csl-citeproc")]
use std::fs;
#[cfg(feature = "csl-citeproc")]
use std::path::Path;

use biblatex::ChunksExt;

#[cfg(feature = "csl-citeproc")]
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationName {
    pub family: String,
    pub given: String,
    pub prefix: String,
    pub suffix: String,
    pub use_prefix: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationRecord {
    pub id: String,
    pub entry_type: String,
    pub fields: BTreeMap<String, String>,
    pub authors: Vec<CitationName>,
    pub editors: Vec<CitationName>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BibliographyError {
    #[error("failed to parse BibTeX: {0}")]
    Parse(String),
    #[error("invalid {field} field in citation {id}: {message}")]
    InvalidField {
        id: String,
        field: String,
        message: String,
    },
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bibliography {
    records: Vec<CitationRecord>,
    by_id: HashMap<String, usize>,
}

pub const METADATA: super::BuiltinMetadata = super::BuiltinMetadata {
    name: "sphinxdocrs::extensions::bibtex",
    version: "0.1.0",
    parallel_read_safe: false,
    parallel_write_safe: false,
};

pub fn register_parser_components() {
    for role in [
        "cite",
        "cite:p",
        "cite:t",
        "cite:ct",
        "cite:alp",
        "cite:author",
        "cite:authorpar",
        "cite:cauthor",
        "cite:cauthorpar",
        "cite:empty",
        "cite:year",
        "cite:yearpar",
        "footcite",
        "footcite:p",
        "footcite:t",
        "footcite:ct",
    ] {
        docutilsrs::roles::register_canonical_role(role);
        docutilsrs::plugins::register_native_role(role, |role_name, content| {
            let mut attrs = BTreeMap::new();
            attrs.insert(
                "role".to_string(),
                docutilsrs::doctree::NodeAttributeValue::String(role_name.to_string()),
            );
            attrs.insert(
                "keys".to_string(),
                docutilsrs::doctree::NodeAttributeValue::String(content.to_string()),
            );
            Some(docutilsrs::doctree::NodeKind::Extension {
                class_name: "BibtexCitation".to_string(),
                attrs,
            })
        });
    }

    docutilsrs::plugins::register_native_directive("bibliography", |_arguments, _content| {
        Some(vec![docutilsrs::parser::Block::Extension {
            class_name: "BibtexBibliography".to_string(),
            attrs: BTreeMap::new(),
            children: Vec::new(),
        }])
    });
    docutilsrs::plugins::register_native_directive("footbibliography", |_arguments, _content| {
        Some(vec![docutilsrs::parser::Block::Extension {
            class_name: "BibtexFootBibliography".to_string(),
            attrs: BTreeMap::new(),
            children: Vec::new(),
        }])
    });
}

pub fn setup(_app: &mut crate::application::SphinxApp) -> Result<(), crate::application::AppError> {
    register_parser_components();
    Ok(())
}

#[cfg(feature = "csl-citeproc")]
pub fn build_finished(
    app: &crate::application::SphinxApp,
) -> Result<(), crate::application::AppError> {
    use citeproc::prelude::{IncludeUncited, SupportedFormat};

    let bibliography = load_configured_bibliography(app)?;
    if bibliography.is_empty() {
        return Ok(());
    }
    let style =
        load_configured_style(app)?.unwrap_or_else(|| super::csl::DEFAULT_STYLE.to_string());
    let rendered = super::csl::render_with_uncited(
        &bibliography,
        &style,
        &[],
        SupportedFormat::Plain,
        IncludeUncited::All,
    )
    .map_err(|error| crate::application::AppError::Extension(error.to_string()))?;
    let tex = bibliography_tex(&rendered.bibliography);
    let jsonld = serde_json::to_string_pretty(&bibliography_jsonld(&bibliography))
        .map_err(|error| crate::application::AppError::Extension(error.to_string()))?;

    fs::write(app.outdir.join("bibliography.tex"), tex)?;
    fs::write(app.outdir.join("bibliography.jsonld"), jsonld)?;
    Ok(())
}

#[cfg(feature = "csl-citeproc")]
fn load_configured_bibliography(
    app: &crate::application::SphinxApp,
) -> Result<Bibliography, crate::application::AppError> {
    let paths = app
        .config
        .get("bibtex_bibfiles")
        .and_then(|value| value.as_list().map(|values| values.to_vec()))
        .unwrap_or_default();
    let mut source = String::new();
    for value in paths {
        let Some(configured_path) = value.as_str() else {
            continue;
        };
        let configured_path = Path::new(configured_path);
        let path = if configured_path.is_absolute() {
            configured_path.to_path_buf()
        } else {
            app.srcdir.join(configured_path)
        };
        let content = crate::environment::read_source_file(&path, &app.config.source_encoding())
            .map_err(|error| {
                crate::application::AppError::Extension(format!(
                    "failed to read bibliography {}: {error}",
                    path.display()
                ))
            })?;
        source.push_str(&content);
        source.push('\n');
    }
    Bibliography::parse(&source)
        .map_err(|error| crate::application::AppError::Extension(error.to_string()))
}

#[cfg(feature = "csl-citeproc")]
fn load_configured_style(
    app: &crate::application::SphinxApp,
) -> Result<Option<String>, crate::application::AppError> {
    let Some(configured_path) = app
        .config
        .get("bibtex_csl_style")
        .and_then(|value| value.as_str().map(str::to_owned))
        .filter(|value| !value.trim().is_empty())
    else {
        return Ok(None);
    };
    let configured_path = Path::new(&configured_path);
    let path = if configured_path.is_absolute() {
        configured_path.to_path_buf()
    } else {
        app.srcdir.join(configured_path)
    };
    crate::environment::read_source_file(&path, &app.config.source_encoding())
        .map(Some)
        .map_err(|error| {
            crate::application::AppError::Extension(format!(
                "failed to read CSL style {}: {error}",
                path.display()
            ))
        })
}

#[cfg(feature = "csl-citeproc")]
fn bibliography_tex(entries: &[super::csl::RenderedBibliographyEntry]) -> String {
    let width = entries.len().max(1).to_string();
    let mut output = format!(
        "\\documentclass{{article}}\n\\begin{{document}}\n\\begin{{thebibliography}}{{{width}}}\n"
    );
    let mut labels = std::collections::HashSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let label = if !entry.key.is_empty()
            && entry
                .key
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || "-_.:".contains(character))
            && labels.insert(entry.key.clone())
        {
            entry.key.clone()
        } else {
            let mut fallback = format!("sfref{}", index + 1);
            while !labels.insert(fallback.clone()) {
                fallback.push('_');
            }
            fallback
        };
        output.push_str(&format!(
            "\\bibitem{{{label}}} {}\n",
            latex_escape(&entry.value)
        ));
    }
    output.push_str("\\end{thebibliography}\n\\end{document}\n");
    output
}

#[cfg(feature = "csl-citeproc")]
fn latex_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        escaped.push_str(match character {
            '&' => r"\&",
            '%' => r"\%",
            '$' => r"\$",
            '#' => r"\#",
            '_' => r"\_",
            '{' => r"\{",
            '}' => r"\}",
            '~' => r"\textasciitilde{}",
            '^' => r"\textasciicircum{}",
            '\\' => r"\textbackslash{}",
            _ => {
                escaped.push(character);
                continue;
            }
        });
    }
    escaped
}

#[cfg(feature = "csl-citeproc")]
fn bibliography_jsonld(bibliography: &Bibliography) -> serde_json::Value {
    let graph: Vec<_> = bibliography.records().iter().map(record_jsonld).collect();
    serde_json::json!({
        "@context": { "@vocab": "https://schema.org/" },
        "@graph": graph,
    })
}

#[cfg(feature = "csl-citeproc")]
fn record_jsonld(record: &CitationRecord) -> serde_json::Value {
    let mut value = serde_json::Map::new();
    let doi = record.fields.get("doi").map(|doi| {
        if doi.starts_with("http://") || doi.starts_with("https://") {
            doi.clone()
        } else {
            format!("https://doi.org/{doi}")
        }
    });
    let record_url = doi
        .clone()
        .or_else(|| record.fields.get("url").cloned())
        .unwrap_or_else(|| format!("urn:bibtex:{}", record.id.replace(' ', "%20")));
    value.insert("@id".into(), serde_json::Value::String(record_url.clone()));
    value.insert(
        "@type".into(),
        serde_json::Value::String(jsonld_type(&record.entry_type).into()),
    );
    value.insert("url".into(), serde_json::Value::String(record_url));

    for (field, property) in [
        ("title", "name"),
        ("abstract", "abstract"),
        ("year", "datePublished"),
        ("date", "datePublished"),
        ("volume", "volumeNumber"),
        ("number", "issueNumber"),
        ("pages", "pagination"),
        ("keywords", "keywords"),
    ] {
        if let Some(text) = record
            .fields
            .get(field)
            .map(String::as_str)
            .filter(|text| !text.trim().is_empty())
        {
            value.insert(property.into(), serde_json::Value::String(text.into()));
        }
    }

    if let Some(journal) = record
        .fields
        .get("journal")
        .or_else(|| record.fields.get("journaltitle"))
    {
        value.insert(
            "isPartOf".into(),
            serde_json::json!({ "@type": "Periodical", "name": journal }),
        );
    } else if let Some(booktitle) = record.fields.get("booktitle") {
        value.insert(
            "isPartOf".into(),
            serde_json::json!({ "@type": "Book", "name": booktitle }),
        );
    }
    if let Some(publisher) = record.fields.get("publisher") {
        value.insert(
            "publisher".into(),
            serde_json::json!({ "@type": "Organization", "name": publisher }),
        );
    }
    if !record.authors.is_empty() {
        value.insert(
            "author".into(),
            serde_json::Value::Array(record.authors.iter().map(person_jsonld).collect()),
        );
    }
    if !record.editors.is_empty() {
        value.insert(
            "editor".into(),
            serde_json::Value::Array(record.editors.iter().map(person_jsonld).collect()),
        );
    }
    if let Some(doi) = doi {
        value.insert("sameAs".into(), serde_json::Value::String(doi));
        value.insert(
            "identifier".into(),
            serde_json::json!({ "@type": "PropertyValue", "propertyID": "DOI", "value": record.fields.get("doi").unwrap() }),
        );
    }
    serde_json::Value::Object(value)
}

#[cfg(feature = "csl-citeproc")]
fn person_jsonld(person: &CitationName) -> serde_json::Value {
    let mut name = Vec::new();
    for part in [
        &person.given,
        &person.prefix,
        &person.family,
        &person.suffix,
    ] {
        if !part.trim().is_empty() {
            name.push(part.trim());
        }
    }
    let mut value = serde_json::Map::new();
    value.insert("@type".into(), serde_json::Value::String("Person".into()));
    value.insert("name".into(), serde_json::Value::String(name.join(" ")));
    if !person.given.is_empty() {
        value.insert(
            "givenName".into(),
            serde_json::Value::String(person.given.clone()),
        );
    }
    if !person.family.is_empty() {
        value.insert(
            "familyName".into(),
            serde_json::Value::String(person.family.clone()),
        );
    }
    serde_json::Value::Object(value)
}

#[cfg(feature = "csl-citeproc")]
fn jsonld_type(entry_type: &str) -> &'static str {
    match entry_type {
        "article" => "ScholarlyArticle",
        "book" | "collection" | "proceedings" => "Book",
        "inbook" | "incollection" | "inproceedings" => "Chapter",
        "mastersthesis" | "phdthesis" | "thesis" => "Thesis",
        "techreport" => "Report",
        _ => "CreativeWork",
    }
}

impl Bibliography {
    pub fn parse(source: &str) -> Result<Self, BibliographyError> {
        let parsed = biblatex::Bibliography::parse(source)
            .map_err(|error| BibliographyError::Parse(error.to_string()))?;
        let mut bibliography = Self::default();

        for entry in parsed.iter() {
            let authors = parse_people(entry, "author")?;
            let editors = parse_people(entry, "editor")?;
            let record = CitationRecord {
                id: entry.key.clone(),
                entry_type: entry.entry_type.to_string(),
                fields: entry
                    .fields
                    .iter()
                    .map(|(key, chunks)| (key.clone(), chunks.format_verbatim()))
                    .collect::<BTreeMap<_, _>>(),
                authors,
                editors,
            };
            bibliography
                .by_id
                .insert(record.id.clone(), bibliography.records.len());
            bibliography.records.push(record);
        }

        Ok(bibliography)
    }

    pub fn get(&self, id: &str) -> Option<&CitationRecord> {
        self.by_id
            .get(id)
            .and_then(|index| self.records.get(*index))
    }

    pub fn records(&self) -> &[CitationRecord] {
        &self.records
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }
}

fn parse_people(
    entry: &biblatex::Entry,
    field: &str,
) -> Result<Vec<CitationName>, BibliographyError> {
    if entry.get(field).is_none() {
        return Ok(Vec::new());
    }

    entry
        .get_as::<Vec<biblatex::Person>>(field)
        .map(|people| {
            people
                .into_iter()
                .map(|person| CitationName {
                    family: person.name,
                    given: person.given_name,
                    prefix: person.prefix,
                    suffix: person.suffix,
                    use_prefix: person.use_prefix,
                })
                .collect()
        })
        .map_err(|error| BibliographyError::InvalidField {
            id: entry.key.clone(),
            field: field.to_string(),
            message: error.to_string(),
        })
}

#[cfg(feature = "csl-citeproc")]
impl CitationRecord {
    pub fn to_csl_reference(&self) -> citeproc::io::Reference {
        use citeproc::csl::{DateVariable, NameVariable};
        use citeproc::io::{NumberLike, Reference};

        let mut reference = Reference::empty(
            citeproc::csl::Atom::from(self.id.as_str()),
            csl_type(&self.entry_type),
        );

        for (field, value) in &self.fields {
            let value = value.trim();
            if value.is_empty() {
                continue;
            }

            if let Some(variable) = ordinary_variable(field) {
                reference.ordinary.insert(variable, value.into());
            } else if let Some(variable) = number_variable(field) {
                reference
                    .number
                    .insert(variable, NumberLike::Str(value.into()));
            }
        }

        if !self.authors.is_empty() {
            reference.name.insert(
                NameVariable::Author,
                self.authors.iter().map(csl_name).collect(),
            );
        }
        if !self.editors.is_empty() {
            reference.name.insert(
                NameVariable::Editor,
                self.editors.iter().map(csl_name).collect(),
            );
        }

        if let Some(issued) = issued_date(&self.fields) {
            reference.date.insert(DateVariable::Issued, issued);
        }

        reference
    }
}

#[cfg(feature = "csl-citeproc")]
fn csl_name(name: &CitationName) -> citeproc::io::Name {
    use citeproc::io::{Name, PersonName};

    let non_dropping_particle = (name.use_prefix != Some(false) && !name.prefix.is_empty())
        .then(|| name.prefix.clone().into());
    let dropping_particle = (name.use_prefix == Some(false) && !name.prefix.is_empty())
        .then(|| name.prefix.clone().into());
    let is_latin_cyrillic = citeproc::io::unicode::is_latin_cyrillic(&format!(
        "{} {} {} {}",
        name.given, name.prefix, name.family, name.suffix
    ));

    Name::Person(PersonName {
        family: (!name.family.is_empty()).then(|| name.family.clone().into()),
        given: (!name.given.is_empty()).then(|| name.given.clone().into()),
        non_dropping_particle,
        dropping_particle,
        suffix: (!name.suffix.is_empty()).then(|| name.suffix.clone().into()),
        comma_suffix: !name.suffix.is_empty(),
        is_latin_cyrillic,
        ..Default::default()
    })
}

#[cfg(feature = "csl-citeproc")]
fn csl_type(entry_type: &str) -> citeproc::csl::CslType {
    use citeproc::csl::CslType;

    match entry_type {
        "article" => CslType::ArticleJournal,
        "book" | "collection" | "proceedings" => CslType::Book,
        "inbook" | "incollection" | "inproceedings" => CslType::Chapter,
        "manual" | "misc" | "online" | "www" => CslType::Webpage,
        "mastersthesis" | "phdthesis" | "thesis" => CslType::Thesis,
        "techreport" => CslType::Report,
        "unpublished" => CslType::Manuscript,
        "conference" => CslType::PaperConference,
        "booklet" => CslType::Pamphlet,
        _ => CslType::Article,
    }
}

#[cfg(feature = "csl-citeproc")]
fn ordinary_variable(field: &str) -> Option<citeproc::csl::Variable> {
    use citeproc::csl::Variable;

    Some(match field {
        "abstract" => Variable::Abstract,
        "booktitle" | "journal" | "journaltitle" => Variable::ContainerTitle,
        "doi" => Variable::DOI,
        "isbn" => Variable::ISBN,
        "issn" => Variable::ISSN,
        "keywords" => Variable::Keyword,
        "language" | "langid" => Variable::Language,
        "location" | "address" => Variable::PublisherPlace,
        "note" => Variable::Note,
        "publisher" => Variable::Publisher,
        "series" => Variable::CollectionTitle,
        "title" => Variable::Title,
        "url" => Variable::URL,
        _ => return None,
    })
}

#[cfg(feature = "csl-citeproc")]
fn number_variable(field: &str) -> Option<citeproc::csl::NumberVariable> {
    use citeproc::csl::NumberVariable;

    Some(match field {
        "chapter" => NumberVariable::ChapterNumber,
        "edition" => NumberVariable::Edition,
        "issue" => NumberVariable::Issue,
        "number" => NumberVariable::Number,
        "pages" => NumberVariable::Page,
        "volume" => NumberVariable::Volume,
        _ => return None,
    })
}

#[cfg(feature = "csl-citeproc")]
fn issued_date(fields: &BTreeMap<String, String>) -> Option<citeproc::io::DateOrRange> {
    use citeproc::io::{Date, DateOrRange};

    if let Some(date) = fields
        .get("date")
        .and_then(|value| DateOrRange::from_str(value.trim()).ok())
    {
        return Some(date);
    }

    let year = fields.get("year")?.trim().parse::<i32>().ok()?;
    let month = fields
        .get("month")
        .and_then(|value| bibtex_month(value.trim()));
    let day = fields
        .get("day")
        .and_then(|value| value.trim().parse::<u32>().ok());

    Some(DateOrRange::from(Date::new(
        year,
        month.unwrap_or(0),
        day.unwrap_or(0),
    )))
}

#[cfg(feature = "csl-citeproc")]
fn bibtex_month(month: &str) -> Option<u32> {
    if let Ok(month) = month.parse::<u32>() {
        return (1..=12).contains(&month).then_some(month);
    }

    match month.to_ascii_lowercase().as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::{Bibliography, BibliographyError};

    #[test]
    fn parses_entries_and_normalizes_fields() {
        let bibliography = Bibliography::parse(
            r#"@Article{smith2024, Author = {Ada Smith}, Title = {A {NASA} Study}, Year = 2024}"#,
        )
        .unwrap();

        assert_eq!(bibliography.len(), 1);
        let record = bibliography.get("smith2024").unwrap();
        assert_eq!(record.entry_type, "article");
        assert_eq!(record.fields.get("author").unwrap(), "Ada Smith");
        assert_eq!(record.fields.get("title").unwrap(), "A NASA Study");
        assert_eq!(record.fields.get("year").unwrap(), "2024");
    }

    #[test]
    fn resolves_crossref_fields() {
        let bibliography = Bibliography::parse(
            r#"
                @incollection{paper, author = {Ada Smith}, crossref = {collection}}
                @collection{collection, title = {Collected Testing}}
            "#,
        )
        .unwrap();

        assert_eq!(
            bibliography
                .get("paper")
                .unwrap()
                .fields
                .get("booktitle")
                .unwrap(),
            "Collected Testing"
        );
    }

    #[test]
    fn rejects_duplicate_citation_keys() {
        let error = Bibliography::parse("@book{same, title={First}} @book{same, title={Second}}")
            .unwrap_err();

        assert!(matches!(error, BibliographyError::Parse(_)));
        assert!(error.to_string().contains("same"));
    }

    #[test]
    fn rejects_malformed_input_without_panicking() {
        assert!(Bibliography::parse("@book{unfinished").is_err());
        assert!(Bibliography::parse("").unwrap().is_empty());
    }

    #[cfg(feature = "csl-citeproc")]
    #[test]
    fn tex_export_escapes_specials_and_resolves_invalid_or_duplicate_labels() {
        use crate::extensions::csl::RenderedBibliographyEntry;

        let entries = [
            RenderedBibliographyEntry {
                key: "sfref2".into(),
                value: "&%$#_{}~^\\ é".into(),
            },
            RenderedBibliographyEntry {
                key: "bad key".into(),
                value: "invalid label".into(),
            },
            RenderedBibliographyEntry {
                key: "same".into(),
                value: "first copy".into(),
            },
            RenderedBibliographyEntry {
                key: "same".into(),
                value: "duplicate copy".into(),
            },
            RenderedBibliographyEntry {
                key: String::new(),
                value: "empty label".into(),
            },
        ];
        let tex = super::bibliography_tex(&entries);

        assert!(tex.contains(
            r"\bibitem{sfref2} \&\%\$\#\_\{\}\textasciitilde{}\textasciicircum{}\textbackslash{} é"
        ));
        assert!(tex.contains(r"\bibitem{sfref2_} invalid label"));
        assert!(tex.contains(r"\bibitem{same} first copy"));
        assert!(tex.contains(r"\bibitem{sfref4} duplicate copy"));
        assert!(tex.contains(r"\bibitem{sfref5} empty label"));
        assert!(tex.contains(r"\begin{thebibliography}{5}"));
    }

    #[cfg(feature = "csl-citeproc")]
    #[test]
    fn jsonld_export_maps_fields_names_and_fallback_urls() {
        let bibliography = Bibliography::parse(
            r#"
                @article{journal,
                    author = {Ada Lovelace},
                    editor = {Grace Hopper},
                    title = {Journal Article},
                    abstract = {A short abstract},
                    year = {1843},
                    volume = {8},
                    number = {2},
                    pages = {10--24},
                    keywords = {math, computing},
                    journal = {Journal of Tests},
                    publisher = {Test Press},
                    doi = {https://doi.org/10.1000/journal}
                }
                @book{book,
                    author = {Madonna},
                    title = {Collected Work},
                    booktitle = {Collected Works},
                    url = {https://example.test/book}
                }
                @techreport{report,
                    title = {Research Report},
                    doi = {http://doi.org/10.1000/report}
                }
                @misc{rawdoi,
                    title = {Raw DOI},
                    doi = {10.1000/raw}
                }
                @misc{urn,
                    title = {Unlinked Work}
                }
            "#,
        )
        .unwrap();
        let jsonld = super::bibliography_jsonld(&bibliography);
        let graph = jsonld["@graph"].as_array().unwrap();
        let by_name = |name: &str| graph.iter().find(|entry| entry["name"] == name).unwrap();

        assert_eq!(jsonld["@context"]["@vocab"], "https://schema.org/");
        let journal = by_name("Journal Article");
        assert_eq!(journal["@type"], "ScholarlyArticle");
        assert_eq!(journal["url"], "https://doi.org/10.1000/journal");
        assert_eq!(journal["sameAs"], "https://doi.org/10.1000/journal");
        assert_eq!(
            journal["identifier"]["value"],
            "https://doi.org/10.1000/journal"
        );
        assert_eq!(journal["isPartOf"]["@type"], "Periodical");
        assert_eq!(journal["isPartOf"]["name"], "Journal of Tests");
        assert_eq!(journal["publisher"]["name"], "Test Press");
        assert_eq!(journal["author"][0]["name"], "Ada Lovelace");
        assert_eq!(journal["editor"][0]["name"], "Grace Hopper");
        assert_eq!(journal["datePublished"], "1843");
        assert_eq!(journal["volumeNumber"], "8");
        assert_eq!(journal["issueNumber"], "2");
        assert_eq!(journal["pagination"], "10–24");
        assert_eq!(journal["keywords"], "math, computing");
        assert_eq!(journal["abstract"], "A short abstract");

        let book = by_name("Collected Work");
        assert_eq!(book["url"], "https://example.test/book");
        assert_eq!(book["isPartOf"]["@type"], "Book");
        assert_eq!(book["author"][0]["familyName"], "Madonna");
        assert!(book["author"][0].get("givenName").is_none());

        assert_eq!(
            by_name("Research Report")["sameAs"],
            "http://doi.org/10.1000/report"
        );
        assert_eq!(by_name("Raw DOI")["url"], "https://doi.org/10.1000/raw");
        let unlinked = by_name("Unlinked Work");
        assert_eq!(unlinked["url"], "urn:bibtex:urn");
        assert!(unlinked.get("author").is_none());
        assert!(unlinked.get("editor").is_none());
        assert!(unlinked.get("isPartOf").is_none());
        assert!(unlinked.get("publisher").is_none());
        assert!(unlinked.get("sameAs").is_none());
    }

    #[cfg(feature = "csl-citeproc")]
    #[test]
    fn jsonld_type_covers_known_bibtex_entry_types() {
        for (bibtex, schema) in [
            ("article", "ScholarlyArticle"),
            ("book", "Book"),
            ("collection", "Book"),
            ("proceedings", "Book"),
            ("inbook", "Chapter"),
            ("incollection", "Chapter"),
            ("inproceedings", "Chapter"),
            ("mastersthesis", "Thesis"),
            ("phdthesis", "Thesis"),
            ("thesis", "Thesis"),
            ("techreport", "Report"),
            ("misc", "CreativeWork"),
        ] {
            assert_eq!(super::jsonld_type(bibtex), schema, "BibTeX type: {bibtex}");
        }
    }

    #[cfg(feature = "csl-citeproc")]
    #[test]
    fn jsonld_person_handles_missing_name_parts() {
        let person = super::person_jsonld(&super::CitationName {
            family: String::new(),
            given: "Ada".into(),
            prefix: " ".into(),
            suffix: String::new(),
            use_prefix: None,
        });

        assert_eq!(person["name"], "Ada");
        assert_eq!(person["givenName"], "Ada");
        assert!(person.get("familyName").is_none());
    }

    #[test]
    fn native_roles_and_bibliography_directive_emit_placeholders() {
        use docutilsrs::doctree::{NodeAttributeValue, NodeKind};

        super::register_parser_components();
        let tree = docutilsrs::parse_rst_for_sphinx(
            r#"See :cite:t:`1987:nelson` for an introduction to non-standard analysis.
Non-standard analysis is fun :cite:p:`1987:nelson`.
Escaped\ whitespace stays joined.

.. bibliography::

See :footcite:t:`1987:nelson` for an introduction to non-standard analysis.
Non-standard analysis is fun\ :footcite:p:`1987:nelson`.

.. footbibliography::"#,
            "index",
            docutilsrs::TitlePromotion::Preserve,
            true,
        );
        let placeholders: Vec<_> = (0..tree.nodes_len())
            .filter_map(|id| match &tree.node(id).kind {
                NodeKind::Extension { class_name, attrs } => Some((class_name.as_str(), attrs)),
                _ => None,
            })
            .collect();

        assert_eq!(placeholders.len(), 6, "placeholders: {placeholders:?}");
        let parsed_text = (0..tree.nodes_len())
            .filter_map(|id| match &tree.node(id).kind {
                NodeKind::Text(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<String>();
        assert!(parsed_text.contains("Escapedwhitespace stays joined."));
        assert_eq!(placeholders[0].0, "BibtexCitation");
        assert_eq!(
            placeholders[0].1.get("keys"),
            Some(&NodeAttributeValue::String("1987:nelson".into()))
        );
        assert_eq!(placeholders[1].0, "BibtexCitation");
        assert_eq!(
            placeholders[1].1.get("role"),
            Some(&NodeAttributeValue::String("cite:p".into()))
        );
        assert_eq!(placeholders[2].0, "BibtexBibliography");
        assert_eq!(placeholders[3].0, "BibtexCitation");
        assert_eq!(
            placeholders[3].1.get("role"),
            Some(&NodeAttributeValue::String("footcite:t".into()))
        );
        assert_eq!(placeholders[4].0, "BibtexCitation");
        assert_eq!(
            placeholders[4].1.get("role"),
            Some(&NodeAttributeValue::String("footcite:p".into()))
        );
        assert_eq!(placeholders[5].0, "BibtexFootBibliography");
        assert!(!(0..tree.nodes_len()).any(|id| matches!(
            tree.node(id).kind,
            NodeKind::SystemMessage { ty: "ERROR", .. }
        )));
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(all(test, feature = "csl-citeproc"))]
mod citeproc_tests {
    use citeproc::prelude::{
        Cite, ClusterPosition, InitOptions, Processor, Reference, SupportedFormat,
    };
    use std::collections::{HashMap, HashSet};
    use std::path::Path;

    use super::{Bibliography, CitationName, METADATA};

    fn app_with_config(
        root: &Path,
        name: &str,
        configuration: &str,
    ) -> crate::application::SphinxApp {
        let src = root.join(format!("{name}-src"));
        let out = root.join(format!("{name}-out"));
        let doctrees = root.join(format!("{name}-doctrees"));
        std::fs::create_dir_all(&src).unwrap();
        std::fs::create_dir_all(&out).unwrap();
        std::fs::create_dir_all(&doctrees).unwrap();
        std::fs::write(src.join("conf.py"), configuration).unwrap();
        crate::application::SphinxApp::new(&src, &out, &doctrees, "html", HashMap::new()).unwrap()
    }

    #[test]
    fn converts_biblatex_records_to_typed_csl_references() {
        let bibliography = Bibliography::parse(
            r#"@article{beethoven2024,
                author = {Ludwig van Beethoven and Ada Lovelace},
                title = {A study},
                journaltitle = {Journal of Tests},
                date = {2024-03-17},
                volume = {8},
                pages = {10--24}
            }
            @article{year_month,
                year = {2023},
                month = jan
            }"#,
        )
        .unwrap();
        let reference = bibliography
            .get("beethoven2024")
            .unwrap()
            .to_csl_reference();

        assert_eq!(reference.csl_type, citeproc::csl::CslType::ArticleJournal);
        assert_eq!(
            reference.ordinary[&citeproc::csl::Variable::ContainerTitle].as_str(),
            "Journal of Tests"
        );
        assert_eq!(
            reference.number[&citeproc::csl::NumberVariable::Volume],
            citeproc::io::NumberLike::Str("8".into())
        );
        assert_eq!(
            reference.number[&citeproc::csl::NumberVariable::Page],
            citeproc::io::NumberLike::Str("10–24".into())
        );
        assert_eq!(
            reference.date[&citeproc::csl::DateVariable::Issued],
            citeproc::io::DateOrRange::new(2024, 3, 17)
        );

        let authors = &reference.name[&citeproc::csl::NameVariable::Author];
        let citeproc::io::Name::Person(first_author) = &authors[0] else {
            panic!("expected a structured author name");
        };
        assert_eq!(first_author.family.as_deref(), Some("Beethoven"));
        assert_eq!(first_author.given.as_deref(), Some("Ludwig"));
        assert_eq!(first_author.non_dropping_particle.as_deref(), Some("van"));
        assert_eq!(authors.len(), 2);

        let year_month = bibliography.get("year_month").unwrap().to_csl_reference();
        assert_eq!(
            year_month.date[&citeproc::csl::DateVariable::Issued],
            citeproc::io::DateOrRange::new(2023, 1, 0)
        );
    }

    #[test]
    fn bibtex_conversion_helpers_cover_aliases_and_fallbacks() {
        use citeproc::csl::{CslType, NameVariable, NumberVariable, Variable};
        use citeproc::io::{DateOrRange, Name};
        use std::collections::BTreeMap;

        for (bibtex, expected) in [
            ("article", CslType::ArticleJournal),
            ("book", CslType::Book),
            ("collection", CslType::Book),
            ("proceedings", CslType::Book),
            ("inbook", CslType::Chapter),
            ("incollection", CslType::Chapter),
            ("inproceedings", CslType::Chapter),
            ("manual", CslType::Webpage),
            ("misc", CslType::Webpage),
            ("online", CslType::Webpage),
            ("www", CslType::Webpage),
            ("mastersthesis", CslType::Thesis),
            ("phdthesis", CslType::Thesis),
            ("thesis", CslType::Thesis),
            ("techreport", CslType::Report),
            ("unpublished", CslType::Manuscript),
            ("conference", CslType::PaperConference),
            ("booklet", CslType::Pamphlet),
            ("unknown", CslType::Article),
        ] {
            assert_eq!(super::csl_type(bibtex), expected, "BibTeX type: {bibtex}");
        }

        for (field, variable) in [
            ("abstract", Variable::Abstract),
            ("booktitle", Variable::ContainerTitle),
            ("journal", Variable::ContainerTitle),
            ("journaltitle", Variable::ContainerTitle),
            ("doi", Variable::DOI),
            ("isbn", Variable::ISBN),
            ("issn", Variable::ISSN),
            ("keywords", Variable::Keyword),
            ("language", Variable::Language),
            ("langid", Variable::Language),
            ("location", Variable::PublisherPlace),
            ("address", Variable::PublisherPlace),
            ("note", Variable::Note),
            ("publisher", Variable::Publisher),
            ("series", Variable::CollectionTitle),
            ("title", Variable::Title),
            ("url", Variable::URL),
        ] {
            assert_eq!(
                super::ordinary_variable(field),
                Some(variable),
                "field: {field}"
            );
        }
        assert_eq!(super::ordinary_variable("custom"), None);

        for (field, variable) in [
            ("chapter", NumberVariable::ChapterNumber),
            ("edition", NumberVariable::Edition),
            ("issue", NumberVariable::Issue),
            ("number", NumberVariable::Number),
            ("pages", NumberVariable::Page),
            ("volume", NumberVariable::Volume),
        ] {
            assert_eq!(
                super::number_variable(field),
                Some(variable),
                "field: {field}"
            );
        }
        assert_eq!(super::number_variable("custom"), None);

        for (month, number) in [
            ("jan", 1),
            ("february", 2),
            ("mar", 3),
            ("april", 4),
            ("may", 5),
            ("jun", 6),
            ("july", 7),
            ("aug", 8),
            ("september", 9),
            ("oct", 10),
            ("november", 11),
            ("dec", 12),
            ("7", 7),
        ] {
            assert_eq!(super::bibtex_month(month), Some(number), "month: {month}");
        }
        assert_eq!(super::bibtex_month("0"), None);
        assert_eq!(super::bibtex_month("not-a-month"), None);

        let mut date_fields = BTreeMap::new();
        date_fields.insert("year".into(), "2024".into());
        date_fields.insert("month".into(), "jan".into());
        date_fields.insert("day".into(), "17".into());
        assert_eq!(
            super::issued_date(&date_fields),
            Some(DateOrRange::new(2024, 1, 17))
        );
        date_fields.insert("date".into(), "2025-02-03".into());
        assert_eq!(
            super::issued_date(&date_fields),
            Some(DateOrRange::new(2025, 2, 3))
        );
        date_fields.insert("date".into(), "not-a-date".into());
        date_fields.insert("month".into(), "13".into());
        date_fields.insert("day".into(), "not-a-day".into());
        assert_eq!(
            super::issued_date(&date_fields),
            Some(DateOrRange::new(2024, 0, 0))
        );
        date_fields.insert("year".into(), "not-a-year".into());
        assert_eq!(super::issued_date(&date_fields), None);
        date_fields.remove("year");
        assert_eq!(super::issued_date(&date_fields), None);

        let empty_field = super::CitationRecord {
            id: "empty-field".into(),
            entry_type: "book".into(),
            fields: BTreeMap::from([
                ("title".into(), "  ".into()),
                ("custom".into(), "ignored".into()),
            ]),
            authors: Vec::new(),
            editors: Vec::new(),
        };
        assert!(empty_field.to_csl_reference().ordinary.is_empty());

        let record =
            Bibliography::parse("@book{editors, editor={Grace Hopper}, title={Compiler Design}}")
                .unwrap();
        let reference = record.get("editors").unwrap().to_csl_reference();
        assert!(reference.name.contains_key(&NameVariable::Editor));
        assert!(reference.date.is_empty());

        let dropping_particle = super::csl_name(&CitationName {
            family: "Beethoven".into(),
            given: "Ludwig".into(),
            prefix: "van".into(),
            suffix: "Jr.".into(),
            use_prefix: Some(false),
        });
        let Name::Person(dropping_particle) = dropping_particle else {
            panic!("expected a structured person name");
        };
        assert!(dropping_particle.non_dropping_particle.is_none());
        assert_eq!(dropping_particle.dropping_particle.as_deref(), Some("van"));

        let no_particle = super::csl_name(&CitationName {
            family: "Lovelace".into(),
            given: "Ada".into(),
            prefix: String::new(),
            suffix: String::new(),
            use_prefix: None,
        });
        let Name::Person(no_particle) = no_particle else {
            panic!("expected a structured person name");
        };
        assert!(no_particle.non_dropping_particle.is_none());
        assert!(no_particle.dropping_particle.is_none());
    }

    #[test]
    fn build_finished_handles_empty_configs_absolute_paths_and_failures() {
        let root = tempfile::tempdir().unwrap();

        let empty = app_with_config(root.path(), "empty", "bibtex_bibfiles = [7]\n");
        super::build_finished(&empty).unwrap();
        assert!(!empty.outdir.join("bibliography.tex").exists());

        let missing_bib = app_with_config(
            root.path(),
            "missing-bib",
            "bibtex_bibfiles = ['missing.bib']\nbibtex_csl_style = ''\n",
        );
        let error = super::build_finished(&missing_bib).unwrap_err().to_string();
        assert!(error.contains("failed to read bibliography"));

        let malformed_bib_path = root.path().join("malformed.bib");
        std::fs::write(&malformed_bib_path, "@book{unfinished").unwrap();
        let malformed_bib = app_with_config(
            root.path(),
            "malformed-bib",
            &format!("bibtex_bibfiles = [{:?}]\n", malformed_bib_path),
        );
        assert!(super::build_finished(&malformed_bib).is_err());

        let bibliography_path = root.path().join("absolute.bib");
        std::fs::write(
            &bibliography_path,
            "@book{absolute, author={Ada Lovelace}, title={Absolute Paths}, year={1843}}",
        )
        .unwrap();
        let style_path = root.path().join("absolute.csl");
        std::fs::write(&style_path, crate::extensions::csl::DEFAULT_STYLE).unwrap();
        let absolute = app_with_config(
            root.path(),
            "absolute",
            &format!(
                "bibtex_bibfiles = [{:?}]\nbibtex_csl_style = {:?}\n",
                bibliography_path, style_path
            ),
        );
        super::build_finished(&absolute).unwrap();
        assert!(absolute.outdir.join("bibliography.tex").exists());

        let default_style = app_with_config(
            root.path(),
            "default-style",
            &format!(
                "bibtex_bibfiles = [{:?}]\nbibtex_csl_style = ''\n",
                bibliography_path
            ),
        );
        super::build_finished(&default_style).unwrap();
        assert!(default_style.outdir.join("bibliography.tex").exists());

        let missing_style = app_with_config(
            root.path(),
            "missing-style",
            &format!(
                "bibtex_bibfiles = [{:?}]\nbibtex_csl_style = 'missing.csl'\n",
                bibliography_path
            ),
        );
        let error = super::build_finished(&missing_style)
            .unwrap_err()
            .to_string();
        assert!(error.contains("failed to read CSL style"));

        let invalid_style_path = root.path().join("invalid.csl");
        std::fs::write(&invalid_style_path, "<style").unwrap();
        let invalid_style = app_with_config(
            root.path(),
            "invalid-style",
            &format!(
                "bibtex_bibfiles = [{:?}]\nbibtex_csl_style = {:?}\n",
                bibliography_path, invalid_style_path
            ),
        );
        assert!(super::build_finished(&invalid_style).is_err());
    }

    #[test]
    fn pinned_citeproc_engine_formats_a_citation() {
        const STYLE: &str = r#"
            <style version="1.0" class="in-text">
              <citation><layout><text variable="title"/></layout></citation>
            </style>
        "#;
        let mut processor = Processor::new(InitOptions {
            style: STYLE,
            format: SupportedFormat::Plain,
            test_mode: true,
            ..Default::default()
        })
        .unwrap();
        let mut reference = Reference::empty(
            citeproc::csl::Atom::from("smith2024"),
            citeproc::csl::CslType::Article,
        );
        reference
            .ordinary
            .insert(citeproc::csl::Variable::Title, "A test title".into());
        processor.reset_references(vec![reference]);

        let cluster_id = processor.cluster_id("chapter:0");
        processor.insert_cites(cluster_id, &[Cite::basic("smith2024")]);
        processor
            .set_cluster_order(&[ClusterPosition::in_text(cluster_id)])
            .unwrap();
        let rendered = processor.all_clusters_str();

        assert_eq!(rendered.get("chapter:0").unwrap().as_str(), "A test title");
    }

    #[test]
    fn resolves_native_citations_and_bibliography_across_documents() {
        use crate::application::SphinxApp;
        use docutilsrs::doctree::NodeKind;

        let src = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();
        let doctrees = tempfile::tempdir().unwrap();
        std::fs::write(
            src.path().join("refs-a.bib"),
            r#"
                @book{alpha, author={Ada Lovelace}, title={Analytical Engine}, year={1843}}
                @book{beta, author={Grace Hopper}, title={Compilers}, year={1952}}
            "#,
        )
        .unwrap();
        std::fs::write(
            src.path().join("refs-b.bib"),
            r#"
                @article{gamma, author={Katherine Johnson}, title={Laser-safe & efficient}, year={2021}, doi={10.1000/example}}
            "#,
        )
        .unwrap();
        std::fs::write(
            src.path().join("test.csl"),
            r#"
                <style version="1.0" class="in-text">
                  <info>
                    <title>Native Sphinx integration test</title>
                    <id>https://example.test/styles/native-integration</id>
                    <link href="https://example.test/styles/native-integration" rel="self"/>
                    <updated>2024-01-01T00:00:00+00:00</updated>
                  </info>
                  <citation><layout><text variable="title"/></layout></citation>
                  <bibliography><layout><text variable="title"/></layout></bibliography>
                </style>
            "#,
        )
        .unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Index\n=====\n\nText :cite:`alpha`.\n\n.. bibliography::\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("chapter.rst"),
            "Chapter\n=======\n\nAlso :cite:`beta`.\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("conf.py"),
            format!(
                "extensions = [{:?}]\nbibtex_bibfiles = ['refs-a.bib', 'refs-b.bib']\nbibtex_csl_style = 'test.csl'\nbibtex_include_uncited = True\n",
                METADATA.name
            ),
        )
        .unwrap();
        let mut app = SphinxApp::new(
            src.path(),
            out.path(),
            doctrees.path(),
            "html",
            HashMap::new(),
        )
        .unwrap();
        app.build().unwrap();

        let raw_tree = app.env.borrow().get_doctree("index").unwrap();
        assert!((0..raw_tree.nodes_len()).any(|id| matches!(
            raw_tree.node(id).kind,
            NodeKind::Extension { ref class_name, .. } if class_name == "BibtexCitation"
        )));

        let resolved = app.env.borrow().get_and_resolve_doctree("index").unwrap();
        let text = (0..resolved.nodes_len())
            .filter_map(|id| match &resolved.node(id).kind {
                NodeKind::Text(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("Analytical Engine"), "rendered text: {text}");
        assert!(text.contains("Compilers"), "rendered text: {text}");
        assert!(
            text.contains("Laser-safe & efficient"),
            "rendered text: {text}"
        );
        assert!(!(0..resolved.nodes_len()).any(|id| matches!(
            resolved.node(id).kind,
            NodeKind::Extension { ref class_name, .. }
                if matches!(class_name.as_str(), "BibtexCitation" | "BibtexBibliography")
        )));

        let persisted_after_resolution = app.env.borrow().get_doctree("index").unwrap();
        assert!(
            (0..persisted_after_resolution.nodes_len()).any(|id| matches!(
                persisted_after_resolution.node(id).kind,
                NodeKind::Extension { ref class_name, .. } if class_name == "BibtexCitation"
            ))
        );

        let tex = std::fs::read_to_string(out.path().join("bibliography.tex")).unwrap();
        assert!(tex.contains("\\begin{thebibliography}"));
        assert!(tex.contains("\\bibitem{alpha}"));
        assert!(tex.contains("\\bibitem{gamma}"));
        assert!(tex.contains("Laser-safe \\& efficient"));

        let jsonld: serde_json::Value =
            serde_json::from_slice(&std::fs::read(out.path().join("bibliography.jsonld")).unwrap())
                .unwrap();
        let graph = jsonld["@graph"].as_array().unwrap();
        assert_eq!(graph.len(), 3);
        assert!(
            graph
                .iter()
                .any(|entry| entry["name"] == "Laser-safe & efficient")
        );
        assert!(
            graph
                .iter()
                .any(|entry| entry["sameAs"] == "https://doi.org/10.1000/example")
        );
        let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(html.contains("Laser-safe &amp; efficient"));
    }

    #[test]
    fn cite_empty_registers_a_reference_without_inline_output() {
        use crate::application::SphinxApp;
        use docutilsrs::doctree::{NodeAttributeValue, NodeKind};

        let src = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();
        let doctrees = tempfile::tempdir().unwrap();
        std::fs::write(
            src.path().join("refs.bib"),
            "@book{uncited, author={Ada Lovelace}, title={Analytical Engine}, year={1843}}",
        )
        .unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Index\n=====\n\nBefore :cite:empty:`uncited` after.\n\n.. bibliography::\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("conf.py"),
            format!(
                "extensions = [{:?}]\nbibtex_bibfiles = ['refs.bib']\n",
                METADATA.name
            ),
        )
        .unwrap();

        let mut app = SphinxApp::new(
            src.path(),
            out.path(),
            doctrees.path(),
            "html",
            HashMap::new(),
        )
        .unwrap();
        app.build().unwrap();

        let raw = app.env.borrow().get_doctree("index").unwrap();
        let citation = (0..raw.nodes_len())
            .find(|&id| {
                matches!(
                    &raw.node(id).kind,
                    NodeKind::Extension { class_name, attrs }
                        if class_name == "BibtexCitation"
                            && attrs.get("role")
                                == Some(&NodeAttributeValue::String("cite:empty".into()))
                )
            })
            .expect("cite:empty is preserved as a citation occurrence");

        let resolved = app.env.borrow().get_and_resolve_doctree("index").unwrap();
        assert!(
            matches!(resolved.node(citation).kind, NodeKind::Text(ref value) if value.is_empty())
        );
        let text = (0..resolved.nodes_len())
            .filter_map(|id| match &resolved.node(id).kind {
                NodeKind::Text(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<String>();
        assert!(text.contains("Before "), "rendered text: {text}");
        assert!(text.contains(" after."), "rendered text: {text}");
        assert!(text.contains("Analytical Engine"), "rendered text: {text}");
        assert!(!text.contains("cite:empty"), "rendered text: {text}");
    }

    #[test]
    fn supports_parenthetical_textual_and_footnote_citation_roles() {
        use crate::application::SphinxApp;
        use docutilsrs::doctree::{NodeAttributeValue, NodeKind};

        let src = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();
        let doctrees = tempfile::tempdir().unwrap();
        std::fs::write(
            src.path().join("refs.bib"),
            "@book{1987:nelson, author={nelson, Edward}, title={Internal Set Theory}, year={1987}}",
        )
        .unwrap();
        std::fs::write(
            src.path().join("author-year.csl"),
            r#"
                <style version="1.0" class="in-text">
                  <info>
                    <title>Footnote role test</title>
                    <id>https://example.test/styles/footnote-role-test</id>
                    <link href="https://example.test/styles/footnote-role-test" rel="self"/>
                    <updated>2024-01-01T00:00:00+00:00</updated>
                  </info>
                  <citation>
                    <layout prefix="(" suffix=")">
                      <group delimiter=", ">
                        <names variable="author"><name form="short"/></names>
                        <date variable="issued"><date-part name="year"/></date>
                      </group>
                    </layout>
                  </citation>
                  <bibliography>
                    <layout><text variable="title"/></layout>
                  </bibliography>
                </style>
            "#,
        )
        .unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Index\n=====\n\nSee :cite:t:`1987:nelson` for an introduction.\n\
             See :cite:ct:`1987:nelson` for a capitalized introduction.\n\
             See :cite:alp:`1987:nelson` without brackets.\n\
             See :cite:year:`1987:nelson` using year only.\n\
             See :cite:yearpar:`1987:nelson` using parenthetical year only.\n\
             Authors: :cite:author:`1987:nelson`; paren: :cite:authorpar:`1987:nelson`; capital: :cite:cauthor:`1987:nelson`; capital paren: :cite:cauthorpar:`1987:nelson`.\n\
             Non-standard analysis is fun :cite:p:`{see }1987:nelson{, page 6}`.\n\n\
             .. bibliography::\n\n\
             See :footcite:t:`1987:nelson` for an introduction.\n\
             See :footcite:ct:`1987:nelson` for a capitalized introduction.\n\
             Non-standard analysis is fun\\ :footcite:p:`1987:nelson`.\n\n\
             .. footbibliography::\n",
        )
        .unwrap();
        std::fs::write(
            src.path().join("conf.py"),
            format!(
                "extensions = [{:?}]\nbibtex_bibfiles = ['refs.bib']\nbibtex_csl_style = 'author-year.csl'\n",
                METADATA.name
            ),
        )
        .unwrap();

        let mut app = SphinxApp::new(
            src.path(),
            out.path(),
            doctrees.path(),
            "html",
            HashMap::new(),
        )
        .unwrap();
        app.build().unwrap();

        let raw = app.env.borrow().get_doctree("index").unwrap();
        let roles: HashMap<_, _> = (0..raw.nodes_len())
            .filter_map(|id| match &raw.node(id).kind {
                NodeKind::Extension { class_name, attrs } if class_name == "BibtexCitation" => {
                    let Some(NodeAttributeValue::String(role)) = attrs.get("role") else {
                        return None;
                    };
                    Some((role.as_str(), id))
                }
                _ => None,
            })
            .collect();
        assert_eq!(roles.len(), 13);

        let resolved = app.env.borrow().get_and_resolve_doctree("index").unwrap();
        let textual = &resolved.node(roles["cite:t"]).kind;
        assert!(matches!(textual, NodeKind::Text(value) if value == "nelson (1987)"));
        let capitalized = &resolved.node(roles["cite:ct"]).kind;
        assert!(matches!(capitalized, NodeKind::Text(value) if value == "Nelson (1987)"));
        let unbracketed = &resolved.node(roles["cite:alp"]).kind;
        assert!(matches!(unbracketed, NodeKind::Text(value) if value == "nelson, 1987"));
        assert!(matches!(
            &resolved.node(roles["cite:year"]).kind,
            NodeKind::Text(value) if value == "1987"
        ));
        assert!(matches!(
            &resolved.node(roles["cite:yearpar"]).kind,
            NodeKind::Text(value) if value == "(1987)"
        ));
        for role in [
            "cite:author",
            "cite:authorpar",
            "cite:cauthor",
            "cite:cauthorpar",
        ] {
            assert!(matches!(
                &resolved.node(roles[role]).kind,
                NodeKind::Text(_)
            ));
        }
        assert!(matches!(
            &resolved.node(roles["cite:author"]).kind,
            NodeKind::Text(value) if value == "nelson"
        ));
        assert!(
            matches!(
                &resolved.node(roles["cite:authorpar"]).kind,
                NodeKind::Text(value) if value == "(nelson)"
            ),
            "author-par citation: {:?}",
            resolved.node(roles["cite:authorpar"]).kind
        );
        assert!(matches!(
            &resolved.node(roles["cite:cauthor"]).kind,
            NodeKind::Text(value) if value == "Nelson"
        ));
        assert!(matches!(
            &resolved.node(roles["cite:cauthorpar"]).kind,
            NodeKind::Text(value) if value == "(Nelson)"
        ));
        let parenthetical = &resolved.node(roles["cite:p"]).kind;
        assert!(
            matches!(
                parenthetical,
                NodeKind::Text(value) if value == "(see nelson, 1987, page 6)"
            ),
            "resolved parenthetical citation: {parenthetical:?}"
        );
        assert!(matches!(
            resolved.node(roles["footcite:t"]).kind,
            NodeKind::Inline { .. }
        ));
        let footcite_text = resolved
            .node(roles["footcite:t"])
            .children
            .iter()
            .filter_map(|&id| match &resolved.node(id).kind {
                NodeKind::Text(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<String>();
        assert_eq!(footcite_text, "nelson (1987)");
        assert!(matches!(
            resolved.node(roles["footcite:ct"]).kind,
            NodeKind::Inline { .. }
        ));
        let footcite_capitalized_text = resolved
            .node(roles["footcite:ct"])
            .children
            .iter()
            .filter_map(|&id| match &resolved.node(id).kind {
                NodeKind::Text(value) => Some(value.as_str()),
                _ => None,
            })
            .collect::<String>();
        assert_eq!(footcite_capitalized_text, "Nelson (1987)");
        assert!(matches!(
            resolved.node(roles["footcite:p"]).kind,
            NodeKind::Inline { .. }
        ));
        let footcite_parent = resolved.node(roles["footcite:p"]).parent.unwrap();
        let footcite_siblings = &resolved.node(footcite_parent).children;
        let footcite_position = footcite_siblings
            .iter()
            .position(|&id| id == roles["footcite:p"])
            .unwrap();
        assert!(matches!(
            &resolved.node(footcite_siblings[footcite_position - 1]).kind,
            NodeKind::Text(value) if value.ends_with("Non-standard analysis is fun")
        ));
        assert_eq!(
            (0..resolved.nodes_len())
                .filter(|&id| matches!(resolved.node(id).kind, NodeKind::Footnote { .. }))
                .count(),
            1
        );
        assert_eq!(
            (0..resolved.nodes_len())
                .filter(|&id| matches!(resolved.node(id).kind, NodeKind::FootnoteReference { .. }))
                .count(),
            3
        );
        let reference_ids: HashSet<_> = (0..resolved.nodes_len())
            .filter_map(|id| match &resolved.node(id).kind {
                NodeKind::FootnoteReference { ids, .. } => Some(ids.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(reference_ids.len(), 3);

        let html = std::fs::read_to_string(out.path().join("index.html")).unwrap();
        assert!(html.contains("href=\"#bibtex-footnote-1\""));
        assert!(html.contains("id=\"bibtex-footnote-1\""));
        assert!(
            reference_ids
                .iter()
                .all(|id| html.contains(&format!("id=\"{id}\"")))
        );
    }
}
