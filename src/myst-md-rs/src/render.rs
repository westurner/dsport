//! Event-stream HTML renderer with MyST hooks.
//!
//! We rebuild a small renderer rather than using `pulldown_cmark::html::push_html`
//! so we can intercept:
//! * `Text` events → split into role / inline-math / plain pieces.
//! * Fenced code blocks whose info string is `{name}` → render as
//!   `<div class="myst-directive" data-name="name">…</div>`.
//! * Fenced code blocks with info string `math` → render as `<div class="math">`.

use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use pulldown_cmark_escape::{escape_html, escape_html_body_text};

use crate::role::{Piece, split_text};

/// Render a preprocessed MyST body to HTML.
pub fn render(body: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(body, opts);
    let mut out = String::with_capacity(body.len());
    let mut state = State::default();
    for event in parser {
        push_event(&mut out, &mut state, event);
    }
    out
}

#[derive(Default)]
struct State {
    /// Stack of "code block kinds" we've opened so we know how to close them.
    code_stack: Vec<CodeKind>,
    /// True while inside a code block (we should not split text for roles).
    in_code: bool,
    /// If the previous Text event ended in `{name}`, the captured role name
    /// is stashed here so the *next* `Code` event becomes a MyST role.
    pending_role: Option<String>,
}

enum CodeKind {
    /// A normal fenced/indented code block — render as `<pre><code>`.
    Plain,
    /// A `{name}` MyST directive — render as `<div class="myst-directive">`.
    Directive,
    /// A `math` info string — render as `<div class="math">`.
    Math,
}

