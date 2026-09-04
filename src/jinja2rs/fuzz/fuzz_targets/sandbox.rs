//! Fuzz strict and sandbox-specific Jinja2rs rendering.
//!
//! In addition to raw template parsing, the generated templates deliberately
//! exercise strict undefined-variable handling and access to a denied
//! `__class__` attribute. The context contains a known object for the latter
//! case so the sandbox policy is reached instead of failing at name lookup.
//!
//! Rendering errors are expected outcomes for rejected templates and denied
//! operations. Panics, aborts, sanitizer findings, and excessive input runtime
//! remain failures.
//!
//! Run this target with:
//!
//! ```text
//! cargo +nightly fuzz run --no-default-features sandbox -- -max_total_time=60
//! ```

#![no_main]

mod support;

use jinja2rs::SandboxedEnvironment;
use libfuzzer_sys::fuzz_target;

use support::{sandbox_template, FuzzCase};

fuzz_target!(|case: FuzzCase| {
    let environment = SandboxedEnvironment::new();
    let template = sandbox_template(&case);
    let input = serde_json::to_value(&case.context).unwrap_or(serde_json::Value::Null);
    let context = serde_json::json!({
        "sandbox_value": {},
        "input": input,
    });
    let _ = environment.render_str(&template, &context);
});
