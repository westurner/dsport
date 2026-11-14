//! EPUB3 builder with Sphinx-compatible ZIP packaging.

use std::collections::{BTreeMap, HashSet};
use std::path::Path;

use docutilsrs::{Doctree, html5, parse_rst_with_source};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use super::{BuildError, BuildResult, Builder};
use crate::config::ConfigVal;
use crate::environment::BuildEnvironment;
use crate::util_strypes::xml_escape;

#[derive(Debug, Default)]
pub struct EpubBuilder;

impl EpubBuilder {
    pub fn new() -> Self {
        Self
    }

    fn document_xhtml(docname: &str, tree: &Doctree) -> String {
        let body = html5(
            tree,
            &docutilsrs::cli::Html5Options {
                epub_compat: true,
                no_section_self_link: Some(String::new()),
                ..docutilsrs::cli::Html5Options::default()
            },
            &docutilsrs::cli::CommonOptions::default(),
        );
        let title = Self::title_text(tree).unwrap_or_else(|| docname.to_string());
        Self::page_xhtml(&title, &format!("  {body}"))
    }

    fn resolve_epub_links(env: &BuildEnvironment, docname: &str, tree: &mut Doctree) {
        env.resolve_xref_nodes(tree, docname);
        env.resolve_toctree_nodes(tree, docname);
        for node_id in 0..tree.nodes_len() {
            let docutilsrs::NodeKind::Reference { refuri, .. } = &mut tree.node_mut(node_id).kind
            else {
                continue;
            };
            *refuri = epub_target_uri(refuri);
        }
    }

    fn genindex_xhtml() -> String {
        Self::page_xhtml(
            "Index",
            "<h1 id=\"index\">Index</h1>\n\n<div class=\"genindex-jumpbox\">\n \n</div>\n",
        )
    }

    fn page_xhtml(title: &str, body: &str) -> String {
        format!(
            "<!DOCTYPE html>\n\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" /><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\n    <title>{}</title>\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"_static/pygments.css\" />\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"_static/epub.css\" /> \n  </head><body>\n\n    <div class=\"document\">\n      <div class=\"documentwrapper\">\n          <div class=\"body\" role=\"main\">\n            \n{}\n\n            <div class=\"clearer\"></div>\n          </div>\n      </div>\n      <div class=\"clearer\"></div>\n    </div>\n  </body>\n</html>",
            xml_escape(title),
            body,
        )
    }

    fn title_text(tree: &Doctree) -> Option<String> {
        fn visit(tree: &Doctree, id: usize, text: &mut String) -> bool {
            if matches!(tree.node(id).kind, docutilsrs::NodeKind::Title) {
                collect_text(tree, id, text);
                return !text.is_empty();
            }
            tree.node(id)
                .children
                .iter()
                .copied()
                .any(|child| visit(tree, child, text))
        }
        fn collect_text(tree: &Doctree, id: usize, text: &mut String) {
            if let docutilsrs::NodeKind::Text(value) = &tree.node(id).kind {
                text.push_str(value);
            }
            for child in tree.node(id).children.iter().copied() {
                collect_text(tree, child, text);
            }
        }
        let mut title = String::new();
        visit(tree, tree.root(), &mut title).then_some(title)
    }

