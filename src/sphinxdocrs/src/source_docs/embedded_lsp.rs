//! Embedded stdio LSP server for Python and Rust `textDocument/documentSymbol`
//! extraction, plus helpers to extract and compare API and test symbols.

use std::collections::HashMap;
use std::io::{BufReader, Read, Write};

use ruff_python_ast::{Expr, Stmt, StmtClassDef, StmtFunctionDef};
use ruff_python_parser::parse_module;
use ruff_text_size::Ranged;
use serde_json::{json, Value};

use super::lsp_backend::{read_frame, write_frame};
use super::model::SourceLanguage;

const MAX_STDIO_BYTES: usize = 8 * 1024 * 1024;

/// Run an LSP JSON-RPC 2.0 server over `stdin`/`stdout` supporting
/// `initialize`, `initialized`, `textDocument/didOpen`, `textDocument/didClose`,
/// `textDocument/documentSymbol`, `shutdown`, and `exit`.
pub fn run_embedded_lsp_stdio(
    language: SourceLanguage,
    stdin: impl Read,
    mut stdout: impl Write,
) -> std::io::Result<()> {
    let mut reader = BufReader::new(stdin);
    let mut open_documents: HashMap<String, String> = HashMap::new();
    let server_name = match language {
        SourceLanguage::Python => "sphinxdocrs-python-lsp",
        SourceLanguage::Rust => "sphinxdocrs-rust-lsp",
        SourceLanguage::Lean => "sphinxdocrs-lean-lsp",
    };

    while let Some(body) = read_frame(&mut reader, MAX_STDIO_BYTES)? {
        let Ok(message) = serde_json::from_slice::<Value>(&body) else {
            break;
        };
        let method = message.get("method").and_then(Value::as_str);
        if method == Some("exit") {
            break;
        }
        let Some(id) = message.get("id").cloned() else {
            if let Some(params) = message.get("params") {
                match method {
                    Some("textDocument/didOpen") => {
                        if let Some(doc) = params.get("textDocument")
                            && let (Some(uri), Some(text)) = (
                                doc.get("uri").and_then(Value::as_str),
                                doc.get("text").and_then(Value::as_str),
                            )
                        {
                            open_documents.insert(uri.to_string(), text.to_string());
                        }
                    }
                    Some("textDocument/didClose") => {
                        if let Some(uri) = params
                            .get("textDocument")
                            .and_then(|doc| doc.get("uri"))
                            .and_then(Value::as_str)
                        {
                            open_documents.remove(uri);
                        }
                    }
                    _ => {}
                }
            }
            continue;
        };

        let result = match method {
            Some("initialize") => json!({
                "capabilities": {
                    "documentSymbolProvider": true
                },
                "serverInfo": {
                    "name": server_name,
                    "version": env!("CARGO_PKG_VERSION")
                }
            }),
            Some("textDocument/documentSymbol") => {
                let uri = message
                    .get("params")
                    .and_then(|params| params.get("textDocument"))
                    .and_then(|doc| doc.get("uri"))
                    .and_then(Value::as_str)
                    .unwrap_or_default();
                let text = open_documents.get(uri).cloned().unwrap_or_default();
                match language {
                    SourceLanguage::Python => Value::Array(python_document_symbols(&text)),
                    SourceLanguage::Rust => Value::Array(rust_document_symbols(&text)),
                    SourceLanguage::Lean => Value::Array(Vec::new()),
                }
            }
            Some("shutdown") => Value::Null,
            _ => Value::Null,
        };

        let response = json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        });
        write_frame(&mut stdout, &response, MAX_STDIO_BYTES)?;
    }

    Ok(())
}

/// Extract hierarchical LSP `DocumentSymbol` objects from Python source text
/// using `ruff_python_parser`.
pub fn python_document_symbols(source: &str) -> Vec<Value> {
    let Ok(parsed) = parse_module(source) else {
        return Vec::new();
    };
    let index = LineColIndex::new(source);
    let mut symbols = Vec::new();
    for stmt in parsed.suite() {
        if let Some(symbol) = python_stmt_to_symbol(stmt, None, &index) {
            symbols.push(symbol);
        }
    }
    symbols
}

