//! Native DocIndex adapters: filesystem discovery and RDF/HDT artifacts.

pub mod backends;
pub mod docling;

use std::path::Path;
use std::time::Instant;

use docindexrs_core::{
    Document, DocumentIndexer, DocumentSearcher, DocumentType, InMemoryIndex, IndexArtifact,
    IndexingStats, SearchQuery, SearchResult, parse_chat_document, parse_html_document,
};
use nbconvertrs::{TransformOptions, markdown_to_notebook, notebook_to_markdown};
use nbformat::v4::Cell;

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
        self.index_directory_with_options(root, &DirectoryIndexOptions::default())
    }

    pub fn index_directory_with_options(
        &mut self,
        root: impl AsRef<Path>,
        options: &DirectoryIndexOptions,
    ) -> Result<IndexingStats, NativeError> {
        let start = Instant::now();
        let mut documents = Vec::new();
        collect_files(root.as_ref(), &mut documents, options)?;
        let stats = self.index.add_documents(&documents);
        Ok(IndexingStats {
            duration_seconds: start.elapsed().as_secs_f64(),
            ..stats
        })
    }

    pub fn index_docling(
        &mut self,
        source: impl AsRef<Path>,
        toc_path: Option<&Path>,
    ) -> Result<IndexingStats, NativeError> {
        let selection = toc_path
            .map(docling::load_toc)
            .transpose()
            .map_err(|error| NativeError::Backend(error.to_string()))?;
        let files = docling::collect_docling_files(source, selection.as_ref())
            .map_err(|error| NativeError::Backend(error.to_string()))?;
        let mut documents = Vec::new();
        for (path, rule) in files {
            let relative = path.to_string_lossy().replace('\\', "/");
            documents.push(
                docling::parse_docling_json(&std::fs::read(&path)?, relative, rule.as_ref())
                    .map_err(|error| NativeError::Backend(error.to_string()))?,
            );
        }
        Ok(self.index.add_documents(&documents))
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

#[derive(Debug, Clone, Default)]
pub struct DirectoryIndexOptions {
    pub transform_markdown: bool,
    pub transform_options: TransformOptions,
}

fn collect_files(
    root: &Path,
    documents: &mut Vec<docindexrs_core::Document>,
    options: &DirectoryIndexOptions,
) -> Result<(), NativeError> {
    if !root.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, documents, options)?;
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
            "md" | "markdown" | "txt" if options.transform_markdown => {
                let notebook = markdown_to_notebook(&text, &options.transform_options)
                    .map_err(|error| NativeError::Backend(error.to_string()))?;
                let content = notebook_to_markdown(&notebook);
                let title = content
                    .lines()
                    .find_map(|line| line.strip_prefix("# "))
                    .unwrap_or(&relative)
                    .to_owned();
                documents.push(Document {
                    id: relative.clone(),
                    document_type: DocumentType::SphinxNb,
                    title,
                    content: content.clone(),
                    filename: relative.clone(),
                    url: None,
                    summary: Some(content.chars().take(240).collect()),
                    code_snippets: Vec::new(),
                    metadata: docindexrs_core::DocumentMetadata {
                        source_file: relative,
                        word_count: Some(content.split_whitespace().count()),
                        ..Default::default()
                    },
                    build_id: None,
                });
            }
            "md" | "markdown" | "txt" => documents.extend(parse_chat_document(&text, relative)),
            "ipynb" => documents.push(parse_notebook_document(&bytes, relative)?),
            _ => {}
        }
    }
    Ok(())
}

pub fn parse_notebook_document(
    bytes: &[u8],
    filename: impl Into<String>,
) -> Result<Document, NativeError> {
    let filename = filename.into();
    let json = std::str::from_utf8(bytes)
        .map_err(|error| NativeError::Backend(format!("invalid UTF-8 notebook: {error}")))?;
    let parsed =
        nbformat::parse_notebook(json).map_err(|error| NativeError::Backend(error.to_string()))?;
    let notebook = match parsed {
        nbformat::Notebook::V4(notebook) => notebook,
        nbformat::Notebook::V4QuirksMode(quirks) => quirks.repair(),
        nbformat::Notebook::Legacy(notebook) => nbformat::upgrade_legacy_notebook(notebook)
            .map_err(|error| NativeError::Backend(error.to_string()))?,
        nbformat::Notebook::V3(notebook) => nbformat::upgrade_v3_notebook(notebook)
            .map_err(|error| NativeError::Backend(error.to_string()))?,
        _ => {
            return Err(NativeError::Backend("unsupported notebook variant".into()));
        }
    };
    let content = notebook
        .cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Markdown { source, .. } | Cell::Code { source, .. } => Some(source.concat()),
            Cell::Raw { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let title = notebook
        .cells
        .iter()
        .flat_map(|cell| cell.source().iter())
        .find_map(|line| {
            line.strip_prefix("# ")
                .map(str::trim)
                .filter(|line| !line.is_empty())
        })
        .unwrap_or(&filename)
        .to_owned();
    Ok(Document {
        id: filename.clone(),
        document_type: DocumentType::SphinxNb,
        title,
        content: content.clone(),
        filename: filename.clone(),
        url: None,
        summary: Some(content.chars().take(240).collect()),
        code_snippets: Vec::new(),
        metadata: docindexrs_core::DocumentMetadata {
            source_file: filename,
            word_count: Some(content.split_whitespace().count()),
            ..Default::default()
        },
        build_id: None,
    })
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

    #[test]
    fn transforms_markdown_in_process_before_indexing() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(
            root.path().join("guide.md"),
            "# Guide\n\n```rust\nfn main() {}\n```\n",
        )
        .unwrap();
        let mut index = NativeDocIndex::new();
        let stats = index
            .index_directory_with_options(
                root.path(),
                &DirectoryIndexOptions {
                    transform_markdown: true,
                    transform_options: TransformOptions {
                        cell_split: Some("m1".into()),
                    },
                },
            )
            .unwrap();
        assert_eq!(stats.indexed_documents, 1);
        let document = index.index().documents().next().unwrap();
        assert_eq!(document.document_type, DocumentType::SphinxNb);
        assert!(document.content.contains("fn main"));
    }

    #[test]
    fn indexes_existing_notebook_directly() {
        let root = tempfile::tempdir().unwrap();
        std::fs::write(
            root.path().join("guide.ipynb"),
            br##"{"nbformat":4,"nbformat_minor":5,"metadata":{},"cells":[{"cell_type":"markdown","metadata":{},"source":["# Guide\n","\n","Notebook body"]},{"cell_type":"code","execution_count":null,"metadata":{},"outputs":[],"source":["print(1)"]}]}"##,
        )
        .unwrap();
        let mut index = NativeDocIndex::new();
        let stats = index.index_directory(root.path()).unwrap();
        let document = index.index().get("guide.ipynb").unwrap();
        assert_eq!(stats.indexed_documents, 1);
        assert_eq!(document.title, "Guide");
        assert!(document.content.contains("Notebook body"));
        assert!(document.content.contains("print(1)"));
    }
}
