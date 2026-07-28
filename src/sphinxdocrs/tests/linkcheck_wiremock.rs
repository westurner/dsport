//! `wiremock`-backed integration tests for
//! `sphinxdocrs::builders::linkcheck::check_uri`, covering the H1b gate:
//! one case per [`LinkStatus`] class, plus rate-limit backoff.
//!
//! Each test starts a real local HTTP server (`wiremock::MockServer`) and
//! calls the crate's synchronous `check_uri` against it — this exercises the
//! actual HTTP request/response path end-to-end (curl by default, or
//! in-process `reqwest` under `--features http-reqwest`), not a mock of the
//! HTTP client itself.

use sphinxdocrs::builders::linkcheck::{CheckResult, LinkStatus, LinkcheckConfig, check_uri};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Run `check_uri` on a plain OS thread, outside the `#[tokio::test]`
/// runtime context. Required because `reqwest::blocking` (used when the
/// `http-reqwest` feature selects the reqwest backend) refuses to build its
/// internal runtime while already inside an active tokio context.
fn check_uri_sync(docname: &'static str, uri: &str, cfg: &LinkcheckConfig) -> CheckResult {
    let uri = uri.to_string();
    let cfg = cfg.clone();
    std::thread::spawn(move || check_uri(docname, &uri, &cfg))
        .join()
        .expect("check_uri thread panicked")
}

#[tokio::test]
async fn working_link_reports_working() {
    let server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path("/ok"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let uri = format!("{}/ok", server.uri());
    let result = check_uri_sync("index", &uri, &LinkcheckConfig::default());

    assert_eq!(result.status, LinkStatus::Working, "{result:?}");
    assert_eq!(result.code, 200);
}

#[tokio::test]
async fn missing_link_reports_broken() {
    let server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path("/missing"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    let uri = format!("{}/missing", server.uri());
    let result = check_uri_sync("index", &uri, &LinkcheckConfig::default());

    assert_eq!(result.status, LinkStatus::Broken, "{result:?}");
    assert_eq!(result.code, 404);
}

#[tokio::test]
async fn redirect_to_unlisted_target_reports_redirected() {
    let server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path("/old"))
        .respond_with(ResponseTemplate::new(301).insert_header("Location", "/new"))
        .mount(&server)
        .await;
    Mock::given(method("HEAD"))
        .and(path("/new"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let uri = format!("{}/old", server.uri());
    let result = check_uri_sync("index", &uri, &LinkcheckConfig::default());

    assert_eq!(result.status, LinkStatus::Redirected, "{result:?}");
    assert!(result.info.ends_with("/new"), "{result:?}");
}

#[tokio::test]
async fn redirect_matching_allowed_redirects_reports_working() {
    let server = MockServer::start().await;
    Mock::given(method("HEAD"))
        .and(path("/old"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", "/new"))
        .mount(&server)
        .await;
    Mock::given(method("HEAD"))
        .and(path("/new"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    // Allow any redirect on this server (from `.*` to `.*`).
    let config = LinkcheckConfig::default().with_allowed_redirects(&[(".*", ".*")]);

    let uri = format!("{}/old", server.uri());
    let result = check_uri_sync("index", &uri, &config);

    assert_eq!(result.status, LinkStatus::Working, "{result:?}");
}

#[tokio::test]
async fn non_http_scheme_reports_ignored() {
    let result = check_uri_sync("index", "mailto:a@b.com", &LinkcheckConfig::default());
    assert_eq!(result.status, LinkStatus::Ignored, "{result:?}");
}

#[tokio::test]
async fn linkcheck_ignore_pattern_reports_ignored() {
    let server = MockServer::start().await;
    // No mock mounted for /skip-me — if the checker ever calls out, wiremock
    // would return 404 rather than the local ignore short-circuit, so
    // asserting `Ignored` here also proves no network call was made.
    let uri = format!("{}/skip-me", server.uri());

    let config = LinkcheckConfig::default().with_ignore(&["skip-me"]);

    let result = check_uri_sync("index", &uri, &config);
    assert_eq!(result.status, LinkStatus::Ignored, "{result:?}");
}

#[tokio::test]
async fn anchor_found_reports_working() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/page"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><h1 id=\"section-a\">A</h1></body></html>"),
        )
        .mount(&server)
        .await;

    let uri = format!("{}/page#section-a", server.uri());
    let result = check_uri_sync("index", &uri, &LinkcheckConfig::default());

    assert_eq!(result.status, LinkStatus::Working, "{result:?}");
}

#[tokio::test]
async fn anchor_missing_reports_broken() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/page"))
        .respond_with(
            ResponseTemplate::new(200).set_body_string("<html><body>no anchors here</body></html>"),
        )
        .mount(&server)
        .await;

    let uri = format!("{}/page#does-not-exist", server.uri());
    let result = check_uri_sync("index", &uri, &LinkcheckConfig::default());

    assert_eq!(result.status, LinkStatus::Broken, "{result:?}");
    assert!(result.info.contains("does-not-exist"), "{result:?}");
}

#[tokio::test]
async fn rate_limited_then_success_reports_working() {
    let server = MockServer::start().await;
    // First request: 429 with a short Retry-After. Second: 200.
    Mock::given(method("HEAD"))
        .and(path("/limited"))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "1"))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("HEAD"))
        .and(path("/limited"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let uri = format!("{}/limited", server.uri());
    let mut config = LinkcheckConfig::default();
    config.retries = 2;
    let result = check_uri_sync("index", &uri, &config);

    assert_eq!(result.status, LinkStatus::Working, "{result:?}");
}
