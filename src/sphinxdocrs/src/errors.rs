//! Sphinx exception hierarchy mirrored into Rust-defined Python
//! exception types.
//!
//! These are created with [`pyo3::create_exception!`] so they appear as
//! real `Exception` subclasses on the Python side. Names and inheritance
//! match `sphinx.errors`.
//!
//! The one exception class that needs custom behavior is
//! [`ExtensionError`], whose `repr()` shows the original exception when
//! present. That is implemented as a Python helper in
//! `sphinxdocrs/python/sphinxdocrs_errors.py`, which subclasses the
//! Rust-side `ExtensionError` to add the upstream-compatible `__repr__`.

use pyo3::create_exception;
use pyo3::exceptions::PyException;

create_exception!(sphinxdocrs, SphinxError, PyException);
create_exception!(sphinxdocrs, SphinxWarning, SphinxError);
create_exception!(sphinxdocrs, ApplicationError, SphinxError);
create_exception!(sphinxdocrs, ExtensionError, SphinxError);
create_exception!(sphinxdocrs, BuildEnvironmentError, SphinxError);
create_exception!(sphinxdocrs, ConfigError, SphinxError);
create_exception!(sphinxdocrs, DocumentError, SphinxError);
create_exception!(sphinxdocrs, ThemeError, SphinxError);
create_exception!(sphinxdocrs, VersionRequirementError, SphinxError);
create_exception!(sphinxdocrs, SphinxParallelError, SphinxError);
create_exception!(sphinxdocrs, PycodeError, PyException);
create_exception!(sphinxdocrs, NoUri, PyException);
create_exception!(sphinxdocrs, FiletypeNotFoundError, PyException);
