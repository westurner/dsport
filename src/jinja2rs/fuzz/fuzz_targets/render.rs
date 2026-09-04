//! Fuzz regular Jinja2rs rendering and Django compatibility mode.
//!
//! The target receives a structured [`support::FuzzCase`]. Some cases are
//! passed through as raw template source to exercise parser and error paths;
//! the remaining cases are generated as valid expressions, control flow, and
//! built-in filter uses so successful rendering receives meaningful coverage.
//!
//! Rendering errors are expected for malformed templates or incompatible
//! contexts and are intentionally ignored. A panic, abort, sanitizer finding,
//! or excessive input runtime is a fuzz failure.
//!
//! Run this target from the independent fuzz workspace with:
//!
//! ```text
//! cargo +nightly fuzz run --no-default-features render -- -max_total_time=60
//! ```

#![no_main]

mod support;

use jinja2rs::{DjangoMode, Environment};
use libfuzzer_sys::fuzz_target;

use support::{render_template, FuzzCase};

fuzz_target!(|case: FuzzCase| {
    let environment = if case.django {
        Environment::with_django_mode(DjangoMode::default())
    } else {
        Environment::new()
    };

    let template = render_template(&case);
    let _ = environment.render_str(&template, &case.context);
});
