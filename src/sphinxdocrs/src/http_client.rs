//! `sphinxdocrs::http_client` — shared HTTP backend for
//! [`crate::builders::linkcheck`] and [`crate::intersphinx`].
//!
//! Two backends produce the same [`Response`] shape:
//!
//! - **`curl`** (default): shells out to the system `curl` binary. No new
//!   dependencies; uses the same hardened invocation pattern already used
//!   throughout this crate (restricted `--proto`/`--proto-redir`, bounded
//!   redirects/time/body size, URI passed after `--`).
//! - **`reqwest`** (opt-in, `http-reqwest` Cargo feature): an in-process
//!   blocking HTTP client. Selected at runtime via the
//!   `SPHINXDOCRS_HTTP_CLIENT=reqwest` environment variable; any other value
//!   (or the feature not being compiled in) falls back to `curl`.
//!
//! Both backends: restrict requests to `http`/`https`, follow up to 10
//! redirect hops, bound the response body, and report the final URL reached
//! (so callers can detect and classify redirects) plus a parsed
//! `Retry-After` header (for linkcheck's rate-limit backoff).
//!
//! ## Why an env var instead of a CLI flag
//!
//! `sphinx-build-rs` has no natural single place to thread a new flag
//! through to both `linkcheck` and `intersphinx` (they're invoked from
//! different points in the build). An environment variable avoids that
//! plumbing while remaining easy to set from a wrapper script or CI config:
//! `SPHINXDOCRS_HTTP_CLIENT=reqwest sphinx-build-rs -b linkcheck ...`.

use std::path::Path;
use std::process::Command;

#[cfg(feature = "http-reqwest")]
use std::io::Read;

/// HTTP method for a [`request`] call.
///
/// `Head` avoids downloading a body (used for plain link checks); `Get` is
/// required when the body must be inspected (anchor checking, inventory
/// downloads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Head,
    Get,
}

/// Outcome of an HTTP request, normalised across both backends.
#[derive(Debug, Clone)]
pub struct Response {
    /// Final HTTP status code, after following redirects.
    pub status: u16,
    /// The URL ultimately reached, after following any redirects.
    pub final_url: String,
    /// Response body — populated only for [`Method::Get`] requests.
    pub body: Option<Vec<u8>>,
    /// The `Retry-After` response header, in seconds, if present and numeric.
    pub retry_after_secs: Option<u64>,
}

/// A failed request: network error, timeout, or disallowed scheme.
#[derive(Debug, Clone)]
pub struct RequestError(pub String);

impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RequestError {}

/// Which backend [`request`] / [`download_to_file`] will use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Curl,
    Reqwest,
}

/// Resolve the active backend from `SPHINXDOCRS_HTTP_CLIENT`.
///
/// Falls back to [`Backend::Curl`] when the variable is unset, not
/// `"reqwest"` (case-insensitive), or the `http-reqwest` feature was not
/// compiled in.
pub fn backend() -> Backend {
    #[cfg(feature = "http-reqwest")]
    {
        if std::env::var("SPHINXDOCRS_HTTP_CLIENT")
            .map(|v| v.eq_ignore_ascii_case("reqwest"))
            .unwrap_or(false)
        {
            return Backend::Reqwest;
        }
    }
    Backend::Curl
}

