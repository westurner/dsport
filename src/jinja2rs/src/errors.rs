//! `jinja2rs::errors` — error types mirroring `jinja2.exceptions`.

use thiserror::Error;

/// Unified error type for jinja2rs operations.
///
/// Mirrors the Jinja2 exception hierarchy:
/// - [`Jinja2Error::TemplateSyntaxError`] ← `jinja2.TemplateSyntaxError`
/// - [`Jinja2Error::TemplateNotFound`] ← `jinja2.TemplateNotFound`
/// - [`Jinja2Error::TemplatesNotFound`] ← `jinja2.TemplatesNotFound`
/// - [`Jinja2Error::UndefinedError`] ← `jinja2.UndefinedError`
/// - [`Jinja2Error::TemplateRuntimeError`] ← `jinja2.TemplateRuntimeError`
/// - [`Jinja2Error::Render`] ← general render / engine error (minijinja)
#[derive(Debug, Error)]
pub enum Jinja2Error {
    #[error("template syntax error in '{name}': {message}")]
    TemplateSyntaxError { name: String, message: String },

    #[error("template not found: {0}")]
    TemplateNotFound(String),

    #[error("none of the templates were found: {0:?}")]
    TemplatesNotFound(Vec<String>),

    #[error("undefined error: {0}")]
    UndefinedError(String),

    #[error("template runtime error: {0}")]
    TemplateRuntimeError(String),

    #[error("render error: {0}")]
    Render(#[from] minijinja::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<Jinja2Error> for pyo3::PyErr {
    fn from(e: Jinja2Error) -> pyo3::PyErr {
        pyo3::exceptions::PyRuntimeError::new_err(e.to_string())
    }
}
