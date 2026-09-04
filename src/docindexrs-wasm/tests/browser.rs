use docindexrs_core::{
    Document, DocumentIndexer, DocumentMetadata, DocumentType, InMemoryIndex, IndexArtifact,
};
use docindexrs_wasm::WasmDocIndex;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

fn document(id: &str, content: &str) -> Document {
    Document {
        id: id.into(),
        document_type: DocumentType::SphinxHtml,
        title: format!("Guide {id}"),
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

fn artifact() -> Vec<u8> {
    let mut index = InMemoryIndex::default();
    index.add_documents(&[
        document("one", "Rust browser search"),
        document("two", "Rust pagination search"),
        document("three", "Rust browser examples"),
    ]);
    index.export_json().expect("serialize test artifact")
}

#[wasm_bindgen_test]
fn loads_shared_json_artifact_and_searches() {
    let index = WasmDocIndex::from_bytes(&artifact()).expect("load artifact");
    let value = index.search("browser", 20, 0).expect("search artifact");
    let results: Vec<docindexrs_core::SearchResult> =
        serde_wasm_bindgen::from_value(value).expect("decode search results");
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].id, "one");
}

#[wasm_bindgen_test]
fn looks_up_documents_and_supports_pagination() {
    let index = WasmDocIndex::from_bytes(&artifact()).expect("load artifact");
    let document = index.document("two").expect("find document");
    let document: Document = serde_wasm_bindgen::from_value(document).expect("decode document");
    assert_eq!(document.title, "Guide two");

    let value = index.search("Rust", 1, 1).expect("paged search");
    let results: Vec<docindexrs_core::SearchResult> =
        serde_wasm_bindgen::from_value(value).expect("decode paged results");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "two");
}

#[wasm_bindgen_test]
fn rejects_malformed_artifacts_and_missing_documents() {
    assert!(WasmDocIndex::from_bytes(b"not-json").is_err());
    let index = WasmDocIndex::from_bytes(&artifact()).expect("load artifact");
    assert!(index.document("missing").is_err());
}
