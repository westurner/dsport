//! Phase 0 — prove the `insta` snapshot loop works on `pygmentsrs`.
//! Phase 1 adds real lexer-output snapshots.

#[test]
fn version_snapshot() {
    insta::assert_snapshot!("version", pygmentsrs::version());
}

#[test]
fn text_lexer_passthrough_snapshot() {
    let toks = pygmentsrs::lex("text", "hello\nworld\n").expect("text lexer registered");
    insta::assert_debug_snapshot!("text_passthrough", toks);
}

#[test]
fn html_formatter_text_snapshot() {
    let html =
        pygmentsrs::highlight("hello & <world>\n", "text", "html").expect("text→html supported");
    insta::assert_snapshot!("html_text", html);
}
