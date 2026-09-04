//! Native DocIndex backend adapters.
//!
//! The core crate owns the portable index and search semantics. This module
//! adds native persistence and service adapters without making the core crate
//! depend on a filesystem, HTTP client, or search service.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use docindexrs_core::{
    Document, DocumentIndexer, DocumentSearcher, DocumentType, InMemoryIndex, IndexArtifact,
    IndexSettings, IndexingStats, SearchQuery, SearchResult,
};
use reqwest::Method;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{NativeDocIndex, NativeError};

pub type BackendResult<T> = Result<T, NativeError>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackendIndexInfo {
    pub name: String,
    pub documents: usize,
}

pub trait Backend: Send {
    fn name(&self) -> &'static str;
    fn verify_connection(&self) -> BackendResult<bool>;
    fn create_or_update_index(&mut self, name: &str, settings: &IndexSettings)
    -> BackendResult<()>;
    fn add_documents(
        &mut self,
        name: &str,
        documents: &[Document],
        batch_size: usize,
    ) -> BackendResult<IndexingStats>;
    fn search(&self, name: &str, query: &SearchQuery) -> BackendResult<Vec<SearchResult>>;
    fn list_indices(&self) -> BackendResult<Vec<BackendIndexInfo>>;
    fn clear_index(&mut self, name: &str) -> BackendResult<()>;
    fn delete_index(&mut self, name: &str) -> BackendResult<bool>;
    fn get_index_stats(&self, name: &str) -> BackendResult<BackendIndexInfo>;
    fn get_synonyms(&self, name: &str) -> BackendResult<BTreeMap<String, Vec<String>>>;
    fn update_synonyms(
        &mut self,
        name: &str,
        synonyms: BTreeMap<String, Vec<String>>,
    ) -> BackendResult<()>;
    fn clear_synonyms(&mut self, name: &str) -> BackendResult<()>;
}

#[derive(Debug, Clone)]
pub struct OxiRsConfig {
    pub storage_path: Option<PathBuf>,
    pub index_name: String,
}

