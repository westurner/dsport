//! `jinja2rs::errors` — error types mirroring `jinja2.exceptions`

use pyo3::create_exception;
use thiserror::Error;

// Define custom Python exceptions that mirror jinja2 exception hierarchy
create_exception!(jinja2rs, TemplateNotFound, pyo3::exceptions::PyException);
create_exception!(jinja2rs, TemplateError, pyo3::exceptions::PyException);
create_exception!(jinja2rs, TemplateSyntaxError, TemplateError);
create_exception!(jinja2rs, UndefinedError, TemplateError);
create_exception!(jinja2rs, TemplateRuntimeError, TemplateError);

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
        match e {
            Jinja2Error::TemplateNotFound(ref name) => {
                TemplateNotFound::new_err(format!("Template '{}' not found", name))
            }
            Jinja2Error::TemplateSyntaxError {
                ref name,
                ref message,
            } => TemplateSyntaxError::new_err(format!("Syntax error in '{}': {}", name, message)),
            Jinja2Error::UndefinedError(ref msg) => UndefinedError::new_err(msg.clone()),
            Jinja2Error::TemplateRuntimeError(ref msg) => {
                TemplateRuntimeError::new_err(msg.clone())
            }
            Jinja2Error::TemplatesNotFound(_) => TemplateNotFound::new_err("No templates found"),
            Jinja2Error::Render(ref err) => TemplateRuntimeError::new_err(err.to_string()),
            Jinja2Error::Io(ref err) => pyo3::exceptions::PyIOError::new_err(err.to_string()),
        }
    }
}
