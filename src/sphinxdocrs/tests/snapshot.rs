//! Proves the `insta` snapshot loop is wired. Real parity snapshots land in phase 4.

#[test]
fn version_snapshot() {
    insta::assert_snapshot!("version", sphinxdocrs::version());
}
