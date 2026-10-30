//! BibTeX parsing and normalized bibliography records.

use std::collections::{BTreeMap, HashMap};

use biblatex::ChunksExt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationRecord {
    pub id: String,
    pub entry_type: String,
    pub fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BibliographyError {
    #[error("failed to parse BibTeX: {0}")]
    Parse(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bibliography {
    records: Vec<CitationRecord>,
    by_id: HashMap<String, usize>,
}

impl Bibliography {
    pub fn parse(source: &str) -> Result<Self, BibliographyError> {
        let parsed = biblatex::Bibliography::parse(source)
            .map_err(|error| BibliographyError::Parse(error.to_string()))?;
        let mut bibliography = Self::default();

        for entry in parsed.iter() {
            let record = CitationRecord {
                id: entry.key.clone(),
                entry_type: entry.entry_type.to_string(),
                fields: entry
                    .fields
                    .iter()
                    .map(|(key, chunks)| (key.clone(), chunks.format_verbatim()))
                    .collect::<BTreeMap<_, _>>(),
            };
            bibliography.by_id.insert(record.id.clone(), bibliography.records.len());
            bibliography.records.push(record);
        }

        Ok(bibliography)
    }

    pub fn get(&self, id: &str) -> Option<&CitationRecord> {
        self.by_id.get(id).and_then(|index| self.records.get(*index))
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
            bibliography.get("paper").unwrap().fields.get("booktitle").unwrap(),
            "Collected Testing"
        );
    }

    #[test]
    fn rejects_duplicate_citation_keys() {
        let error = Bibliography::parse(
            "@book{same, title={First}} @book{same, title={Second}}",
        )
        .unwrap_err();

        assert!(matches!(error, BibliographyError::Parse(_)));
        assert!(error.to_string().contains("same"));
    }

    #[test]
    fn rejects_malformed_input_without_panicking() {
        assert!(Bibliography::parse("@book{unfinished").is_err());
        assert!(Bibliography::parse("").unwrap().is_empty());
    }
}