/// Return `true` only for `http://` / `https://` URIs (case-insensitive).
pub fn is_http_url(uri: &str) -> bool {
    let lower = uri.to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

/// Default cap on response bodies read into memory (anchor checking).
pub const DEFAULT_MAX_BODY_BYTES: u64 = 10 * 1024 * 1024;

/// Perform an HTTP request, following redirects, via the active backend.
///
/// Rejects non-`http(s)` URIs before doing anything else, regardless of
/// backend.
pub fn request(uri: &str, method: Method, timeout_secs: u32) -> Result<Response, RequestError> {
    request_with_limit(uri, method, timeout_secs, DEFAULT_MAX_BODY_BYTES)
}

/// Like [`request`], with an explicit response-body size cap.
pub fn request_with_limit(
    uri: &str,
    method: Method,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<Response, RequestError> {
    if !is_http_url(uri) {
        return Err(RequestError(format!("unsupported scheme in {uri:?}")));
    }
    match backend() {
        Backend::Curl => curl_request(uri, method, timeout_secs, max_bytes),
        Backend::Reqwest => reqwest_dispatch(uri, method, timeout_secs, max_bytes),
    }
}

/// Download `uri` to `dest`, following redirects, via the active backend.
///
/// Used by [`crate::intersphinx`] to fetch `objects.inv` payloads. Returns an
/// error (and does not create `dest`) on any failure, including an HTTP
/// status of 400 or above.
pub fn download_to_file(
    uri: &str,
    dest: &Path,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<(), RequestError> {
    if !is_http_url(uri) {
        return Err(RequestError(format!("unsupported scheme in {uri:?}")));
    }
    match backend() {
        Backend::Curl => curl_download(uri, dest, timeout_secs, max_bytes),
        Backend::Reqwest => reqwest_download_dispatch(uri, dest, timeout_secs, max_bytes),
    }
}

#[cfg(feature = "http-reqwest")]
fn reqwest_dispatch(
    uri: &str,
    method: Method,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<Response, RequestError> {
    reqwest_request(uri, method, timeout_secs, max_bytes)
}

#[cfg(not(feature = "http-reqwest"))]
fn reqwest_dispatch(
    uri: &str,
    method: Method,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<Response, RequestError> {
    // `backend()` never returns `Reqwest` when the feature is off.
    let _ = (uri, method, timeout_secs, max_bytes);
    unreachable!("Backend::Reqwest selected without the http-reqwest feature")
}

#[cfg(feature = "http-reqwest")]
fn reqwest_download_dispatch(
    uri: &str,
    dest: &Path,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<(), RequestError> {
    reqwest_download(uri, dest, timeout_secs, max_bytes)
}

#[cfg(not(feature = "http-reqwest"))]
fn reqwest_download_dispatch(
    uri: &str,
    dest: &Path,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<(), RequestError> {
    let _ = (uri, dest, timeout_secs, max_bytes);
    unreachable!("Backend::Reqwest selected without the http-reqwest feature")
}

// ── curl backend ────────────────────────────────────────────────────────────

const CODE_MARKER: &str = "__SPHINXDOCRS_CODE__:";
const URL_MARKER: &str = "__SPHINXDOCRS_URL__:";

/// Perform one request via `curl`, capturing the final status code, final
/// URL, and (for `Method::Get`) the response body, in a single invocation.
fn curl_request(
    uri: &str,
    method: Method,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<Response, RequestError> {
    let body_path = (method == Method::Get).then(|| {
        std::env::temp_dir().join(format!("sphinxdocrs-http-{}.body", uuid::Uuid::new_v4()))
    });
    let body_arg = body_path
        .as_ref()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/dev/null".to_string());

    let cleanup = || {
        if let Some(p) = &body_path {
            let _ = std::fs::remove_file(p);
        }
    };

    let mut args: Vec<String> = vec![
        "--silent".into(),
        "--show-error".into(),
        "--dump-header".into(),
        "-".into(),
        "-o".into(),
        body_arg,
        "--location".into(),
        "--proto".into(),
        "=https,http".into(),
        "--proto-redir".into(),
        "=https,http".into(),
        "--max-redirs".into(),
        "10".into(),
        "--max-time".into(),
        timeout_secs.to_string(),
        "--max-filesize".into(),
        max_bytes.to_string(),
        "-w".into(),
        format!("\n{CODE_MARKER}%{{http_code}}\n{URL_MARKER}%{{url_effective}}\n"),
    ];
    if method == Method::Head {
        args.push("--head".into());
    }
    args.push("--".into());
    args.push(uri.to_string());

    let output = Command::new("curl").args(&args).output();
    let out = match output {
        Ok(o) => o,
        Err(e) => {
            cleanup();
            return Err(RequestError(format!("failed to run curl: {e}")));
        }
    };

    let stdout = String::from_utf8_lossy(&out.stdout);
    let mut status: u16 = 0;
    let mut final_url = uri.to_string();
    let mut retry_after_secs = None;
    for line in stdout.lines() {
        if let Some(v) = line.strip_prefix(CODE_MARKER) {
            status = v.trim().parse().unwrap_or(0);
        } else if let Some(v) = line.strip_prefix(URL_MARKER) {
            final_url = v.trim().to_string();
        } else {
            let lower = line.to_ascii_lowercase();
            if let Some(v) = lower.strip_prefix("retry-after:") {
                retry_after_secs = v.trim().parse().ok();
            }
        }
    }

    let body = if method == Method::Get {
        let path = body_path.as_ref().expect("body_path set for Method::Get");
        let data = std::fs::read(path).ok();
        cleanup();
        data
    } else {
        None
    };

    if status == 0 {
        return Err(RequestError(if out.status.success() {
            "curl produced no status code".into()
        } else {
            format!(
                "curl exited with {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            )
        }));
    }

    Ok(Response {
        status,
        final_url,
        body,
        retry_after_secs,
    })
}

/// Download `uri` directly to `dest` via `curl`. Fails (and leaves `dest`
/// untouched) on any non-2xx/3xx status.
fn curl_download(
    uri: &str,
    dest: &Path,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<(), RequestError> {
    let dest_str = dest
        .to_str()
        .ok_or_else(|| RequestError("destination path is not valid UTF-8".into()))?;
    let status = Command::new("curl")
        .args([
            "--silent",
            "--fail",
            "--location",
            "--proto",
            "=https,http",
            "--proto-redir",
            "=https,http",
            "--max-redirs",
            "10",
            "--max-time",
            &timeout_secs.to_string(),
            "--max-filesize",
            &max_bytes.to_string(),
            "-o",
            dest_str,
            "--",
            uri,
        ])
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(RequestError(format!(
            "curl exited {}",
            s.code().unwrap_or(-1)
        ))),
        Err(e) => Err(RequestError(format!("failed to run curl: {e}"))),
    }
}

// ── reqwest backend ─────────────────────────────────────────────────────────

#[cfg(feature = "http-reqwest")]
fn reqwest_request(
    uri: &str,
    method: Method,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<Response, RequestError> {
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs as u64))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| RequestError(format!("failed to build reqwest client: {e}")))?;

    let builder = match method {
        Method::Head => client.head(uri),
        Method::Get => client.get(uri),
    };
    let resp = builder
        .send()
        .map_err(|e| RequestError(format!("request failed: {e}")))?;

    let final_url = resp.url().to_string();
    let status = resp.status().as_u16();
    let retry_after_secs = resp
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.trim().parse().ok());

    let body = if method == Method::Get {
        let mut buf = Vec::new();
        resp.take(max_bytes)
            .read_to_end(&mut buf)
            .map_err(|e| RequestError(format!("failed to read response body: {e}")))?;
        Some(buf)
    } else {
        None
    };

    Ok(Response {
        status,
        final_url,
        body,
        retry_after_secs,
    })
}

#[cfg(feature = "http-reqwest")]
fn reqwest_download(
    uri: &str,
    dest: &Path,
    timeout_secs: u32,
    max_bytes: u64,
) -> Result<(), RequestError> {
    let resp = reqwest_request(uri, Method::Get, timeout_secs, max_bytes)?;
    if resp.status >= 400 {
        return Err(RequestError(format!("HTTP status {}", resp.status)));
    }
    let body = resp.body.unwrap_or_default();
    std::fs::write(dest, &body)
        .map_err(|e| RequestError(format!("failed to write {}: {e}", dest.display())))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `SPHINXDOCRS_HTTP_CLIENT` is process-global state; serialize the tests
    /// that touch it so they don't race under the default parallel test
    /// harness (same pattern as `locale.rs` / `roles.rs`).
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn is_http_url_only_accepts_http() {
        assert!(is_http_url("http://example.com"));
        assert!(is_http_url("HTTPS://EXAMPLE.COM"));
        assert!(!is_http_url("mailto:a@b.com"));
        assert!(!is_http_url("ftp://x/"));
    }

    #[test]
    fn backend_defaults_to_curl() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: guarded by ENV_LOCK above; no other test in this process
        // reads or writes SPHINXDOCRS_HTTP_CLIENT concurrently.
        unsafe {
            std::env::remove_var("SPHINXDOCRS_HTTP_CLIENT");
        }
        assert_eq!(backend(), Backend::Curl);
    }

    #[cfg(feature = "http-reqwest")]
    #[test]
    fn backend_switches_to_reqwest_via_env() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: guarded by ENV_LOCK above.
        unsafe {
            std::env::set_var("SPHINXDOCRS_HTTP_CLIENT", "reqwest");
        }
        assert_eq!(backend(), Backend::Reqwest);
        unsafe {
            std::env::set_var("SPHINXDOCRS_HTTP_CLIENT", "curl");
        }
        assert_eq!(backend(), Backend::Curl);
        unsafe {
            std::env::remove_var("SPHINXDOCRS_HTTP_CLIENT");
        }
    }

    #[test]
    fn request_rejects_non_http_schemes() {
        let err = request("mailto:a@b.com", Method::Head, 5).unwrap_err();
        assert!(err.0.contains("unsupported scheme"));
    }

    #[test]
    fn download_rejects_non_http_schemes() {
        let dest = std::env::temp_dir().join("sphinxdocrs-http-client-test-reject");
        let err = download_to_file("ftp://example.com/x", &dest, 5, 1024).unwrap_err();
        assert!(err.0.contains("unsupported scheme"));
        assert!(!dest.exists());
    }
}
