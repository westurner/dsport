//! Embedded stdio LSP server for Python and Rust `textDocument/documentSymbol`
//! extraction, plus helpers to extract and compare API and test symbols.

use std::collections::HashMap;
use std::io::{BufReader, Read, Write};

use ruff_python_ast::{Expr, Stmt, StmtClassDef, StmtFunctionDef};
use ruff_python_parser::parse_module;
use ruff_text_size::Ranged;
use serde_json::{json, Value};

use super::lsp_backend::{read_frame, write_frame, LspServerConfig, LspSnapshotProvider};
use super::model::{
    AnalysisError, AnalysisSnapshot, DeclarationKind, SourceAnalysisRequest, SourceDeclaration,
    SourceLanguage,
};
use super::provider::SourceSnapshotProvider;

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
        } else if b == b'\'' {
            if let Some(advance) = rust_char_literal_len(&bytes[i..]) {
                i += advance;
                continue;
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

fn rust_char_literal_len(slice: &[u8]) -> Option<usize> {
    if slice.first() != Some(&b'\'') {
        return None;
    }
    if slice.get(1) == Some(&b'\\') {
        for (idx, b) in slice.iter().enumerate().skip(2).take(10) {
            if *b == b'\'' {
                return Some(idx + 1);
            }
        }
        None
    } else if slice.len() >= 3 && slice[2] == b'\'' {
        Some(3)
    } else {
        None
    }
}

fn has_unquoted_char(code: &str, target: char) -> bool {
    let mut in_str = false;
    let mut escaped = false;
    let bytes = code.as_bytes();
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
        } else if b == b'\'' {
            if let Some(advance) = rust_char_literal_len(&bytes[i..]) {
                i += advance;
                continue;
            }
        } else if b == b'"' {
            in_str = true;
        } else if char::from(b) == target {
            return true;
        }
        i += 1;
    }
    false
}

fn brace_delta(code: &str) -> i32 {
    let mut delta = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    let bytes = code.as_bytes();
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
        } else if b == b'\'' {
            if let Some(advance) = rust_char_literal_len(&bytes[i..]) {
                i += advance;
                continue;
            }
        } else {
            match b {
                b'"' => in_str = true,
                b'{' => delta += 1,
                b'}' => delta -= 1,
                _ => {}
            }
        }
        i += 1;
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

/// Summary of LSP-extracted symbols for a single source or test tree.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TreeSymbolSummary {
    pub files: usize,
    pub total_declarations: usize,
    pub api_functions_and_methods: usize,
    pub api_types_and_classes: usize,
    pub test_functions: usize,
}

/// One row comparing an upstream `sphinx` subsystem with its `sphinxdocrs` port.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SubsystemComparisonRow {
    pub subsystem: String,
    pub sphinx_api_files: usize,
    pub sphinx_api_callables: usize,
    pub sphinx_api_types: usize,
    pub sphinx_test_files: usize,
    pub sphinx_test_functions: usize,
    pub sphinxdocrs_api_files: usize,
    pub sphinxdocrs_api_callables: usize,
    pub sphinxdocrs_api_types: usize,
    pub sphinxdocrs_test_files: usize,
    pub sphinxdocrs_test_functions: usize,
}

/// Complete LSP-based comparison report for `sphinx` and `sphinxdocrs`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LspComparisonReport {
    pub sphinx_api: TreeSymbolSummary,
    pub sphinx_tests: TreeSymbolSummary,
    pub sphinxdocrs_src: TreeSymbolSummary,
    pub sphinxdocrs_integration_tests: TreeSymbolSummary,
    pub sphinxdocrs_py_tests: TreeSymbolSummary,
    pub subsystems: Vec<SubsystemComparisonRow>,
}

