//! HTML renderer with MyST event-stream hooks.
//!
//! We delegate the bulk of CommonMark/GFM rendering to
//! `pulldown_cmark::html::push_html` — which gives us reference-quality output
//! for headings, lists, tables, link-reference definitions, HTML blocks, etc.
//! — and only rewrite the event stream for MyST-specific constructs:
//!
//! * Inline roles `` {name}`content` `` (either entirely inside one `Text`
//!   event, or split across `Text("...{name}")` + `Code("content")` after the
//!   cmark inline parser has eaten the backtick run).
//! * Inline math `$…$` inside a `Text` event.
//! * Fenced code blocks whose info string is `{name}` → wrap as
//!   `<div class="myst-directive" data-name="name">…</div>`.
//! * Fenced code blocks with info string `math` → wrap as
//!   `<div class="math">…</div>`.
//!
//! All other events flow through untouched.

use std::collections::VecDeque;

use mathrenderrs::{MathBackend, MathDisplay, render as render_math};
use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use pulldown_cmark_escape::{escape_html, escape_html_body_text};

use crate::role::{Piece, split_text};

/// Render a preprocessed MyST body to HTML using the default math
/// backend ([`MathBackend::Ratex`]).
pub fn render(body: &str) -> String {
    render_with(body, MathBackend::default())
}

/// Render a preprocessed MyST body to HTML, choosing the math
/// rendering backend explicitly. See [`mathrenderrs::MathBackend`].
pub fn render_with(body: &str, math_backend: MathBackend) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    opts.insert(Options::ENABLE_GFM);
    // Note: ENABLE_SMART_PUNCTUATION intentionally left off — it rewrites
    // `---`/`--`/`"` into en/em-dashes and curly quotes which breaks plain
    // CommonMark spec parity. MyST exposes it as the `smartquotes` /
    // `replacements` opt-in extension; we'll wire that to a config flag in a
    // later wave.

    let parser = Parser::new_ext(body, opts);
    let events = MystEvents::new(parser, math_backend);
    let mut out = String::with_capacity(body.len());
    html::push_html(&mut out, events);
    out
}

/// What to emit at the matching `End(CodeBlock)` for a re-routed code block.
enum CodeCloser {
    /// Forward the original `End(CodeBlock)` event verbatim.
    Forward,
    /// Emit a raw HTML closer.
    Html(&'static str),
    /// Drain the buffered math source through [`mathrenderrs`] and emit
    /// the rendered fragment in place of the usual `<div>…</div>` body.
    Math,
}

/// Event-stream adapter that lowers MyST extensions to stock `pulldown_cmark`
/// events (mostly raw `Html(...)` blobs the stock renderer will pass through).
struct MystEvents<'a, I: Iterator<Item = Event<'a>>> {
    inner: I,
    /// Buffered events to emit before pulling the next one from `inner`.
    queue: VecDeque<Event<'a>>,
    /// Stack of closers, one entry per open code block.
    closer_stack: Vec<CodeCloser>,
    /// When the previous `Text` event ended in `{name}` (a role marker), the
    /// captured name is stashed here so the *next* `Code` event becomes a
    /// MyST role rather than a `<code>` element.
    pending_role: Option<String>,
    /// True while inside a code block — text inside must stay verbatim.
    in_code_block: bool,
    /// When `Some`, the current fenced block has info string `math`; raw
    /// `Text` events are diverted here instead of being forwarded, so on
    /// `End(CodeBlock)` we can hand the whole source to [`mathrenderrs`].
    math_buf: Option<String>,
    /// Backend used to render `$…$` and `$$…$$` / `` ```math `` blocks.
    math_backend: MathBackend,
}

impl<'a, I: Iterator<Item = Event<'a>>> MystEvents<'a, I> {
    fn new(inner: I, math_backend: MathBackend) -> Self {
        Self {
            inner,
            queue: VecDeque::new(),
            closer_stack: Vec::new(),
            pending_role: None,
            in_code_block: false,
            math_buf: None,
            math_backend,
        }
    }

