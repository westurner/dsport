//! Proves the `insta` snapshot loop is wired. Real parity snapshots land in phase 1.

#[test]
fn version_snapshot() {
    insta::assert_snapshot!("version", docutilsrs::version());
}
