use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use docindexrs_core::{DocumentIndexer, DocumentType, IndexSettings, SearchQuery};
use docindexrs_native::NativeDocIndex;
use docindexrs_native::backends::{
    Backend, BackendIndexInfo, MeilisearchBackend, MeilisearchConfig, MultiBackend, OxiRsBackend,
    OxiRsConfig,
};
use serde_json::Value;

#[derive(Debug, Parser)]
#[command(name = "docindex", version, about = "Index and search documentation")]
struct Args {
    #[arg(long, global = true, default_value = "oxirs")]
    backend: String,
    #[arg(long, global = true, default_value = "127.0.0.1")]
    host: String,
    #[arg(long, global = true, default_value_t = 7700)]
    port: u16,
    #[arg(long, global = true)]
    api_key: Option<String>,
    #[arg(long, global = true)]
    oxirs_storage_path: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone)]
struct BackendOptions {
    backend: String,
    host: String,
    port: u16,
    api_key: Option<String>,
    oxirs_storage_path: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Index HTML, Markdown, or chat files below a directory.
    Index {
        directory: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Index chat files into a configured backend.
    IndexChats {
        #[arg(long)]
        source: PathBuf,
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value_t = 1000)]
        batch_size: usize,
    },
    /// Index generated Sphinx HTML into a configured backend.
    IndexHtml {
        #[arg(long)]
        source: PathBuf,
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, default_value = "1-3")]
        headings: String,
        #[arg(long)]
        exclude: Vec<String>,
    },
    /// Backward-compatible non-atomic HTML indexing command.
    IndexHtmlLegacy {
        #[arg(long)]
        source: PathBuf,
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Search an artifact using `search ARTIFACT QUERY`, or a backend using `search QUERY`.
    Search {
        first: String,
        second: Option<String>,
        #[arg(long)]
        artifact: Option<PathBuf>,
        #[arg(short, long, default_value_t = 20)]
        limit: usize,
        #[arg(long, default_value_t = 0)]
        offset: usize,
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long, default_value = "json")]
        format: String,
        #[arg(long)]
        document_type: Option<String>,
    },
    /// Show backend connectivity and index counts.
    Status,
    /// List indexes known to the configured backend.
    ListIndices,
    /// Clear all documents from an index.
    ClearIndex {
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Delete an index.
    DeleteIndex {
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Replace synonyms on one or more indexes.
    UpdateSynonyms {
        #[arg(long)]
        file: PathBuf,
        #[arg(long, default_value = "all")]
        indices: String,
    },
    /// Display synonyms stored on an index.
    ShowSynonyms {
        #[arg(long, default_value = "all")]
        index: String,
        #[arg(long)]
        filter: Option<String>,
    },
    /// Suggest acronym synonym pairs from a corpus.
    SuggestSynonyms {
        #[arg(long)]
        source: PathBuf,
        #[arg(long, default_value_t = 2)]
        min_count: usize,
    },
    /// Export glossary synonym entries into a synonym YAML file.
    ExportSynonyms {
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "merge")]
        mode: String,
    },
    /// Generate a MyST glossary page from a glossary YAML file.
    GenerateGlossary {
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        output: PathBuf,
        #[arg(long, default_value = "glossary")]
        label: String,
        #[arg(long, default_value = "Glossary")]
        title: String,
        #[arg(long)]
        force: bool,
    },
    /// Convert an artifact to an RDF-HDT file.
    ExportHdt { artifact: PathBuf, output: PathBuf },
}

fn backend(options: &BackendOptions) -> Result<Box<dyn Backend>, Box<dyn std::error::Error>> {
    let mut backends: Vec<Box<dyn Backend>> = Vec::new();
    for name in options
        .backend
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        match name {
            "oxirs" => backends.push(Box::new(OxiRsBackend::new(OxiRsConfig {
                storage_path: options.oxirs_storage_path.clone(),
                index_name: "all".into(),
            })?)),
            "milli" | "meilisearch" => {
                backends.push(Box::new(MeilisearchBackend::new(MeilisearchConfig {
                    base_url: format!("http://{}:{}", options.host, options.port),
                    api_key: options.api_key.clone(),
                    ..Default::default()
                })?))
            }
            "memory" => backends.push(Box::new(OxiRsBackend::new(OxiRsConfig::default())?)),
            other => return Err(format!("unsupported backend {other:?}").into()),
        }
    }
    Ok(if backends.len() == 1 {
        backends.remove(0)
    } else {
        Box::new(MultiBackend::new(backends)?)
    })
}

