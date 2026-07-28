//! `sphinxdocrs::search` — Rust port of the search-index core of
//! `sphinx.search.IndexBuilder`.
//!
//! Produces a `searchindex.js` file with the same top-level schema Sphinx
//! emits (`docnames`, `filenames`, `titles`, `terms`, `titleterms`,
//! `alltitles`, `objects`, `objtypes`, `objnames`, `indexentries`,
//! `envversion`), wrapped in the `Search.setIndex(...)` call the JS runtime
//! expects.
//!
//! ## What is ported
//!
//! | upstream symbol | Rust target | notes |
//! |-----------------|-------------|-------|
//! | `IndexBuilder.feed` | [`SearchIndex::feed`] | collect body + title words |
//! | `IndexBuilder.get_terms` | term int/list encoding | single doc → int, many → sorted list |
//! | `IndexBuilder.freeze` | [`SearchIndex::to_json`] | assemble the final object |
//! | `_word_re = \w+` | [`split_words`] | word tokeniser |
//! | `word_filter` | [`SearchIndex::is_indexable`] | drop digits + stopwords |
//! | `SearchEnglish.stem` | [`crate::stemmer::stem`] | Porter2, behind the `search-stemming` feature |
//!
//! ## Accepted deviations
//!
//! - `objects` (**H3d**) is populated from every registered domain's
//!   `get_objects()` via [`SearchIndex::set_objects`], encoded the same
//!   way as upstream's `IndexBuilder.get_objects` (`prefix -> [[docindex,
//!   typeindex, priority, shortanchor, name], ...]`), except every entry
//!   is treated as default priority (`0`) since [`ObjectEntry`] doesn't
//!   carry `sphinx.domains.Domain.get_objects()`'s per-object priority,
//!   and the human-readable type name shown in `objnames` is just the
//!   raw `objtype` string rather than a localized display name.
//! - `indexentries` (**H5e** input) is populated from `.. index::`
//!   scans via [`SearchIndex::set_index_entries`]; entries aren't
//!   attached to a real in-page anchor id (`entry_id` is a synthetic
//!   `"index-N"` per document), matching the page-level granularity
//!   the label/toctree scanners already use.
//! - Only the English stemmer exists; other `search_language` values index the
//!   lowercased surface form. Building without the `search-stemming` feature
//!   does the same, and then term keys differ from Python for inflected words.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};

use crate::domains::{IndexEntry, ObjectEntry};

/// The `envversion` block Sphinx writes.  Values track the on-disk format
/// version of each environment component; consumers only check `sphinx`.
const ENV_VERSION_SPHINX: i64 = 65;

/// Minimal English stopword set (matches `sphinx.search._stopwords.en`).
/// Words containing apostrophes are omitted because the `\w+` tokeniser
/// never produces them.
const STOPWORDS: &[&str] = &[
    "a",
    "about",
    "above",
    "after",
    "again",
    "against",
    "all",
    "am",
    "an",
    "and",
    "any",
    "are",
    "as",
    "at",
    "be",
    "because",
    "been",
    "before",
    "being",
    "below",
    "between",
    "both",
    "but",
    "by",
    "cannot",
    "could",
    "did",
    "do",
    "does",
    "doing",
    "down",
    "during",
    "each",
    "few",
    "for",
    "from",
    "further",
    "had",
    "has",
    "have",
    "having",
    "he",
    "her",
    "here",
    "hers",
    "herself",
    "him",
    "himself",
    "his",
    "how",
    "i",
    "if",
    "in",
    "into",
    "is",
    "it",
    "its",
    "itself",
    "me",
    "more",
    "most",
    "my",
    "myself",
    "no",
    "nor",
    "not",
    "of",
    "off",
    "on",
    "once",
    "only",
    "or",
    "other",
    "ought",
    "our",
    "ours",
    "ourselves",
    "out",
    "over",
    "own",
    "same",
    "she",
    "should",
    "so",
    "some",
    "such",
    "than",
    "that",
    "the",
    "their",
    "theirs",
    "them",
    "themselves",
    "then",
    "there",
    "these",
    "they",
    "this",
    "those",
    "through",
    "to",
    "too",
    "under",
    "until",
    "up",
    "very",
    "was",
    "we",
    "were",
    "what",
    "when",
    "where",
    "which",
    "while",
    "who",
    "whom",
    "why",
    "with",
    "would",
    "you",
    "your",
    "yours",
    "yourself",
    "yourselves",
];