    fn map_event(&mut self, event: Event<'a>) {
        match event {
            Event::Text(s) if !self.in_code_block => self.handle_text(s),
            Event::Code(s) if self.pending_role.is_some() => {
                let name = self.pending_role.take().unwrap();
                let html = render_role(&name, &s);
                self.queue.push_back(Event::InlineHtml(CowStr::from(html)));
            }
            Event::Start(Tag::CodeBlock(ref kind)) => {
                self.in_code_block = true;
                let info: &str = match kind {
                    CodeBlockKind::Fenced(s) => s.as_ref(),
                    CodeBlockKind::Indented => "",
                };
                if let Some(name) = directive_name(info) {
                    let open = format!(
                        r#"<div class="myst-directive" data-name="{}"><pre><code>"#,
                        escape_attr(name)
                    );
                    self.queue.push_back(Event::Html(CowStr::from(open)));
                    self.closer_stack
                        .push(CodeCloser::Html("</code></pre></div>\n"));
                } else if info == "math" {
                    // Buffer the body verbatim; on End(CodeBlock) we
                    // hand it to mathrenderrs and emit the rendered
                    // fragment in one piece.
                    self.math_buf = Some(String::new());
                    self.closer_stack.push(CodeCloser::Math);
                } else {
                    self.queue.push_back(event);
                    self.closer_stack.push(CodeCloser::Forward);
                }
            }
            Event::Text(ref s) if self.math_buf.is_some() => {
                self.math_buf.as_mut().unwrap().push_str(s.as_ref());
            }
            Event::End(TagEnd::CodeBlock) => {
                self.in_code_block = false;
                match self.closer_stack.pop() {
                    Some(CodeCloser::Html(s)) => {
                        self.queue.push_back(Event::Html(CowStr::from(s)));
                    }
                    Some(CodeCloser::Math) => {
                        let body = self.math_buf.take().unwrap_or_default();
                        // Trim a single trailing newline that pulldown_cmark
                        // appends to the last text event of a fenced block,
                        // so RaTeX doesn't see stray whitespace.
                        let trimmed = body.strip_suffix('\n').unwrap_or(&body);
                        let html = render_math(self.math_backend, MathDisplay::Block, trimmed);
                        let mut html = html;
                        html.push('\n');
                        self.queue.push_back(Event::Html(CowStr::from(html)));
                    }
                    _ => self.queue.push_back(event),
                }
            }
            other => self.queue.push_back(other),
        }
    }

    fn handle_text(&mut self, text: CowStr<'a>) {
        let (head_owned, trailing_role) = {
            let s: &str = text.as_ref();
            let (head, trailing) = strip_trailing_role_marker(s);
            (head.to_string(), trailing.map(str::to_string))
        };

        let head: &str = &head_owned;
        if has_inline_markers(head) {
            for piece in split_text(head) {
                match piece {
                    Piece::Text(t) if !t.is_empty() => {
                        self.queue
                            .push_back(Event::Text(CowStr::from(t.to_string())));
                    }
                    Piece::Text(_) => {}
                    Piece::Role { name, content } => {
                        self.queue
                            .push_back(Event::InlineHtml(CowStr::from(render_role(name, content))));
                    }
                    Piece::InlineMath(content) => {
                        self.queue
                            .push_back(Event::InlineHtml(CowStr::from(render_math(
                                self.math_backend,
                                MathDisplay::Inline,
                                content,
                            ))));
                    }
                }
            }
        } else if !head.is_empty() {
            if head.len() == text.len() {
                self.queue.push_back(Event::Text(text));
            } else {
                self.queue.push_back(Event::Text(CowStr::from(head_owned)));
            }
        }

        if let Some(name) = trailing_role {
            self.pending_role = Some(name);
        }
    }
}

impl<'a, I: Iterator<Item = Event<'a>>> Iterator for MystEvents<'a, I> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.queue.pop_front() {
                return Some(e);
            }
            let event = self.inner.next()?;
            self.map_event(event);
        }
    }
}

fn render_role(name: &str, content: &str) -> String {
    let mut cbuf = String::new();
    let _ = escape_html_body_text(&mut cbuf, content);
    format!(
        r#"<span class="myst-role" data-role="{}">{}</span>"#,
        escape_attr(name),
        cbuf
    )
}

fn escape_attr(s: &str) -> String {
    let mut buf = String::new();
    let _ = escape_html(&mut buf, s);
    buf
}

/// If `info` is `{name}` (optionally followed by args), return the name.
fn directive_name(info: &str) -> Option<&str> {
    let first = info.split_whitespace().next().unwrap_or("");
    let inner = first.strip_prefix('{')?.strip_suffix('}')?;
    if inner.is_empty() { None } else { Some(inner) }
}

/// Quick check so we only allocate split-vectors for runs that actually
/// contain a `{` or `$`.
fn has_inline_markers(text: &str) -> bool {
    text.contains('{') || text.contains('$')
}

/// If `text` ends with a bare `{name}` role marker (immediately before what
/// will be a `Code` event in the cmark stream), strip it off and return the
/// name. Otherwise return `(text, None)`.
fn strip_trailing_role_marker(text: &str) -> (&str, Option<&str>) {
    let bytes = text.as_bytes();
    if bytes.last().is_none_or(|b| *b != b'}') {
        return (text, None);
    }
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