    fn archive(
        entries: &[(String, Vec<u8>)],
        stagedir: Option<&Path>,
    ) -> Result<Vec<u8>, BuildError> {
        Python::attach(|py| -> PyResult<Vec<u8>> {
            let zipfile = PyModule::import(py, "zipfile")?;
            let io = PyModule::import(py, "io")?;
            let buffer = io.getattr("BytesIO")?.call0()?;
            let kwargs = pyo3::types::PyDict::new(py);
            kwargs.set_item("compression", zipfile.getattr("ZIP_DEFLATED")?)?;
            let archive = zipfile
                .getattr("ZipFile")?
                .call((buffer.clone(), "w"), Some(&kwargs))?;
            let zip_info = zipfile.getattr("ZipInfo")?;
            for (name, contents) in entries {
                let info = if let Some(stagedir) = stagedir {
                    let path = stagedir.join(name);
                    let path = path.to_str().ok_or_else(|| {
                        pyo3::exceptions::PyValueError::new_err("non-UTF8 EPUB path")
                    })?;
                    zip_info.call_method1("from_file", (path, name))?
                } else {
                    zip_info.call1((name,))?
                };
                let compression = if name == "mimetype" {
                    zipfile.getattr("ZIP_STORED")?
                } else {
                    zipfile.getattr("ZIP_DEFLATED")?
                };
                info.setattr("compress_type", compression)?;
                archive.call_method1("writestr", (info, PyBytes::new(py, contents)))?;
            }
            archive.call_method0("close")?;
            buffer.call_method0("getvalue")?.extract::<Vec<u8>>()
        })
        .map_err(|error| BuildError::Other(format!("failed to create EPUB archive: {error}")))
    }