/// Tokenise `text` into lowercase `\w+` words (Unicode word characters).
///
/// Mirrors Sphinx's `_word_re = re.compile(r'\w+')` split, lowercased.
pub fn split_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut cur = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            cur.extend(ch.to_lowercase());
        } else if !cur.is_empty() {
            words.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        words.push(cur);
    }
    words
}

/// Per-document word data collected from a doctree.
struct WordStore {
    /// The document's main title (first `Title` in document order).
    title: String,
    /// Lowercased words found in any title/section heading.
    title_words: Vec<String>,
    /// Lowercased words found in the document body (outside titles).
    body_words: Vec<String>,
    /// All section titles (for the `alltitles` map).
    all_titles: Vec<String>,
}

/// Return `true` when `id` is a descendant of any `Title` node.
fn under_title(tree: &Doctree, id: NodeId) -> bool {
    let mut cur = tree.node(id).parent;
    while let Some(p) = cur {
        if matches!(tree.node(p).kind, NodeKind::Title) {
            return true;
        }
        cur = tree.node(p).parent;
    }
    false
}

/// Collect the full text under a node (all descendant `Text` nodes joined).
fn node_text(tree: &Doctree, id: NodeId) -> String {
    let mut out = String::new();
    fn walk(tree: &Doctree, id: NodeId, out: &mut String) {
        if let NodeKind::Text(s) = &tree.node(id).kind {
            out.push_str(s);
        }
        for &c in &tree.node(id).children {
            walk(tree, c, out);
        }
    }
    walk(tree, id, &mut out);
    out
}

/// Walk a doctree and gather the words that feed the search index.
fn collect_words(tree: &Doctree) -> WordStore {
    let mut title = String::new();
    let mut title_words = Vec::new();
    let mut body_words = Vec::new();
    let mut all_titles = Vec::new();

    for id in 0..tree.nodes_len() {
        match &tree.node(id).kind {
            NodeKind::Title => {
                let t = node_text(tree, id).trim().to_owned();
                if !t.is_empty() {
                    if title.is_empty() {
                        title = t.clone();
                    }
                    all_titles.push(t.clone());
                    title_words.extend(split_words(&t));
                }
            }
            NodeKind::Text(s) if !under_title(tree, id) => {
                body_words.extend(split_words(s));
            }
            _ => {}
        }
    }

    WordStore {
        title,
        title_words,
        body_words,
        all_titles,
    }
}

/// Accumulating search-index builder.
#[derive(Default)]
pub struct SearchIndex {
    /// docname → title text.
    titles: BTreeMap<String, String>,
    /// docname → source filename (`{docname}.rst`).
    filenames: BTreeMap<String, String>,
    /// body term → set of docnames.
    mapping: BTreeMap<String, BTreeSet<String>>,
    /// title term → set of docnames.
    title_mapping: BTreeMap<String, BTreeSet<String>>,
    /// title text → set of docnames (for `alltitles`).
    all_titles: BTreeMap<String, BTreeSet<String>>,
    /// `(domain, ObjectEntry)` pairs from every registered domain
    /// (**H3d** — see [`SearchIndex::set_objects`]).
    objects: Vec<(String, ObjectEntry)>,
    /// docname → `.. index::` entries (**H5e** input).
    index_entries: BTreeMap<String, Vec<IndexEntry>>,
    /// Stemmer for the configured `search_language`.
    #[cfg(feature = "search-stemming")]
    stemmer: crate::stemmer::Stemmer,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an index for a specific `search_language`.
    ///
    /// Mirrors `html_search_language` / `search_language` selecting a
    /// `SearchLanguage` subclass in `sphinx.search`.
    #[cfg(feature = "search-stemming")]
    pub fn with_language(language: &str) -> Self {
        Self {
            stemmer: crate::stemmer::Stemmer::new(language),
            ..Self::default()
        }
    }

    /// Build an index for a specific `search_language` without stemming
    /// support compiled in. The argument is accepted so callers need no
    /// `cfg` of their own; only the tokeniser differs.
    #[cfg(not(feature = "search-stemming"))]
    pub fn with_language(_language: &str) -> Self {
        Self::default()
    }

