//! BibTeX parsing and normalized bibliography records.

use std::collections::{BTreeMap, HashMap};

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
    for role in ["cite", "cite:p", "cite:t"] {
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
}

pub fn setup(_app: &mut crate::application::SphinxApp) -> Result<(), crate::application::AppError> {
    register_parser_components();
    Ok(())
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
        use citeproc::csl::{DateVariable, NameVariable, NumberVariable, Variable};
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
                if let Ok(variable) = Variable::from_str(variable) {
                    reference.ordinary.insert(variable, value.into());
                }
            } else if let Some(variable) = number_variable(field) {
                if let Ok(variable) = NumberVariable::from_str(variable) {
                    reference
                        .number
                        .insert(variable, NumberLike::Str(value.into()));
                }
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
fn ordinary_variable(field: &str) -> Option<&'static str> {
    Some(match field {
        "abstract" => "abstract",
        "booktitle" | "journal" | "journaltitle" => "container-title",
        "doi" => "DOI",
        "isbn" => "ISBN",
        "issn" => "ISSN",
        "keywords" => "keyword",
        "language" | "langid" => "language",
        "location" | "address" => "publisher-place",
        "note" => "note",
        "publisher" => "publisher",
        "series" => "collection-title",
        "title" => "title",
        "url" => "URL",
        _ => return None,
    })
}

#[cfg(feature = "csl-citeproc")]
fn number_variable(field: &str) -> Option<&'static str> {
    Some(match field {
        "chapter" => "chapter-number",
        "edition" => "edition",
        "issue" => "issue",
        "number" => "number",
        "pages" => "page",
        "volume" => "volume",
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

    #[test]
    fn native_roles_and_bibliography_directive_emit_placeholders() {
        use docutilsrs::doctree::{NodeAttributeValue, NodeKind};

        super::register_parser_components();
        let tree = docutilsrs::parse_rst_for_sphinx(
            ":cite:`first` :cite:p:`second` :cite:t:`third`\n\n.. bibliography::\n",
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

        assert_eq!(placeholders.len(), 4);
        assert_eq!(placeholders[0].0, "BibtexCitation");
        assert_eq!(
            placeholders[0].1.get("keys"),
            Some(&NodeAttributeValue::String("first".into()))
        );
        assert_eq!(placeholders[1].0, "BibtexCitation");
        assert_eq!(
            placeholders[1].1.get("role"),
            Some(&NodeAttributeValue::String("cite:p".into()))
        );
        assert_eq!(placeholders[2].0, "BibtexCitation");
        assert_eq!(
            placeholders[2].1.get("role"),
            Some(&NodeAttributeValue::String("cite:t".into()))
        );
        assert_eq!(placeholders[3].0, "BibtexBibliography");
        assert!(!(0..tree.nodes_len()).any(|id| matches!(
            tree.node(id).kind,
            NodeKind::SystemMessage { ty: "ERROR", .. }
        )));
    }
}

#[cfg(all(test, feature = "csl-citeproc"))]
mod citeproc_tests {
    use citeproc::prelude::{
        Cite, ClusterPosition, InitOptions, Processor, Reference, SupportedFormat,
    };
    use std::collections::HashMap;

    use super::{Bibliography, METADATA};

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
            src.path().join("refs.bib"),
            r#"
                @book{alpha, author={Ada Lovelace}, title={Analytical Engine}, year={1843}}
                @book{beta, author={Grace Hopper}, title={Compilers}, year={1952}}
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
                "extensions = [{:?}]\nbibtex_bibfiles = ['refs.bib']\nbibtex_csl_style = 'test.csl'\n",
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
        app.read().unwrap();

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
    }
}