impl LspComparisonReport {
    pub fn total_sphinxdocrs_test_functions(&self) -> usize {
        self.sphinxdocrs_src.test_functions
            + self.sphinxdocrs_integration_tests.test_functions
            + self.sphinxdocrs_py_tests.test_functions
    }

    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Sphinx (`sphinxdoc`) vs. `sphinxdocrs` LSP API & Test Comparison Report\n\n");
        out.push_str(
            "Generated via `sphinxdocrs` LSP `textDocument/documentSymbol` analysis (`sphinx-source-status --compare-tests`).\n\n",
        );
        out.push_str("## 1. Aggregate Tree Summary\n\n");
        out.push_str("| Tree | Files | Total Symbols | API Callables (`fn`/`def`) | API Types (`class`/`struct`/`enum`/`trait`) | Test Functions |\n");
        out.push_str("| --- | ---: | ---: | ---: | ---: | ---: |\n");
        for (label, summary) in [
            ("`sphinx` API (`src/sphinx/sphinx`)", &self.sphinx_api),
            ("`sphinx` tests (`src/sphinx/tests`)", &self.sphinx_tests),
            ("`sphinxdocrs` crate (`src/sphinxdocrs/src`, incl. inline `#[cfg(test)]`)", &self.sphinxdocrs_src),
            ("`sphinxdocrs` Rust integration tests (`src/sphinxdocrs/tests`)", &self.sphinxdocrs_integration_tests),
            ("`sphinxdocrs` Python bridge tests (`src/tests/test_sphinx*`)", &self.sphinxdocrs_py_tests),
        ] {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                label,
                summary.files,
                summary.total_declarations,
                summary.api_functions_and_methods,
                summary.api_types_and_classes,
                summary.test_functions,
            ));
        }
        let rs_total_tests = self.total_sphinxdocrs_test_functions();
        let rs_total_test_files = self.sphinxdocrs_src.files
            + self.sphinxdocrs_integration_tests.files
            + self.sphinxdocrs_py_tests.files;
        out.push_str(&format!(
            "| **`sphinxdocrs` combined** | **{}** | **{}** | **{}** | **{}** | **{}** |\n\n",
            rs_total_test_files,
            self.sphinxdocrs_src.total_declarations
                + self.sphinxdocrs_integration_tests.total_declarations
                + self.sphinxdocrs_py_tests.total_declarations,
            self.sphinxdocrs_src.api_functions_and_methods,
            self.sphinxdocrs_src.api_types_and_classes,
            rs_total_tests,
        ));

        out.push_str("## 2. Per-Subsystem API & Test Function Breakdown\n\n");
        out.push_str("| Subsystem | `sphinx` API (`fn`/`cls`) | `sphinxdocrs` API (`fn`/`type`) | `sphinx` Test Files | `sphinx` Test Fns | `sphinxdocrs` Test Sources | `sphinxdocrs` Test Fns |\n");
        out.push_str("| --- | ---: | ---: | ---: | ---: | ---: | ---: |\n");
        for row in &self.subsystems {
            out.push_str(&format!(
                "| `{}` | {} / {} | {} / {} | {} | {} | {} | {} |\n",
                row.subsystem,
                row.sphinx_api_callables,
                row.sphinx_api_types,
                row.sphinxdocrs_api_callables,
                row.sphinxdocrs_api_types,
                row.sphinx_test_files,
                row.sphinx_test_functions,
                row.sphinxdocrs_test_files,
                row.sphinxdocrs_test_functions,
            ));
        }
        out
    }
}

/// Run an LSP `textDocument/documentSymbol` analysis over `selected_files`
/// inside `workspace_root` by spawning `server_executable --lsp-stdio <lang>`.
pub fn analyze_with_embedded_lsp_server(
    server_executable: &std::path::Path,
    language: SourceLanguage,
    workspace_root: &std::path::Path,
    selected_files: Vec<std::path::PathBuf>,
) -> Result<AnalysisSnapshot, AnalysisError> {
    let config = LspServerConfig {
        command: vec![
            server_executable.to_string_lossy().into_owned(),
            "--lsp-stdio".into(),
            language.to_string(),
        ],
        workspace_root: workspace_root.to_path_buf(),
        language,
        request_timeout: std::time::Duration::from_secs(120),
        allow_fallback: false,
        environment: Vec::new(),
        server_environment: Vec::new(),
        max_message_bytes: MAX_STDIO_BYTES,
    };
    let provider = LspSnapshotProvider::new_trusted_local(config);
    let mut request = SourceAnalysisRequest::new(workspace_root);
    request.selected = selected_files;
    provider.analyze(&request)
}

