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
    let src = "let $a + b = c$ then\n";
    insta::assert_snapshot!("inline_math", myst_md_rs::render_html(src));
}

#[test]
fn block_math() {
    let src = "$$\nE = mc^2\n$$\n";
    insta::assert_snapshot!("block_math", myst_md_rs::render_html(src));
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
