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

#[cfg(test)]
pub(crate) static HTTP_ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread::JoinHandle;

    /// `SPHINXDOCRS_HTTP_CLIENT` is process-global state; serialize the tests
    /// that touch it so they don't race under the default parallel test
    /// harness (same pattern as `locale.rs` / `roles.rs`).
    use super::HTTP_ENV_LOCK as ENV_LOCK;

    fn local_http_server(response: &str) -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let response = response.as_bytes().to_vec();
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 4096];
            let _ = stream.read(&mut request);
            stream.write_all(&response).unwrap();
        });
        (format!("http://{address}/payload"), handle)
    }

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

    #[test]
    fn curl_request_and_download_cover_success_and_failure_paths() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let (head_url, head_server) = local_http_server(
            "HTTP/1.1 204 No Content\r\nRetry-After: invalid\r\nContent-Length: 0\r\n\r\n",
        );
        let head = request(&head_url, Method::Head, 5).unwrap();
        assert_eq!(head.status, 204);
        assert_eq!(head.body, None);
        assert_eq!(head.retry_after_secs, None);
        head_server.join().unwrap();

        let (get_url, get_server) = local_http_server(
            "HTTP/1.1 200 OK\r\nRetry-After: 7\r\nContent-Length: 5\r\n\r\nhello",
        );
        let get = request(&get_url, Method::Get, 5).unwrap();
        assert_eq!(get.status, 200);
        assert_eq!(get.final_url, get_url);
        assert_eq!(get.body, Some(b"hello".to_vec()));
        assert_eq!(get.retry_after_secs, Some(7));
        get_server.join().unwrap();

        let (download_url, download_server) =
            local_http_server("HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\ndownloaded");
        let tmp = tempfile::tempdir().unwrap();
        let destination = tmp.path().join("payload.bin");
        download_to_file(&download_url, &destination, 5, 1024).unwrap();
        assert_eq!(std::fs::read(&destination).unwrap(), b"downloaded");
        download_server.join().unwrap();

        let (failure_url, failure_server) =
            local_http_server("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        let failure =
            download_to_file(&failure_url, &tmp.path().join("missing.bin"), 5, 1024).unwrap_err();
        assert!(failure.0.contains("curl exited"));
        failure_server.join().unwrap();

        let curl_error = request("http://", Method::Head, 1).unwrap_err();
        assert!(curl_error.0.contains("curl exited"));
    }

    #[cfg(unix)]
    #[test]
    fn curl_download_rejects_non_utf8_destination() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        use std::os::unix::ffi::OsStringExt;

        let invalid = std::path::PathBuf::from(std::ffi::OsString::from_vec(vec![0xff]));
        let error = download_to_file("http://127.0.0.1/unused", &invalid, 1, 1).unwrap_err();
        assert!(error.0.contains("not valid UTF-8"));
    }

    #[cfg(unix)]
    #[test]
    fn curl_handles_missing_status_and_missing_executable() {
        use std::os::unix::fs::PermissionsExt;

        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let fake_bin = tempfile::tempdir().unwrap();
        let fake_curl = fake_bin.path().join("curl");
        std::fs::write(&fake_curl, "#!/bin/sh\nexit 0\n").unwrap();
        std::fs::set_permissions(&fake_curl, std::fs::Permissions::from_mode(0o755)).unwrap();

        let previous_path = std::env::var_os("PATH");
        // SAFETY: HTTP_ENV_LOCK serializes every test that invokes curl.
        unsafe {
            std::env::set_var("PATH", fake_bin.path());
        }
        let no_status = request("http://example.com", Method::Head, 1).unwrap_err();
        assert!(no_status.0.contains("curl produced no status code"));

        let missing_bin = tempfile::tempdir().unwrap();
        // SAFETY: HTTP_ENV_LOCK serializes every test that invokes curl.
        unsafe {
            std::env::set_var("PATH", missing_bin.path());
        }
        let missing_request = request("http://example.com", Method::Head, 1).unwrap_err();
        assert!(missing_request.0.contains("failed to run curl"));
        let missing_download = download_to_file(
            "http://example.com",
            &fake_bin.path().join("unused.bin"),
            1,
            1,
        )
        .unwrap_err();
        assert!(missing_download.0.contains("failed to run curl"));

        // SAFETY: HTTP_ENV_LOCK serializes every test that invokes curl.
        unsafe {
            match previous_path {
                Some(path) => std::env::set_var("PATH", path),
                None => std::env::remove_var("PATH"),
            }
        }
    }

    #[cfg(feature = "http-reqwest")]
    #[test]
    fn reqwest_request_and_download_cover_dispatch() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // SAFETY: guarded by ENV_LOCK above.
        unsafe {
            std::env::set_var("SPHINXDOCRS_HTTP_CLIENT", "reqwest");
        }

        let (head_url, head_server) = local_http_server(
            "HTTP/1.1 204 No Content\r\nRetry-After: 3\r\nContent-Length: 0\r\n\r\n",
        );
        let head = request(&head_url, Method::Head, 5).unwrap();
        assert_eq!(head.status, 204);
        assert_eq!(head.body, None);
        assert_eq!(head.retry_after_secs, Some(3));
        head_server.join().unwrap();

        let (get_url, get_server) =
            local_http_server("HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello");
        let get = request_with_limit(&get_url, Method::Get, 5, 3).unwrap();
        assert_eq!(get.body, Some(b"hel".to_vec()));
        get_server.join().unwrap();

        let tmp = tempfile::tempdir().unwrap();
        let (download_url, download_server) =
            local_http_server("HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\ndownloaded");
        let destination = tmp.path().join("payload.bin");
        download_to_file(&download_url, &destination, 5, 1024).unwrap();
        assert_eq!(std::fs::read(&destination).unwrap(), b"downloaded");
        download_server.join().unwrap();

        let (failure_url, failure_server) =
            local_http_server("HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        let failure =
            download_to_file(&failure_url, &tmp.path().join("missing.bin"), 5, 1024).unwrap_err();
        assert!(failure.0.contains("HTTP status 404"));
        failure_server.join().unwrap();

        let request_error = request("http://127.0.0.1:1/unused", Method::Get, 1).unwrap_err();
        assert!(request_error.0.contains("request failed"));

        // SAFETY: guarded by ENV_LOCK above.
        unsafe {
            std::env::remove_var("SPHINXDOCRS_HTTP_CLIENT");
        }
    }
}
