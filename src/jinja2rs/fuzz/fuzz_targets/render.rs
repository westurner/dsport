#![no_main]

mod support;

use jinja2rs::{DjangoMode, Environment};
use libfuzzer_sys::fuzz_target;

use support::FuzzCase;

fuzz_target!(|case: FuzzCase| {
    let environment = if case.sandbox {
        Environment::with_django_mode(DjangoMode::default())
    } else {
        Environment::new()
    };

    let _ = environment.render_str(&case.template, &case.context);
});