fn read_documents(
    source: &Path,
) -> Result<(Vec<docindexrs_core::Document>, usize), Box<dyn std::error::Error>> {
    let mut index = NativeDocIndex::new();
    let stats = index.index_directory(source)?;
    Ok((
        index.index().documents().cloned().collect(),
        stats.indexed_documents,
    ))
}

fn write_stats(stats: &docindexrs_core::IndexingStats) {
    println!(
        "indexed {}/{} documents ({:.1}% success)",
        stats.indexed_documents,
        stats.total_documents,
        stats.success_rate()
    );
}

fn read_synonyms(path: &Path) -> Result<BTreeMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path)?;
    if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
        Ok(serde_json::from_str(&text)?)
    } else {
        Ok(serde_yaml::from_str(&text)?)
    }
}

fn write_yaml(
    path: &Path,
    value: &BTreeMap<String, Vec<String>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_yaml::to_string(value)?)?;
    Ok(())
}

fn glossary_synonyms(
    path: &Path,
) -> Result<BTreeMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let value: Value = serde_yaml::from_str(&fs::read_to_string(path)?)?;
    let mut synonyms = BTreeMap::new();
    let Some(terms) = value.get("terms").and_then(Value::as_object) else {
        return Ok(synonyms);
    };
    for (term, value) in terms {
        if let Some(values) = value.get("synonyms").and_then(Value::as_array) {
            let values = values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            if !values.is_empty() {
                synonyms.insert(term.clone(), values);
            }
        }
    }
    Ok(synonyms)
}

fn print_results(
    results: &[docindexrs_core::SearchResult],
    format: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(results)?),
        "yaml" => println!("{}", serde_yaml::to_string(results)?),
        other => return Err(format!("unsupported output format {other:?}").into()),
    }
    Ok(())
}

fn print_indices(indices: &[BackendIndexInfo]) {
    for info in indices {
        println!("{}: {} documents", info.name, info.documents);
    }
}

