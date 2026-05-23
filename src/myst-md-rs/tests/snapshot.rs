//! Phase 0 snapshot tests for `myst-md-rs`.

#[test]
fn version_snapshot() {
    insta::assert_snapshot!("version", myst_md_rs::version());
}

#[test]
fn features_snapshot() {
    insta::assert_debug_snapshot!("features", myst_md_rs::features());
}

#[test]
fn commonmark_basic() {
    let src = "# hi\n\nhello *world*\n";
    insta::assert_snapshot!("commonmark_basic", myst_md_rs::render_html(src));
}

#[test]
fn front_matter_extracted() {
    let src = "---\ntitle: hi\ntags: [a, b]\n---\n\n# body\n";
    let r = myst_md_rs::parse_to_html(src);
    insta::assert_snapshot!("front_matter_html", r.html);
    insta::assert_snapshot!(
        "front_matter_yaml",
        r.front_matter_yaml.expect("yaml should parse")
    );
}

#[test]
fn inline_role() {
    let src = "see {ref}`my-label` for details\n";
    insta::assert_snapshot!("inline_role", myst_md_rs::render_html(src));
}

#[test]
fn inline_math() {
    // Pick the MathJax backend for snapshot stability — the default
    // RaTeX backend emits a multi-kB inline SVG that is exercised in
    // `inline_math_default_is_ratex` below.
    let src = "let $a + b = c$ then\n";
    insta::assert_snapshot!(
        "inline_math",
        myst_md_rs::render_html_with(src, myst_md_rs::MathBackend::MathJax)
    );
}

#[test]
fn block_math() {
    let src = "$$\nE = mc^2\n$$\n";
    insta::assert_snapshot!(
        "block_math",
        myst_md_rs::render_html_with(src, myst_md_rs::MathBackend::MathJax)
    );
}

#[test]
fn inline_math_default_is_ratex() {
    let src = "let $a + b = c$ then\n";
    let out = myst_md_rs::render_html(src);
    assert!(
        out.contains("<svg") && out.contains(r#"data-renderer="ratex""#),
        "expected default backend to be RaTeX (SVG output); got:\n{out}"
    );
}

#[test]
fn block_math_default_is_ratex() {
    let src = "$$\nE = mc^2\n$$\n";
    let out = myst_md_rs::render_html(src);
    assert!(
        out.contains("<svg") && out.contains(r#"data-renderer="ratex""#),
        "expected default backend to be RaTeX (SVG output); got:\n{out}"
    );
}

#[test]
fn block_math_imgmath_backend_emits_data_url() {
    let src = "$$\nE = mc^2\n$$\n";
    let out = myst_md_rs::render_html_with(src, myst_md_rs::MathBackend::ImgMath);
    assert!(
        out.contains("data:image/svg+xml;base64,")
            && out.contains(r#"data-renderer="imgmath""#),
        "expected imgmath backend to emit a data: URL; got:\n{out}"
    );
}

#[test]
fn colon_fence_directive() {
    let src = ":::note\nHeads up: this is **important**.\n:::\n";
    insta::assert_snapshot!("colon_fence_directive", myst_md_rs::render_html(src));
}

#[test]
fn fenced_directive_via_backticks() {
    let src = "```{warning}\nbeware\n```\n";
    insta::assert_snapshot!("backtick_directive", myst_md_rs::render_html(src));
}

#[test]
fn gfm_table() {
    let src = "| a | b |\n|---|---|\n| 1 | 2 |\n";
    insta::assert_snapshot!("gfm_table", myst_md_rs::render_html(src));
}
