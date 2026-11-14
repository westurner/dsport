//! EPUB3 builder with Sphinx-compatible ZIP packaging.

use std::collections::{BTreeMap, HashSet};
use std::path::{Component, Path, PathBuf};

use docutilsrs::{Doctree, html5, parse_rst_with_source};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use super::{BuildError, BuildResult, Builder};
use crate::config::ConfigVal;
use crate::environment::BuildEnvironment;
use crate::util_strypes::xml_escape;

#[derive(Debug, Default)]
pub struct EpubBuilder;

struct EpubPackage<'a> {
    documents: &'a [(String, String, String)],
    extra_pages: &'a [(String, String)],
    pre_pages: &'a [(String, String, String)],
    post_pages: &'a [(String, String, String)],
    cover_page: Option<&'a (String, String, String)>,
    cover_image: Option<&'a str>,
    title: &'a str,
    author: &'a str,
    contributor: &'a str,
    language: &'a str,
    uid: &'a str,
    identifier: &'a str,
    description: &'a str,
    publisher: &'a str,
    rights: &'a str,
    page_progression_direction: &'a str,
    guides: &'a [(String, String, String)],
    assets: &'a [(String, Vec<u8>)],
    epub_version: &'a str,
    scheme: &'a str,
    project_version: &'a str,
    modified_date: &'a str,
    ibook_scroll_axis: &'a str,
}

impl EpubBuilder {
    pub fn new() -> Self {
        Self
    }