    /// Pin specific words to the stem Python produces.
    ///
    /// Surfaces the `search_stemming_overrides` configuration value, letting a
    /// project reach exact parity when `rust_stemmers` and `snowballstemmer`
    /// disagree on a word.
    #[cfg(feature = "search-stemming")]
    pub fn with_stemming_overrides(mut self, overrides: crate::stemmer::ParityOverrides) -> Self {
        self.stemmer =
            crate::stemmer::Stemmer::new(self.stemmer.language()).with_overrides(overrides);
        self
    }

    /// Return `true` when a word should be indexed: not purely numeric and
    /// not a stopword.  Mirrors `SearchLanguage.word_filter`.
    fn is_indexable(word: &str) -> bool {
        if word.is_empty() {
            return false;
        }
        if word.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        !STOPWORDS.contains(&word)
    }

    /// Stem a word for use as a term key.
    ///
    /// Mirrors the memoised `stem()` closure in `IndexBuilder.feed`, which is
    /// `self.lang.stem(word).lower()`.
    #[cfg(feature = "search-stemming")]
    fn stem(&self, word: &str) -> String {
        self.stemmer.stem(word)
    }

    /// Identity stem used when the `search-stemming` feature is disabled.
    #[cfg(not(feature = "search-stemming"))]
    fn stem(&self, word: &str) -> String {
        word.to_lowercase()
    }

    /// Feed one parsed document into the index.
    ///
    /// `docname` is the relative name (no extension); `tree` is its parsed
    /// doctree.
    pub fn feed(&mut self, docname: &str, tree: &Doctree) {
        let ws = collect_words(tree);
        self.titles.insert(docname.to_owned(), ws.title.clone());
        self.filenames
            .insert(docname.to_owned(), format!("{docname}.rst"));

        for word in &ws.title_words {
            // Index the stemmed form, falling back to the surface form: the
            // stemmer must not drop words from the index.
            let stemmed = self.stem(word);
            let key = if Self::is_indexable(&stemmed) {
                stemmed
            } else if Self::is_indexable(word) {
                word.clone()
            } else {
                continue;
            };
            self.title_mapping
                .entry(key)
                .or_default()
                .insert(docname.to_owned());
        }
        for word in &ws.body_words {
            let stemmed = self.stem(word);
            let key = if !Self::is_indexable(&stemmed) && Self::is_indexable(word) {
                word.clone()
            } else {
                stemmed
            };
            if !Self::is_indexable(&key) {
                continue;
            }
            // Skip body words already indexed as title words for this doc
            // (matches upstream `already_indexed` behaviour).
            if self
                .title_mapping
                .get(&key)
                .is_some_and(|s| s.contains(docname))
            {
                continue;
            }
            self.mapping
                .entry(key)
                .or_default()
                .insert(docname.to_owned());
        }
        for t in &ws.all_titles {
            self.all_titles
                .entry(t.clone())
                .or_default()
                .insert(docname.to_owned());
        }
    }

    /// Register every domain-registered object for `objects`/`objtypes`/
    /// `objnames` population (**H3d**).
    ///
    /// `objects` is `(domain_name, ObjectEntry)` pairs, typically
    /// `env.domain_objects()`.
    pub fn set_objects(&mut self, objects: Vec<(String, ObjectEntry)>) {
        self.objects = objects;
    }

    /// Register `.. index::` entries recovered per document (**H5e**
    /// input), typically `env.indexentries.clone()`.
    pub fn set_index_entries(&mut self, index_entries: BTreeMap<String, Vec<IndexEntry>>) {
        self.index_entries = index_entries;
    }

