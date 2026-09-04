#![no_main]

mod support;

use jinja2rs::SandboxedEnvironment;
use libfuzzer_sys::fuzz_target;

use support::FuzzCase;

fuzz_target!(|case: FuzzCase| {
    let environment = SandboxedEnvironment::new();
    let _ = environment.render_str(&case.template, &case.context);
});