    fn package_entries(
        documents: &[(String, String, String)],
        extra_pages: &[(String, String)],
        title: &str,
        author: &str,
        contributor: &str,
        language: &str,
        uid: &str,
        identifier: &str,
        description: &str,
        publisher: &str,
        rights: &str,
        page_progression_direction: &str,
        guides: &[(String, String, String)],
        assets: &[(String, Vec<u8>)],
        epub_version: &str,
        scheme: &str,
        project_version: &str,
        modified_date: &str,
        ibook_scroll_axis: &str,
    ) -> Vec<(String, Vec<u8>)> {
        let mut pages: Vec<(String, Vec<u8>)> = documents
            .iter()
            .map(|(docname, content, _)| (format!("{docname}.xhtml"), content.as_bytes().to_vec()))
            .chain(
                extra_pages
                    .iter()
                    .map(|(filename, content)| (filename.clone(), content.as_bytes().to_vec())),
            )
            .collect();
        pages.sort_by(|left, right| left.0.cmp(&right.0));
        let mut page_ids = BTreeMap::new();
        let mut manifest_items = String::new();
        for (index, (filename, _)) in pages.iter().enumerate() {
            let id = format!("epub-{index}");
            page_ids.insert(filename.clone(), id.clone());
            manifest_items.push_str(&format!(
                "    <item id=\"{id}\" href=\"{}\" media-type=\"application/xhtml+xml\" />\n",
                xml_escape(filename)
            ));
        }
        for (index, (path, _)) in assets.iter().enumerate() {
            manifest_items.push_str(&format!(
                "    <item id=\"epub-{}\" href=\"{}\" media-type=\"{}\" />\n",
                pages.len() + index,
                xml_escape(path),
                epub_media_type(path).unwrap()
            ));
        }
        let spine: String = documents
            .iter()
            .filter_map(|(docname, _, _)| page_ids.get(&format!("{docname}.xhtml")))
            .chain(
                extra_pages
                    .iter()
                    .filter_map(|(filename, _)| page_ids.get(filename)),
            )
            .map(|id| format!("    <itemref idref=\"{id}\" />\n"))
            .collect();
        let guide: String = guides
            .iter()
            .map(|(kind, uri, title)| {
                format!(
                    "    <reference type=\"{}\" title=\"{}\" href=\"{}\" />\n",
                    xml_escape(kind),
                    xml_escape(title),
                    xml_escape(uri)
                )
            })
            .collect();
        let identifier = if epub_version == "3.1" {
            format!(
                "<dc:identifier id=\"{}\" opf:scheme=\"{}\">{}</dc:identifier>",
                xml_escape(uid),
                xml_escape(scheme),
                xml_escape(identifier)
            )
        } else {
            format!(
                "<dc:identifier id=\"{}\">{}</dc:identifier>",
                xml_escape(uid),
                xml_escape(identifier)
            )
        };
        let opf = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<package xmlns=\"http://www.idpf.org/2007/opf\" version=\"{epub_version}\" xml:lang=\"{language}\"\n unique-identifier=\"{uid}\"\n prefix=\"ibooks: http://vocabulary.itunes.apple.com/rdf/ibooks/vocabulary-extensions-1.0/\">\n  <metadata xmlns:opf=\"http://www.idpf.org/2007/opf\"\n        xmlns:dc=\"http://purl.org/dc/elements/1.1/\">\n    <dc:language>{language}</dc:language>\n    <dc:title>{title}</dc:title>\n    <dc:description>{description}</dc:description>\n    <dc:creator>{author}</dc:creator>\n    <dc:contributor>{contributor}</dc:contributor>\n    <dc:publisher>{publisher}</dc:publisher>\n    <dc:rights>{rights}</dc:rights>\n    {identifier}\n    <dc:date>{modified_date}</dc:date>\n    <meta property=\"dcterms:modified\">{modified_date}</meta>\n    <meta property=\"ibooks:version\">{project_version}</meta>\n    <meta property=\"ibooks:specified-fonts\">true</meta>\n    <meta property=\"ibooks:binding\">true</meta>\n    <meta property=\"ibooks:scroll-axis\">{ibook_scroll_axis}</meta>\n  </metadata>\n  <manifest>\n    <item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\" />\n    <item id=\"nav\" href=\"nav.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n{manifest_items}  </manifest>\n  <spine toc=\"ncx\" page-progression-direction=\"{page_progression_direction}\">\n{spine}  </spine>\n  <guide>\n{guide}  </guide>\n</package>",
            epub_version = xml_escape(epub_version),
            language = xml_escape(language),
            uid = xml_escape(uid),
            title = xml_escape(title),
            description = xml_escape(description),
            author = xml_escape(author),
            contributor = xml_escape(contributor),
            publisher = xml_escape(publisher),
            rights = xml_escape(rights),
            modified_date = xml_escape(modified_date),
            project_version = xml_escape(project_version),
            ibook_scroll_axis = xml_escape(ibook_scroll_axis),
        );
        let nav: String = documents
            .iter()
            .map(|(docname, _, title)| {
                format!(
                    "        <li>\n          <a href=\"{}.xhtml\">{}</a>\n        </li>\n",
                    xml_escape(docname),
                    xml_escape(title)
                )
            })
            .collect();
        let nav = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE html>\n<html xmlns=\"http://www.w3.org/1999/xhtml\"\n      xmlns:epub=\"http://www.idpf.org/2007/ops\" lang=\"{}\" xml:lang=\"{}\">\n  <head>\n    <title>Table of Contents</title>\n  </head>\n  <body>\n    <nav epub:type=\"toc\">\n      <h1>Table of Contents</h1>\n      <ol>\n{}      </ol>\n    </nav>\n  </body>\n</html>",
            xml_escape(language),
            xml_escape(language),
            nav
        );
        let nav_points: String = documents
            .iter()
            .enumerate()
            .map(|(index, (docname, _, title))| {
                format!(
                    "    <navPoint id=\"navPoint{}\" playOrder=\"{}\">\n      <navLabel>\n        <text>{}</text>\n      </navLabel>\n      <content src=\"{}.xhtml\" />  \n    </navPoint>\n",
                    index + 1,
                    index + 1,
                    xml_escape(title),
                    xml_escape(docname)
                )
            })
            .collect();
        let toc = format!(
            "<?xml version=\"1.0\"?>\n<ncx version=\"2005-1\" xmlns=\"http://www.daisy.org/z3986/2005/ncx/\">\n  <head>\n    <meta name=\"dtb:uid\" content=\"{}\"/>\n    <meta name=\"dtb:depth\" content=\"1\"/>\n    <meta name=\"dtb:totalPageCount\" content=\"0\"/>\n    <meta name=\"dtb:maxPageNumber\" content=\"0\"/>\n  </head>\n  <docTitle>\n    <text>{}</text>\n  </docTitle>\n  <navMap>    \n{}  </navMap>\n</ncx>",
            xml_escape(uid),
            xml_escape(title),
            nav_points
        );
        let mut entries = vec![
            ("mimetype".to_string(), b"application/epub+zip".to_vec()),
            (
                "META-INF/container.xml".to_string(),
                include_bytes!("../../../sphinx/sphinx/templates/epub3/container.xml").to_vec(),
            ),
            ("content.opf".to_string(), opf.into_bytes()),
            ("toc.ncx".to_string(), toc.into_bytes()),
        ];
        entries.extend(pages);
        entries.extend(assets.iter().cloned());
        entries.push(("nav.xhtml".to_string(), nav.into_bytes()));
        entries
    }
}

