//! Platform-independent DocIndex models, parsing, indexing, and search.
//!
//! This crate intentionally has no filesystem, network, Python, or native
//! search-engine dependency so the same artifact and search behavior can be
//! used by native applications and WebAssembly.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub const ARTIFACT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentType {
    Chat,
    ChatInput,
    ChatThinking,
    ChatOutput,
    SphinxRst,
    SphinxMd,
    SphinxNb,
    SphinxHtml,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CodeSnippet {
    pub language: String,
    pub code: String,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    pub source_file: String,
    pub date_indexed: Option<String>,
    pub chat_type: Option<String>,
    pub tags: Vec<String>,
    pub concepts: Vec<String>,
    pub word_count: Option<usize>,
    pub heading_level: Option<usize>,
    pub breadcrumb: Option<Vec<String>>,
    pub last_built: Option<String>,
    pub sphinx_role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub title: String,
    pub content: String,
    pub filename: String,
    pub url: Option<String>,
    pub summary: Option<String>,
    #[serde(default)]
    pub code_snippets: Vec<CodeSnippet>,
    pub metadata: DocumentMetadata,
    pub build_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexSettings {
    pub searchable_attributes: Vec<String>,
    pub filterable_attributes: Vec<String>,
    pub sortable_attributes: Vec<String>,
    #[serde(default)]
    pub synonyms: BTreeMap<String, Vec<String>>,
}

impl Default for IndexSettings {
    fn default() -> Self {
        Self {
            searchable_attributes: vec!["title".into(), "content".into(), "summary".into()],
            filterable_attributes: vec!["type".into(), "filename".into()],
            sortable_attributes: vec!["metadata.date_indexed".into()],
            synonyms: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: String,
    pub limit: usize,
    pub offset: usize,
    pub document_type: Option<DocumentType>,
}

impl Default for SearchQuery {
    fn default() -> Self {
        Self {
            text: String::new(),
            limit: 20,
            offset: 0,
            document_type: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    #[serde(rename = "type")]
    pub document_type: DocumentType,
    pub title: String,
    pub url: Option<String>,
    pub source_uri: Option<String>,
    pub content: String,
    pub content_snippet: String,
    pub date_indexed: Option<String>,
    pub relevance_score: f64,
    pub matched_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexingStats {
    pub total_documents: usize,
    pub indexed_documents: usize,
    pub skipped_documents: usize,
    pub errors: usize,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration_seconds: f64,
}

impl IndexingStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_documents == 0 {
            0.0
        } else {
            self.indexed_documents as f64 * 100.0 / self.total_documents as f64
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocIndexConfig {
    pub backend: String,
    pub batch_size: usize,
    pub index_name: String,
    pub enabled: bool,
}

impl Default for DocIndexConfig {
    fn default() -> Self {
        Self {
            backend: "memory".into(),
            batch_size: 1000,
            index_name: "all".into(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Artifact {
    schema_version: u32,
    settings: IndexSettings,
    documents: BTreeMap<String, Document>,
}

#[derive(Debug, thiserror::Error)]
pub enum DocIndexError {
    #[error("invalid artifact: {0}")]
    InvalidArtifact(String),
    #[error("unsupported artifact schema version {0}")]
    UnsupportedSchema(u32),
    #[error("invalid configuration: {0}")]
    InvalidConfiguration(String),
}

pub trait DocumentSearcher {
    fn search(&self, query: &SearchQuery) -> Vec<SearchResult>;
}

pub trait DocumentIndexer {
    fn add_documents(&mut self, documents: &[Document]) -> IndexingStats;
    fn clear(&mut self);
}

pub trait IndexArtifact: Sized {
    fn export_json(&self) -> Result<Vec<u8>, DocIndexError>;
    fn import_json(bytes: &[u8]) -> Result<Self, DocIndexError>;
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryIndex {
    settings: IndexSettings,
    documents: BTreeMap<String, Document>,
}

impl InMemoryIndex {
    pub fn new(settings: IndexSettings) -> Self {
        Self {
            settings,
            documents: BTreeMap::new(),
        }
    }
    pub fn settings(&self) -> &IndexSettings {
        &self.settings
    }
    pub fn settings_mut(&mut self) -> &mut IndexSettings {
        &mut self.settings
    }
    pub fn documents(&self) -> impl Iterator<Item = &Document> {
        self.documents.values()
    }
    pub fn get(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }
    pub fn add(&mut self, document: Document) {
        self.documents.insert(document.id.clone(), document);
    }

    fn terms(&self, query: &str) -> Vec<String> {
        let mut terms = Vec::new();
        for raw in query.split_whitespace() {
            let term = normalize(raw);
            if term.is_empty() {
                continue;
            }
            terms.push(term.clone());
            if let Some(values) = self.settings.synonyms.get(&term) {
                terms.extend(
                    values
                        .iter()
                        .map(|value| normalize(value))
                        .filter(|value| !value.is_empty()),
                );
            }
            for (key, values) in &self.settings.synonyms {
                if values.iter().any(|value| normalize(value) == term) {
                    terms.push(key.clone());
                }
            }
        }
        terms.sort();
        terms.dedup();
        terms
    }
}

impl DocumentIndexer for InMemoryIndex {
    fn add_documents(&mut self, documents: &[Document]) -> IndexingStats {
        let total = documents.len();
        for document in documents {
            self.add(document.clone());
        }
        IndexingStats {
            total_documents: total,
            indexed_documents: total,
            skipped_documents: 0,
            errors: 0,
            start_time: None,
            end_time: None,
            duration_seconds: 0.0,
        }
    }
    fn clear(&mut self) {
        self.documents.clear();
    }
}

impl DocumentSearcher for InMemoryIndex {
    fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        let terms = self.terms(&query.text);
        let mut results = self
            .documents
            .values()
            .filter_map(|document| {
                if query
                    .document_type
                    .is_some_and(|kind| kind != document.document_type)
                {
                    return None;
                }
                if terms.is_empty() {
                    return Some((
                        0.0,
                        SearchResult::from_document(document, String::new(), Vec::new()),
                    ));
                }
                let title = normalize(&document.title);
                let content = normalize(&document.content);
                let summary = normalize(document.summary.as_deref().unwrap_or_default());
                let mut score = 0.0;
                let mut fields = BTreeSet::new();
                for term in &terms {
                    if title.contains(term) {
                        score += 3.0;
                        fields.insert("title".to_string());
                    }
                    if summary.contains(term) {
                        score += 2.0;
                        fields.insert("summary".to_string());
                    }
                    if content.contains(term) {
                        score += 1.0;
                        fields.insert("content".to_string());
                    }
                }
                (score > 0.0).then(|| {
                    let mut result = SearchResult::from_document(
                        document,
                        snippet(&document.content, &terms),
                        fields.into_iter().collect(),
                    );
                    result.relevance_score = score;
                    (score, result)
                })
            })
            .collect::<Vec<_>>();
        results.sort_by(|left, right| {
            right
                .0
                .total_cmp(&left.0)
                .then_with(|| left.1.id.cmp(&right.1.id))
        });
        results
            .into_iter()
            .skip(query.offset)
            .take(query.limit)
            .map(|(_, result)| result)
            .collect()
    }
}

impl IndexArtifact for InMemoryIndex {
    fn export_json(&self) -> Result<Vec<u8>, DocIndexError> {
        serde_json::to_vec_pretty(&Artifact {
            schema_version: ARTIFACT_SCHEMA_VERSION,
            settings: self.settings.clone(),
            documents: self.documents.clone(),
        })
        .map_err(|error| DocIndexError::InvalidArtifact(error.to_string()))
    }
    fn import_json(bytes: &[u8]) -> Result<Self, DocIndexError> {
        let artifact: Artifact = serde_json::from_slice(bytes)
            .map_err(|error| DocIndexError::InvalidArtifact(error.to_string()))?;
        if artifact.schema_version != ARTIFACT_SCHEMA_VERSION {
            return Err(DocIndexError::UnsupportedSchema(artifact.schema_version));
        }
        Ok(Self {
            settings: artifact.settings,
            documents: artifact.documents,
        })
    }
}

impl SearchResult {
    fn from_document(
        document: &Document,
        content_snippet: String,
        matched_fields: Vec<String>,
    ) -> Self {
        Self {
            id: document.id.clone(),
            document_type: document.document_type,
            title: document.title.clone(),
            url: document.url.clone(),
            source_uri: Some(document.metadata.source_file.clone()),
            content: document.content.clone(),
            content_snippet,
            date_indexed: document.metadata.date_indexed.clone(),
            relevance_score: 0.0,
            matched_fields,
        }
    }
}

pub fn normalize(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| character.to_lowercase())
        .collect::<String>()
}

fn snippet(content: &str, terms: &[String]) -> String {
    let words: Vec<&str> = content.split_whitespace().collect();
    let index = words
        .iter()
        .position(|word| terms.iter().any(|term| normalize(word).contains(term)))
        .unwrap_or(0);
    let start = index.saturating_sub(8);
    let end = (start + 24).min(words.len());
    words[start..end].join(" ")
}

pub fn parse_html_document(
    html: &str,
    filename: impl Into<String>,
    url: Option<String>,
) -> Document {
    let filename = filename.into();
    let title = extract_tag(html, "title")
        .or_else(|| extract_heading(html))
        .unwrap_or_else(|| filename.clone());
    let content = strip_tags(html);
    let snippets = extract_code_snippets(html);
    Document {
        id: filename.clone(),
        document_type: DocumentType::SphinxHtml,
        title,
        content: content.clone(),
        filename: filename.clone(),
        url,
        summary: Some(content.chars().take(240).collect()),
        code_snippets: snippets,
        metadata: DocumentMetadata {
            source_file: filename,
            word_count: Some(content.split_whitespace().count()),
            ..Default::default()
        },
        build_id: None,
    }
}

pub fn parse_chat_document(text: &str, filename: impl Into<String>) -> Vec<Document> {
    let filename = filename.into();
    let mut documents = Vec::new();
    let mut current_role = String::from("chat");
    let mut current = Vec::new();
    let mut ordinal = 0;
    let flush =
        |documents: &mut Vec<Document>, role: &str, body: &mut Vec<String>, ordinal: &mut usize| {
            let content = body.join("\n").trim().to_string();
            if content.is_empty() {
                return;
            }
            let document_type = match role {
                "user" | "human" => DocumentType::ChatInput,
                "assistant" | "model" => DocumentType::ChatOutput,
                "thinking" => DocumentType::ChatThinking,
                _ => DocumentType::Chat,
            };
            let id = format!("{filename}#{ordinal}");
            *ordinal += 1;
            documents.push(Document {
                id,
                document_type,
                title: role.to_string(),
                content: content.clone(),
                filename: filename.clone(),
                url: None,
                summary: Some(content.chars().take(240).collect()),
                code_snippets: Vec::new(),
                metadata: DocumentMetadata {
                    source_file: filename.clone(),
                    chat_type: Some(role.to_string()),
                    word_count: Some(content.split_whitespace().count()),
                    ..Default::default()
                },
                build_id: None,
            });
            body.clear();
        };
    for line in text.lines() {
        if let Some(role) = line
            .strip_prefix("## ")
            .or_else(|| line.strip_prefix("### "))
        {
            flush(&mut documents, &current_role, &mut current, &mut ordinal);
            current_role = role.trim().to_ascii_lowercase();
        } else {
            current.push(line.to_string());
        }
    }
    flush(&mut documents, &current_role, &mut current, &mut ordinal);
    documents
}

fn extract_tag(input: &str, tag: &str) -> Option<String> {
    let lower = input.to_ascii_lowercase();
    let open = format!("<{tag}");
    let open_start = lower.find(&open)?;
    let start = open_start.checked_add(lower[open_start..].find('>')? + 1)?;
    let close = lower[start..].find(&format!("</{tag}>"))? + start;
    Some(strip_tags(&input[start..close]).trim().to_string()).filter(|value| !value.is_empty())
}

fn extract_heading(input: &str) -> Option<String> {
    (1..=6).find_map(|level| extract_tag(input, &format!("h{level}")))
}

fn strip_tags(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut in_tag = false;
    for character in input.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => output.push(character),
            _ => {}
        }
    }
    output.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_code_snippets(input: &str) -> Vec<CodeSnippet> {
    let mut result = Vec::new();
    let lower = input.to_ascii_lowercase();
    let mut cursor = 0;
    while let Some(relative) = lower[cursor..].find("<code") {
        let start = cursor + relative;
        let body_start = match lower[start..].find('>') {
            Some(value) => start + value + 1,
            None => break,
        };
        let end = match lower[body_start..].find("</code>") {
            Some(value) => body_start + value,
            None => break,
        };
        result.push(CodeSnippet {
            language: String::new(),
            code: strip_tags(&input[body_start..end]),
            line_start: None,
            line_end: None,
        });
        cursor = end + 7;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn document(id: &str, title: &str, content: &str) -> Document {
        Document {
            id: id.into(),
            document_type: DocumentType::SphinxHtml,
            title: title.into(),
            content: content.into(),
            filename: format!("{id}.html"),
            url: Some(format!("/{id}.html")),
            summary: None,
            code_snippets: Vec::new(),
            metadata: DocumentMetadata {
                source_file: format!("{id}.rst"),
                ..Default::default()
            },
            build_id: None,
        }
    }

    #[test]
    fn search_ranks_title_and_is_stable() {
        let mut index = InMemoryIndex::default();
        index.add_documents(&[
            document("b", "Other", "rust guide"),
            document("a", "Rust guide", "other"),
        ]);
        let results = index.search(&SearchQuery {
            text: "rust".into(),
            ..Default::default()
        });
        assert_eq!(results[0].id, "a");
        assert!(
            results[0].relevance_score == 0.0
                || results[0].matched_fields.contains(&"title".into())
        );
    }

    #[test]
    fn artifact_round_trip_is_deterministic() {
        let mut index = InMemoryIndex::default();
        index.add(document("a", "A", "content"));
        let first = index.export_json().unwrap();
        let second = InMemoryIndex::import_json(&first)
            .unwrap()
            .export_json()
            .unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn parses_html_and_code() {
        let document = parse_html_document(
            "<html><title>Guide</title><h1>Guide</h1><p>Hello</p><code>x &lt; y</code></html>",
            "guide.html",
            Some("/guide.html".into()),
        );
        assert_eq!(document.title, "Guide");
        assert!(document.content.contains("Hello"));
        assert_eq!(document.code_snippets.len(), 1);
    }

    #[test]
    fn parses_chat_roles() {
        let documents = parse_chat_document("## user\nhello\n## assistant\nworld", "chat.md");
        assert_eq!(documents.len(), 2);
        assert_eq!(documents[0].document_type, DocumentType::ChatInput);
        assert_eq!(documents[1].document_type, DocumentType::ChatOutput);
    }
}
