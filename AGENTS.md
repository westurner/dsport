# AGENTS

Purpose: define how coding agents and contributors should navigate this repository and execute work safely.

## Repository map (from tree)

.
|- README.md
|- AGENTS.md
`- src/
   |- docutils/
   |  |- docutils/
   |  |- sandbox/
   |  `- web/
   |- docutilsrs/   (target Rust port, currently empty)
   |- sphinx/
   |  |- sphinx/
   |  |- tests/
   |  |- doc/
   |  `- utils/
   `- sphinxdocrs/  (target Rust port, currently empty)

## Mission

- Port Python docutils into `src/docutilsrs`.
- Port Python sphinx into `src/sphinxdocrs`.
- Keep behavior compatible with upstream Python projects.
- Preserve plugin interoperability across Python and Rust implementations.

## Working rules

- Use test-first or test-near development for all ports.
- Start with parser and core data model parity before feature expansion.
- Prefer small, reviewable commits with explicit compatibility notes.
- Do not modify upstream vendored sources under `src/docutils/` or `src/sphinx/` unless the task explicitly requires it.
- Keep Rust code idiomatic, but match Python-visible behavior where compatibility matters.

## Priority order

1. Bootstrap `docutilsrs` and `sphinxdocrs` Cargo crates.
2. Port docutils parser tests and parser implementation.
3. Port remaining docutils subsystems in dependency order.
4. Port sphinx tests/features incrementally, prioritizing extension hooks.
5. Implement metadata-based plugin mapping and fallback.

## Test strategy

- Reuse upstream tests wherever possible.
- Mark each ported test as one of: exact parity, accepted deviation, or pending.
- Keep fast local loops:
  - unit tests for parser and transforms
  - snapshot tests for structured outputs (insta)
  - focused integration tests for Python<->Rust boundaries

### Rust test tools

| Need | Tool | Notes |
|------|------|-------|
| Fixture injection | [`rstest`](https://docs.rs/rstest) | `#[fixture]` for setup; `#[once]` for session-scoped init |
| Parametrization | `rstest` `#[case]` | `#[rstest] #[case(a, b)] fn test(#[case] x: T, #[case] y: T)` |
| Trait mocking | [`mockall`](https://docs.rs/mockall) | `#[automock]` on trait; `mock.expect_*()` DSL |
| HTTP server mock | [`wiremock`](https://docs.rs/wiremock) | Spin up a local server; assert request patterns |
| HTTP record/replay | [`rvcr`](https://docs.rs/rvcr) | Records to a JSON cassette on first run, replays in CI — equivalent to `requests-cache` / `vcrpy` |
| Snapshot testing | [`insta`](https://docs.rs/insta) | Already in use; `insta::assert_snapshot!` |

### Fixture and parametrization patterns

```rust
// Shared fixture — built once per test binary (session scope)
#[fixture]
#[once]
fn coverage_tree() -> Doctree { common::build_coverage_tree(&common::coverage_rst("html")) }

// Function-scoped fixture (default)
#[fixture]
fn default_options() -> Html5Options { Html5Options::default() }

// Parametrized test
#[rstest]
#[case("html",  "<!DOCTYPE")]
#[case("latex", r"\documentclass")]
fn test_writer_output(#[case] fmt: &str, #[case] prefix: &str) {
    assert!(render(fmt).contains(prefix));
}
```

### Mocking pattern

Define behavior under test behind a trait, then use `#[automock]` in tests:

```rust
#[cfg_attr(test, automock)]
trait Renderer { fn render(&self, src: &str) -> String; }
```

### HTTP record/replay pattern (rvcr)

```rust
// First run: records to tests/cassettes/my_test.json
// Subsequent runs: replays from cassette, no network needed
let client = reqwest::Client::builder()
    .with(VcrMiddleware::new("tests/cassettes/my_test.json", VcrMode::Record))
    .build();
```

Commit cassette files. Set `VcrMode::Replay` in CI or use the `rvcr` auto-detect feature.

## Python/Rust interop requirements

- Python must be able to import and call `docutilsrs` and `sphinxdocrs` modules.
- Rust must be able to load and execute compatible Python plugins.
- Plugin resolution should prefer Rust equivalent when declared; otherwise fall back to Python implementation.

## Suggested workflow for agents

1. Identify target behavior and locate corresponding upstream tests.
2. Port tests into Rust crate test modules.
3. Implement minimal code to pass tests.
4. Add interop checks when Python boundary is touched.
5. Update compatibility notes in README or milestone docs.

## Out of scope by default

- Large-scale refactors of vendored Python source trees.
- Formatting-only sweeps unrelated to active work.
- Feature additions that do not improve parity or interop.