    fn document_xhtml(docname: &str, tree: &Doctree, css_links: &str) -> String {
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
        Self::page_xhtml(docname, &title, &format!("  {body}"), css_links)
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

    fn genindex_xhtml(css_links: &str) -> String {
        Self::page_xhtml(
            "genindex",
            "Index",
            "<h1 id=\"index\">Index</h1>\n\n<div class=\"genindex-jumpbox\">\n \n</div>\n",
            css_links,
        )
    }

    fn page_xhtml(docname: &str, title: &str, body: &str, css_links: &str) -> String {
        let static_prefix =
            crate::util_osutil::relative_uri(&format!("{docname}.xhtml"), "_static");
        format!(
            "<!DOCTYPE html>\n\n<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" /><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n\n    <title>{}</title>\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"{static_prefix}/pygments.css\" />\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"{static_prefix}/epub.css\" /> \n{}  </head><body>\n\n    <div class=\"document\">\n      <div class=\"documentwrapper\">\n          <div class=\"body\" role=\"main\">\n            \n{}\n\n            <div class=\"clearer\"></div>\n          </div>\n      </div>\n      <div class=\"clearer\"></div>\n    </div>\n  </body>\n</html>",
            xml_escape(title),
            css_links,
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

    fn package_entries(package: EpubPackage<'_>) -> Vec<(String, Vec<u8>)> {
        let EpubPackage {
            documents,
            extra_pages,
            pre_pages,
            post_pages,
            cover_page,
            cover_image,
            title,
            author,
            contributor,
            language,
            uid,
            identifier,
            description,
            publisher,
            rights,
            page_progression_direction,
            guides,
            assets,
            epub_version,
            scheme,
            project_version,
            modified_date,
            ibook_scroll_axis,
        } = package;
        let mut pages: Vec<(String, Vec<u8>)> = cover_page
            .into_iter()
            .chain(pre_pages.iter())
            .map(|(filename, content, _)| (filename.clone(), content.as_bytes().to_vec()))
            .chain(documents.iter().map(|(docname, content, _)| {
                (format!("{docname}.xhtml"), content.as_bytes().to_vec())
            }))
            .chain(
                post_pages
                    .iter()
                    .map(|(filename, content, _)| (filename.clone(), content.as_bytes().to_vec())),
            )
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
        let cover_image_id = cover_image.and_then(|cover_image| {
            assets
                .iter()
                .position(|(path, _)| path == cover_image)
                .map(|index| format!("epub-{}", pages.len() + index))
        });
        for (index, (path, _)) in assets.iter().enumerate() {
            let properties = if cover_image == Some(path.as_str()) && epub_version.starts_with('3')
            {
                " properties=\"cover-image\""
            } else {
                ""
            };
            manifest_items.push_str(&format!(
                "    <item id=\"epub-{}\" href=\"{}\" media-type=\"{}\"{properties} />\n",
                pages.len() + index,
                xml_escape(path),
                epub_media_type(path).unwrap()
            ));
        }
        let cover_meta = cover_image_id
            .map(|id| format!("    <meta name=\"cover\" content=\"{id}\" />\n"))
            .unwrap_or_default();
        let spine: String = cover_page
            .into_iter()
            .filter_map(|(filename, _, _)| page_ids.get(filename))
            .chain(
                pre_pages
                    .iter()
                    .filter_map(|(filename, _, _)| page_ids.get(filename)),
            )
            .chain(
                documents
                    .iter()
                    .filter_map(|(docname, _, _)| page_ids.get(&format!("{docname}.xhtml"))),
            )
            .chain(
                post_pages
                    .iter()
                    .filter_map(|(filename, _, _)| page_ids.get(filename)),
            )
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
        let opf = if cover_meta.is_empty() {
            opf
        } else {
            opf.replacen(
                "    <meta property=\"dcterms:modified\">",
                &format!("{cover_meta}    <meta property=\"dcterms:modified\">"),
                1,
            )
        };
        let nav_documents: Vec<_> = pre_pages
            .iter()
            .map(|(filename, _, title)| (filename.clone(), title.clone()))
            .chain(
                documents
                    .iter()
                    .map(|(docname, _, title)| (format!("{docname}.xhtml"), title.clone())),
            )
            .chain(
                post_pages
                    .iter()
                    .map(|(filename, _, title)| (filename.clone(), title.clone())),
            )
            .collect();
        let nav: String = nav_documents
            .iter()
            .filter(|(_, title)| !title.is_empty())
            .map(|(filename, title)| {
                format!(
                    "        <li>\n          <a href=\"{}\">{}</a>\n        </li>\n",
                    xml_escape(filename),
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
        let nav_points: String = nav_documents
            .iter()
            .filter(|(_, title)| !title.is_empty())
            .enumerate()
            .map(|(index, (filename, title))| {
                format!(
                    "    <navPoint id=\"navPoint{}\" playOrder=\"{}\">\n      <navLabel>\n        <text>{}</text>\n      </navLabel>\n      <content src=\"{}\" />  \n    </navPoint>\n",
                    index + 1,
                    index + 1,
                    xml_escape(title),
                    xml_escape(filename)
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
        "webp" => Some("image/webp"),
        "otf" => Some("font/otf"),
        "ttf" => Some("font/ttf"),
        "woff" => Some("font/woff"),
        _ => None,
    }
}

fn collect_epub_assets(outdir: &Path) -> Result<Vec<(String, Vec<u8>)>, BuildError> {
    fn collect(
        root: &Path,
        dir: &Path,
        archive_root: &str,
        files: &mut BTreeMap<String, Vec<u8>>,
    ) -> Result<(), BuildError> {
        let mut entries = std::fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(std::fs::DirEntry::path);
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                collect(root, &path, archive_root, files)?;
            } else if path.is_file() {
                let relative = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/");
                if epub_media_type(&relative).is_some() {
                    let packaged_path = format!("{archive_root}/{relative}");
                    files.insert(packaged_path, std::fs::read(path)?);
                }
            }
        }
        Ok(())
    }

    let mut files = BTreeMap::new();
    for archive_root in ["_static", "_images"] {
        let root = outdir.join(archive_root);
        if root.is_dir() {
            collect(&root, &root, archive_root, &mut files)?;
        }
    }
    Ok(files.into_iter().collect())
}

fn epub_css_links(env: &BuildEnvironment, docname: &str) -> Result<String, BuildError> {
    let configured = if env.config.raw_config().contains_key("epub_css_files") {
        env.config.get("epub_css_files")
    } else {
        env.config.get("html_css_files")
    };
    let Some(ConfigVal::List(files)) = configured else {
        return Ok(String::new());
    };
    let base = format!("{docname}.xhtml");
    let mut links = String::new();
    for file in &files {
        let (filename, attribute_values) = match file {
            ConfigVal::Str(filename) => (filename.as_str(), None),
            ConfigVal::List(parts) if parts.len() == 2 => {
                let Some(filename) = parts[0].as_str() else {
                    continue;
                };
                (filename, parts[1].as_map())
            }
            _ => continue,
        };
        if filename.is_empty() {
            continue;
        }
        let filename = filename.replace('\\', "/");
        let href = if crate::util_uri::is_url(&filename)
            || filename.starts_with('/')
            || filename.starts_with("_static/")
        {
            filename
        } else {
            format!("_static/{filename}")
        };
        let href = if crate::util_uri::is_url(&href) || href.starts_with('/') {
            href
        } else {
            crate::util_osutil::relative_uri(&base, &href)
        };
        let mut media_attribute = String::new();
        let mut other_attributes = String::new();
        if let Some(attributes_map) = attribute_values {
            for (name, value) in attributes_map {
                let Some(value) = value.as_str() else {
                    continue;
                };
                if name.is_empty()
                    || !name.chars().all(|character| {
                        character.is_ascii_alphanumeric() || "-_:".contains(character)
                    })
                {
                    continue;
                }
                let attribute = format!(" {name}=\"{}\"", xml_escape(value));
                if name == "media" {
                    media_attribute = attribute;
                } else {
                    other_attributes.push_str(&attribute);
                }
            }
        }
        links.push_str(&format!(
            "    <link{media_attribute} rel=\"stylesheet\"{other_attributes} type=\"text/css\" href=\"{}\" />\n",
            xml_escape(&href)
        ));
    }
    Ok(links)
}

fn safe_epub_path(path: &str) -> Result<String, BuildError> {
    let path = path.replace('\\', "/");
    let parsed = Path::new(&path);
    if parsed.is_absolute()
        || parsed.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(BuildError::Other(format!(
            "EPUB configured path must be relative and stay within the output directory: {path:?}"
        )));
    }
    Ok(path)
}

fn stage_configured_epub_file(
    env: &BuildEnvironment,
    srcdir: &Path,
    outdir: &Path,
    filename: &str,
) -> Result<(String, PathBuf), BuildError> {
    let filename = safe_epub_path(filename)?;
    let destination = outdir.join(&filename);
    if destination.is_file() {
        return Ok((filename, destination));
    }

    let mut candidates = vec![srcdir.join(&filename)];
    if let Some(ConfigVal::List(extra_paths)) = env.config.raw_config().get("html_extra_path") {
        for extra_path in extra_paths.iter().filter_map(ConfigVal::as_str) {
            candidates.push(srcdir.join(extra_path).join(&filename));
        }
    }
    let source = candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            BuildError::Other(format!(
                "EPUB configured file {:?} was not found in the source or html_extra_path",
                filename
            ))
        })?;
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    crate::theme_static::copy_file_preserving_modified_time(&source, &destination)?;
    Ok((filename, destination))
}

fn configured_epub_pages(
    env: &BuildEnvironment,
    srcdir: &Path,
    outdir: &Path,
    option: &str,
) -> Result<Vec<(String, String, String)>, BuildError> {
    let Some(ConfigVal::List(entries)) = env.config.get(option) else {
        return Ok(Vec::new());
    };
    entries
        .iter()
        .map(|entry| {
            let ConfigVal::List(fields) = entry else {
                return Err(BuildError::Other(format!(
                    "{option} entries must be (filename, title) pairs"
                )));
            };
            let [ConfigVal::Str(filename), ConfigVal::Str(title)] = fields.as_slice() else {
                return Err(BuildError::Other(format!(
                    "{option} entries must be (filename, title) pairs"
                )));
            };
            let (filename, path) = stage_configured_epub_file(env, srcdir, outdir, filename)?;
            let content = std::fs::read_to_string(path)?;
            Ok((filename, content, title.clone()))
        })
        .collect()
}

fn configured_epub_cover(
    env: &BuildEnvironment,
    srcdir: &Path,
    outdir: &Path,
) -> Result<Option<(String, Option<String>)>, BuildError> {
    let Some(value) = env.config.get("epub_cover") else {
        return Ok(None);
    };
    let ConfigVal::List(fields) = value else {
        return Err(BuildError::Other(
            "epub_cover must be an (image, template) pair".into(),
        ));
    };
    if fields.is_empty() {
        return Ok(None);
    }
    let [ConfigVal::Str(image), template] = fields.as_slice() else {
        return Err(BuildError::Other(
            "epub_cover must be an (image, template) pair".into(),
        ));
    };
    let template = match template {
        ConfigVal::Null => None,
        ConfigVal::Str(template) if template.is_empty() => None,
        ConfigVal::Str(template) => Some(template.clone()),
        _ => {
            return Err(BuildError::Other(
                "epub_cover template must be a filename or None".into(),
            ));
        }
    };
    let (image, path) = stage_configured_epub_file(env, srcdir, outdir, image)?;
    if !epub_media_type(&image).is_some_and(|media_type| media_type.starts_with("image/")) {
        return Err(BuildError::Other(format!(
            "epub_cover image has an unsupported media type: {image:?}"
        )));
    }
    if !path.is_file() {
        return Err(BuildError::Other(format!(
            "epub_cover image was not staged: {image:?}"
        )));
    }
    Ok(Some((image, template)))
}

fn render_epub_cover_page(
    env: &BuildEnvironment,
    srcdir: &Path,
    image: &str,
    template: &str,
    css_links: &str,
) -> Result<(String, String, String), BuildError> {
    let template = safe_epub_path(template)?;
    let mut candidates = vec![srcdir.join(&template)];
    if let Some(ConfigVal::List(template_dirs)) = env.config.get("templates_path") {
        candidates.extend(
            template_dirs
                .iter()
                .filter_map(ConfigVal::as_str)
                .map(|dir| srcdir.join(dir).join(&template)),
        );
    }
    candidates.push(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../sphinx/sphinx/themes/epub")
            .join(&template),
    );
    let template_path = candidates
        .into_iter()
        .find(|path| path.is_file())
        .ok_or_else(|| {
            BuildError::Other(format!("EPUB cover template was not found: {template:?}"))
        })?;
    let source = std::fs::read_to_string(template_path)?;
    let source = if let Some(start) = source.find("{% block content %}") {
        let body_start = start + "{% block content %}".len();
        let body_end = source[body_start..]
            .find("{% endblock %}")
            .map(|offset| body_start + offset)
            .ok_or_else(|| {
                BuildError::Other("EPUB cover template has an unterminated content block".into())
            })?;
        &source[body_start..body_end]
    } else {
        source.as_str()
    };
    let context = BTreeMap::from([
        (
            "image".to_string(),
            serde_json::Value::String(xml_escape(image)),
        ),
        (
            "title".to_string(),
            serde_json::Value::String(xml_escape(&env.config.project())),
        ),
    ]);
    let rendered = jinja2rs::Environment::new()
        .render_str(source, &context)
        .map_err(|error| BuildError::Other(format!("failed to render EPUB cover: {error}")))?;
    let title = config_string(env, "epub_title", &env.config.project());
    let content = if rendered.to_ascii_lowercase().contains("<html") {
        rendered
    } else {
        EpubBuilder::page_xhtml("epub-cover", &title, &rendered, css_links)
    };
    Ok(("epub-cover.xhtml".into(), content, title))
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
        "# Sphinx build info version 1\n\
         # This file records the configuration used when building these files. When it is not found, a full rebuild will be done.\n\
         config: sphinxdocrs\n\
         tags: head\n",
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
        let document = Self::document_xhtml(docname, &tree, "");
        let title = Self::title_text(&tree).unwrap_or_else(|| docname.to_string());
        let path = outdir.join(format!("{docname}.epub"));
        std::fs::create_dir_all(outdir)?;
        std::fs::write(
            path,
            Self::archive(
                &Self::package_entries(EpubPackage {
                    documents: &[(docname.to_owned(), document, title)],
                    extra_pages: &[],
                    pre_pages: &[],
                    post_pages: &[],
                    cover_page: None,
                    cover_image: None,
                    title: "Sphinx documentation",
                    author: "",
                    contributor: "unknown",
                    language: "en",
                    uid: "unknown",
                    identifier: "unknown",
                    description: "",
                    publisher: "",
                    rights: "",
                    page_progression_direction: "ltr",
                    guides: &[],
                    assets: &[],
                    epub_version: "3.0",
                    scheme: "unknown",
                    project_version: "",
                    modified_date: &epub_modified_date(),
                    ibook_scroll_axis: "vertical",
                }),
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
            let image_prefix =
                crate::util_osutil::relative_uri(&format!("{docname}.xhtml"), "_images");
            super::html::copy_image_assets(&mut tree, srcdir, outdir, docname, &image_prefix)?;
            let title = Self::title_text(&tree).unwrap_or_else(|| docname.clone());
            let css_links = epub_css_links(env, docname)?;
            documents.push((
                docname.clone(),
                Self::document_xhtml(docname, &tree, &css_links),
                title,
            ));
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
        let mut assets = collect_epub_assets(outdir)?;
        let cover = configured_epub_cover(env, srcdir, outdir)?;
        let cover_image = cover.as_ref().map(|(image, _)| image.as_str());
        if let Some(cover_image) = cover_image
            && !assets.iter().any(|(path, _)| path == cover_image)
        {
            assets.push((
                cover_image.to_string(),
                std::fs::read(outdir.join(cover_image))?,
            ));
            assets.sort_by(|left, right| left.0.cmp(&right.0));
        }
        let cover_page = match cover.as_ref() {
            Some((image, Some(template))) => Some(render_epub_cover_page(
                env,
                srcdir,
                image,
                template,
                &epub_css_links(env, "epub-cover")?,
            )?),
            _ => None,
        };
        let pre_pages = configured_epub_pages(env, srcdir, outdir, "epub_pre_files")?;
        let post_pages = configured_epub_pages(env, srcdir, outdir, "epub_post_files")?;
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
        let guides = epub_guide_entries(env, !documents.is_empty(), cover_page.is_some())?;
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
            vec![(
                "genindex.xhtml".to_string(),
                Self::genindex_xhtml(&epub_css_links(env, "genindex")?),
            )]
        } else {
            Vec::new()
        };
        let package_entries = Self::package_entries(EpubPackage {
            documents: &documents,
            extra_pages: &extra_pages,
            pre_pages: &pre_pages,
            post_pages: &post_pages,
            cover_page: cover_page.as_ref(),
            cover_image,
            title: &title,
            author: &author,
            contributor: &contributor,
            language: &language,
            uid: &uid,
            identifier: &identifier,
            description: &description,
            publisher: &publisher,
            rights: &rights,
            page_progression_direction,
            guides: &guides,
            assets: &assets,
            epub_version: &epub_version,
            scheme: &scheme,
            project_version: &project_version,
            modified_date: &modified_date,
            ibook_scroll_axis,
        });
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
    has_cover_page: bool,
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
    if has_cover_page && !guides.iter().any(|(kind, _, _)| kind == "cover") {
        guides.push(("cover".into(), "epub-cover.xhtml".into(), "Cover".into()));
    }
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

        let guides = epub_guide_entries(&env, true, false).unwrap();
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
