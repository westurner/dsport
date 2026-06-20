//! Integration tests for `markupsafers`.
//!
//! These tests mirror the Python MarkupSafe test suite
//! (tests/test_markupsafe.py) to ensure API and behaviour parity.

use markupsafers::{Markup, escape, escape_silent, soft_str};

// ── escape() ─────────────────────────────────────────────────────────────────

#[test]
fn escape_ampersand() {
    assert_eq!(escape("&").as_str(), "&amp;");
}
#[test]
fn escape_less_than() {
    assert_eq!(escape("<").as_str(), "&lt;");
}
#[test]
fn escape_greater_than() {
    assert_eq!(escape(">").as_str(), "&gt;");
}
#[test]
fn escape_double_quote() {
    assert_eq!(escape("\"").as_str(), "&#34;");
}
#[test]
fn escape_single_quote() {
    assert_eq!(escape("'").as_str(), "&#39;");
}

#[test]
fn escape_mixed_special_chars() {
    assert_eq!(
        escape("<script>alert('xss')</script>").as_str(),
        "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
    );
}

#[test]
fn escape_clean_string_unchanged() {
    let s = "Hello, World! 123";
    assert_eq!(escape(s).as_str(), s);
}

#[test]
fn escape_unicode_passthrough() {
    let s = "日本語テスト — emoji 😀";
    assert_eq!(escape(s).as_str(), s);
}

#[test]
fn escape_empty_string() {
    assert_eq!(escape("").as_str(), "");
}

// ── Markup::from_safe ─────────────────────────────────────────────────────────

#[test]
fn markup_from_safe_does_not_escape() {
    let m = Markup::from_safe("<b>bold</b>");
    assert_eq!(m.as_str(), "<b>bold</b>");
}

#[test]
fn markup_escape_class_method() {
    let m = Markup::escape("<b>");
    assert_eq!(m.as_str(), "&lt;b&gt;");
}

// ── Markup::unescape ──────────────────────────────────────────────────────────

#[test]
fn unescape_all_five_entities() {
    let m = Markup::from_safe("&amp; &lt; &gt; &#34; &#39;");
    assert_eq!(m.unescape(), "& < > \" '");
}

#[test]
fn unescape_alternate_quote_entities() {
    let m = Markup::from_safe("&quot; &apos;");
    assert_eq!(m.unescape(), "\" '");
}

#[test]
fn unescape_round_trip() {
    let original = "<script>alert(\"'xss'\")</script> & more";
    assert_eq!(escape(original).unescape(), original);
}

#[test]
fn unescape_clean_string() {
    let m = Markup::from_safe("no entities here");
    assert_eq!(m.unescape(), "no entities here");
}

// ── Concatenation ─────────────────────────────────────────────────────────────

#[test]
fn add_markup_markup_no_escape() {
    let a = Markup::from_safe("<a>");
    let b = Markup::from_safe("</a>");
    assert_eq!((a + b).as_str(), "<a></a>");
}

#[test]
fn add_markup_str_escapes() {
    let a = Markup::from_safe("<b>");
    let r = a + "<em>";
    assert_eq!(r.as_str(), "<b>&lt;em&gt;");
}

#[test]
fn add_markup_string_escapes() {
    let a = Markup::from_safe("<b>");
    let r = a + String::from("<em>");
    assert_eq!(r.as_str(), "<b>&lt;em&gt;");
}

#[test]
fn add_assign_markup() {
    let mut m = Markup::from_safe("<b>");
    m += Markup::from_safe("</b>");
    assert_eq!(m.as_str(), "<b></b>");
}

#[test]
fn add_assign_str_escapes() {
    let mut m = Markup::from_safe("<b>");
    m += "<em>";
    assert_eq!(m.as_str(), "<b>&lt;em&gt;");
}

// ── Join ──────────────────────────────────────────────────────────────────────

#[test]
fn join_escapes_items() {
    let sep = Markup::from_safe(", ");
    let result = sep.join(["<a>", "<b>", "<c>"]);
    assert_eq!(result.as_str(), "&lt;a&gt;, &lt;b&gt;, &lt;c&gt;");
}

#[test]
fn join_markup_items_no_escape() {
    let sep = Markup::from_safe(" | ");
    let result = sep.join_markup([Markup::from_safe("<a>"), Markup::from_safe("<b>")]);
    assert_eq!(result.as_str(), "<a> | <b>");
}

