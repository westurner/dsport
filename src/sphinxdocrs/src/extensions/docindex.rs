//! Native DocIndex Sphinx extension.

use std::fs;

use docindexrs_native::NativeDocIndex;

use crate::application::{AppError, SphinxApp};

pub const METADATA: super::BuiltinMetadata = super::BuiltinMetadata {
    name: "sphinxdocrs::extensions::docindex",
    version: "0.1.0",
    parallel_read_safe: true,
    parallel_write_safe: true,
};

pub fn setup(app: &mut SphinxApp) -> Result<(), AppError> {
    let _ = app;
    Ok(())
}

pub fn build_finished(app: &SphinxApp) -> Result<(), AppError> {
    if !matches!(app.buildername.as_str(), "html" | "singlehtml") {
        return Ok(());
    }
    if !app
        .config
        .get("docindex_enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(true)
    {
        return Ok(());
    }
    let mut index = NativeDocIndex::new();
    index
        .index_directory(&app.outdir)
        .map_err(|error| AppError::Extension(error.to_string()))?;
    if app
        .config
        .get("docindex_artifact_enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(true)
    {
        let artifact_path = app
            .config
            .get("docindex_artifact_path")
            .and_then(|value| value.as_str().map(std::path::PathBuf::from))
            .unwrap_or_else(|| std::path::PathBuf::from("_static/docindex.json"));
        let artifact_path = if artifact_path.is_absolute() {
            artifact_path
        } else {
            app.outdir.join(artifact_path)
        };
        if let Some(parent) = artifact_path.parent() {
            fs::create_dir_all(parent)?;
        }
        index
            .write_artifact(artifact_path)
            .map_err(|error| AppError::Extension(error.to_string()))?;
    }
    if app
        .config
        .get("docindex_rdf_hdt_enabled")
        .and_then(|value| value.as_bool())
        .unwrap_or(true)
    {
        let static_dir = app.outdir.join("_static");
        fs::create_dir_all(&static_dir)?;
        index
            .write_hdt(static_dir.join("docindex.hdt"))
            .map_err(|error| AppError::Extension(error.to_string()))?;
    }
    Ok(())
}
