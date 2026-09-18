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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigVal;
    use std::collections::HashMap;

    fn app(builder: &str) -> (tempfile::TempDir, tempfile::TempDir, SphinxApp) {
        let src = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();
        let doctrees = tempfile::tempdir().unwrap();
        std::fs::write(src.path().join("index.rst"), "Index\n=====\n").unwrap();
        let app = SphinxApp::new(
            src.path(),
            out.path(),
            doctrees.path(),
            builder,
            HashMap::new(),
        )
        .unwrap();
        (src, out, app)
    }

    #[test]
    fn setup_is_a_successful_noop() {
        let (_src, _out, mut app) = app("html");
        setup(&mut app).unwrap();
    }

    #[test]
    fn build_finished_skips_non_html_and_disabled_index() {
        let (_src, _out, latex_app) = app("latex");
        build_finished(&latex_app).unwrap();

        let (_src, _out, mut app) = app("html");
        app.config.set("docindex_enabled", ConfigVal::Bool(false));
        build_finished(&app).unwrap();
    }

    #[test]
    fn build_finished_writes_relative_artifact_and_hdt() {
        let (_src, out, mut app) = app("html");
        app.config.set(
            "docindex_artifact_path",
            ConfigVal::Str("metadata/index.json".into()),
        );
        build_finished(&app).unwrap();
        assert!(out.path().join("metadata/index.json").is_file());
        assert!(out.path().join("_static/docindex.hdt").is_file());
    }

    #[test]
    fn build_finished_writes_absolute_artifact_without_hdt() {
        let (_src, out, mut app) = app("singlehtml");
        let artifact = out.path().join("absolute.json");
        app.config.set(
            "docindex_artifact_path",
            ConfigVal::Str(artifact.to_string_lossy().into_owned()),
        );
        app.config
            .set("docindex_rdf_hdt_enabled", ConfigVal::Bool(false));
        build_finished(&app).unwrap();
        assert!(artifact.is_file());
        assert!(!out.path().join("_static/docindex.hdt").exists());
    }

    #[test]
    fn build_finished_can_disable_artifact_but_keep_hdt() {
        let (_src, out, mut app) = app("html");
        app.config
            .set("docindex_artifact_enabled", ConfigVal::Bool(false));
        build_finished(&app).unwrap();
        assert!(!out.path().join("_static/docindex.json").exists());
        assert!(out.path().join("_static/docindex.hdt").is_file());
    }
}