/// Summarize a slice of declarations for a given file set.
pub fn summarize_declarations(
    files: usize,
    declarations: &[&SourceDeclaration],
    treat_all_callables_as_non_api: bool,
) -> TreeSymbolSummary {
    let mut summary = TreeSymbolSummary {
        files,
        total_declarations: declarations.len(),
        ..Default::default()
    };
    for decl in declarations {
        let is_test = decl
            .attributes
            .get("lsp:is_test")
            .is_some_and(|v| v == "true");
        let in_test_mod = decl.qualified_name.contains("::tests::")
            || decl.qualified_name.starts_with("tests::");
        match decl.kind {
            DeclarationKind::Function | DeclarationKind::Method => {
                if is_test {
                    summary.test_functions += 1;
                } else if !treat_all_callables_as_non_api && !in_test_mod {
                    summary.api_functions_and_methods += 1;
                }
            }
            DeclarationKind::Class
            | DeclarationKind::Struct
            | DeclarationKind::Enum
            | DeclarationKind::Trait
            | DeclarationKind::TypeAlias
            | DeclarationKind::Union => {
                if !treat_all_callables_as_non_api && !in_test_mod {
                    summary.api_types_and_classes += 1;
                }
            }
            _ => {}
        }
    }
    summary
}

/// Build the full `LspComparisonReport` for `repo_root` using `server_executable`
/// over the LSP stdio protocol.
pub fn build_repository_comparison_report(
    server_executable: &std::path::Path,
    repo_root: &std::path::Path,
) -> Result<LspComparisonReport, AnalysisError> {
    let sphinx_api_root = repo_root.join("src/sphinx/sphinx");
    let sphinx_tests_root = repo_root.join("src/sphinx/tests");
    let rs_src_root = repo_root.join("src/sphinxdocrs/src");
    let rs_tests_root = repo_root.join("src/sphinxdocrs/tests");
    let py_bridge_tests_root = repo_root.join("src/tests");

    let sphinx_api_files = collect_matching_files(&sphinx_api_root, "py", &[])?;
    let sphinx_test_files = collect_matching_files(&sphinx_tests_root, "py", &["roots"])?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("test_"))
        })
        .collect::<Vec<_>>();
    let rs_src_files = collect_matching_files(&rs_src_root, "rs", &[])?;
    let rs_test_files = collect_matching_files(&rs_tests_root, "rs", &["fixtures"])?;
    let py_bridge_files = collect_matching_files(&py_bridge_tests_root, "py", &["__pycache__"])?
        .into_iter()
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.contains("sphinx"))
        })
        .collect::<Vec<_>>();

    let sphinx_api_snap = analyze_with_embedded_lsp_server(
        server_executable,
        SourceLanguage::Python,
        &sphinx_api_root,
        sphinx_api_files.clone(),
    )?;
    let sphinx_tests_snap = analyze_with_embedded_lsp_server(
        server_executable,
        SourceLanguage::Python,
        &sphinx_tests_root,
        sphinx_test_files.clone(),
    )?;
    let rs_src_snap = analyze_with_embedded_lsp_server(
        server_executable,
        SourceLanguage::Rust,
        &rs_src_root,
        rs_src_files.clone(),
    )?;
    let rs_tests_snap = analyze_with_embedded_lsp_server(
        server_executable,
        SourceLanguage::Rust,
        &rs_tests_root,
        rs_test_files.clone(),
    )?;
    let py_bridge_snap = analyze_with_embedded_lsp_server(
        server_executable,
        SourceLanguage::Python,
        &py_bridge_tests_root,
        py_bridge_files.clone(),
    )?;

    let sphinx_api_refs: Vec<_> = sphinx_api_snap.declarations.iter().collect();
    let sphinx_tests_refs: Vec<_> = sphinx_tests_snap.declarations.iter().collect();
    let rs_src_refs: Vec<_> = rs_src_snap.declarations.iter().collect();
    let rs_tests_refs: Vec<_> = rs_tests_snap.declarations.iter().collect();
    let py_bridge_refs: Vec<_> = py_bridge_snap.declarations.iter().collect();

    let sphinx_api = summarize_declarations(sphinx_api_files.len(), &sphinx_api_refs, false);
    let sphinx_tests = summarize_declarations(sphinx_test_files.len(), &sphinx_tests_refs, true);
    let sphinxdocrs_src = summarize_declarations(rs_src_files.len(), &rs_src_refs, false);
    let sphinxdocrs_integration_tests =
        summarize_declarations(rs_test_files.len(), &rs_tests_refs, true);
    let sphinxdocrs_py_tests =
        summarize_declarations(py_bridge_files.len(), &py_bridge_refs, true);

    let subsystems = build_subsystem_rows(
        &sphinx_api_snap.declarations,
        &sphinx_tests_snap.declarations,
        &rs_src_snap.declarations,
        &rs_tests_snap.declarations,
        &py_bridge_snap.declarations,
    );

    Ok(LspComparisonReport {
        sphinx_api,
        sphinx_tests,
        sphinxdocrs_src,
        sphinxdocrs_integration_tests,
        sphinxdocrs_py_tests,
        subsystems,
    })
}