fn config_string(env: &BuildEnvironment, key: &str, fallback: &str) -> String {
    env.config
        .get(key)
        .and_then(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| fallback.to_string())
}
fn order_epub_documents(env: &BuildEnvironment, mut candidates: Vec<String>) -> Vec<String> {
    candidates.sort();
    let available: HashSet<_> = candidates.iter().cloned().collect();
    let root = env.config.root_doc();
    let mut ordered = Vec::new();
    let mut seen = HashSet::new();
    if available.contains(&root) && seen.insert(root.clone()) {
        ordered.push(root.clone());
    }
    fn append(
        entries: &[crate::toctree::TocEntry],
        available: &HashSet<String>,
        seen: &mut HashSet<String>,
        ordered: &mut Vec<String>,
    ) {
        for entry in entries {
            if available.contains(&entry.docname) && seen.insert(entry.docname.clone()) {
                ordered.push(entry.docname.clone());
            }
            append(&entry.children, available, seen, ordered);
        }
    }
    append(
        &crate::toctree::resolve(env, &root, 0),
        &available,
        &mut seen,
        &mut ordered,
    );
    for candidate in candidates {
        if seen.insert(candidate.clone()) {
            ordered.push(candidate);
        }
    }
    ordered
}

fn epub_use_index(env: &BuildEnvironment) -> bool {
    if env.config.raw_config().contains_key("epub_use_index") {
        env.config
            .get("epub_use_index")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    } else {
        env.config
            .get("html_use_index")
            .and_then(|value| value.as_bool())
            .unwrap_or(true)
    }
}

fn epub_modified_date() -> String {
    let timestamp = std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|value| value.parse::<i64>().ok())
        .and_then(|seconds| chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, 0));
    timestamp
        .unwrap_or_else(chrono::Utc::now)
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string()
}

fn epub_version(env: &BuildEnvironment) -> String {
    match env.config.get("epub_version") {
        Some(ConfigVal::Float(version)) if version.fract() == 0.0 => {
            format!("{version:.1}")
        }
        Some(value) => value.display(),
        None => "3.0".into(),
    }
}

fn epub_media_type(path: &str) -> Option<&'static str> {
    match Path::new(path)
        .extension()?
        .to_str()?
        .to_ascii_lowercase()
        .as_str()
    {
        "css" => Some("text/css"),
        "gif" => Some("image/gif"),
        "jpeg" | "jpg" => Some("image/jpeg"),
        "png" => Some("image/png"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

fn collect_epub_assets(outdir: &Path) -> Result<Vec<(String, Vec<u8>)>, BuildError> {
    fn collect(
        root: &Path,
        dir: &Path,
        files: &mut BTreeMap<String, Vec<u8>>,
    ) -> Result<(), BuildError> {
        let mut entries = std::fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(std::fs::DirEntry::path);
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                collect(root, &path, files)?;
            } else if path.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                if epub_media_type(&relative).is_some() {
                    let packaged_path = format!("_static/{relative}");
                    files.insert(packaged_path, std::fs::read(path)?);
                }
            }
        }
        Ok(())
    }

    let static_root = outdir.join("_static");
    let mut files = BTreeMap::new();
    if static_root.is_dir() {
        collect(&static_root, &static_root, &mut files)?;
    }
    Ok(files.into_iter().collect())
}