fn parse_document_type(
    value: Option<String>,
) -> Result<Option<DocumentType>, Box<dyn std::error::Error>> {
    value
        .map(|value| {
            serde_json::from_value(Value::String(value))
                .map_err(|error| format!("invalid document type: {error}").into())
        })
        .transpose()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let backend_options = BackendOptions {
        backend: args.backend.clone(),
        host: args.host.clone(),
        port: args.port,
        api_key: args.api_key.clone(),
        oxirs_storage_path: args.oxirs_storage_path.clone(),
    };
    match args.command {
        Command::Index { directory, output } => {
            let mut index = NativeDocIndex::new();
            let stats = index.index_directory(directory)?;
            index.write_artifact(output)?;
            write_stats(&stats);
        }
        Command::IndexChats {
            source,
            index: index_name,
            output,
            batch_size,
        } => {
            let (documents, count) = read_documents(&source)?;
            let mut selected = backend(&backend_options)?;
            selected.create_or_update_index(&index_name, &IndexSettings::default())?;
            let stats = selected.add_documents(&index_name, &documents, batch_size)?;
            if let Some(output) = output {
                let mut artifact = NativeDocIndex::new();
                artifact.index_mut().add_documents(&documents);
                artifact.write_artifact(output)?;
            }
            println!("discovered {count} documents");
            write_stats(&stats);
        }
        Command::IndexHtml {
            source,
            index: index_name,
            output,
            headings: _,
            exclude: _,
        }
        | Command::IndexHtmlLegacy {
            source,
            index: index_name,
            output,
        } => {
            let (documents, count) = read_documents(&source)?;
            let mut selected = backend(&backend_options)?;
            selected.create_or_update_index(&index_name, &IndexSettings::default())?;
            let stats = selected.add_documents(&index_name, &documents, 1000)?;
            if let Some(output) = output {
                let mut artifact = NativeDocIndex::new();
                artifact.index_mut().add_documents(&documents);
                artifact.write_artifact(output)?;
            }
            println!("discovered {count} documents");
            write_stats(&stats);
        }
        Command::Search {
            first,
            second,
            artifact,
            limit,
            offset,
            index: index_name,
            format,
            document_type,
        } => {
            let (artifact_path, query) = match (artifact, second) {
                (Some(path), None) => (Some(path), first),
                (None, Some(query)) => (Some(PathBuf::from(first)), query),
                (None, None) => (None, first),
                (Some(_), Some(_)) => {
                    return Err(
                        "provide either --artifact or the legacy ARTIFACT QUERY form".into(),
                    );
                }
            };
            let query = SearchQuery {
                text: query,
                limit,
                offset,
                document_type: parse_document_type(document_type)?,
            };
            let results = if let Some(path) = artifact_path {
                NativeDocIndex::read_artifact(path)?.search(&query)
            } else {
                backend(&backend_options)?.search(&index_name, &query)?
            };
            print_results(&results, &format)?;
        }
        Command::Status => {
            let selected = backend(&backend_options)?;
            println!("connected: {}", selected.verify_connection()?);
            print_indices(&selected.list_indices()?);
        }
        Command::ListIndices => print_indices(&backend(&backend_options)?.list_indices()?),
        Command::ClearIndex { index, confirm } => {
            if !confirm {
                return Err("refusing to clear an index without --confirm".into());
            }
            backend(&backend_options)?.clear_index(&index)?;
            println!("cleared index {index:?}");
        }
        Command::DeleteIndex { index, confirm } => {
            if !confirm {
                return Err("refusing to delete an index without --confirm".into());
            }
            println!(
                "deleted: {}",
                backend(&backend_options)?.delete_index(&index)?
            );
        }
        Command::UpdateSynonyms { file, indices } => {
            let synonyms = read_synonyms(&file)?;
            let mut selected = backend(&backend_options)?;
            for index in indices
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
            {
                selected.update_synonyms(index, synonyms.clone())?;
                println!("updated {index}");
            }
        }
        Command::ShowSynonyms { index, filter } => {
            let synonyms = backend(&backend_options)?.get_synonyms(&index)?;
            for (term, values) in synonyms {
                if filter.as_ref().is_some_and(|value| !term.contains(value)) {
                    continue;
                }
                println!("{term}: {}", values.join(", "));
            }
        }
        Command::SuggestSynonyms { source, min_count } => {
            let mut counts: BTreeMap<(String, String), usize> = BTreeMap::new();
            collect_acronyms(&source, &mut counts)?;
            for ((acronym, expansion), count) in counts {
                if count >= min_count {
                    println!("{acronym}: {expansion} ({count}x)");
                }
            }
        }
        Command::ExportSynonyms {
            source,
            output,
            mode,
        } => {
            let extracted = glossary_synonyms(&source)?;
            let result = if mode.eq_ignore_ascii_case("replace") || !output.is_file() {
                extracted
            } else {
                let mut existing = read_synonyms(&output)?;
                for (key, values) in extracted {
                    existing.entry(key).or_default().extend(values);
                }
                existing
            };
            write_yaml(&output, &result)?;
            println!("wrote {} synonym groups", result.len());
        }
        Command::GenerateGlossary {
            source,
            output,
            label,
            title,
            force,
        } => {
            if output.is_file() && !force {
                println!("unchanged {}", output.display());
            } else {
                let value: Value = serde_yaml::from_str(&fs::read_to_string(&source)?)?;
                let mut text = format!(".. _{label}:\n\n{title}\n{}\n\n", "=".repeat(title.len()));
                if let Some(terms) = value.get("terms").and_then(Value::as_object) {
                    for (term, definition) in terms {
                        text.push_str(&format!("{term}\n{}\n\n", "-".repeat(term.len())));
                        if let Some(description) =
                            definition.get("description").and_then(Value::as_str)
                        {
                            text.push_str(description);
                            text.push_str("\n\n");
                        }
                    }
                }
                if let Some(parent) = output.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(&output, text)?;
                println!("wrote {}", output.display());
            }
        }
        Command::ExportHdt { artifact, output } => {
            NativeDocIndex::read_artifact(artifact)?.write_hdt(output)?;
        }
    }
    Ok(())
}

fn collect_acronyms(
    source: &Path,
    counts: &mut BTreeMap<(String, String), usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    if source.is_dir() {
        for entry in fs::read_dir(source)? {
            collect_acronyms(&entry?.path(), counts)?;
        }
        return Ok(());
    }
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if !matches!(extension, "md" | "markdown" | "txt" | "json") {
        return Ok(());
    }
    let text = fs::read_to_string(source)?;
    for (position, rest) in text.match_indices('(') {
        let Some((acronym, _)) = rest.split_once(')') else {
            continue;
        };
        let acronym = acronym.trim();
        if acronym.len() < 2
            || !acronym
                .chars()
                .all(|character| character.is_ascii_uppercase())
        {
            continue;
        }
        let expansion = text[..position]
            .split_whitespace()
            .rev()
            .take(3)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join(" ");
        if !expansion.is_empty() {
            *counts.entry((acronym.to_owned(), expansion)).or_default() += 1;
        }
    }
    Ok(())
}
