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
//!
//! ## Accepted deviations
//!
//! - **No stemming.** Upstream applies the Snowball English stemmer so
//!   `running` and `runs` collapse to `run`.  This port indexes the
//!   lowercased surface form.  The search index remains fully functional;
//!   term keys differ from Python for inflected words.
//! - `objects` / `objtypes` / `objnames` / `indexentries` are emitted empty
//!   because the domain object pipeline is not yet ported.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use docutilsrs::doctree::{Doctree, NodeId, NodeKind};

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
}

impl SearchIndex {
    pub fn new() -> Self {
        Self::default()
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
            if Self::is_indexable(word) {
                self.title_mapping
                    .entry(word.clone())
                    .or_default()
                    .insert(docname.to_owned());
            }
        }
        for word in &ws.body_words {
            if !Self::is_indexable(word) {
                continue;
            }
            // Skip body words already indexed as title words for this doc
            // (matches upstream `already_indexed` behaviour).
            if self
                .title_mapping
                .get(word)
                .is_some_and(|s| s.contains(docname))
            {
                continue;
            }
            self.mapping
                .entry(word.clone())
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

        serde_json::json!({
            "docnames": docnames,
            "filenames": filenames,
            "titles": titles,
            "terms": terms,
            "titleterms": titleterms,
            "alltitles": alltitles,
            "objects": {},
            "objtypes": {},
            "objnames": {},
            "indexentries": {},
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
        // Body words present, stopwords absent.
        assert!(json["terms"].get("rendering").is_some());
        assert!(json["terms"].get("layout").is_some());
        assert!(json["terms"].get("the").is_none());
        assert!(json["terms"].get("and").is_none());
    }

    #[test]
    fn term_encoding_int_vs_list() {
        // "shared" appears in two docs → list; "only1" in one → int.
        let d1 = parse_rst_with_source("T1\n==\n\nshared only1 word.\n", "d1");
        let d2 = parse_rst_with_source("T2\n==\n\nshared other word.\n", "d2");
        let mut idx = SearchIndex::new();
        idx.feed("d1", &d1);
        idx.feed("d2", &d2);
        let json = idx.to_json();
        assert!(
            json["terms"]["shared"].is_array(),
            "shared should be a list"
        );
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
}