fn collect_matching_files(
    root: &std::path::Path,
    ext: &str,
    excluded_dir_names: &[&str],
) -> Result<Vec<std::path::PathBuf>, AnalysisError> {
    let mut out = Vec::new();
    fn walk(
        dir: &std::path::Path,
        ext: &str,
        excluded_dir_names: &[&str],
        out: &mut Vec<std::path::PathBuf>,
    ) -> Result<(), AnalysisError> {
        let entries = std::fs::read_dir(dir).map_err(|error| AnalysisError::Io {
            path: dir.display().to_string(),
            message: error.to_string(),
        })?;
        for entry in entries {
            let entry = entry.map_err(|error| AnalysisError::Io {
                path: dir.display().to_string(),
                message: error.to_string(),
            })?;
            let path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if path.is_dir() {
                if excluded_dir_names.iter().any(|ex| *ex == name_str) {
                    continue;
                }
                walk(&path, ext, excluded_dir_names, out)?;
            } else if path.is_file() && path.extension().is_some_and(|e| e == ext) {
                out.push(path);
            }
        }
        Ok(())
    }
    if root.is_dir() {
        walk(root, ext, excluded_dir_names, &mut out)?;
    }
    out.sort();
    Ok(out)
}

struct SubsystemSpec {
    name: &'static str,
    sphinx_api_prefixes: &'static [&'static str],
    sphinx_test_prefixes: &'static [&'static str],
    rs_api_prefixes: &'static [&'static str],
    rs_test_prefixes: &'static [&'static str],
    py_bridge_prefixes: &'static [&'static str],
}