fn push_event(out: &mut String, state: &mut State, event: Event<'_>) {
    use Event::*;
    match event {
        Start(tag) => start_tag(out, state, tag),
        End(tag_end) => end_tag(out, state, tag_end),
        Text(s) => {
            if state.in_code {
                let mut buf = String::new();
                let _ = escape_html_body_text(&mut buf, &s);
                out.push_str(&buf);
            } else {
                render_text(out, state, &s);
            }
        }
        Code(s) => {
            if let Some(name) = state.pending_role.take() {
                let mut nbuf = String::new();
                let _ = escape_html(&mut nbuf, &name);
                let mut cbuf = String::new();
                let _ = escape_html_body_text(&mut cbuf, &s);
                out.push_str(&format!(
                    r#"<span class="myst-role" data-role="{nbuf}">{cbuf}</span>"#
                ));
            } else {
                out.push_str("<code>");
                let mut buf = String::new();
                let _ = escape_html_body_text(&mut buf, &s);
                out.push_str(&buf);
                out.push_str("</code>");
            }
        }
        Html(s) | InlineHtml(s) => out.push_str(&s),
        SoftBreak => out.push('\n'),
        HardBreak => out.push_str("<br />\n"),
        Rule => out.push_str("<hr />\n"),
        FootnoteReference(name) => {
            out.push_str("<sup class=\"footnote-ref\"><a href=\"#fn-");
            let mut buf = String::new();
            let _ = escape_html(&mut buf, &name);
            out.push_str(&buf);
            out.push_str("\">");
            out.push_str(&buf);
            out.push_str("</a></sup>");
        }
        TaskListMarker(checked) => {
            if checked {
                out.push_str(r#"<input type="checkbox" checked disabled /> "#);
            } else {
                out.push_str(r#"<input type="checkbox" disabled /> "#);
            }
        }
        DisplayMath(s) => {
            out.push_str(r#"<div class="math">"#);
            let mut buf = String::new();
            let _ = escape_html_body_text(&mut buf, &s);
            out.push_str(&buf);
            out.push_str("</div>");
        }
        InlineMath(s) => {
            out.push_str(r#"<span class="math">"#);
            let mut buf = String::new();
            let _ = escape_html_body_text(&mut buf, &s);
            out.push_str(&buf);
            out.push_str("</span>");
        }
    }
}

fn start_tag(out: &mut String, state: &mut State, tag: Tag<'_>) {
    use Tag::*;
    match tag {
        Paragraph => out.push_str("<p>"),
        Heading { level, .. } => {
            out.push_str(&format!("<{level}>"));
        }
        BlockQuote(_) => out.push_str("<blockquote>\n"),
        CodeBlock(kind) => {
            let info = match &kind {
                CodeBlockKind::Fenced(s) => s.as_ref(),
                CodeBlockKind::Indented => "",
            };
            if let Some(name) = directive_name(info) {
                let mut buf = String::new();
                let _ = escape_html(&mut buf, name);
                out.push_str(&format!(
                    r#"<div class="myst-directive" data-name="{buf}"><pre><code>"#
                ));
                state.code_stack.push(CodeKind::Directive);
            } else if info == "math" {
                out.push_str(r#"<div class="math">"#);
                state.code_stack.push(CodeKind::Math);
            } else {
                if info.is_empty() {
                    out.push_str("<pre><code>");
                } else {
                    let lang = info.split_whitespace().next().unwrap_or("");
                    let mut buf = String::new();
                    let _ = escape_html(&mut buf, lang);
                    out.push_str(&format!(r#"<pre><code class="language-{buf}">"#));
                }
                state.code_stack.push(CodeKind::Plain);
            }
            state.in_code = true;
        }
        List(Some(start)) if start != 1 => out.push_str(&format!("<ol start=\"{start}\">\n")),
        List(Some(_)) => out.push_str("<ol>\n"),
        List(None) => out.push_str("<ul>\n"),
        Item => out.push_str("<li>"),
        Emphasis => out.push_str("<em>"),
        Strong => out.push_str("<strong>"),
        Strikethrough => out.push_str("<del>"),
        Link { dest_url, title, .. } => {
            out.push_str("<a href=\"");
            let mut buf = String::new();
            let _ = escape_html(&mut buf, &dest_url);
            out.push_str(&buf);
            if !title.is_empty() {
                out.push_str("\" title=\"");
                let mut tbuf = String::new();
                let _ = escape_html(&mut tbuf, &title);
                out.push_str(&tbuf);
            }
            out.push_str("\">");
        }
        Image { dest_url, title, .. } => {
            out.push_str("<img src=\"");
            let mut buf = String::new();
            let _ = escape_html(&mut buf, &dest_url);
            out.push_str(&buf);
            out.push_str("\" alt=\"");
            if !title.is_empty() {
                out.push_str("\" title=\"");
                let mut tbuf = String::new();
                let _ = escape_html(&mut tbuf, &title);
                out.push_str(&tbuf);
            }
            out.push_str("\" />");
        }
        Table(_) => out.push_str("<table>\n"),
        TableHead => out.push_str("<thead><tr>"),
        TableRow => out.push_str("<tr>"),
        TableCell => out.push_str("<td>"),
        FootnoteDefinition(name) => {
            out.push_str("<div class=\"footnote-def\" id=\"fn-");
            let mut buf = String::new();
            let _ = escape_html(&mut buf, &name);
            out.push_str(&buf);
            out.push_str("\">");
        }
        // MyST-specific shapes pulldown-cmark doesn't currently produce — keep
        // as catch-all rather than panicking.
        other => {
            let _ = other;
        }
    }
}

fn end_tag(out: &mut String, state: &mut State, tag_end: TagEnd) {
    use TagEnd::*;
    match tag_end {
        Paragraph => out.push_str("</p>\n"),
        Heading(level) => out.push_str(&format!("</{level}>\n")),
        BlockQuote(_) => out.push_str("</blockquote>\n"),
        CodeBlock => {
            state.in_code = false;
            match state.code_stack.pop().unwrap_or(CodeKind::Plain) {
                CodeKind::Plain => out.push_str("</code></pre>\n"),
                CodeKind::Directive => out.push_str("</code></pre></div>\n"),
                CodeKind::Math => out.push_str("</div>\n"),
            }
        }
        List(true) => out.push_str("</ol>\n"),
        List(false) => out.push_str("</ul>\n"),
        Item => out.push_str("</li>\n"),
        Emphasis => out.push_str("</em>"),
        Strong => out.push_str("</strong>"),
        Strikethrough => out.push_str("</del>"),
        Link => out.push_str("</a>"),
        Image => {}
        Table => out.push_str("</table>\n"),
        TableHead => out.push_str("</tr></thead>\n"),
        TableRow => out.push_str("</tr>\n"),
        TableCell => out.push_str("</td>"),
        FootnoteDefinition => out.push_str("</div>\n"),
        _ => {}
    }
}

/// If `info` is `{name}` (optionally followed by args), return the name.
fn directive_name(info: &str) -> Option<&str> {
    let first = info.split_whitespace().next().unwrap_or("");
    let inner = first.strip_prefix('{')?.strip_suffix('}')?;
    if inner.is_empty() {
        None
    } else {
        Some(inner)
    }
}

fn render_text(out: &mut String, state: &mut State, text: &str) {
    // Detect a trailing role marker `{name}` so the following `Code` event
    // (cmark sees the backtick run as inline code) can be rewritten.
    let (head, trailing_role) = strip_trailing_role_marker(text);
    for piece in split_text(head) {
        match piece {
            Piece::Text(s) => {
                let mut buf = String::new();
                let _ = escape_html_body_text(&mut buf, s);
                out.push_str(&buf);
            }
            Piece::Role { name, content } => {
                let mut nbuf = String::new();
                let _ = escape_html(&mut nbuf, name);
                let mut cbuf = String::new();
                let _ = escape_html_body_text(&mut cbuf, content);
                out.push_str(&format!(
                    r#"<span class="myst-role" data-role="{nbuf}">{cbuf}</span>"#
                ));
            }
            Piece::InlineMath(content) => {
                let mut cbuf = String::new();
                let _ = escape_html_body_text(&mut cbuf, content);
                out.push_str(&format!(r#"<span class="math">{cbuf}</span>"#));
            }
        }
    }
    if let Some(name) = trailing_role {
        state.pending_role = Some(name.to_string());
    }
}

/// If `text` ends with a bare `{name}` role marker (immediately before what
/// will be a `Code` event in the cmark stream), strip it off and return the
/// name. Otherwise return `(text, None)`.
fn strip_trailing_role_marker(text: &str) -> (&str, Option<&str>) {
    let bytes = text.as_bytes();
    if !bytes.last().is_some_and(|b| *b == b'}') {
        return (text, None);
    }
    // Find the matching `{`.
    let p = bytes.len() - 1; // at `}`
    if p == 0 {
        return (text, None);
    }
    let mut q = p;
    while q > 0 {
        q -= 1;
        let c = bytes[q];
        if c == b'{' {
            let name = &text[q + 1..p];
            if name.is_empty() {
                return (text, None);
            }
            if !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b':' | b'.'))
            {
                return (text, None);
            }
            return (&text[..q], Some(name));
        }
        if !(c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_' | b':' | b'.')) {
            return (text, None);
        }
    }
    (text, None)
}
