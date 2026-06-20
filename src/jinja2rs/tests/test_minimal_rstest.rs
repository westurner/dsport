#![allow(clippy::needless_borrows_for_generic_args)]


use jinja2rs::Environment;
use rstest::{fixture, rstest};
use serde_json::json;

#[fixture]
fn env() -> Environment {
    Environment::new()
}

#[rstest]
#[case("hello")]
fn test_simple_fixture(env: Environment, #[case] text: &str) {
    let out = env
        .render_str("{{ text }}", json!({"text": text}))
        .expect("render should work");
    assert_eq!(out, text);
}

#[rstest]
#[case("a")]
#[case("b")]
fn test_simple_params(#[case] text: &str) {
    assert!(!text.is_empty());
}