const SUBSYSTEM_SPECS: &[SubsystemSpec] = &[
    SubsystemSpec {
        name: "errors",
        sphinx_api_prefixes: &["errors.py"],
        sphinx_test_prefixes: &["test_errors.py"],
        rs_api_prefixes: &["errors.rs", "error_log.rs", "error_log_disabled.rs", "bin/sphinx_errors.rs"],
        rs_test_prefixes: &[],
        py_bridge_prefixes: &["test_sphinxdocrs_errors.py"],
    },
    SubsystemSpec {
        name: "events",
        sphinx_api_prefixes: &["events.py"],
        sphinx_test_prefixes: &["test_events.py"],
        rs_api_prefixes: &["events.rs", "app_events.rs"],
        rs_test_prefixes: &["events_app.rs"],
        py_bridge_prefixes: &["test_sphinxdocrs_events.py"],
    },
    SubsystemSpec {
        name: "project",
        sphinx_api_prefixes: &["project.py"],
        sphinx_test_prefixes: &["test_project.py"],
        rs_api_prefixes: &["project.rs"],
        rs_test_prefixes: &[],
        py_bridge_prefixes: &["test_sphinxdocrs_project.py", "test_sphinxdocrs_project_discover.py"],
    },
    SubsystemSpec {
        name: "addnodes",
        sphinx_api_prefixes: &["addnodes.py"],
        sphinx_test_prefixes: &["test_addnodes.py"],
        rs_api_prefixes: &["addnodes.rs"],
        rs_test_prefixes: &["addnodes.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "config",
        sphinx_api_prefixes: &["config.py"],
        sphinx_test_prefixes: &["test_config/"],
        rs_api_prefixes: &["config.rs"],
        rs_test_prefixes: &["config.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "extension & registry",
        sphinx_api_prefixes: &["extension.py", "registry.py"],
        sphinx_test_prefixes: &["test_extensions/"],
        rs_api_prefixes: &["extension.rs", "extensions/", "registry.rs", "app_facade.rs", "scan.rs"],
        rs_test_prefixes: &["registry.rs", "scan_requirements.rs"],
        py_bridge_prefixes: &["test_sphinxdocrs_extension.py", "test_sphinxdocrs_hybrid.py"],
    },
    SubsystemSpec {
        name: "versioning",
        sphinx_api_prefixes: &["versioning.py"],
        sphinx_test_prefixes: &["test_versioning.py"],
        rs_api_prefixes: &["versioning.rs"],
        rs_test_prefixes: &["versioning.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "util",
        sphinx_api_prefixes: &["util/"],
        sphinx_test_prefixes: &["test_util/"],
        rs_api_prefixes: &[
            "util_console.rs",
            "util_display.rs",
            "util_docstrings.rs",
            "util_lines.rs",
            "util_matching.rs",
            "util_md.rs",
            "util_osutil.rs",
            "util_rst.rs",
            "util_strypes.rs",
            "util_uri.rs",
            "assets.rs",
            "http_client.rs",
        ],
        rs_test_prefixes: &["util_rst_osutil.rs", "util_extra.rs", "assets.rs"],
        py_bridge_prefixes: &["test_sphinxdocrs_util_console.py", "test_sphinxdocrs_util_matching.py"],
    },
    SubsystemSpec {
        name: "locale & intl",
        sphinx_api_prefixes: &["locale/"],
        sphinx_test_prefixes: &["test_intl/"],
        rs_api_prefixes: &["locale.rs", "intl.rs"],
        rs_test_prefixes: &["locale.rs", "intl.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "quickstart (C1)",
        sphinx_api_prefixes: &["cmd/quickstart.py"],
        sphinx_test_prefixes: &["test_quickstart.py"],
        rs_api_prefixes: &["quickstart/", "bin/sphinx_quickstart.rs"],
        rs_test_prefixes: &["quickstart.rs", "quickstart_cli.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "build & cli (C2)",
        sphinx_api_prefixes: &["cmd/build.py", "cmd/make_mode.py", "_cli/"],
        sphinx_test_prefixes: &["test_command_line.py", "test__cli/"],
        rs_api_prefixes: &["build/", "cli/", "bin/sphinx_build.rs"],
        rs_test_prefixes: &["build.rs", "cli_bins.rs"],
        py_bridge_prefixes: &["cli_tests/test_sphinx_cli.py"],
    },
    SubsystemSpec {
        name: "apidoc (C3)",
        sphinx_api_prefixes: &["ext/apidoc/"],
        sphinx_test_prefixes: &["test_ext_apidoc/", "test_extensions/test_ext_apidoc.py"],
        rs_api_prefixes: &["apidoc/", "bin/sphinx_apidoc.rs"],
        rs_test_prefixes: &["apidoc.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "autosummary / autogen (C4)",
        sphinx_api_prefixes: &["ext/autosummary/"],
        sphinx_test_prefixes: &["test_ext_autosummary/"],
        rs_api_prefixes: &["autogen/", "bin/sphinx_autogen.rs"],
        rs_test_prefixes: &["autogen.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "application",
        sphinx_api_prefixes: &["application.py"],
        sphinx_test_prefixes: &["test_application.py"],
        rs_api_prefixes: &["application.rs"],
        rs_test_prefixes: &["application.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "environment & toctree",
        sphinx_api_prefixes: &["environment/"],
        sphinx_test_prefixes: &["test_environment/"],
        rs_api_prefixes: &["environment.rs", "toctree.rs", "genindex.rs"],
        rs_test_prefixes: &["environment.rs", "toctree.rs", "genindex.rs", "incremental.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "builders & writers",
        sphinx_api_prefixes: &["builders/", "writers/"],
        sphinx_test_prefixes: &["test_builders/", "test_writers/"],
        rs_api_prefixes: &["builders/"],
        rs_test_prefixes: &[
            "builders.rs",
            "builders_changes.rs",
            "builders_dirhtml.rs",
            "builders_epub.rs",
            "builders_gettext.rs",
            "builders_json.rs",
            "builders_pseudoxml.rs",
            "builders_singlehtml.rs",
            "builders_source_suffix.rs",
            "builders_texinfo.rs",
            "builders_text.rs",
            "builders_xml.rs",
            "linkcheck_wiremock.rs",
            "myst_bridge.rs",
            "otherdocs.rs",
            "parity.rs",
            "snapshot.rs",
        ],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "roles & directives",
        sphinx_api_prefixes: &["roles.py", "directives/"],
        sphinx_test_prefixes: &["test_roles.py", "test_directives/", "test_markup/"],
        rs_api_prefixes: &["roles.rs"],
        rs_test_prefixes: &["roles.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "domains & source_docs (LSP/H14)",
        sphinx_api_prefixes: &["domains/"],
        sphinx_test_prefixes: &["test_domains/"],
        rs_api_prefixes: &[
            "domains/",
            "source_analysis.rs",
            "source_analysis/",
            "source_docs.rs",
            "source_docs/",
            "bin/sphinx_source_port.rs",
        ],
        rs_test_prefixes: &[
            "domains_js.rs",
            "domains_py.rs",
            "domains_rst.rs",
            "domains_std.rs",
        ],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "theming",
        sphinx_api_prefixes: &["theming.py"],
        sphinx_test_prefixes: &["test_theming/"],
        rs_api_prefixes: &["theme.rs", "theme_render.rs", "theme_static.rs"],
        rs_test_prefixes: &[],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "search & stemming",
        sphinx_api_prefixes: &["search/"],
        sphinx_test_prefixes: &["test_search.py"],
        rs_api_prefixes: &["search.rs", "stemmer.rs"],
        rs_test_prefixes: &["search_objects.rs", "stemmer_parity.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "autodoc & pycode",
        sphinx_api_prefixes: &["ext/autodoc/", "pycode/"],
        sphinx_test_prefixes: &["test_ext_autodoc/", "test_pycode/"],
        rs_api_prefixes: &["autodoc.rs", "autodoc_runtime.rs", "bin/sphinx_autodoc.rs"],
        rs_test_prefixes: &["autodoc_cli.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "intersphinx",
        sphinx_api_prefixes: &["ext/intersphinx/"],
        sphinx_test_prefixes: &["test_ext_intersphinx/", "test_util/test_util_inventory.py"],
        rs_api_prefixes: &["intersphinx.rs"],
        rs_test_prefixes: &["intersphinx.rs"],
        py_bridge_prefixes: &[],
    },
    SubsystemSpec {
        name: "highlighting & transforms & other ext",
        sphinx_api_prefixes: &["highlighting.py", "transforms/", "ext/"],
        sphinx_test_prefixes: &[
            "test_highlighting.py",
            "test_transforms/",
            "test_ext_",
        ],
        rs_api_prefixes: &["lib.rs"],
        rs_test_prefixes: &[],
        py_bridge_prefixes: &[],
    },
];

fn matches_any_prefix(path: &str, prefixes: &[&str], exclude_prefixes: &[&str]) -> bool {
    if exclude_prefixes.iter().any(|ex| path == *ex || path.starts_with(ex)) {
        return false;
    }
    prefixes.iter().any(|prefix| path == *prefix || path.starts_with(prefix))
}

fn build_subsystem_rows(
    sphinx_api: &[SourceDeclaration],
    sphinx_tests: &[SourceDeclaration],
    rs_src: &[SourceDeclaration],
    rs_tests: &[SourceDeclaration],
    py_bridge: &[SourceDeclaration],
) -> Vec<SubsystemComparisonRow> {
    let specific_ext_api = &["ext/apidoc/", "ext/autosummary/", "ext/autodoc/", "ext/intersphinx/"];
    let specific_ext_tests = &[
        "test_ext_apidoc/",
        "test_ext_autosummary/",
        "test_ext_autodoc/",
        "test_ext_intersphinx/",
    ];

    let mut rows = Vec::with_capacity(SUBSYSTEM_SPECS.len());
    for spec in SUBSYSTEM_SPECS {
        let exclude_api: &[&str] = if spec.name.starts_with("highlighting") {
            specific_ext_api
        } else {
            &[]
        };
        let exclude_tests: &[&str] = if spec.name.starts_with("highlighting") {
            specific_ext_tests
        } else if spec.name == "extension & registry" {
            &["test_extensions/test_ext_apidoc.py"]
        } else {
            &[]
        };

        let sp_api: Vec<&SourceDeclaration> = sphinx_api
            .iter()
            .filter(|d| matches_any_prefix(&d.source.path, spec.sphinx_api_prefixes, exclude_api))
            .collect();
        let sp_tst: Vec<&SourceDeclaration> = sphinx_tests
            .iter()
            .filter(|d| matches_any_prefix(&d.source.path, spec.sphinx_test_prefixes, exclude_tests))
            .collect();
        let rs_api_decls: Vec<&SourceDeclaration> = rs_src
            .iter()
            .filter(|d| matches_any_prefix(&d.source.path, spec.rs_api_prefixes, &[]))
            .collect();
        let rs_int_decls: Vec<&SourceDeclaration> = rs_tests
            .iter()
            .filter(|d| matches_any_prefix(&d.source.path, spec.rs_test_prefixes, &[]))
            .collect();
        let py_br_decls: Vec<&SourceDeclaration> = py_bridge
            .iter()
            .filter(|d| matches_any_prefix(&d.source.path, spec.py_bridge_prefixes, &[]))
            .collect();

        let sp_api_files = unique_file_count(&sp_api);
        let sp_tst_files = unique_file_count(&sp_tst);
        let rs_api_files = unique_file_count(&rs_api_decls);

        let sp_api_sum = summarize_declarations(sp_api_files, &sp_api, false);
        let sp_tst_sum = summarize_declarations(sp_tst_files, &sp_tst, true);
        let rs_src_sum = summarize_declarations(rs_api_files, &rs_api_decls, false);
        let rs_int_sum = summarize_declarations(unique_file_count(&rs_int_decls), &rs_int_decls, true);
        let py_br_sum = summarize_declarations(unique_file_count(&py_br_decls), &py_br_decls, true);

        let inline_test_files = rs_api_decls
            .iter()
            .filter(|d| d.attributes.get("lsp:is_test").is_some_and(|v| v == "true"))
            .map(|d| d.source.path.as_str())
            .collect::<std::collections::BTreeSet<_>>()
            .len();

        rows.push(SubsystemComparisonRow {
            subsystem: spec.name.to_string(),
            sphinx_api_files: sp_api_files,
            sphinx_api_callables: sp_api_sum.api_functions_and_methods,
            sphinx_api_types: sp_api_sum.api_types_and_classes,
            sphinx_test_files: sp_tst_files,
            sphinx_test_functions: sp_tst_sum.test_functions,
            sphinxdocrs_api_files: rs_api_files,
            sphinxdocrs_api_callables: rs_src_sum.api_functions_and_methods,
            sphinxdocrs_api_types: rs_src_sum.api_types_and_classes,
            sphinxdocrs_test_files: inline_test_files + rs_int_sum.files + py_br_sum.files,
            sphinxdocrs_test_functions: rs_src_sum.test_functions
                + rs_int_sum.test_functions
                + py_br_sum.test_functions,
        });
    }
    rows
}

fn unique_file_count(decls: &[&SourceDeclaration]) -> usize {
    decls
        .iter()
        .map(|d| d.source.path.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len()
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

    #[test]
    fn summarize_declarations_and_subsystem_rows_distinguish_api_and_tests() {
        use super::super::model::{SourcePosition, SourceSpan, Visibility};

        let mut py_fn = SourceDeclaration::new(
            SourceLanguage::Python,
            "lsp",
            "build_all",
            ".",
            DeclarationKind::Function,
            Some("def build_all()".into()),
            None,
            "",
            Visibility::Unknown,
            SourceSpan::new(
                "builders/html.py",
                SourcePosition {
                    byte: 0,
                    line: 1,
                    column: 0,
                },
                None,
            ),
        );
        py_fn.attributes.insert("lsp:uri".into(), "file:///x".into());

        let mut py_test = SourceDeclaration::new(
            SourceLanguage::Python,
            "lsp",
            "test_build_html",
            ".",
            DeclarationKind::Function,
            Some("def test_build_html()".into()),
            None,
            "",
            Visibility::Unknown,
            SourceSpan::new(
                "test_builders/test_build_html.py",
                SourcePosition {
                    byte: 0,
                    line: 1,
                    column: 0,
                },
                None,
            ),
        );
        py_test.attributes.insert("lsp:is_test".into(), "true".into());

        let rs_type = SourceDeclaration::new(
            SourceLanguage::Rust,
            "lsp",
            "HtmlBuilder",
            "::",
            DeclarationKind::Struct,
            None,
            Some("pub struct HtmlBuilder".into()),
            "",
            Visibility::Unknown,
            SourceSpan::new(
                "builders/html.rs",
                SourcePosition {
                    byte: 0,
                    line: 1,
                    column: 0,
                },
                None,
            ),
        );
        let mut rs_test = SourceDeclaration::new(
            SourceLanguage::Rust,
            "lsp",
            "tests::test_html_builder",
            "::",
            DeclarationKind::Function,
            Some("fn test_html_builder()".into()),
            None,
            "",
            Visibility::Unknown,
            SourceSpan::new(
                "builders.rs",
                SourcePosition {
                    byte: 0,
                    line: 1,
                    column: 0,
                },
                None,
            ),
        );
        rs_test.attributes.insert("lsp:is_test".into(), "true".into());

        let rows = build_subsystem_rows(&[py_fn], &[py_test], &[rs_type], &[rs_test], &[]);
        let builders_row = rows
            .iter()
            .find(|r| r.subsystem == "builders & writers")
            .unwrap();
        assert_eq!(builders_row.sphinx_api_callables, 1);
        assert_eq!(builders_row.sphinx_test_functions, 1);
        assert_eq!(builders_row.sphinxdocrs_api_types, 1);
        assert_eq!(builders_row.sphinxdocrs_test_functions, 1);
    }
}