fn python_stmt_to_symbol(
    stmt: &Stmt,
    parent_class: Option<&str>,
    index: &LineColIndex<'_>,
) -> Option<Value> {
    match stmt {
        Stmt::FunctionDef(func) => Some(python_func_symbol(func, parent_class, index)),
        Stmt::ClassDef(class) => Some(python_class_symbol(class, index)),
        _ => None,
    }
}

fn python_func_symbol(
    func: &StmtFunctionDef,
    parent_class: Option<&str>,
    index: &LineColIndex<'_>,
) -> Value {
    let name = func.name.as_str();
    let kind = if parent_class.is_some() { 6u64 } else { 12u64 };
    let sig = crate::autodoc::format_signature_with_hints(
        &func.parameters,
        crate::autodoc::TypeHints::Signature,
    );
    let prefix = if func.is_async { "async def" } else { "def" };
    let detail = format!("{prefix} {name}({sig})");
    let is_test = name.starts_with("test_")
        || (name == "test"
            && parent_class.is_some_and(|cls| cls.starts_with("Test") || cls.ends_with("Test")));
    let range = index.lsp_range(
        func.range().start().to_usize(),
        func.range().end().to_usize(),
    );
    let selection_range = index.lsp_range(
        func.name.range().start().to_usize(),
        func.name.range().end().to_usize(),
    );
    let mut children = Vec::new();
    for child in &func.body {
        if let Some(sym) = python_stmt_to_symbol(child, None, index) {
            children.push(sym);
        }
    }
    json!({
        "name": name,
        "kind": kind,
        "detail": detail,
        "range": range,
        "selectionRange": selection_range,
        "isTest": is_test,
        "children": children,
    })
}