fn stage_epub_files(outdir: &Path, entries: &[(String, Vec<u8>)]) -> Result<(), BuildError> {
    for (name, contents) in entries {
        let path = outdir.join(name);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let template_path = match name.as_str() {
            "mimetype" => Some(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../sphinx/sphinx/templates/epub3/mimetype"),
            ),
            "META-INF/container.xml" => Some(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../sphinx/sphinx/templates/epub3/container.xml"),
            ),
            _ => None,
        };
        if let Some(template_path) = template_path
            && std::fs::read(&template_path)? == *contents
        {
            crate::theme_static::copy_file_preserving_modified_time(&template_path, &path)?;
            continue;
        }
        if name.starts_with("_static/") && path.is_file() && std::fs::read(&path)? == *contents {
            continue;
        }
        std::fs::write(path, contents)?;
    }
    std::fs::write(
        outdir.join(".buildinfo"),
        "# Sphinx build info version 1\nconfig: sphinxdocrs\ntags: head\n",
    )?;
    Ok(())
}

impl Builder for EpubBuilder {
    fn name(&self) -> &str {
        "epub"
    }
    fn format(&self) -> &str {
        "epub"
    }
    fn out_suffix(&self) -> &str {
        ".epub"
    }
    fn get_target_uri(&self, docname: &str) -> String {
        format!("{docname}.xhtml")
    }
    fn build_doc(&self, docname: &str, source: &str, outdir: &Path) -> Result<(), BuildError> {
        let tree = parse_rst_with_source(source, docname);
        let document = Self::document_xhtml(docname, &tree);
        let title = Self::title_text(&tree).unwrap_or_else(|| docname.to_string());
        let path = outdir.join(format!("{docname}.epub"));
        std::fs::create_dir_all(outdir)?;
        std::fs::write(
            path,
            Self::archive(
                &Self::package_entries(
                    &[(docname.to_owned(), document, title)],
                    &[],
                    "Sphinx documentation",
                    "",
                    "unknown",
                    "en",
                    "unknown",
                    "unknown",
                    "",
                    "",
                    "",
                    "ltr",
                    &[],
                    &[],
                    "3.0",
                    "unknown",
                    "",
                    &epub_modified_date(),
                    "vertical",
                ),
                None,
            )?,
        )?;
        Ok(())
    }
    fn build_all(
        &self,
        srcdir: &Path,
        outdir: &Path,
        env: &BuildEnvironment,
    ) -> Result<BuildResult, BuildError> {
        let docnames: Vec<String> = if env.all_docs.is_empty() {
            super::html::discover_docnames_pub(srcdir, &env.config)
        } else {
            env.all_docs.keys().cloned().collect()
        };
        let docnames = order_epub_documents(env, docnames);
        let mut documents = Vec::new();
        for docname in &docnames {
            let mut tree = match env.get_and_resolve_doctree(docname) {
                Ok(tree) => tree,
                Err(_) => {
                    let path = super::html::src_path_for_docname_with_suffixes(
                        srcdir,
                        docname,
                        &env.config,
                    )?;
                    let source =
                        crate::environment::read_source_file(&path, &env.config.source_encoding())
                            .map_err(|e| {
                                BuildError::Other(format!("failed to read {}: {e}", path.display()))
                            })?;
                    parse_rst_with_source(&source, docname)
                }
            };
            Self::resolve_epub_links(env, docname, &mut tree);
            let title = Self::title_text(&tree).unwrap_or_else(|| docname.clone());
            documents.push((docname.clone(), Self::document_xhtml(docname, &tree), title));
        }
        std::fs::create_dir_all(outdir)?;
        let epub_theme = config_string(env, "epub_theme", "epub");
        crate::theme_static::copy_theme_static_files_for_named_theme(
            &env.config,
            outdir,
            srcdir,
            &epub_theme,
            "epub",
        )?;
        super::html::copy_html_static_path(srcdir, outdir, &env.config)?;
        let assets = collect_epub_assets(outdir)?;
        let title = config_string(env, "epub_title", &env.config.project());
        let author = config_string(env, "epub_author", &env.config.author());
        let contributor = config_string(env, "epub_contributor", "unknown");
        let language = config_string(env, "epub_language", &env.config.language());
        let uid = config_string(env, "epub_uid", "unknown");
        let identifier = config_string(env, "epub_identifier", "unknown");
        let description = config_string(env, "epub_description", "unknown");
        let publisher = config_string(env, "epub_publisher", &env.config.author());
        let rights = config_string(
            env,
            "epub_copyright",
            &env.config
                .get("project_copyright")
                .and_then(|value| value.as_str().map(str::to_owned))
                .unwrap_or_default(),
        );
        let basename = env
            .config
            .get("epub_basename")
            .and_then(|value| value.as_str().map(str::to_owned))
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| crate::util_osutil::make_filename(&env.config.project()));
        let page_progression_direction =
            match config_string(env, "epub_writing_mode", "horizontal").as_str() {
                "horizontal" => "ltr",
                "vertical" => "rtl",
                writing_mode => {
                    return Err(BuildError::Other(format!(
                        "epub_writing_mode must be 'horizontal' or 'vertical', got {writing_mode:?}"
                    )));
                }
            };
        let guides = epub_guide_entries(env, !documents.is_empty())?;
        let epub_version = epub_version(env);
        let scheme = config_string(env, "epub_scheme", "unknown");
        let project_version = env.config.version();
        let modified_date = epub_modified_date();
        let ibook_scroll_axis = if page_progression_direction == "ltr" {
            "vertical"
        } else {
            "horizontal"
        };
        let extra_pages = if epub_use_index(env) {
            vec![("genindex.xhtml".to_string(), Self::genindex_xhtml())]
        } else {
            Vec::new()
        };
        let package_entries = Self::package_entries(
            &documents,
            &extra_pages,
            &title,
            &author,
            &contributor,
            &language,
            &uid,
            &identifier,
            &description,
            &publisher,
            &rights,
            page_progression_direction,
            &guides,
            &assets,
            &epub_version,
            &scheme,
            &project_version,
            &modified_date,
            ibook_scroll_axis,
        );
        stage_epub_files(outdir, &package_entries)?;
        std::fs::write(
            outdir.join(format!("{basename}.epub")),
            Self::archive(&package_entries, Some(outdir))?,
        )?;
        Ok(BuildResult {
            written: docnames.len(),
            ..BuildResult::default()
        })
    }
}

