//! Docling JSON import and `_toc.yml` selection rules.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use docindexrs_core::{Document, DocumentMetadata, DocumentType};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum DoclingError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("invalid Docling document: {0}")]
    Invalid(String),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TocRule {
    pub pattern: String,
    pub index: bool,
    pub build: bool,
    pub link: bool,
    pub metadata: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct TocSelection {
    pub rules: Vec<TocRule>,
}

impl TocSelection {
    pub fn rule_for(&self, relative_path: &str) -> Option<&TocRule> {
        self.rules
            .iter()
            .rev()
            .find(|rule| wildcard_matches(&rule.pattern, relative_path))
    }

    pub fn allows_index(&self, relative_path: &str) -> bool {
        self.rule_for(relative_path)
            .map(|rule| rule.index)
            .unwrap_or(self.rules.is_empty())
    }
}

/// Parse Jupyter Book / `toctreeyml` entries plus an optional `docindex` block.
///
/// `build: false, index: true, link: true` is the supported representation for
/// a document that should be searchable and linked, but omitted from Sphinx's
/// build toctree. The Sphinx YAML extension consumes the same fields when it
/// renders its toctree.
pub fn load_toc(path: impl AsRef<Path>) -> Result<TocSelection, DoclingError> {
    let value: Value = serde_yaml::from_str(&fs::read_to_string(path)?)?;
    let mut rules = Vec::new();
    if let Some(entries) = value.get("chapters").and_then(Value::as_array) {
        collect_entries(entries, &mut rules, true);
    }
    for block_name in ["docindex", "docling"] {
        if let Some(entries) = value
            .get(block_name)
            .and_then(|value| value.get("paths").or_else(|| value.get("files")))
            .and_then(Value::as_array)
        {
            collect_entries(entries, &mut rules, false);
        }
    }
    if let Some(entries) = value
        .get("sustainablefactory")
        .and_then(|value| value.get("docling").or_else(|| value.get("docindex")))
        .and_then(|value| value.get("paths").or_else(|| value.get("files")))
        .and_then(Value::as_array)
    {
        collect_entries(entries, &mut rules, false);
    }
    Ok(TocSelection { rules })
}

pub fn parse_docling_json(
    bytes: &[u8],
    filename: impl Into<String>,
    rule: Option<&TocRule>,
) -> Result<Document, DoclingError> {
    let value: Value = serde_json::from_slice(bytes)?;
    if value.get("schema_name").and_then(Value::as_str) != Some("DoclingDocument")
        && value.get("texts").is_none()
        && value.get("body").is_none()
    {
        return Err(DoclingError::Invalid(
            "expected a DoclingDocument JSON object".into(),
        ));
    }
    let filename = filename.into();
    let mut paragraphs = Vec::new();
    let mut title = value
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    collect_text(&value, &mut paragraphs, &mut title);
    paragraphs.dedup();
    let content = paragraphs.join("\n\n");
    if title.is_empty() {
        title = filename.clone();
    }
    let mut metadata = DocumentMetadata {
        source_file: filename.clone(),
        word_count: Some(content.split_whitespace().count()),
        ..Default::default()
    };
    let mut url = None;
    if let Some(rule) = rule {
        apply_rule(&mut metadata, &mut url, rule);
        if rule.link && url.is_none() {
            url = Some(format!("/{filename}"));
        }
    }
    Ok(Document {
        id: filename.clone(),
        document_type: DocumentType::Docling,
        title,
        content: content.clone(),
        filename,
        url,
        summary: Some(content.chars().take(240).collect()),
        code_snippets: Vec::new(),
        metadata,
        build_id: None,
    })
}

pub fn collect_docling_files(
    source: impl AsRef<Path>,
    toc: Option<&TocSelection>,
) -> Result<Vec<(PathBuf, Option<TocRule>)>, DoclingError> {
    let source = source.as_ref();
    if source.is_file() {
        let relative = source
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        if toc.is_none_or(|selection| selection.allows_index(&relative)) {
            return Ok(vec![(
                source.to_path_buf(),
                toc.and_then(|s| s.rule_for(&relative).cloned()),
            )]);
        }
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    collect_directory(source, source, toc, &mut files)?;
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(files)
}

fn collect_directory(
    root: &Path,
    directory: &Path,
    toc: Option<&TocSelection>,
    files: &mut Vec<(PathBuf, Option<TocRule>)>,
) -> Result<(), DoclingError> {
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_directory(root, &path, toc, files)?;
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        if toc.is_none_or(|selection| selection.allows_index(&relative)) {
            files.push((path, toc.and_then(|s| s.rule_for(&relative).cloned())));
        }
    }
    Ok(())
}

fn collect_entries(entries: &[Value], rules: &mut Vec<TocRule>, chapter_default: bool) {
    for entry in entries {
        match entry {
            Value::String(pattern) => rules.push(TocRule {
                pattern: pattern.clone(),
                index: chapter_default,
                build: chapter_default,
                link: true,
                ..Default::default()
            }),
            Value::Object(object) => {
                let pattern = object
                    .get("path")
                    .or_else(|| object.get("glob"))
                    .or_else(|| object.get("file"))
                    .and_then(Value::as_str);
                if let Some(pattern) = pattern {
                    rules.push(TocRule {
                        pattern: pattern.to_string(),
                        index: object.get("index").and_then(Value::as_bool).unwrap_or(true),
                        build: object
                            .get("build")
                            .and_then(Value::as_bool)
                            .unwrap_or(chapter_default),
                        link: object.get("link").and_then(Value::as_bool).unwrap_or(true),
                        metadata: object
                            .get("metadata")
                            .and_then(Value::as_object)
                            .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                            .unwrap_or_default(),
                    });
                }
                for key in ["chapters", "sections", "files", "paths"] {
                    if let Some(children) = object.get(key).and_then(Value::as_array) {
                        collect_entries(children, rules, chapter_default);
                    }
                }
            }
            _ => {}
        }
    }
}

fn collect_text(value: &Value, paragraphs: &mut Vec<String>, title: &mut String) {
    match value {
        Value::Array(values) => values
            .iter()
            .for_each(|value| collect_text(value, paragraphs, title)),
        Value::Object(object) => {
            let label = object
                .get("label")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if matches!(label, "title" | "document_title") {
                if let Some(text) = object.get("text").and_then(Value::as_str) {
                    *title = text.to_owned();
                }
            }
            if let Some(text) = object.get("text").and_then(Value::as_str) {
                if !text.trim().is_empty() {
                    paragraphs.push(text.trim().to_owned());
                }
            }
            if let Some(text) = object.get("orig").and_then(Value::as_str) {
                if !text.trim().is_empty() && !object.contains_key("text") {
                    paragraphs.push(text.trim().to_owned());
                }
            }
            object
                .values()
                .for_each(|value| collect_text(value, paragraphs, title));
        }
        _ => {}
    }
}

fn apply_rule(metadata: &mut DocumentMetadata, url: &mut Option<String>, rule: &TocRule) {
    if let Some(value) = rule.metadata.get("url").and_then(Value::as_str) {
        *url = Some(value.into());
    }
    if let Some(values) = rule.metadata.get("tags").and_then(Value::as_array) {
        metadata.tags = values
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
    }
    if let Some(values) = rule.metadata.get("concepts").and_then(Value::as_array) {
        metadata.concepts = values
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect();
    }
    if let Some(value) = rule.metadata.get("heading_level").and_then(Value::as_u64) {
        metadata.heading_level = Some(value as usize);
    }
    if let Some(values) = rule.metadata.get("breadcrumb").and_then(Value::as_array) {
        metadata.breadcrumb = Some(
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
        );
    }
}

fn wildcard_matches(pattern: &str, value: &str) -> bool {
    wildcard_match(pattern.as_bytes(), value.as_bytes())
}

fn wildcard_match(pattern: &[u8], value: &[u8]) -> bool {
    if pattern.is_empty() {
        return value.is_empty();
    }
    if pattern[0] == b'*' {
        return wildcard_match(&pattern[1..], value)
            || (!value.is_empty() && wildcard_match(pattern, &value[1..]));
    }
    !value.is_empty()
        && (pattern[0] == b'?' || pattern[0] == value[0])
        && wildcard_match(&pattern[1..], &value[1..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_docling_text_and_metadata() {
        let rule = TocRule {
            pattern: "*.json".into(),
            metadata: [("tags".into(), serde_json::json!(["paper"]))]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let document = parse_docling_json(
            br#"{"schema_name":"DoclingDocument","name":"Paper","texts":[{"label":"title","text":"A title"},{"text":"Body"}]}"#,
            "paper.json",
            Some(&rule),
        )
        .unwrap();
        assert_eq!(document.document_type, DocumentType::Docling);
        assert_eq!(document.title, "A title");
        assert_eq!(document.metadata.tags, vec!["paper"]);
        assert!(document.content.contains("Body"));
    }

    #[test]
    fn link_rule_defaults_to_source_url() {
        let rule = TocRule {
            pattern: "paper.json".into(),
            link: true,
            ..Default::default()
        };
        let document = parse_docling_json(
            br#"{"schema_name":"DoclingDocument","texts":[{"text":"Body"}]}"#,
            "paper.json",
            Some(&rule),
        )
        .unwrap();
        assert_eq!(document.url.as_deref(), Some("/paper.json"));
    }

    #[test]
    fn toc_supports_link_only_globs() {
        let value = "docindex:\n  paths:\n    - path: papers/**/*.json\n      index: true\n      build: false\n      link: true\n";
        let path = tempfile::NamedTempFile::new().unwrap();
        fs::write(path.path(), value).unwrap();
        let selection = load_toc(path.path()).unwrap();
        let rule = selection.rule_for("papers/one/paper.json").unwrap();
        assert!(rule.index);
        assert!(!rule.build);
        assert!(rule.link);
    }
}
