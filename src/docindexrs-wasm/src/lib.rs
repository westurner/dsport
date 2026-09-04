use docindexrs_core::{DocumentSearcher, InMemoryIndex, IndexArtifact, SearchQuery};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmDocIndex {
    index: InMemoryIndex,
}

#[wasm_bindgen]
impl WasmDocIndex {
    #[wasm_bindgen(js_name = fromBytes)]
    pub fn from_bytes(bytes: &[u8]) -> Result<WasmDocIndex, JsValue> {
        let index = InMemoryIndex::import_json(bytes)
            .map_err(|error| JsValue::from_str(&error.to_string()))?;
        Ok(Self { index })
    }

    pub fn search(&self, query: &str, limit: usize, offset: usize) -> Result<JsValue, JsValue> {
        let results = self.index.search(&SearchQuery {
            text: query.to_string(),
            limit,
            offset,
            ..Default::default()
        });
        serde_wasm_bindgen::to_value(&results)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }

    pub fn document(&self, id: &str) -> Result<JsValue, JsValue> {
        let document = self
            .index
            .get(id)
            .ok_or_else(|| JsValue::from_str("document not found"))?;
        serde_wasm_bindgen::to_value(document)
            .map_err(|error| JsValue::from_str(&error.to_string()))
    }
}
