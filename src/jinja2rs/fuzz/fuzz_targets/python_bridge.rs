#![no_main]

mod support;

use libfuzzer_sys::fuzz_target;
use pyo3::prelude::*;
use pyo3::types::PyModule;

use support::FuzzCase;

fuzz_target!(|case: FuzzCase| {
    let context_json = match serde_json::to_string(&case.context) {
        Ok(value) => value,
        Err(_) => return,
    };

    Python::attach(|py| {
        let module = match PyModule::new(py, "jinja2rs_fuzz") {
            Ok(module) => module,
            Err(_) => return,
        };
        if jinja2rs::jinja2rs(&module).is_err() {
            return;
        }

        let json = match PyModule::import(py, "json") {
            Ok(json) => json,
            Err(_) => return,
        };
        let context = match json
            .getattr("loads")
            .and_then(|loads| loads.call1((context_json.as_str(),)))
        {
            Ok(context) => context,
            Err(_) => return,
        };
        let class_name = if case.sandbox {
            "SandboxedEnvironment"
        } else {
            "Environment"
        };
        let environment = match module.getattr(class_name).and_then(|class| class.call0()) {
            Ok(environment) => environment,
            Err(_) => return,
        };

        let _ = environment.call_method1("render_str", (case.template.as_str(), context));
    });
});
