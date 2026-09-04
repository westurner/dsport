//! Native DocIndex adapters: filesystem discovery and RDF/HDT artifacts.

pub mod backends;

use std::path::Path;
use std::time::Instant;

use docindexrs_core::{
    DocumentIndexer, DocumentSearcher, InMemoryIndex, IndexArtifact, IndexingStats, SearchQuery,
    SearchResult, parse_chat_document, parse_html_document,
};

#[derive(Debug, thiserror::Error)]
pub enum NativeError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("docindex error: {0}")]
    Core(#[from] docindexrs_core::DocIndexError),
    #[error("HDT error: {0}")]
    Hdt(String),
    #[error("backend error: {0}")]
    Backend(String),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct NativeDocIndex {
    index: InMemoryIndex,
}

impl Default for NativeDocIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl NativeDocIndex {
    pub fn new() -> Self {
        Self {
            index: InMemoryIndex::default(),
        }
    }
    pub fn index(&self) -> &InMemoryIndex {
        &self.index
    }
    pub fn index_mut(&mut self) -> &mut InMemoryIndex {
        &mut self.index
    }

    pub fn index_directory(
        &mut self,
        root: impl AsRef<Path>,
    ) -> Result<IndexingStats, NativeError> {
        let start = Instant::now();
        let mut documents = Vec::new();
        collect_files(root.as_ref(), &mut documents)?;
        let stats = self.index.add_documents(&documents);
        Ok(IndexingStats {
            duration_seconds: start.elapsed().as_secs_f64(),
            ..stats
        })
    }

    pub fn search(&self, query: &SearchQuery) -> Vec<SearchResult> {
        self.index.search(query)
    }
    pub fn write_artifact(&self, path: impl AsRef<Path>) -> Result<(), NativeError> {
        std::fs::write(path, self.index.export_json()?)?;
        Ok(())
    }
    pub fn read_artifact(path: impl AsRef<Path>) -> Result<Self, NativeError> {
        Ok(Self {
            index: InMemoryIndex::import_json(&std::fs::read(path)?)?,
        })
    }

    /// Export the indexed documents as a small RDF dataset and encode it as HDT.
    /// The document artifact remains the authoritative search representation.
    pub fn write_hdt(&self, path: impl AsRef<Path>) -> Result<(), NativeError> {
        let mut nt = String::new();
        for document in self.index.documents() {
            let subject = format!("<urn:docindex:{}>", escape_iri(&document.id));
            nt.push_str(&format!(
                "{subject} <urn:docindex:title> {} .\n",
                literal(&document.title)
            ));
            nt.push_str(&format!(
                "{subject} <urn:docindex:content> {} .\n",
                literal(&document.content)
            ));
            nt.push_str(&format!(
                "{subject} <urn:docindex:type> {} .\n",
                literal(&format!("{:?}", document.document_type).to_ascii_lowercase())
            ));
        }
        let mut input = std::io::Cursor::new(nt.as_bytes());
        let mut output = std::fs::File::create(path)?;
        rdfhdt::ntriples_to_hdt(&mut input, "urn:docindex:", &mut output)
            .map(|_| ())
            .map_err(|error| NativeError::Hdt(error.to_string()))
    }
}

impl DocumentIndexer for NativeDocIndex {
    fn add_documents(&mut self, documents: &[docindexrs_core::Document]) -> IndexingStats {
        self.index.add_documents(documents)
    }
    fn clear(&mut self) {
        self.index.clear();
    }
}

fn collect_files(
    root: &Path,
    documents: &mut Vec<docindexrs_core::Document>,
) -> Result<(), NativeError> {
    if !root.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, documents)?;
            continue;
        }
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        let bytes = std::fs::read(&path)?;
        let text = String::from_utf8_lossy(&bytes);
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        match extension {
            "html" | "htm" => documents.push(parse_html_document(
                &text,
                relative.clone(),
                Some(format!("/{relative}")),
            )),
            "md" | "markdown" | "txt" => documents.extend(parse_chat_document(&text, relative)),
            _ => {}
        }
    }
    Ok(())
}

fn escape_iri(value: &str) -> String {
    value
        .replace('>', "%3E")
        .replace('<', "%3C")
        .replace(' ', "%20")
}
fn literal(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use docindexrs_core::SearchQuery;

    #[test]
    fn indexes_html_directory_and_writes_artifact() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(
            root.path().join("guide.html"),
            "<title>Guide</title><p>Rust docs</p>",
        )
        .unwrap();
        let mut index = NativeDocIndex::new();
        let stats = index.index_directory(root.path()).unwrap();
        assert_eq!(stats.indexed_documents, 1);
        assert_eq!(
            index
                .search(&SearchQuery {
                    text: "rust".into(),
                    ..Default::default()
                })
                .len(),
            1
        );
        let artifact = root.path().join("index.json");
        index.write_artifact(&artifact).unwrap();
        assert_eq!(
            NativeDocIndex::read_artifact(&artifact)
                .unwrap()
                .index()
                .documents()
                .count(),
            1
        );
    }
}