    /// Encode `self.objects` into `(objects, objtypes, objnames)`,
    /// matching `IndexBuilder.get_objects`'s schema:
    /// `objects: {prefix: [[docindex, typeindex, priority, shortanchor, name], ...]}`,
    /// `objtypes: {typeindex: "domain:objtype"}`,
    /// `objnames: {typeindex: [domain, objtype, human_readable_name]}`.
    fn encode_objects(
        &self,
        fn2index: &BTreeMap<String, usize>,
    ) -> (serde_json::Value, serde_json::Value, serde_json::Value) {
        let mut sorted = self.objects.clone();
        sorted.sort_by(|a, b| (&a.0, &a.1.name).cmp(&(&b.0, &b.1.name)));

        let mut otypes: BTreeMap<(String, String), usize> = BTreeMap::new();
        let mut onames: BTreeMap<usize, (String, String, String)> = BTreeMap::new();
        let mut rv: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();

        for (domain, entry) in &sorted {
            let Some(&docindex) = fn2index.get(&entry.docname) else {
                continue;
            };
            let (prefix, name) = match entry.name.rfind('.') {
                Some(pos) => (
                    entry.name[..pos].to_string(),
                    entry.name[pos + 1..].to_string(),
                ),
                None => (String::new(), entry.name.clone()),
            };
            let key = (domain.clone(), entry.obj_type.clone());
            let typeindex = *otypes.entry(key.clone()).or_insert_with(|| {
                let idx = onames.len();
                onames.insert(
                    idx,
                    (
                        domain.clone(),
                        entry.obj_type.clone(),
                        entry.obj_type.clone(),
                    ),
                );
                idx
            });
            let shortanchor = if entry.anchor == entry.name {
                String::new()
            } else if entry.anchor == format!("{}-{}", entry.obj_type, entry.name) {
                "-".to_string()
            } else {
                entry.anchor.clone()
            };
            rv.entry(prefix).or_default().push(serde_json::json!([
                docindex,
                typeindex,
                0,
                shortanchor,
                name
            ]));
        }

        let objects = serde_json::Value::Object(
            rv.into_iter()
                .map(|(k, v)| (k, serde_json::Value::Array(v)))
                .collect(),
        );
        let objtypes = serde_json::Value::Object(
            otypes
                .iter()
                .map(|((domain, ty), idx)| {
                    (idx.to_string(), serde_json::json!(format!("{domain}:{ty}")))
                })
                .collect(),
        );
        let objnames = serde_json::Value::Object(
            onames
                .iter()
                .map(|(idx, (domain, ty, human))| {
                    (idx.to_string(), serde_json::json!([domain, ty, human]))
                })
                .collect(),
        );
        (objects, objtypes, objnames)
    }

    /// Encode `self.index_entries` into `entry.lower() -> [[docindex,
    /// entry_id, main], ...]`, matching `IndexBuilder.freeze`'s
    /// `index_entries` key.
    fn encode_index_entries(&self, fn2index: &BTreeMap<String, usize>) -> serde_json::Value {
        let mut rv: BTreeMap<String, Vec<serde_json::Value>> = BTreeMap::new();
        for (docname, entries) in &self.index_entries {
            let Some(&docindex) = fn2index.get(docname) else {
                continue;
            };
            for (i, entry) in entries.iter().enumerate() {
                let entry_id = format!("index-{i}");
                let key = entry.text.to_lowercase();
                rv.entry(key)
                    .or_default()
                    .push(serde_json::json!([docindex, entry_id, entry.main]));
            }
        }
        serde_json::Value::Object(
            rv.into_iter()
                .map(|(k, v)| (k, serde_json::Value::Array(v)))
                .collect(),
        )
    }

    /// Encode a term mapping into `term → int | sorted[int]`, matching
    /// `IndexBuilder.get_terms`.
    fn encode_terms(
        mapping: &BTreeMap<String, BTreeSet<String>>,
        fn2index: &BTreeMap<String, usize>,
    ) -> serde_json::Value {
        let mut obj = serde_json::Map::new();
        for (term, docs) in mapping {
            let mut indices: Vec<usize> = docs
                .iter()
                .filter_map(|d| fn2index.get(d).copied())
                .collect();
            indices.sort_unstable();
            let val = if indices.len() == 1 {
                serde_json::json!(indices[0])
            } else {
                serde_json::json!(indices)
            };
            obj.insert(term.clone(), val);
        }
        serde_json::Value::Object(obj)
    }