#[test]
fn join_empty_iter() {
    let sep = Markup::from_safe(", ");
    let result = sep.join(Vec::<&str>::new());
    assert_eq!(result.as_str(), "");
}

// ── format_args ───────────────────────────────────────────────────────────────

#[test]
fn format_args_positional() {
    let tmpl = Markup::from_safe("<em>{}</em>");
    let r = tmpl.format_args(&["<script>"]);
    assert_eq!(r.as_str(), "<em>&lt;script&gt;</em>");
}

#[test]
fn format_args_multiple() {
    let tmpl = Markup::from_safe("{}: {}");
    let r = tmpl.format_args(&["key", "<value>"]);
    assert_eq!(r.as_str(), "key: &lt;value&gt;");
}

#[test]
fn format_args_indexed() {
    let tmpl = Markup::from_safe("{1} then {0}");
    let r = tmpl.format_args(&["second", "first"]);
    assert_eq!(r.as_str(), "first then second");
}

// ── escape_silent ─────────────────────────────────────────────────────────────

#[test]
fn escape_silent_none_is_empty() {
    assert_eq!(escape_silent(None).as_str(), "");
}

#[test]
fn escape_silent_some_escapes() {
    assert_eq!(escape_silent(Some("<b>")).as_str(), "&lt;b&gt;");
}

// ── soft_str ──────────────────────────────────────────────────────────────────

#[test]
fn soft_str_returns_inner() {
    let m = Markup::from_safe("<b>already safe</b>");
    assert_eq!(soft_str(&m), "<b>already safe</b>");
}

// ── Serde ─────────────────────────────────────────────────────────────────────

#[test]
fn serde_serialize_markup() {
    let m = Markup::from_safe("<b>ok</b>");
    let json = serde_json::to_string(&m).unwrap();
    assert_eq!(json, r#""<b>ok</b>""#);
}

#[test]
fn serde_deserialize_markup() {
    let json = r#""&lt;b&gt;ok&lt;/b&gt;""#;
    let m: Markup = serde_json::from_str(json).unwrap();
    // serde deserialises the JSON string bytes as-is (no additional escaping)
    assert_eq!(m.as_str(), "&lt;b&gt;ok&lt;/b&gt;");
}

// ── Display / Debug ───────────────────────────────────────────────────────────

#[test]
fn display_formats_inner() {
    let m = Markup::from_safe("<b>bold</b>");
    assert_eq!(format!("{}", m), "<b>bold</b>");
}

#[test]
fn debug_wraps_in_markup() {
    let m = Markup::from_safe("hi");
    assert!(format!("{:?}", m).contains("Markup"));
}

// ── Deref / AsRef ─────────────────────────────────────────────────────────────

#[test]
fn deref_to_str() {
    let m = Markup::from_safe("hello");
    let s: &str = &m;
    assert_eq!(s, "hello");
}

#[test]
fn asref_str() {
    let m = Markup::from_safe("hello");
    let s: &str = m.as_ref();
    assert_eq!(s, "hello");
}

// ── From conversions ──────────────────────────────────────────────────────────

#[test]
fn from_str_ref_escapes() {
    let m = Markup::from("<script>");
    assert_eq!(m.as_str(), "&lt;script&gt;");
}

#[test]
fn from_string_escapes() {
    let m = Markup::from(String::from("<b>"));
    assert_eq!(m.as_str(), "&lt;b&gt;");
}

#[test]
fn into_string_from_markup() {
    let m = Markup::from_safe("ok");
    let s: String = m.into();
    assert_eq!(s, "ok");
}

// ── MarkupEscapeWriter ────────────────────────────────────────────────────────

#[test]
fn escape_writer_basic() {
    use markupsafers::escape::MarkupEscapeWriter;
    use std::fmt::Write as _;

    let mut w = MarkupEscapeWriter::new();
    write!(w, "<em>hello</em>").unwrap();
    assert_eq!(w.into_markup().as_str(), "&lt;em&gt;hello&lt;/em&gt;");
}

#[test]
fn escape_writer_multiple_writes() {
    use markupsafers::escape::MarkupEscapeWriter;
    use std::fmt::Write as _;

    let mut w = MarkupEscapeWriter::new();
    write!(w, "<a>").unwrap();
    write!(w, "&").unwrap();
    write!(w, "</a>").unwrap();
    assert_eq!(w.into_markup().as_str(), "&lt;a&gt;&amp;&lt;/a&gt;");
}
