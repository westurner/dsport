//! Tests for `sphinxdocrs::assets`.
//!
//! The network-touching `fetch_and_cache` test uses a `file://` URL
//! pointing at a fixture written into the system temp directory so it
//! works offline.

use std::io::Write;

use sphinxdocrs::assets::{
    DEFAULT_SRI_ALGO, SriAlgo, cache_path_for, fetch_and_cache, sri_hash, sri_hash_file,
};

fn tmp(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("sphinxdocrs-assets-tests");
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

#[test]
fn default_algo_is_sha384() {
    assert_eq!(DEFAULT_SRI_ALGO, SriAlgo::Sha384);
    assert_eq!(DEFAULT_SRI_ALGO.name(), "sha384");
}

#[test]
fn sri_known_vectors() {
    // Reference SHA-256 of the empty string, base64-encoded, is
    // `47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=` (NIST FIPS-180-4
    // vector + RFC 4648 §4). The SRI 2 spec specifies standard base64
    // (not base64url) and `<algo>-<b64>` formatting.
    assert_eq!(
        sri_hash(b"", SriAlgo::Sha256).unwrap(),
        "sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU="
    );
    // SHA-384 of `"abc"` from FIPS-180-4 appendix.
    assert_eq!(
        sri_hash(b"abc", SriAlgo::Sha384).unwrap(),
        "sha384-ywB1P0WjXou1oD1pmsZQBycsMqsO3tFjGotgWkP/W+2AhgcroefMI1i67KE0yCWn"
    );
}

#[test]
fn sri_hash_file_roundtrips() {
    let path = tmp("hello.js");
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(b"console.log('hi');").unwrap();
    let from_file = sri_hash_file(&path, SriAlgo::Sha384).unwrap();
    let from_bytes = sri_hash(b"console.log('hi');", SriAlgo::Sha384).unwrap();
    assert_eq!(from_file, from_bytes);
    assert!(from_file.starts_with("sha384-"));
}

#[test]
fn cache_path_is_stable_and_distinguishes_urls() {
    let dir = std::path::Path::new("/tmp/cache");
    let a = cache_path_for(dir, "https://cdn.example.com/lib.js").unwrap();
    let b = cache_path_for(dir, "https://cdn.example.com/lib.js").unwrap();
    let c = cache_path_for(dir, "https://other.example.com/lib.js").unwrap();
    assert_eq!(a, b, "same URL must hash to same cache path");
    assert_ne!(a, c, "different URLs must produce different cache dirs");
    assert!(a.starts_with(dir));
    assert_eq!(a.file_name().unwrap(), "lib.js");
}

#[test]
fn cache_path_strips_query_and_fragment() {
    let dir = std::path::Path::new("/tmp/cache");
    let p = cache_path_for(dir, "https://cdn.example.com/lib.js?v=1#x").unwrap();
    assert_eq!(p.file_name().unwrap(), "lib.js");
}

#[test]
fn fetch_and_cache_reads_file_url_offline() {
    // Write a fixture and serve it back via the file:// URL scheme so
    // the test stays offline. Python's `urllib.request.urlopen`
    // supports `file://` natively.
    let fixture = tmp("payload.js");
    std::fs::write(&fixture, b"// payload\nconsole.log(1);\n").unwrap();
    let url = format!("file://{}", fixture.display());

    let cache_dir = tmp("cache");
    let _ = std::fs::remove_dir_all(&cache_dir);

    let cached = fetch_and_cache(&url, &cache_dir).unwrap();
    assert!(cached.exists(), "cached file should exist");
    assert_eq!(
        std::fs::read(&cached).unwrap(),
        b"// payload\nconsole.log(1);\n"
    );

    // Second call must be a no-op (still returns the same path) — we
    // verify by deleting the source after the first fetch.
    std::fs::remove_file(&fixture).unwrap();
    let cached2 = fetch_and_cache(&url, &cache_dir).unwrap();
    assert_eq!(cached, cached2);

    let integrity = sri_hash_file(&cached, SriAlgo::Sha384).unwrap();
    assert!(integrity.starts_with("sha384-"));
}
