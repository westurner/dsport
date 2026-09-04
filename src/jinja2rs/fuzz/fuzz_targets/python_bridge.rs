//! Fuzz the optional PyO3 compatibility bridge.
//!
//! Each input initializes the exported `jinja2rs` module, constructs either
//! `Environment` or `SandboxedEnvironment`, and calls its Python-facing
//! `render_str` method. The generated recursive value is converted into native
//! Python containers and scalars, while a `bytes` value exercises the bridge's
//! fallback conversion path. Non-finite floats also reach the conversion code
//! without being discarded by an intermediate JSON round-trip.
//!
//! Python interpreter shutdown allocations are known to be reported by
//! LeakSanitizer in this embedded-interpreter target. Use
//! `LSAN_OPTIONS=detect_leaks=0`; the shared runner applies that setting only to
//! this target, and retains leak checking for the Rust targets.
//!
//! Run this target with:
//!
//! ```text
//! LSAN_OPTIONS=detect_leaks=0 cargo +nightly fuzz run --no-default-features \
//!     --features python python_bridge -- -max_total_time=60
//! ```

#![no_main]

mod support;

use libfuzzer_sys::fuzz_target;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyModule, PyString};

use support::{render_template, FuzzCase, FuzzValue};

fuzz_target!(|case: FuzzCase| {
    let _: PyResult<()> = Python::attach(|py| {
        let module = PyModule::new(py, "jinja2rs_fuzz")?;
        jinja2rs::jinja2rs(&module)?;

        let context = PyDict::new(py);
        context.set_item("value", fuzz_value_to_python(py, &case.context)?)?;
        context.set_item("text", case.template.as_str())?;
        context.set_item("fallback", PyBytes::new(py, case.template.as_bytes()))?;
        context.set_item("sandbox_value", PyDict::new(py))?;

        let class_name = if case.sandbox {
            "SandboxedEnvironment"
        } else {
            "Environment"
        };
        let environment = module.getattr(class_name)?.call0()?;

        let template = if case.sandbox {
            support::sandbox_template(&case)
        } else {
            render_template(&case)
        };
        let _ = environment.call_method1("render_str", (template.as_str(), context));
        Ok(())
    });
});

fn fuzz_value_to_python<'py>(py: Python<'py>, value: &FuzzValue) -> PyResult<Py<PyAny>> {
    match value {
        FuzzValue::None => Ok(py.None()),
        FuzzValue::Bool(value) => Ok(value.into_pyobject(py)?.to_owned().unbind().into_any()),
        FuzzValue::Integer(value) => Ok(value.into_pyobject(py)?.into_any().unbind()),
        FuzzValue::Float(value) => Ok(value.into_pyobject(py)?.into_any().unbind()),
        FuzzValue::String(value) => Ok(PyString::new(py, value).unbind().into_any()),
        FuzzValue::List(values) => {
            let list = PyList::empty(py);
            for value in values {
                list.append(fuzz_value_to_python(py, value)?)?;
            }
            Ok(list.unbind().into_any())
        }
        FuzzValue::Map(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, fuzz_value_to_python(py, value)?)?;
            }
            Ok(dict.unbind().into_any())
        }
    }
}