impl Default for OxiRsConfig {
    fn default() -> Self {
        Self {
            storage_path: None,
            index_name: "all".into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct OxiRsState {
    indexes: BTreeMap<String, Value>,
    synonyms: BTreeMap<String, BTreeMap<String, Vec<String>>>,
}

/// A native RDF-oriented adapter with OxiRS-compatible named-index behavior.
///
/// Each index is represented by the same deterministic DocIndex artifact used
/// by the WASM runtime. This keeps local persistence portable while the HDT
/// exporter supplies the RDF representation for graph consumers.
pub struct OxiRsBackend {
    config: OxiRsConfig,
    indexes: BTreeMap<String, InMemoryIndex>,
    synonyms: BTreeMap<String, BTreeMap<String, Vec<String>>>,
}

impl OxiRsBackend {
    pub fn new(config: OxiRsConfig) -> BackendResult<Self> {
        let mut backend = Self {
            config,
            indexes: BTreeMap::new(),
            synonyms: BTreeMap::new(),
        };
        backend.load()?;
        Ok(backend)
    }

    pub fn config(&self) -> &OxiRsConfig {
        &self.config
    }

    fn state_path(&self) -> Option<PathBuf> {
        self.config
            .storage_path
            .as_ref()
            .map(|path| path.join("docindex-state.json"))
    }

    fn load(&mut self) -> BackendResult<()> {
        let Some(path) = self.state_path() else {
            return Ok(());
        };
        if !path.is_file() {
            return Ok(());
        }
        let state: OxiRsState = serde_json::from_slice(&fs::read(path)?)?;
        for (name, artifact) in state.indexes {
            self.indexes.insert(
                name.clone(),
                InMemoryIndex::import_json(&serde_json::to_vec(&artifact)?)?,
            );
        }
        self.synonyms = state.synonyms;
        Ok(())
    }

    fn save(&self) -> BackendResult<()> {
        let Some(path) = self.state_path() else {
            return Ok(());
        };
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let indexes = self
            .indexes
            .iter()
            .map(|(name, index)| {
                serde_json::from_slice(
                    index
                        .export_json()
                        .as_ref()
                        .map_err(|error| NativeError::Backend(error.to_string()))?,
                )
                .map(|artifact| (name.clone(), artifact))
                .map_err(NativeError::from)
            })
            .collect::<BackendResult<BTreeMap<String, Value>>>()?;
        let state = OxiRsState {
            indexes,
            synonyms: self.synonyms.clone(),
        };
        fs::write(path, serde_json::to_vec_pretty(&state)?)?;
        Ok(())
    }

    fn index_mut(&mut self, name: &str) -> &mut InMemoryIndex {
        self.indexes.entry(name.to_owned()).or_default()
    }

    pub fn export_hdt(&self, name: &str, path: impl AsRef<Path>) -> BackendResult<()> {
        let index = self
            .indexes
            .get(name)
            .ok_or_else(|| NativeError::Backend(format!("index {name:?} does not exist")))?;
        let mut native = NativeDocIndex::new();
        native
            .index_mut()
            .add_documents(&index.documents().cloned().collect::<Vec<_>>());
        native.write_hdt(path)
    }
}

impl Backend for OxiRsBackend {
    fn name(&self) -> &'static str {
        "oxirs"
    }

    fn verify_connection(&self) -> BackendResult<bool> {
        if let Some(path) = &self.config.storage_path {
            fs::create_dir_all(path)?;
        }
        Ok(true)
    }

    fn create_or_update_index(
        &mut self,
        name: &str,
        settings: &IndexSettings,
    ) -> BackendResult<()> {
        let index = self.index_mut(name);
        index.settings_mut().clone_from(settings);
        self.save()
    }

    fn add_documents(
        &mut self,
        name: &str,
        documents: &[Document],
        _batch_size: usize,
    ) -> BackendResult<IndexingStats> {
        let started = Instant::now();
        let index = self.index_mut(name);
        let stats = index.add_documents(documents);
        self.save()?;
        Ok(IndexingStats {
            duration_seconds: started.elapsed().as_secs_f64(),
            ..stats
        })
    }

    fn search(&self, name: &str, query: &SearchQuery) -> BackendResult<Vec<SearchResult>> {
        Ok(self
            .indexes
            .get(name)
            .map(|index| index.search(query))
            .unwrap_or_default())
    }

    fn list_indices(&self) -> BackendResult<Vec<BackendIndexInfo>> {
        Ok(self
            .indexes
            .iter()
            .map(|(name, index)| BackendIndexInfo {
                name: name.clone(),
                documents: index.documents().count(),
            })
            .collect())
    }

    fn clear_index(&mut self, name: &str) -> BackendResult<()> {
        self.index_mut(name).clear();
        self.save()
    }

    fn delete_index(&mut self, name: &str) -> BackendResult<bool> {
        let removed = self.indexes.remove(name).is_some();
        self.synonyms.remove(name);
        self.save()?;
        Ok(removed)
    }

    fn get_index_stats(&self, name: &str) -> BackendResult<BackendIndexInfo> {
        Ok(BackendIndexInfo {
            name: name.to_owned(),
            documents: self
                .indexes
                .get(name)
                .map_or(0, |index| index.documents().count()),
        })
    }

    fn get_synonyms(&self, name: &str) -> BackendResult<BTreeMap<String, Vec<String>>> {
        Ok(self.synonyms.get(name).cloned().unwrap_or_default())
    }

    fn update_synonyms(
        &mut self,
        name: &str,
        synonyms: BTreeMap<String, Vec<String>>,
    ) -> BackendResult<()> {
        self.synonyms.insert(name.to_owned(), synonyms.clone());
        self.index_mut(name).settings_mut().synonyms = synonyms;
        self.save()
    }

    fn clear_synonyms(&mut self, name: &str) -> BackendResult<()> {
        self.synonyms.remove(name);
        self.index_mut(name).settings_mut().synonyms.clear();
        self.save()
    }
}

#[derive(Debug, Clone)]
pub struct MeilisearchConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for MeilisearchConfig {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:7700".into(),
            api_key: None,
            timeout_seconds: 30,
        }
    }
}

pub struct MeilisearchBackend {
    config: MeilisearchConfig,
    client: Client,
}

