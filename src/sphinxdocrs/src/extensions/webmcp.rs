//! Built-in WebMCP extension for native HTML builders.

use std::collections::BTreeMap;
use std::fs;

use serde::Serialize;

use crate::application::{AppError, SphinxApp};

pub const METADATA: super::BuiltinMetadata = super::BuiltinMetadata {
    name: "sphinxdocrs::extensions::webmcp",
    version: "0.1.0",
    parallel_read_safe: true,
    parallel_write_safe: true,
};

const WEBMCP_JS: &str = include_str!("webmcp.js");

#[derive(Debug, Serialize)]
struct Manifest {
    schema_version: u32,
    doctree_schema: u32,
    builder: String,
    project: String,
    version: String,
    pages: Vec<Page>,
    navigation: Navigation,
    search: Search,
    webmcp: Webmcp,
    artifacts: Artifacts,
}

#[derive(Debug, Serialize)]
struct Page {
    docname: String,
    title: String,
    url: String,
    source: String,
}
#[derive(Debug, Serialize)]
struct Navigation {
    root: String,
    children: Vec<String>,
}
#[derive(Debug, Serialize)]
struct Search {
    mode: String,
    native: NativeSearch,
    docindex: DocindexSearch,
}
#[derive(Debug, Serialize)]
struct NativeSearch {
    page: String,
    index: String,
}
#[derive(Debug, Serialize)]
struct DocindexSearch {
    enabled: bool,
    artifact: String,
}
#[derive(Debug, Serialize)]
struct Webmcp {
    exposed_to: Vec<String>,
    tools: BTreeMap<String, bool>,
}
#[derive(Debug, Serialize)]
struct Artifacts {
    docindex: Option<String>,
    rdf_hdt: Option<String>,
}

pub fn setup(app: &mut SphinxApp) -> Result<(), AppError> {
    if matches!(app.buildername.as_str(), "html" | "singlehtml") {
        let static_dir = app.outdir.join("_static");
        fs::create_dir_all(&static_dir)?;
        fs::write(static_dir.join("webmcp.js"), WEBMCP_JS)?;
        app.registry
            .borrow_mut()
            .add_js_file(Some("webmcp.js"), std::collections::HashMap::new());
    }
    Ok(())
}

pub fn build_finished(app: &SphinxApp) -> Result<(), AppError> {
    if !matches!(app.buildername.as_str(), "html" | "singlehtml") {
        return Ok(());
    }
    let env = app.env.borrow();
    let mut pages = env
        .found_docs()
        .iter()
        .map(|docname| {
            let path = env.doc2path(docname);
            let source = path
                .strip_prefix(&app.srcdir)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            Page {
                docname: docname.clone(),
                title: env.get_title(docname).unwrap_or(docname).to_string(),
                url: if app.buildername == "singlehtml" {
                    format!("index.html#{docname}")
                } else {
                    format!("{docname}.html")
                },
                source,
            }
        })
        .collect::<Vec<_>>();
    drop(env);
    pages.sort_by(|left, right| left.docname.cmp(&right.docname));
    let root = app.config.root_doc();
    let docindex_path = app
        .config
        .get("docindex_artifact_path")
        .and_then(|value| value.as_str().map(std::path::PathBuf::from))
        .unwrap_or_else(|| std::path::PathBuf::from("_static/docindex.json"));
    let docindex_active = app
        .native_extensions
        .contains("sphinxdocrs::extensions::docindex");
    let docindex_enabled = docindex_active
        && app
            .config
            .get("docindex_artifact_enabled")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
        && app.outdir.join(&docindex_path).exists();
    let docindex_url = docindex_path.to_string_lossy().replace('\\', "/");
    let rdf_hdt_url = "_static/docindex.hdt";
    let rdf_hdt = docindex_active
        .then(|| {
            app.outdir
                .join(rdf_hdt_url)
                .exists()
                .then_some(rdf_hdt_url.into())
        })
        .flatten();
    let mut tools = BTreeMap::new();
    tools.insert("page_context".into(), true);
    tools.insert("list_navigation".into(), true);
    tools.insert("get_documentation_metadata".into(), true);
    tools.insert("search".into(), true);
    tools.insert("navigate".into(), true);
    let manifest = Manifest {
        schema_version: 1,
        doctree_schema: 1,
        builder: app.buildername.clone(),
        project: app.config.project(),
        version: app
            .config
            .get("version")
            .map(|value| value.as_str().unwrap_or_default().to_string())
            .unwrap_or_default(),
        navigation: Navigation {
            root,
            children: pages.iter().map(|page| page.docname.clone()).collect(),
        },
        pages,
        search: Search {
            mode: if docindex_enabled {
                "docindex+native".into()
            } else {
                "native".into()
            },
            native: NativeSearch {
                page: "search.html".into(),
                index: "searchindex.js".into(),
            },
            docindex: DocindexSearch {
                enabled: docindex_enabled,
                artifact: docindex_url.clone(),
            },
        },
        webmcp: Webmcp {
            exposed_to: Vec::new(),
            tools,
        },
        artifacts: Artifacts {
            docindex: docindex_enabled.then_some(docindex_url),
            rdf_hdt,
        },
    };
    let output = app.outdir.join("webmcp.json");
    let temporary = output.with_extension("json.tmp");
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(&manifest)
            .map_err(|error| AppError::Extension(error.to_string()))?,
    )?;
    fs::rename(temporary, output)?;
    let static_dir = app.outdir.join("_static");
    fs::create_dir_all(&static_dir)?;
    fs::write(static_dir.join("webmcp.js"), WEBMCP_JS)?;
    Ok(())
}