fn epub_target_uri(uri: &str) -> String {
    if crate::util_uri::is_url(uri) {
        return uri.to_owned();
    }
    let (path, fragment) = uri
        .split_once('#')
        .map_or((uri, None), |(path, fragment)| (path, Some(fragment)));
    let Some(stem) = path.strip_suffix(".html") else {
        return uri.to_owned();
    };
    match fragment {
        Some(fragment) => format!("{stem}.xhtml#{fragment}"),
        None => format!("{stem}.xhtml"),
    }
}

fn epub_guide_entries(
    env: &BuildEnvironment,
    has_documents: bool,
) -> Result<Vec<(String, String, String)>, BuildError> {
    let guides = env
        .config
        .get("epub_guide")
        .unwrap_or_else(|| ConfigVal::List(vec![]));
    let ConfigVal::List(guides) = guides else {
        return Err(BuildError::Other(
            "epub_guide must be a sequence of (type, uri, title) triples".into(),
        ));
    };
    let mut guides: Vec<(String, String, String)> = guides
        .into_iter()
        .map(|guide| {
            let ConfigVal::List(fields) = guide else {
                return Err(BuildError::Other(
                    "epub_guide must be a sequence of (type, uri, title) triples".into(),
                ));
            };
            let [
                ConfigVal::Str(kind),
                ConfigVal::Str(uri),
                ConfigVal::Str(title),
            ] = fields.as_slice()
            else {
                return Err(BuildError::Other(
                    "epub_guide must be a sequence of (type, uri, title) triples".into(),
                ));
            };
            Ok((kind.clone(), uri.clone(), title.clone()))
        })
        .collect::<Result<_, _>>()?;
    if has_documents && !guides.iter().any(|(kind, _, _)| kind == "toc") {
        guides.push((
            "toc".into(),
            format!("{}.xhtml", env.config.root_doc()),
            "Table of Contents".into(),
        ));
    }
    Ok(guides)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn writes_epub_container_and_document() {
        let out = TempDir::new().unwrap();
        EpubBuilder::new()
            .build_doc("index", "Title\n=====\n\nBody.\n", out.path())
            .unwrap();
        let bytes = std::fs::read(out.path().join("index.epub")).unwrap();
        assert!(bytes.windows(8).any(|window| window == b"mimetype"));
        assert!(
            bytes
                .windows(20)
                .any(|window| window == b"application/epub+zip")
        );
        assert!(bytes.windows(11).any(|window| window == b"content.opf"));
    }

    #[test]
    fn build_all_uses_configured_metadata_and_explicit_docnames() {
        let src = TempDir::new().unwrap();
        let out = TempDir::new().unwrap();
        std::fs::write(src.path().join("index.rst"), "Index\n=====\n\nBody.\n").unwrap();
        let mut config = crate::config::SphinxConfig::new_defaults();
        for (key, value) in [
            ("epub_title", "Book"),
            ("epub_author", "Author"),
            ("epub_language", "en-GB"),
            ("epub_uid", "book-id"),
            ("epub_description", "Description"),
            ("epub_publisher", "Publisher"),
            ("epub_copyright", "Rights"),
            ("epub_basename", "book"),
        ] {
            config.set(key, crate::config::ConfigVal::Str(value.into()));
        }
        let project =
            crate::environment::EnvProject::new(src.path(), &[(".rst", "restructuredtext")]);
        let mut env =
            crate::environment::BuildEnvironment::new(config, project, src.path(), out.path());
        env.all_docs.insert("index".into(), 1);
        let result = EpubBuilder::new()
            .build_all(src.path(), out.path(), &env)
            .unwrap();
        assert_eq!(result.written, 1);
        let bytes = std::fs::read(out.path().join("book.epub")).unwrap();
        assert!(
            bytes
                .windows(b"page-progression-direction=\"ltr\"".len())
                .any(|window| window == b"page-progression-direction=\"ltr\"")
        );
    }

    #[test]
    fn explicit_toc_guide_suppresses_automatic_entry() {
        let mut config = crate::config::SphinxConfig::new_defaults();
        config.set(
            "epub_guide",
            ConfigVal::List(vec![ConfigVal::List(vec![
                ConfigVal::Str("toc".into()),
                ConfigVal::Str("custom-toc.xhtml".into()),
                ConfigVal::Str("My Contents".into()),
            ])]),
        );
        let project = crate::environment::EnvProject::new(
            std::path::PathBuf::from("/tmp"),
            &[(".rst", "restructuredtext")],
        );
        let env = BuildEnvironment::new(config, project, "/tmp", "/tmp/doctrees");

        let guides = epub_guide_entries(&env, true).unwrap();
        assert_eq!(
            guides,
            vec![(
                "toc".into(),
                "custom-toc.xhtml".into(),
                "My Contents".into()
            )]
        );
    }
}
