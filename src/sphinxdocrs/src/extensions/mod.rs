//! Built-in native Sphinx extensions.
//!
//! These extensions use the same application lifecycle as Python extensions,
//! but are resolved before Python import so native builds do not require the
//! reference packages to be installed.

use crate::application::{AppError, SphinxApp};

pub mod docindex;
pub mod webmcp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuiltinMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub parallel_read_safe: bool,
    pub parallel_write_safe: bool,
}

pub fn metadata(name: &str) -> Option<BuiltinMetadata> {
    match name {
        "sphinxdocrs::extensions::docindex" | "sphinxdocrs.extensions.docindex" => {
            Some(docindex::METADATA)
        }
        "sphinxdocrs::extensions::webmcp" | "sphinxdocrs.extensions.webmcp" => {
            Some(webmcp::METADATA)
        }
        _ => None,
    }
}

pub fn setup(name: &str, app: &mut SphinxApp) -> Result<bool, AppError> {
    match name {
        "sphinxdocrs::extensions::docindex" | "sphinxdocrs.extensions.docindex" => {
            docindex::setup(app)?;
            Ok(true)
        }
        "sphinxdocrs::extensions::webmcp" | "sphinxdocrs.extensions.webmcp" => {
            webmcp::setup(app)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub fn build_finished(app: &SphinxApp) -> Result<(), AppError> {
    if app
        .native_extensions
        .contains("sphinxdocrs::extensions::docindex")
    {
        docindex::build_finished(app)?;
    }
    if app
        .native_extensions
        .contains("sphinxdocrs::extensions::webmcp")
    {
        webmcp::build_finished(app)?;
    }
    Ok(())
}