    /// Serialise the index to the JSON object Sphinx wraps in
    /// `Search.setIndex(...)`.
    pub fn to_json(&self) -> serde_json::Value {
        // docnames sorted; parallel filenames / titles.
        let docnames: Vec<String> = self.titles.keys().cloned().collect();
        let fn2index: BTreeMap<String, usize> = docnames
            .iter()
            .enumerate()
            .map(|(i, d)| (d.clone(), i))
            .collect();

        let filenames: Vec<String> = docnames
            .iter()
            .map(|d| self.filenames.get(d).cloned().unwrap_or_default())
            .collect();
        let titles: Vec<String> = docnames
            .iter()
            .map(|d| self.titles.get(d).cloned().unwrap_or_default())
            .collect();

        let terms = Self::encode_terms(&self.mapping, &fn2index);
        let titleterms = Self::encode_terms(&self.title_mapping, &fn2index);

        // alltitles: title → [[docindex, null]] (anchors not yet tracked).
        let mut alltitles = serde_json::Map::new();
        for (title, docs) in &self.all_titles {
            let entries: Vec<serde_json::Value> = docs
                .iter()
                .filter_map(|d| fn2index.get(d))
                .map(|&i| serde_json::json!([i, serde_json::Value::Null]))
                .collect();
            alltitles.insert(title.clone(), serde_json::Value::Array(entries));
        }

        let (objects, objtypes, objnames) = self.encode_objects(&fn2index);
        let indexentries = self.encode_index_entries(&fn2index);

        serde_json::json!({
            "docnames": docnames,
            "filenames": filenames,
            "titles": titles,
            "terms": terms,
            "titleterms": titleterms,
            "alltitles": alltitles,
            "objects": objects,
            "objtypes": objtypes,
            "objnames": objnames,
            "indexentries": indexentries,
            "envversion": { "sphinx": ENV_VERSION_SPHINX },
        })
    }

    /// Render the full `searchindex.js` file contents.
    ///
    /// The JS runtime expects `Search.setIndex(<json>)`.
    pub fn to_js(&self) -> String {
        format!("Search.setIndex({})", self.to_json())
    }

    /// Build a search index by feeding every `.rst` document under `srcdir`
    /// and write `searchindex.js` into `outdir`.
    pub fn build_and_write(
        srcdir: &Path,
        outdir: &Path,
        docnames: &[String],
    ) -> std::io::Result<()> {
        use docutilsrs::parse_rst_with_source;
        let mut idx = SearchIndex::new();
        for docname in docnames {
            let src_path = srcdir.join(format!("{docname}.rst"));
            let Ok(source) = std::fs::read_to_string(&src_path) else {
                continue;
            };
            let tree = parse_rst_with_source(&source, docname);
            idx.feed(docname, &tree);
        }
        std::fs::write(outdir.join("searchindex.js"), idx.to_js().as_bytes())
    }