fn python_class_symbol(class: &StmtClassDef, index: &LineColIndex<'_>) -> Value {
    let name = class.name.as_str();
    let bases = class
        .arguments
        .as_ref()
        .map(|args| {
            args.args
                .iter()
                .map(crate::autodoc::render_expr)
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    let detail = if bases.is_empty() {
        format!("class {name}")
    } else {
        format!("class {name}({bases})")
    };
    let range = index.lsp_range(
        class.range().start().to_usize(),
        class.range().end().to_usize(),
    );
    let selection_range = index.lsp_range(
        class.name.range().start().to_usize(),
        class.name.range().end().to_usize(),
    );
    let mut children = Vec::new();
    for stmt in &class.body {
        match stmt {
            Stmt::FunctionDef(func) => {
                children.push(python_func_symbol(func, Some(name), index));
            }
            Stmt::ClassDef(nested) => {
                children.push(python_class_symbol(nested, index));
            }
            Stmt::Assign(assign) => {
                for target in &assign.targets {
                    if let Expr::Name(target_name) = target {
                        let field_range = index.lsp_range(
                            assign.range().start().to_usize(),
                            assign.range().end().to_usize(),
                        );
                        let sel_range = index.lsp_range(
                            target_name.range().start().to_usize(),
                            target_name.range().end().to_usize(),
                        );
                        children.push(json!({
                            "name": target_name.id.as_str(),
                            "kind": 8u64,
                            "range": field_range,
                            "selectionRange": sel_range,
                            "isTest": false,
                            "children": [],
                        }));
                    }
                }
            }
            Stmt::AnnAssign(ann) => {
                if let Expr::Name(target_name) = ann.target.as_ref() {
                    let field_range = index.lsp_range(
                        ann.range().start().to_usize(),
                        ann.range().end().to_usize(),
                    );
                    let sel_range = index.lsp_range(
                        target_name.range().start().to_usize(),
                        target_name.range().end().to_usize(),
                    );
                    children.push(json!({
                        "name": target_name.id.as_str(),
                        "kind": 8u64,
                        "range": field_range,
                        "selectionRange": sel_range,
                        "isTest": false,
                        "children": [],
                    }));
                }
            }
            _ => {}
        }
    }
    json!({
        "name": name,
        "kind": 5u64,
        "detail": detail,
        "range": range,
        "selectionRange": selection_range,
        "isTest": false,
        "children": children,
    })
}

/// Extract hierarchical LSP `DocumentSymbol` objects from Rust source text.
pub fn rust_document_symbols(source: &str) -> Vec<Value> {
    let lines: Vec<&str> = source.lines().collect();
    let mut idx = 0usize;
    parse_rust_block(&lines, &mut idx, false, false)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RustBlockKind {
    Module,
    Impl,
    Trait,
    Other,
}

fn parse_rust_block(
    lines: &[&str],
    idx: &mut usize,
    in_impl_or_trait: bool,
    in_test_mod: bool,
) -> Vec<Value> {
    let mut symbols = Vec::new();
    let mut pending_test_attr = false;
    let mut pending_cfg_test = false;
    let mut brace_depth = 0i32;

    while *idx < lines.len() {
        let line_no = *idx;
        let line = lines[line_no];
        let code = strip_rust_line_comment(line);
        let trimmed = code.trim();
        *idx += 1;

        if trimmed.is_empty() {
            continue;
        }

        if is_rust_test_attr_line(trimmed) {
            pending_test_attr = true;
        }
        if trimmed.starts_with("#[cfg(") && trimmed.contains("test") {
            pending_cfg_test = true;
        }

        let delta = brace_delta(code);

        if brace_depth == 0 {
            if let Some((sym_kind, name, detail, block_kind)) =
                recognize_rust_declaration(trimmed, in_impl_or_trait)
            {
                let is_test = (sym_kind == 12 || sym_kind == 6) && pending_test_attr;
                let child_in_test_mod =
                    in_test_mod || (block_kind == Some(RustBlockKind::Module) && (pending_cfg_test || name == "tests"));
                pending_test_attr = false;
                pending_cfg_test = false;

                let mut children = Vec::new();
                let mut end_line = line_no;
                let has_open_brace = has_unquoted_char(code, '{');
                if delta > 0 {
                    if let Some(bk) = block_kind {
                        let child_in_impl =
                            matches!(bk, RustBlockKind::Impl | RustBlockKind::Trait);
                        if matches!(
                            bk,
                            RustBlockKind::Module | RustBlockKind::Impl | RustBlockKind::Trait
                        ) {
                            children =
                                parse_rust_block(lines, idx, child_in_impl, child_in_test_mod);
                        } else {
                            skip_balanced_braces(lines, idx, delta);
                        }
                        end_line = (*idx).saturating_sub(1);
                    } else {
                        skip_balanced_braces(lines, idx, delta);
                        end_line = (*idx).saturating_sub(1);
                    }
                } else if !has_open_brace && !trimmed.ends_with(';') {
                    // Multi-line header before `{` or `;`
                    while *idx < lines.len() {
                        let next_line = lines[*idx];
                        let next_code = strip_rust_line_comment(next_line);
                        let next_trimmed = next_code.trim();
                        let next_delta = brace_delta(next_code);
                        let next_has_open = has_unquoted_char(next_code, '{');
                        *idx += 1;
                        end_line = (*idx).saturating_sub(1);
                        if next_delta > 0 {
                            if let Some(bk) = block_kind {
                                let child_in_impl =
                                    matches!(bk, RustBlockKind::Impl | RustBlockKind::Trait);
                                if matches!(
                                    bk,
                                    RustBlockKind::Module
                                        | RustBlockKind::Impl
                                        | RustBlockKind::Trait
                                ) && next_delta == 1
                                {
                                    children = parse_rust_block(
                                        lines,
                                        idx,
                                        child_in_impl,
                                        child_in_test_mod,
                                    );
                                } else {
                                    skip_balanced_braces(lines, idx, next_delta);
                                }
                                end_line = (*idx).saturating_sub(1);
                            } else {
                                skip_balanced_braces(lines, idx, next_delta);
                                end_line = (*idx).saturating_sub(1);
                            }
                            break;
                        }
                        if next_has_open || next_trimmed.ends_with(';') {
                            break;
                        }
                    }
                }

                let start_col = line.len().saturating_sub(line.trim_start().len());
                let end_col = lines.get(end_line).map_or(0, |l| l.encode_utf16().count());
                let start_utf16 = line[..start_col].encode_utf16().count();
                let range = json!({
                    "start": { "line": line_no, "character": start_utf16 },
                    "end": { "line": end_line, "character": end_col }
                });
                let selection_range = json!({
                    "start": { "line": line_no, "character": start_utf16 },
                    "end": { "line": line_no, "character": start_utf16 }
                });

                symbols.push(json!({
                    "name": name,
                    "kind": sym_kind,
                    "detail": detail,
                    "range": range,
                    "selectionRange": selection_range,
                    "isTest": is_test,
                    "inTestMod": in_test_mod,
                    "children": children,
                }));
                continue;
            }
        }

        if !trimmed.starts_with("#[") && !trimmed.starts_with("//") {
            pending_test_attr = false;
            pending_cfg_test = false;
        }

        brace_depth += delta;
        if brace_depth < 0 {
            break;
        }
    }

    symbols
}

fn skip_balanced_braces(lines: &[&str], idx: &mut usize, mut depth: i32) {
    while depth > 0 && *idx < lines.len() {
        let code = strip_rust_line_comment(lines[*idx]);
        depth += brace_delta(code);
        *idx += 1;
    }
}

fn is_rust_test_attr_line(trimmed: &str) -> bool {
    trimmed.starts_with("#[test")
        || trimmed.starts_with("#[rstest")
        || trimmed.starts_with("#[tokio::test")
        || trimmed.contains("::test]")
        || trimmed.contains("::test(")
        || trimmed.contains("::rstest]")
        || trimmed.contains("::rstest(")
}

fn recognize_rust_declaration(
    trimmed: &str,
    in_impl_or_trait: bool,
) -> Option<(u64, String, String, Option<RustBlockKind>)> {
    if trimmed.starts_with('#') || trimmed.starts_with("//") || trimmed.starts_with("/*") {
        return None;
    }
    let rest = strip_rust_qualifiers(trimmed);
    for (kw, kind, block_kind) in [
        ("fn ", if in_impl_or_trait { 6u64 } else { 12u64 }, Some(RustBlockKind::Other)),
        ("struct ", 23u64, Some(RustBlockKind::Other)),
        ("enum ", 10u64, Some(RustBlockKind::Other)),
        ("trait ", 11u64, Some(RustBlockKind::Trait)),
        ("mod ", 2u64, Some(RustBlockKind::Module)),
        ("type ", 26u64, None),
        ("const ", 14u64, None),
        ("static ", 14u64, None),
        ("macro_rules! ", 12u64, Some(RustBlockKind::Other)),
    ] {
        if let Some(after) = rest.strip_prefix(kw) {
            let ident = extract_rust_ident(after)?;
            let detail = trimmed
                .split('{')
                .next()
                .unwrap_or(trimmed)
                .trim_end_matches(';')
                .trim()
                .to_string();
            return Some((kind, ident, detail, block_kind));
        }
    }
    if rest == "impl" || rest.starts_with("impl ") || rest.starts_with("impl<") {
        let header = rest
            .split('{')
            .next()
            .unwrap_or(rest)
            .split("where")
            .next()
            .unwrap_or(rest)
            .trim();
        let target = extract_impl_target(header)?;
        return Some((5u64, target, header.to_string(), Some(RustBlockKind::Impl)));
    }
    None
}

fn strip_rust_qualifiers(mut s: &str) -> &str {
    loop {
        let before = s;
        s = s.trim_start();
        if let Some(rest) = s.strip_prefix("pub") {
            if let Some(after_paren) = rest.strip_prefix('(') {
                if let Some((_, tail)) = after_paren.split_once(')') {
                    s = tail.trim_start();
                }
            } else if rest.starts_with(char::is_whitespace) {
                s = rest.trim_start();
            }
        }
        for kw in ["default ", "const ", "async ", "unsafe "] {
            if let Some(rest) = s.strip_prefix(kw) {
                // Keep `const ` if it's `const NAME: Type` rather than `const fn`
                if kw == "const " && !rest.trim_start().starts_with("fn ") && !rest.trim_start().starts_with("unsafe ") {
                    continue;
                }
                s = rest.trim_start();
            }
        }
        if let Some(rest) = s.strip_prefix("extern") {
            let rest = rest.trim_start();
            if let Some(after_quote) = rest.strip_prefix('"') {
                if let Some((_, tail)) = after_quote.split_once('"') {
                    s = tail.trim_start();
                }
            } else if rest.starts_with("fn ") {
                s = rest;
            }
        }
        if s == before {
            break;
        }
    }
    s
}

fn extract_rust_ident(s: &str) -> Option<String> {
    let s = s.trim_start();
    let s = s.strip_prefix("r#").unwrap_or(s);
    let ident: String = s
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect();
    (!ident.is_empty() && !ident.chars().next().unwrap().is_ascii_digit()).then_some(ident)
}

fn extract_impl_target(header: &str) -> Option<String> {
    let after_impl = header.strip_prefix("impl")?.trim_start();
    let after_generics = if after_impl.starts_with('<') {
        let mut depth = 0i32;
        let mut split_at = None;
        for (idx, ch) in after_impl.char_indices() {
            match ch {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        split_at = Some(idx + 1);
                        break;
                    }
                }
                _ => {}
            }
        }
        after_impl.get(split_at?..)?
    } else {
        after_impl
    };
    let target_part = if let Some((_, rhs)) = after_generics.split_once(" for ") {
        rhs.trim()
    } else {
        after_generics.trim()
    };
    let clean = target_part
        .trim_start_matches('&')
        .trim_start_matches("mut ")
        .trim();
    let name: String = clean
        .chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | ':'))
        .collect();
    let short = name.rsplit("::").next().unwrap_or(&name);
    (!short.is_empty()).then(|| short.to_string())
}