impl MeilisearchBackend {
    pub fn new(config: MeilisearchConfig) -> BackendResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()?;
        Ok(Self { config, client })
    }

    fn request(&self, method: Method, path: &str, body: Option<Value>) -> BackendResult<Value> {
        let url = format!("{}/{}", self.config.base_url.trim_end_matches('/'), path);
        let mut request = self.client.request(method, url);
        if let Some(key) = &self.config.api_key {
            request = request.header("Authorization", format!("Bearer {key}"));
        }
        if let Some(body) = body {
            request = request.json(&body);
        }
        let response = request.send()?;
        let status = response.status();
        let text = response.text()?;
        if !status.is_success() {
            return Err(NativeError::Backend(format!(
                "Meilisearch {status}: {text}"
            )));
        }
        if text.trim().is_empty() {
            return Ok(Value::Null);
        }
        Ok(serde_json::from_str(&text)?)
    }

    fn task(&self, response: &Value) -> BackendResult<()> {
        let Some(uid) = response.get("taskUid").and_then(Value::as_u64) else {
            return Ok(());
        };
        let result = self.request(Method::GET, &format!("tasks/{uid}"), None)?;
        if result.get("status").and_then(Value::as_str) == Some("failed") {
            return Err(NativeError::Backend(format!(
                "Meilisearch task {uid} failed"
            )));
        }
        Ok(())
    }

    fn settings_json(settings: &IndexSettings) -> Value {
        json!({
            "searchableAttributes": settings.searchable_attributes,
            "filterableAttributes": settings.filterable_attributes,
            "sortableAttributes": settings.sortable_attributes,
            "synonyms": settings.synonyms,
        })
    }

    fn parse_hit(hit: &Value) -> SearchResult {
        let document_type = hit
            .get("type")
            .and_then(Value::as_str)
            .and_then(|value| serde_json::from_value(Value::String(value.to_owned())).ok())
            .unwrap_or(DocumentType::SphinxHtml);
        let content = hit
            .get("content")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        SearchResult {
            id: hit
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .into(),
            document_type,
            title: hit
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .into(),
            url: hit.get("url").and_then(Value::as_str).map(str::to_owned),
            source_uri: hit
                .get("source_uri")
                .and_then(Value::as_str)
                .map(str::to_owned),
            content: content.clone(),
            content_snippet: hit
                .get("content_snippet")
                .and_then(Value::as_str)
                .unwrap_or(&content)
                .chars()
                .take(240)
                .collect(),
            date_indexed: hit
                .get("date_indexed")
                .and_then(Value::as_str)
                .map(str::to_owned),
            relevance_score: hit
                .get("_rankingScore")
                .and_then(Value::as_f64)
                .unwrap_or_default(),
            matched_fields: hit
                .get("_matchedFields")
                .and_then(Value::as_array)
                .map(|fields| {
                    fields
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_owned)
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl Backend for MeilisearchBackend {
    fn name(&self) -> &'static str {
        "milli"
    }

    fn verify_connection(&self) -> BackendResult<bool> {
        self.request(Method::GET, "health", None).map(|_| true)
    }

    fn create_or_update_index(
        &mut self,
        name: &str,
        settings: &IndexSettings,
    ) -> BackendResult<()> {
        let created = self.request(
            Method::POST,
            "indexes",
            Some(json!({"uid": name, "primaryKey": "id"})),
        );
        match created {
            Err(error) => {
                if !error.to_string().contains("already_exists")
                    && !error.to_string().contains("already exists")
                {
                    return Err(error);
                }
            }
            Ok(response) => {
                self.task(&response)?;
            }
        }
        let response = self.request(
            Method::PATCH,
            &format!("indexes/{name}/settings"),
            Some(Self::settings_json(settings)),
        )?;
        self.task(&response)
    }

    fn add_documents(
        &mut self,
        name: &str,
        documents: &[Document],
        batch_size: usize,
    ) -> BackendResult<IndexingStats> {
        let started = Instant::now();
        let mut indexed = 0;
        for batch in documents.chunks(batch_size.max(1)) {
            let response = self.request(
                Method::POST,
                &format!("indexes/{name}/documents"),
                Some(Value::Array(
                    batch
                        .iter()
                        .map(serde_json::to_value)
                        .collect::<Result<Vec<_>, _>>()?,
                )),
            )?;
            self.task(&response)?;
            indexed += batch.len();
        }
        Ok(IndexingStats {
            total_documents: documents.len(),
            indexed_documents: indexed,
            skipped_documents: 0,
            errors: 0,
            start_time: None,
            end_time: None,
            duration_seconds: started.elapsed().as_secs_f64(),
        })
    }

    fn search(&self, name: &str, query: &SearchQuery) -> BackendResult<Vec<SearchResult>> {
        let mut body = json!({"q": query.text, "limit": query.limit, "offset": query.offset});
        if let Some(document_type) = query.document_type {
            body["filter"] = Value::String(format!("type = {:?}", document_type));
        }
        let response = self.request(Method::POST, &format!("indexes/{name}/search"), Some(body))?;
        Ok(response
            .get("hits")
            .and_then(Value::as_array)
            .map(|hits| hits.iter().map(Self::parse_hit).collect())
            .unwrap_or_default())
    }

    fn list_indices(&self) -> BackendResult<Vec<BackendIndexInfo>> {
        let response = self.request(Method::GET, "indexes", None)?;
        Ok(response
            .get("results")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .map(|item| BackendIndexInfo {
                        name: item
                            .get("uid")
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .into(),
                        documents: 0,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    fn clear_index(&mut self, name: &str) -> BackendResult<()> {
        let response = self.request(Method::DELETE, &format!("indexes/{name}/documents"), None)?;
        self.task(&response)
    }

    fn delete_index(&mut self, name: &str) -> BackendResult<bool> {
        let response = self.request(Method::DELETE, &format!("indexes/{name}"), None)?;
        self.task(&response).map(|_| true)
    }

    fn get_index_stats(&self, name: &str) -> BackendResult<BackendIndexInfo> {
        let response = self.request(Method::GET, &format!("indexes/{name}/stats"), None)?;
        Ok(BackendIndexInfo {
            name: name.to_owned(),
            documents: response
                .get("numberOfDocuments")
                .and_then(Value::as_u64)
                .unwrap_or_default() as usize,
        })
    }

    fn get_synonyms(&self, name: &str) -> BackendResult<BTreeMap<String, Vec<String>>> {
        Ok(self
            .request(
                Method::GET,
                &format!("indexes/{name}/settings/synonyms"),
                None,
            )?
            .as_object()
            .map(|object| {
                object
                    .iter()
                    .filter_map(|(key, values)| {
                        Some((
                            key.clone(),
                            values
                                .as_array()?
                                .iter()
                                .filter_map(Value::as_str)
                                .map(str::to_owned)
                                .collect(),
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    fn update_synonyms(
        &mut self,
        name: &str,
        synonyms: BTreeMap<String, Vec<String>>,
    ) -> BackendResult<()> {
        let response = self.request(
            Method::PUT,
            &format!("indexes/{name}/settings/synonyms"),
            Some(serde_json::to_value(synonyms)?),
        )?;
        self.task(&response)
    }

    fn clear_synonyms(&mut self, name: &str) -> BackendResult<()> {
        self.update_synonyms(name, BTreeMap::new())
    }
}

pub struct MultiBackend {
    backends: Vec<Box<dyn Backend>>,
}

impl MultiBackend {
    pub fn new(backends: Vec<Box<dyn Backend>>) -> BackendResult<Self> {
        if backends.is_empty() {
            return Err(NativeError::Backend(
                "at least one backend is required".into(),
            ));
        }
        Ok(Self { backends })
    }

    pub fn backend_names(&self) -> Vec<&'static str> {
        self.backends.iter().map(|backend| backend.name()).collect()
    }
}

impl Backend for MultiBackend {
    fn name(&self) -> &'static str {
        "multi"
    }

    fn verify_connection(&self) -> BackendResult<bool> {
        Ok(self
            .backends
            .iter()
            .map(|backend| backend.verify_connection())
            .collect::<BackendResult<Vec<_>>>()?
            .into_iter()
            .all(|connected| connected))
    }

    fn create_or_update_index(
        &mut self,
        name: &str,
        settings: &IndexSettings,
    ) -> BackendResult<()> {
        for backend in &mut self.backends {
            backend.create_or_update_index(name, settings)?;
        }
        Ok(())
    }

    fn add_documents(
        &mut self,
        name: &str,
        documents: &[Document],
        batch_size: usize,
    ) -> BackendResult<IndexingStats> {
        let mut result = None;
        for backend in &mut self.backends {
            let stats = backend.add_documents(name, documents, batch_size)?;
            result.get_or_insert(stats);
        }
        result.ok_or_else(|| NativeError::Backend("at least one backend is required".into()))
    }

    fn search(&self, name: &str, query: &SearchQuery) -> BackendResult<Vec<SearchResult>> {
        self.backends[0].search(name, query)
    }

    fn list_indices(&self) -> BackendResult<Vec<BackendIndexInfo>> {
        self.backends[0].list_indices()
    }

    fn clear_index(&mut self, name: &str) -> BackendResult<()> {
        for backend in &mut self.backends {
            backend.clear_index(name)?;
        }
        Ok(())
    }

    fn delete_index(&mut self, name: &str) -> BackendResult<bool> {
        let mut removed = false;
        for backend in &mut self.backends {
            removed |= backend.delete_index(name)?;
        }
        Ok(removed)
    }

    fn get_index_stats(&self, name: &str) -> BackendResult<BackendIndexInfo> {
        self.backends[0].get_index_stats(name)
    }

    fn get_synonyms(&self, name: &str) -> BackendResult<BTreeMap<String, Vec<String>>> {
        self.backends[0].get_synonyms(name)
    }

    fn update_synonyms(
        &mut self,
        name: &str,
        synonyms: BTreeMap<String, Vec<String>>,
    ) -> BackendResult<()> {
        for backend in &mut self.backends {
            backend.update_synonyms(name, synonyms.clone())?;
        }
        Ok(())
    }

    fn clear_synonyms(&mut self, name: &str) -> BackendResult<()> {
        for backend in &mut self.backends {
            backend.clear_synonyms(name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docindexrs_core::DocumentMetadata;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn document(id: &str) -> Document {
        Document {
            id: id.into(),
            document_type: DocumentType::SphinxHtml,
            title: "Rust Guide".into(),
            content: "Searchable Rust documentation".into(),
            filename: format!("{id}.html"),
            url: Some(format!("/{id}.html")),
            summary: Some("Searchable guide".into()),
            code_snippets: Vec::new(),
            metadata: DocumentMetadata {
                source_file: format!("{id}.rst"),
                ..Default::default()
            },
            build_id: None,
        }
    }

    #[test]
    fn oxirs_persists_named_indexes_and_synonyms() {
        let directory = tempfile::tempdir().unwrap();
        let config = OxiRsConfig {
            storage_path: Some(directory.path().into()),
            index_name: "docs".into(),
        };
        let mut backend = OxiRsBackend::new(config.clone()).unwrap();
        backend
            .create_or_update_index("docs", &IndexSettings::default())
            .unwrap();
        backend
            .add_documents("docs", &[document("guide")], 10)
            .unwrap();
        backend
            .update_synonyms(
                "docs",
                BTreeMap::from([(String::from("rust"), vec![String::from("rustlang")])]),
            )
            .unwrap();
        drop(backend);

        let backend = OxiRsBackend::new(config).unwrap();
        assert_eq!(backend.get_index_stats("docs").unwrap().documents, 1);
        assert_eq!(backend.get_synonyms("docs").unwrap()["rust"], ["rustlang"]);
    }

    #[test]
    fn multi_backend_writes_all_and_searches_primary() {
        let first = OxiRsBackend::new(OxiRsConfig::default()).unwrap();
        let second = OxiRsBackend::new(OxiRsConfig::default()).unwrap();
        let mut backend = MultiBackend::new(vec![Box::new(first), Box::new(second)]).unwrap();
        backend
            .create_or_update_index("all", &IndexSettings::default())
            .unwrap();
        backend
            .add_documents("all", &[document("guide")], 10)
            .unwrap();
        assert_eq!(
            backend
                .search(
                    "all",
                    &SearchQuery {
                        text: "rust".into(),
                        ..Default::default()
                    }
                )
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn multi_backend_fans_out_to_secondary_and_propagates_failures() {
        let first_directory = tempfile::tempdir().unwrap();
        let second_directory = tempfile::tempdir().unwrap();
        let first = OxiRsBackend::new(OxiRsConfig {
            storage_path: Some(first_directory.path().into()),
            ..Default::default()
        })
        .unwrap();
        let second = OxiRsBackend::new(OxiRsConfig {
            storage_path: Some(second_directory.path().into()),
            ..Default::default()
        })
        .unwrap();
        let mut backend = MultiBackend::new(vec![Box::new(first), Box::new(second)]).unwrap();
        backend
            .create_or_update_index("all", &IndexSettings::default())
            .unwrap();
        backend
            .add_documents("all", &[document("guide")], 10)
            .unwrap();

        let secondary = OxiRsBackend::new(OxiRsConfig {
            storage_path: Some(second_directory.path().into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(secondary.get_index_stats("all").unwrap().documents, 1);

        let failing = MeilisearchBackend::new(MeilisearchConfig {
            base_url: "http://127.0.0.1:1".into(),
            timeout_seconds: 1,
            ..Default::default()
        })
        .unwrap();
        let healthy = OxiRsBackend::new(OxiRsConfig::default()).unwrap();
        let mut backend = MultiBackend::new(vec![Box::new(healthy), Box::new(failing)]).unwrap();
        assert!(
            backend
                .create_or_update_index("all", &IndexSettings::default())
                .is_err()
        );
    }

    #[test]
    fn meilisearch_adapter_sends_api_key_and_parses_search() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            let mut buffer = [0_u8; 1024];
            loop {
                let bytes_read = stream.read(&mut buffer).unwrap();
                request.extend_from_slice(&buffer[..bytes_read]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            let request = String::from_utf8(request).unwrap();
            assert!(
                request
                    .to_ascii_lowercase()
                    .contains("authorization: bearer test-key")
            );
            assert!(request.contains("POST /indexes/docs/search"));
            let response = r#"{"hits":[{"id":"guide","type":"sphinx_html","title":"Guide","content":"Rust docs","url":"/guide.html"}]}"#;
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                response.len(),
                response
            )
            .unwrap();
        });
        let config = MeilisearchConfig {
            base_url: format!("http://{address}"),
            api_key: Some("test-key".into()),
            ..Default::default()
        };
        let backend = MeilisearchBackend::new(config).unwrap();
        let results = backend
            .search(
                "docs",
                &SearchQuery {
                    text: "rust".into(),
                    ..Default::default()
                },
            )
            .unwrap();
        server.join().unwrap();
        assert_eq!(results[0].id, "guide");
        assert_eq!(results[0].document_type, DocumentType::SphinxHtml);
    }

    #[test]
    fn meilisearch_adapter_maps_admin_and_batch_operations() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = thread::spawn(move || {
            let mut paths = Vec::new();
            loop {
                let (mut stream, _) = listener.accept().unwrap();
                let mut request = Vec::new();
                let mut buffer = [0_u8; 1024];
                loop {
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    if bytes_read == 0 {
                        break;
                    }
                    request.extend_from_slice(&buffer[..bytes_read]);
                    if request.windows(4).any(|window| window == b"\r\n\r\n") {
                        break;
                    }
                }
                let request = String::from_utf8(request).unwrap();
                let path = request
                    .lines()
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .nth(1)
                    .unwrap();
                paths.push(path.to_owned());
                let body = if path.starts_with("/tasks/") {
                    r#"{"status":"succeeded"}"#.to_owned()
                } else if path == "/indexes" {
                    r#"{"results":[{"uid":"docs"}]}"#.to_owned()
                } else if path == "/indexes/docs/stats" {
                    r#"{"numberOfDocuments":1}"#.to_owned()
                } else if path == "/indexes/docs/settings/synonyms" {
                    r#"{"rust":["rustlang"]}"#.to_owned()
                } else if path == "/indexes/docs" && request.starts_with("DELETE") {
                    r#"{"taskUid":6}"#.to_owned()
                } else {
                    r#"{"taskUid":1}"#.to_owned()
                };
                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
                .unwrap();
                if path == "/tasks/6" {
                    break;
                }
            }
            paths
        });

        let config = MeilisearchConfig {
            base_url: format!("http://{address}"),
            ..Default::default()
        };
        let mut backend = MeilisearchBackend::new(config).unwrap();
        backend
            .create_or_update_index("docs", &IndexSettings::default())
            .unwrap();
        backend
            .add_documents("docs", &[document("guide")], 1)
            .unwrap();
        assert_eq!(backend.list_indices().unwrap()[0].name, "docs");
        assert_eq!(backend.get_index_stats("docs").unwrap().documents, 1);
        assert_eq!(backend.get_synonyms("docs").unwrap()["rust"], ["rustlang"]);
        backend
            .update_synonyms(
                "docs",
                BTreeMap::from([(String::from("rust"), vec![String::from("rustlang")])]),
            )
            .unwrap();
        backend.clear_index("docs").unwrap();
        assert!(backend.delete_index("docs").unwrap());
        let paths = server.join().unwrap();
        assert!(paths.iter().any(|path| path == "/indexes/docs/settings"));
        assert!(paths.iter().any(|path| path == "/indexes/docs/documents"));
        assert!(
            paths
                .iter()
                .any(|path| path == "/indexes/docs/settings/synonyms")
        );
        assert!(paths.iter().any(|path| path == "/indexes/docs"));
    }
}