    /// Like [`build_and_write`](Self::build_and_write), but also
    /// populates `objects`/`objtypes`/`objnames`/`indexentries` from the
    /// domain data an [`BuildEnvironment`](crate::environment::BuildEnvironment)
    /// accumulated during the read phase (**H3d**).
    pub fn build_and_write_with_env(
        env: &crate::environment::BuildEnvironment,
        srcdir: &Path,
        outdir: &Path,
        docnames: &[String],
    ) -> std::io::Result<()> {
        use docutilsrs::parse_rst_with_source;
        let mut idx = SearchIndex::new();
        for docname in docnames {
            let src_path = srcdir.join(format!("{docname}.rst"));
            let Ok(source) = std::fs::read_to_string(&src_path) else {
                continue;
            };
            let tree = parse_rst_with_source(&source, docname);
            idx.feed(docname, &tree);
        }
        idx.set_objects(env.domain_objects());
        idx.set_index_entries(env.indexentries.clone().into_iter().collect());
        std::fs::write(outdir.join("searchindex.js"), idx.to_js().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use docutilsrs::parse_rst_with_source;

    #[test]
    fn split_words_basic() {
        assert_eq!(split_words("Hello, World!"), vec!["hello", "world"]);
        assert_eq!(split_words("foo_bar baz123"), vec!["foo_bar", "baz123"]);
        assert!(split_words("   ").is_empty());
    }

    #[test]
    fn stopwords_and_digits_filtered() {
        assert!(!SearchIndex::is_indexable("the"));
        assert!(!SearchIndex::is_indexable("123"));
        assert!(!SearchIndex::is_indexable(""));
        assert!(SearchIndex::is_indexable("widget"));
        assert!(SearchIndex::is_indexable("foo42"));
    }

    #[test]
    fn feed_collects_title_and_body_terms() {
        let rst = "\
Widget Guide\n\
============\n\
\n\
The widget handles rendering and layout.\n";
        let tree = parse_rst_with_source(rst, "guide");
        let mut idx = SearchIndex::new();
        idx.feed("guide", &tree);
        let json = idx.to_json();

        // Title recorded.
        assert_eq!(json["titles"][0], "Widget Guide");
        // "widget" appears in the title → titleterms.
        assert!(json["titleterms"].get("widget").is_some());
        // Body words present (stemmed), stopwords absent.
        assert!(json["terms"].get("render").is_some());
        assert!(json["terms"].get("layout").is_some());
        assert!(json["terms"].get("the").is_none());
        assert!(json["terms"].get("and").is_none());
    }

    #[test]
    fn term_encoding_int_vs_list() {
        // "shared" stems to "share", which appears in two docs → list;
        // "only1" in one → int.
        let d1 = parse_rst_with_source("T1\n==\n\nshared only1 word.\n", "d1");
        let d2 = parse_rst_with_source("T2\n==\n\nshared other word.\n", "d2");
        let mut idx = SearchIndex::new();
        idx.feed("d1", &d1);
        idx.feed("d2", &d2);
        let json = idx.to_json();
        assert!(json["terms"]["share"].is_array(), "share should be a list");
        assert!(json["terms"]["only1"].is_number(), "only1 should be an int");
    }

    #[test]
    fn to_js_wraps_setindex() {
        let tree = parse_rst_with_source("Title\n=====\n\nBody text.\n", "index");
        let mut idx = SearchIndex::new();
        idx.feed("index", &tree);
        let js = idx.to_js();
        assert!(js.starts_with("Search.setIndex("));
        assert!(js.ends_with(')'));
        // The payload must be valid JSON.
        let inner = &js["Search.setIndex(".len()..js.len() - 1];
        let v: serde_json::Value = serde_json::from_str(inner).unwrap();
        assert!(v["docnames"].is_array());
    }

    #[test]
    fn required_schema_keys_present() {
        let tree = parse_rst_with_source("A\n=\n\ntext.\n", "index");
        let mut idx = SearchIndex::new();
        idx.feed("index", &tree);
        let json = idx.to_json();
        for key in [
            "docnames",
            "filenames",
            "titles",
            "terms",
            "titleterms",
            "alltitles",
            "objects",
            "objtypes",
            "objnames",
            "indexentries",
            "envversion",
        ] {
            assert!(json.get(key).is_some(), "missing key {key}");
        }
        assert_eq!(json["envversion"]["sphinx"], ENV_VERSION_SPHINX);
    }

    #[test]
    fn build_and_write_creates_file() {
        use tempfile::TempDir;
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(
            src.path().join("index.rst"),
            "Home\n====\n\nWelcome to the docs.\n",
        )
        .unwrap();
        SearchIndex::build_and_write(src.path(), out.path(), &["index".to_string()]).unwrap();
        let path = out.path().join("searchindex.js");
        assert!(path.exists());
        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("Search.setIndex("));
        assert!(content.contains("\"docnames\""));
    }

    #[test]
    fn set_objects_populates_objects_objtypes_objnames() {
        let tree = parse_rst_with_source("Guide\n=====\n\nText.\n", "guide");
        let mut idx = SearchIndex::new();
        idx.feed("guide", &tree);
        idx.set_objects(vec![(
            "py".to_string(),
            ObjectEntry {
                obj_type: "function".to_string(),
                name: "pkg.mod.greet".to_string(),
                docname: "guide".to_string(),
                anchor: "py-function-greet".to_string(),
            },
        )]);
        let json = idx.to_json();
        assert_eq!(json["objtypes"]["0"], "py:function");
        assert_eq!(json["objnames"]["0"][0], "py");
        let prefix_entries = json["objects"]["pkg.mod"].as_array().unwrap();
        assert_eq!(prefix_entries.len(), 1);
        assert_eq!(prefix_entries[0][4], "greet");
    }

    #[test]
    fn set_index_entries_populates_indexentries() {
        let tree = parse_rst_with_source("Guide\n=====\n\nText.\n", "guide");
        let mut idx = SearchIndex::new();
        idx.feed("guide", &tree);
        let mut entries = BTreeMap::new();
        entries.insert(
            "guide".to_string(),
            vec![IndexEntry {
                kind: crate::domains::IndexEntryKind::Single,
                text: "Widget".to_string(),
                see_target: None,
                main: false,
            }],
        );
        idx.set_index_entries(entries);
        let json = idx.to_json();
        let hits = json["indexentries"]["widget"].as_array().unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0][1], "index-0");
        assert_eq!(hits[0][2], false);
    }
}