fn strip_rust_line_comment(line: &str) -> &str {
    let mut in_str = false;
    let mut escaped = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_str {
            if escaped {
                escaped = false;
            } else if b == b'\\' {
                escaped = true;
            } else if b == b'"' {
                in_str = false;
            }
        } else if b == b'"' {
            in_str = true;
        } else if b == b'/' && bytes.get(i + 1) == Some(&b'/') {
            return &line[..i];
        }
        i += 1;
    }
    line
}

fn has_unquoted_char(code: &str, target: char) -> bool {
    let mut in_str = false;
    let mut escaped = false;
    for ch in code.chars() {
        if in_str {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_str = false;
            }
        } else if ch == '"' {
            in_str = true;
        } else if ch == target {
            return true;
        }
    }
    false
}

fn brace_delta(code: &str) -> i32 {
    let mut delta = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    for ch in code.chars() {
        if in_str {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_str = false;
            }
        } else {
            match ch {
                '"' => in_str = true,
                '{' => delta += 1,
                '}' => delta -= 1,
                _ => {}
            }
        }
    }
    delta
}

struct LineColIndex<'a> {
    source: &'a str,
    line_starts: Vec<usize>,
}

impl<'a> LineColIndex<'a> {
    fn new(source: &'a str) -> Self {
        let mut line_starts = vec![0];
        for (idx, b) in source.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(idx + 1);
            }
        }
        Self {
            source,
            line_starts,
        }
    }

    fn line_and_utf16_col(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.source.len());
        let line_idx = match self.line_starts.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx.saturating_sub(1),
        };
        let line_start = self.line_starts[line_idx];
        let line_end = self
            .line_starts
            .get(line_idx + 1)
            .copied()
            .unwrap_or(self.source.len());
        let line_slice = self
            .source
            .get(line_start..line_end)
            .unwrap_or_default()
            .trim_end_matches(['\r', '\n']);
        let rel_byte = offset.saturating_sub(line_start).min(line_slice.len());
        let prefix = line_slice.get(..rel_byte).unwrap_or_default();
        let utf16_col = prefix.encode_utf16().count();
        (line_idx, utf16_col)
    }

    fn lsp_range(&self, start_byte: usize, end_byte: usize) -> Value {
        let (start_line, start_col) = self.line_and_utf16_col(start_byte);
        let (end_line, end_col) = self.line_and_utf16_col(end_byte);
        json!({
            "start": { "line": start_line, "character": start_col },
            "end": { "line": end_line, "character": end_col }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn python_document_symbols_extracts_classes_methods_and_tests() {
        let src = r#"
class MyDocumenter:
    def render(self, x: int) -> str:
        return str(x)

def test_render_output():
    assert True
"#;
        let symbols = python_document_symbols(src);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0]["name"], "MyDocumenter");
        assert_eq!(symbols[0]["kind"], 5);
        assert_eq!(symbols[0]["children"][0]["name"], "render");
        assert_eq!(symbols[0]["children"][0]["kind"], 6);
        assert_eq!(symbols[0]["children"][0]["isTest"], false);
        assert_eq!(symbols[1]["name"], "test_render_output");
        assert_eq!(symbols[1]["kind"], 12);
        assert_eq!(symbols[1]["isTest"], true);
    }

    #[test]
    fn rust_document_symbols_extracts_structs_impls_fns_and_tests() {
        let src = r#"
pub struct HtmlBuilder {
    pub count: usize,
}

impl HtmlBuilder {
    pub fn build_all(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[rstest]
    fn test_build_all() {
        assert!(true);
    }
}
"#;
        let symbols = rust_document_symbols(src);
        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0]["name"], "HtmlBuilder");
        assert_eq!(symbols[0]["kind"], 23);
        assert_eq!(symbols[1]["name"], "HtmlBuilder");
        assert_eq!(symbols[1]["kind"], 5);
        assert_eq!(symbols[1]["children"][0]["name"], "build_all");
        assert_eq!(symbols[1]["children"][0]["kind"], 6);
        assert_eq!(symbols[2]["name"], "tests");
        assert_eq!(symbols[2]["kind"], 2);
        assert_eq!(symbols[2]["children"][0]["name"], "test_build_all");
        assert_eq!(symbols[2]["children"][0]["isTest"], true);
    }

    #[test]
    fn embedded_stdio_server_handles_initialize_and_document_symbol_roundtrip() {
        let mut input = Vec::new();
        write_frame(
            &mut input,
            &json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {}
            }),
            MAX_STDIO_BYTES,
        )
        .unwrap();
        write_frame(
            &mut input,
            &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": {
                        "uri": "file:///workspace/demo.py",
                        "languageId": "python",
                        "version": 1,
                        "text": "def hello():\n    pass\n"
                    }
                }
            }),
            MAX_STDIO_BYTES,
        )
        .unwrap();
        write_frame(
            &mut input,
            &json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "textDocument/documentSymbol",
                "params": {
                    "textDocument": { "uri": "file:///workspace/demo.py" }
                }
            }),
            MAX_STDIO_BYTES,
        )
        .unwrap();
        write_frame(
            &mut input,
            &json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "shutdown",
                "params": null
            }),
            MAX_STDIO_BYTES,
        )
        .unwrap();
        write_frame(
            &mut input,
            &json!({
                "jsonrpc": "2.0",
                "method": "exit"
            }),
            MAX_STDIO_BYTES,
        )
        .unwrap();

        let mut output = Vec::new();
        run_embedded_lsp_stdio(SourceLanguage::Python, Cursor::new(input), &mut output).unwrap();

        let mut reader = BufReader::new(Cursor::new(output));
        let init_resp: Value =
            serde_json::from_slice(&read_frame(&mut reader, MAX_STDIO_BYTES).unwrap().unwrap())
                .unwrap();
        assert_eq!(
            init_resp["result"]["capabilities"]["documentSymbolProvider"],
            true
        );
        let sym_resp: Value =
            serde_json::from_slice(&read_frame(&mut reader, MAX_STDIO_BYTES).unwrap().unwrap())
                .unwrap();
        assert_eq!(sym_resp["result"][0]["name"], "hello");
    }
}
