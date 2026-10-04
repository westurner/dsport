//! Optional trusted-local LSP process adapter.
//!
//! This module is deliberately feature-gated. It is not a sandbox: callers
//! should use it only for trusted local language-server commands. Protected
//! modes remain unavailable until an audited OS sandbox provider exists.

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::config::SourceDocsSettings;
use super::model::{
    AnalysisDiagnostic, AnalysisError, AnalysisSnapshot, DeclarationKind, DiagnosticSeverity,
    SourceAnalysisRequest, SourceBackendKind, SourceDeclaration, SourceLanguage, SourcePosition,
    SourceSpan, Visibility, normalize_text,
};
use super::provider::SourceSnapshotProvider;

const MAX_HEADER_BYTES: usize = 8 * 1024;
const MAX_MESSAGE_BYTES: usize = 8 * 1024 * 1024;
const MAX_SOURCE_DOCUMENT_BYTES: usize = 1024 * 1024;
const MAX_REQUEST_TIMEOUT_MS: u64 = 5 * 60 * 1000;
const MAX_SESSION_DURATION: Duration = Duration::from_secs(30 * 60);
const MAX_SOURCE_FILES: usize = 2_048;
const MAX_DIAGNOSTICS: usize = 10_000;
const MAX_DIRECTORY_ENTRIES: usize = 100_000;
const MAX_HOVER_REQUESTS: usize = 256;
const MAX_HOVER_CONTENT_BYTES: usize = 64 * 1024;
const MAX_SERVER_INFO_FIELD_BYTES: usize = 256;
const CLIENT_NAME: &str = "sphinxdocrs";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct LspServerConfig {
    pub command: Vec<String>,
    pub workspace_root: PathBuf,
    pub language: SourceLanguage,
    pub request_timeout: Duration,
    pub allow_fallback: bool,
    pub environment: Vec<(String, String)>,
    pub max_message_bytes: usize,
}

trait SourceLspClient {
    fn request(&mut self, method: &str, params: Value) -> Result<Value, AnalysisError>;
    fn notify(&self, method: &str, params: Value) -> Result<(), AnalysisError>;
    fn take_diagnostics(
        &self,
        workspace_root: &Path,
        limit: usize,
        output: &mut Vec<AnalysisDiagnostic>,
    );
    fn supports_hover(&self) -> bool;
    fn shutdown(&mut self);
}

impl LspServerConfig {
    pub fn from_settings(
        settings: &SourceDocsSettings,
        language: SourceLanguage,
        source_root: &Path,
    ) -> Result<Self, AnalysisError> {
        let key = language.to_string();
        let command = settings
            .lsp_servers
            .get(&key)
            .cloned()
            .ok_or_else(|| unavailable(&key, "no server argv configured"))?;
        let workspace_root = settings
            .lsp_workspace_root
            .clone()
            .unwrap_or_else(|| source_root.to_path_buf());
        if !workspace_root.is_absolute() {
            return Err(AnalysisError::InvalidRequest(
                "LSP workspace root must be absolute".to_string(),
            ));
        }
        if command.iter().any(|part| part.contains('\0')) {
            return Err(AnalysisError::InvalidRequest(
                "LSP command arguments cannot contain NUL".to_string(),
            ));
        }
        if settings.lsp_timeout_ms == 0 || settings.lsp_timeout_ms > MAX_REQUEST_TIMEOUT_MS {
            return Err(AnalysisError::InvalidRequest(format!(
                "LSP timeout must be between 1 and {MAX_REQUEST_TIMEOUT_MS} milliseconds"
            )));
        }
        Ok(Self {
            command,
            workspace_root,
            language,
            request_timeout: Duration::from_millis(settings.lsp_timeout_ms),
            allow_fallback: settings.lsp_allow_fallback,
            environment: Vec::new(),
            max_message_bytes: MAX_MESSAGE_BYTES,
        })
    }
}

#[derive(Debug)]
enum WorkerRequest {
    Rpc {
        id: u64,
        method: String,
        params: Value,
        response: Sender<Result<Value, String>>,
    },
    Notify {
        method: String,
        params: Value,
    },
    Shutdown,
}

#[derive(Debug)]
struct RpcMessage {
    id: Option<u64>,
    method: Option<String>,
    result: Option<Value>,
    error: Option<Value>,
    params: Option<Value>,
}

#[cfg(windows)]
struct WindowsJob {
    handle: windows_sys::Win32::Foundation::HANDLE,
}

#[cfg(windows)]
impl WindowsJob {
    fn assign(child: &Child) -> std::io::Result<Self> {
        use std::os::windows::io::AsRawHandle;
        use std::ptr::null;
        use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
        use windows_sys::Win32::System::JobObjects::{
            AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
            JobObjectExtendedLimitInformation, SetInformationJobObject,
        };

        unsafe {
            let handle = CreateJobObjectW(null(), null());
            if handle.is_null() {
                return Err(std::io::Error::last_os_error());
            }
            let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            limits.BasicLimitInformation.LimitFlags =
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_ACTIVE_PROCESS;
            limits.BasicLimitInformation.ActiveProcessLimit = 64;
            let configured = SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &limits as *const _ as *const _,
                std::mem::size_of_val(&limits) as u32,
            );
            if configured == 0 {
                let error = std::io::Error::last_os_error();
                CloseHandle(handle);
                return Err(error);
            }
            let process: HANDLE = child.as_raw_handle() as HANDLE;
            if AssignProcessToJobObject(handle, process) == 0 {
                let error = std::io::Error::last_os_error();
                CloseHandle(handle);
                return Err(error);
            }
            Ok(Self { handle })
        }
    }
}

#[cfg(windows)]
impl Drop for WindowsJob {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::CloseHandle(self.handle);
        }
    }
}

struct LspProcess {
    child: Child,
    requests: Sender<WorkerRequest>,
    reader: Option<JoinHandle<()>>,
    writer: Option<JoinHandle<()>>,
    notifications: Receiver<RpcMessage>,
    stderr: Option<JoinHandle<String>>,
    stderr_tail: String,
    next_id: u64,
    config: LspServerConfig,
    terminated: bool,
    session_deadline: Instant,
    server_identity: String,
    hover_supported: bool,
    #[cfg(windows)]
    job: Option<WindowsJob>,
}

impl LspProcess {
    fn start(mut config: LspServerConfig) -> Result<Self, AnalysisError> {
        if !cfg!(any(unix, windows)) {
            return Err(unavailable(
                "platform",
                "LSP process-tree cleanup is unsupported on this platform",
            ));
        }
        if config.request_timeout.is_zero()
            || config.request_timeout > Duration::from_millis(MAX_REQUEST_TIMEOUT_MS)
        {
            return Err(AnalysisError::InvalidRequest(format!(
                "LSP timeout must not exceed {MAX_REQUEST_TIMEOUT_MS} milliseconds"
            )));
        }
        config.workspace_root =
            config
                .workspace_root
                .canonicalize()
                .map_err(|error| AnalysisError::Io {
                    path: config.workspace_root.display().to_string(),
                    message: error.to_string(),
                })?;
        let executable = config.command.first().ok_or_else(|| {
            AnalysisError::InvalidRequest("LSP command argv is empty".to_string())
        })?;
        let executable = resolve_executable(executable, &config.workspace_root)?;
        let mut command = Command::new(executable);
        command
            .args(&config.command[1..])
            .current_dir(&config.workspace_root)
            .env_clear()
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (key, value) in &config.environment {
            if key.starts_with("LC_") || key == "LANG" {
                command.env(key, value);
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::process::CommandExt;
            command.process_group(0);
        }
        let mut child = command
            .spawn()
            .map_err(|error| AnalysisError::BackendUnavailable {
                backend: "lsp".to_string(),
                message: format!("failed to start configured server: {error}"),
            })?;
        #[cfg(windows)]
        // Assignment is immediately after spawn; this guarantees cleanup of
        // descendants after assignment, but is not a sandbox boundary.
        let job = match WindowsJob::assign(&child) {
            Ok(job) => Some(job),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(AnalysisError::BackendUnavailable {
                    backend: "lsp-windows-job".into(),
                    message: format!("unable to establish child cleanup job: {error}"),
                });
            }
        };
        let stderr = child.stderr.take().map(|mut stderr| {
            thread::spawn(move || {
                let mut captured = Vec::with_capacity(8192);
                let mut buffer = [0u8; 4096];
                loop {
                    match stderr.read(&mut buffer) {
                        Ok(0) | Err(_) => break,
                        Ok(count) => {
                            let remaining = 8192usize.saturating_sub(captured.len());
                            captured.extend_from_slice(&buffer[..count.min(remaining)]);
                        }
                    }
                }
                String::from_utf8_lossy(&captured).into_owned()
            })
        });
        let writer = child
            .stdin
            .take()
            .ok_or_else(|| unavailable("lsp", "stdin unavailable"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| unavailable("lsp", "stdout unavailable"))?;
        let (requests, receiver) = mpsc::channel();
        let (responses, response_receiver) = mpsc::channel();
        let (notifications, notification_receiver) = mpsc::sync_channel(256);
        let reader = spawn_reader(stdout, responses, notifications, config.max_message_bytes);
        let writer_thread = spawn_writer(
            writer,
            receiver,
            response_receiver,
            config.max_message_bytes.min(MAX_MESSAGE_BYTES),
        );
        let mut process = Self {
            child,
            requests,
            reader: Some(reader),
            writer: Some(writer_thread),
            notifications: notification_receiver,
            stderr,
            stderr_tail: String::new(),
            next_id: 1,
            config,
            terminated: false,
            session_deadline: Instant::now() + MAX_SESSION_DURATION,
            server_identity: "unreported".into(),
            hover_supported: false,
            #[cfg(windows)]
            job,
        };
        let root_uri = file_uri(&process.config.workspace_root)?;
        let initialized = process.request(
            "initialize",
            json!({
                "processId": std::process::id(),
                "rootUri": root_uri,
                "workspaceFolders": [{ "uri": root_uri, "name": "workspace" }],
                "capabilities": {
                    "textDocument": {
                        "documentSymbol": { "hierarchicalDocumentSymbolSupport": true },
                        "hover": { "contentFormat": ["markdown", "plaintext"] }
                    }
                },
                "clientInfo": { "name": CLIENT_NAME, "version": CLIENT_VERSION }
            }),
        )?;
        if !initialized.is_object() {
            process.terminate();
            return Err(protocol_error("initialize returned a non-object result"));
        }
        process.server_identity = initialize_server_identity(&initialized);
        let capabilities = initialized.get("capabilities");
        process.hover_supported = capabilities
            .and_then(|capabilities| capabilities.get("hoverProvider"))
            .is_some_and(|capability| !capability.is_null() && capability != false);
        if !capabilities
            .and_then(|capabilities| capabilities.get("documentSymbolProvider"))
            .is_some_and(|capability| !capability.is_null() && capability != false)
        {
            process.terminate();
            return Err(unavailable(
                "server",
                "documentSymbolProvider is not advertised",
            ));
        }
        process.notify("initialized", json!({}))?;
        Ok(process)
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value, AnalysisError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let (response, receiver) = mpsc::channel();
        self.requests
            .send(WorkerRequest::Rpc {
                id,
                method: method.to_string(),
                params,
                response,
            })
            .map_err(|_| protocol_error("LSP writer stopped"))?;
        let deadline = (Instant::now() + self.config.request_timeout).min(self.session_deadline);
        loop {
            let now = Instant::now();
            if now >= deadline {
                self.terminate();
                return Err(AnalysisError::BackendUnavailable {
                    backend: "lsp".to_string(),
                    message: format!("request {method} timed out"),
                });
            }
            match receiver.recv_timeout((deadline - now).min(Duration::from_millis(25))) {
                Ok(Ok(value)) => return Ok(value),
                Ok(Err(error)) => return Err(protocol_error(&error)),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(protocol_error("LSP reader stopped"));
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if self.child.try_wait().ok().flatten().is_some() {
                        self.terminate();
                        return Err(AnalysisError::BackendUnavailable {
                            backend: "lsp".into(),
                            message: format!("server exited during {method}"),
                        });
                    }
                }
            }
        }
    }

    fn notify(&self, method: &str, params: Value) -> Result<(), AnalysisError> {
        self.requests
            .send(WorkerRequest::Notify {
                method: method.to_string(),
                params,
            })
            .map_err(|_| protocol_error("LSP writer stopped"))
    }

    fn take_diagnostics(
        &self,
        workspace_root: &Path,
        limit: usize,
        output: &mut Vec<AnalysisDiagnostic>,
    ) {
        let Ok(workspace_root) = workspace_root.canonicalize() else {
            return;
        };
        while let Ok(message) = self.notifications.try_recv() {
            if message.method.as_deref() != Some("textDocument/publishDiagnostics") {
                continue;
            }
            let params = message.params.unwrap_or(Value::Null);
            if let Some(items) = params.get("diagnostics").and_then(Value::as_array) {
                for item in items.iter().take(limit.saturating_sub(output.len())) {
                    let message = item
                        .get("message")
                        .and_then(Value::as_str)
                        .unwrap_or("language server diagnostic")
                        .to_string();
                    let diagnostic_path = params
                        .get("uri")
                        .and_then(Value::as_str)
                        .and_then(|uri| diagnostic_path_from_uri(uri, &workspace_root));
                    let source_text = diagnostic_path
                        .as_ref()
                        .and_then(|path| std::fs::read_to_string(path).ok());
                    let source = item.get("range").and_then(|range| {
                        let source_text = source_text.as_deref()?;
                        let (start, end) = positions_from_range(range, source_text)?;
                        Some(SourceSpan::new(diagnostic_path.as_ref()?, start, Some(end)))
                    });
                    output.push(AnalysisDiagnostic {
                        severity: match item.get("severity").and_then(Value::as_u64) {
                            Some(1) => DiagnosticSeverity::Error,
                            Some(3 | 4) => DiagnosticSeverity::Info,
                            _ => DiagnosticSeverity::Warning,
                        },
                        backend: "lsp".into(),
                        message,
                        source,
                        declaration: None,
                    });
                }
            }
        }
    }

    fn shutdown(&mut self) {
        let timeout = self.config.request_timeout.min(Duration::from_millis(500));
        let _ = self.request_with_timeout("shutdown", Value::Null, timeout);
        let _ = self.notify("exit", Value::Null);
        self.terminate();
    }

    fn request_with_timeout(
        &mut self,
        method: &str,
        params: Value,
        timeout: Duration,
    ) -> Result<Value, AnalysisError> {
        let original = self.config.request_timeout;
        self.config.request_timeout = timeout;
        let result = self.request(method, params);
        self.config.request_timeout = original;
        result
    }

    fn terminate(&mut self) {
        if self.terminated {
            return;
        }
        self.terminated = true;
        let _ = self.requests.send(WorkerRequest::Shutdown);
        #[cfg(unix)]
        unsafe {
            // Child::kill only targets the direct process. Signal the isolated
            // process group so language-server descendants are terminated too.
            libc::kill(-(self.child.id() as i32), libc::SIGTERM);
        }
        #[cfg(unix)]
        {
            let grace_deadline = Instant::now() + Duration::from_millis(150);
            let mut process_group_exists = true;
            while Instant::now() < grace_deadline {
                unsafe {
                    if libc::kill(-(self.child.id() as i32), 0) != 0
                        && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
                    {
                        process_group_exists = false;
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(10));
            }
            if process_group_exists {
                unsafe {
                    libc::kill(-(self.child.id() as i32), libc::SIGKILL);
                }
            }
        }
        #[cfg(windows)]
        {
            // Job handles are created with KILL_ON_JOB_CLOSE.
            drop(self.job.take());
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if let Some(writer) = self.writer.take() {
            let _ = writer.join();
        }
        if let Some(stderr) = self.stderr.take() {
            if let Ok(captured) = stderr.join() {
                self.stderr_tail = captured;
            }
        }
    }
}

impl Drop for LspProcess {
    fn drop(&mut self) {
        if !self.terminated {
            self.shutdown();
        }
    }
}

impl SourceLspClient for LspProcess {
    fn request(&mut self, method: &str, params: Value) -> Result<Value, AnalysisError> {
        LspProcess::request(self, method, params)
    }

    fn notify(&self, method: &str, params: Value) -> Result<(), AnalysisError> {
        LspProcess::notify(self, method, params)
    }

    fn take_diagnostics(
        &self,
        workspace_root: &Path,
        limit: usize,
        output: &mut Vec<AnalysisDiagnostic>,
    ) {
        LspProcess::take_diagnostics(self, workspace_root, limit, output)
    }

    fn supports_hover(&self) -> bool {
        self.hover_supported
    }

    fn shutdown(&mut self) {
        LspProcess::shutdown(self)
    }
}

fn spawn_writer(
    mut writer: ChildStdin,
    receiver: Receiver<WorkerRequest>,
    responses: Receiver<RpcMessage>,
    max_bytes: usize,
) -> JoinHandle<()> {
    let (writer_stop_tx, writer_stop_rx) = mpsc::channel::<()>();
    let worker = thread::spawn(move || {
        let mut pending = std::collections::HashMap::<u64, Sender<Result<Value, String>>>::new();
        loop {
            loop {
                match responses.try_recv() {
                    Ok(message) => {
                        let Some(id) = message.id else {
                            for (_, sender) in pending.drain() {
                                let _ = sender.send(Err("LSP response has no id".into()));
                            }
                            return;
                        };
                        let Some(sender) = pending.remove(&id) else {
                            for (_, sender) in pending.drain() {
                                let _ = sender.send(Err(format!("unknown LSP response id {id}")));
                            }
                            return;
                        };
                        let result = match (message.result, message.error) {
                            (Some(result), _) => Ok(result),
                            (_, Some(error)) => Err(error.to_string()),
                            _ => Err("response has neither result nor error".into()),
                        };
                        let _ = sender.send(result);
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) if pending.is_empty() => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        for (_, sender) in pending.drain() {
                            let _ = sender.send(Err("LSP process closed stdout".into()));
                        }
                        return;
                    }
                }
            }
            if writer_stop_rx.try_recv().is_ok() {
                break;
            }
            match receiver.recv_timeout(Duration::from_millis(10)) {
                Ok(WorkerRequest::Rpc {
                    id,
                    method,
                    params,
                    response,
                }) => {
                    pending.insert(id, response);
                    let body = json!({"jsonrpc":"2.0", "id":id, "method":method, "params":params});
                    if write_frame(&mut writer, &body, max_bytes).is_err() {
                        for (_, sender) in pending.drain() {
                            let _ = sender.send(Err("failed to write LSP request".into()));
                        }
                        break;
                    }
                }
                Ok(WorkerRequest::Notify { method, params }) => {
                    if write_frame(
                        &mut writer,
                        &json!({"jsonrpc":"2.0", "method":method, "params":params}),
                        max_bytes,
                    )
                    .is_err()
                    {
                        for (_, sender) in pending.drain() {
                            let _ = sender.send(Err("failed to write LSP notification".into()));
                        }
                        break;
                    }
                }
                Ok(WorkerRequest::Shutdown) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
        for (_, sender) in pending {
            let _ = sender.send(Err("LSP process stopped".into()));
        }
    });
    let _ = writer_stop_tx;
    worker
}

fn spawn_reader(
    stdout: ChildStdout,
    responses: Sender<RpcMessage>,
    notifications: mpsc::SyncSender<RpcMessage>,
    max_bytes: usize,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        while let Ok(Some(body)) = read_frame(&mut reader, max_bytes) {
            let Ok(parsed) = parse_rpc_message(&body) else {
                break;
            };
            let sent = if parsed.method.is_some() {
                match notifications.try_send(parsed) {
                    Ok(()) | Err(mpsc::TrySendError::Full(_)) => true,
                    Err(mpsc::TrySendError::Disconnected(_)) => false,
                }
            } else {
                responses.send(parsed).is_ok()
            };
            if !sent {
                break;
            }
        }
        drop(responses);
        drop(notifications);
    })
}

fn parse_rpc_message(body: &[u8]) -> Result<RpcMessage, ()> {
    let value = serde_json::from_slice::<Value>(body).map_err(|_| ())?;
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
        return Err(());
    }
    Ok(RpcMessage {
        id: value.get("id").and_then(Value::as_u64),
        method: value
            .get("method")
            .and_then(Value::as_str)
            .map(str::to_string),
        result: value.get("result").cloned(),
        error: value.get("error").cloned(),
        params: value.get("params").cloned(),
    })
}

fn write_frame(writer: &mut impl Write, body: &Value, max_bytes: usize) -> std::io::Result<()> {
    let bytes = serde_json::to_vec(body).map_err(std::io::Error::other)?;
    if bytes.len() > max_bytes.min(MAX_MESSAGE_BYTES) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "outgoing LSP message too large",
        ));
    }
    write!(writer, "Content-Length: {}\r\n\r\n", bytes.len())?;
    writer.write_all(&bytes)?;
    writer.flush()
}

fn read_frame(reader: &mut impl BufRead, max_bytes: usize) -> std::io::Result<Option<Vec<u8>>> {
    let mut length = None;
    let mut header_total = 0;
    loop {
        let remaining = MAX_HEADER_BYTES.saturating_sub(header_total);
        let mut line = Vec::with_capacity(remaining.min(256));
        let read = (&mut *reader)
            .take((remaining as u64).saturating_add(1))
            .read_until(b'\n', &mut line)?;
        if read == 0 {
            if header_total == 0 {
                return Ok(None);
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "incomplete LSP header",
            ));
        }
        header_total = header_total.saturating_add(read);
        if header_total > MAX_HEADER_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "LSP header too large",
            ));
        }
        if line == b"\r\n" || line == b"\n" {
            break;
        }
        let line = std::str::from_utf8(&line).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "non-UTF-8 LSP header")
        })?;
        if let Some((name, value)) = line.split_once(':') {
            if name.trim().eq_ignore_ascii_case("content-length") {
                if length.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "duplicate Content-Length",
                    ));
                }
                length = Some(value.trim().parse::<usize>().map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid Content-Length")
                })?);
            }
        }
    }
    let length = length.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "missing Content-Length")
    })?;
    if length > max_bytes.min(MAX_MESSAGE_BYTES) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "LSP message too large",
        ));
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(Some(body))
}

fn file_uri(path: &Path) -> Result<String, AnalysisError> {
    let canonical = path.canonicalize().map_err(|error| AnalysisError::Io {
        path: path.display().to_string(),
        message: error.to_string(),
    })?;
    let mut encoded = String::new();
    for byte in canonical.to_string_lossy().as_bytes() {
        if byte.is_ascii_alphanumeric() || b"/-._~".contains(byte) {
            encoded.push(char::from(*byte));
        } else {
            use std::fmt::Write as _;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    Ok(format!("file://{encoded}"))
}

fn local_path_from_file_uri(uri: &str) -> Option<PathBuf> {
    let encoded = uri.strip_prefix("file://")?;
    if !encoded.starts_with('/') {
        return None;
    }
    let bytes = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = *bytes.get(index + 1)?;
            let low = *bytes.get(index + 2)?;
            decoded.push((hex_value(high)? << 4) | hex_value(low)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    Some(PathBuf::from(String::from_utf8(decoded).ok()?))
}

fn diagnostic_path_from_uri(uri: &str, workspace_root: &Path) -> Option<PathBuf> {
    let workspace_root = workspace_root.canonicalize().ok()?;
    let path = local_path_from_file_uri(uri)?.canonicalize().ok()?;
    path.starts_with(workspace_root).then_some(path)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn resolve_executable(executable: &str, workspace_root: &Path) -> Result<PathBuf, AnalysisError> {
    let executable_path = Path::new(executable);
    let resolved = if executable_path.is_absolute() || executable_path.components().count() > 1 {
        let candidate = if executable_path.is_absolute() {
            executable_path.to_path_buf()
        } else {
            workspace_root.join(executable_path)
        };
        candidate.canonicalize()
    } else {
        let path = std::env::var_os("PATH").unwrap_or_default();
        let candidate = std::env::split_paths(&path)
            .filter(|directory| directory.is_absolute())
            .map(|directory| directory.join(executable_path))
            .find(|candidate| candidate.is_file())
            .ok_or_else(|| AnalysisError::BackendUnavailable {
                backend: "lsp".into(),
                message: "configured executable was not found on PATH".into(),
            })?;
        candidate.canonicalize()
    };
    resolved.map_err(|error| AnalysisError::BackendUnavailable {
        backend: "lsp".into(),
        message: format!("unable to resolve configured executable: {error}"),
    })
}

fn unavailable(language: &str, message: &str) -> AnalysisError {
    AnalysisError::BackendUnavailable {
        backend: format!("lsp-{language}"),
        message: message.to_string(),
    }
}

fn protocol_error(message: &str) -> AnalysisError {
    AnalysisError::Parse {
        backend: "lsp-json-rpc".to_string(),
        message: message.to_string(),
    }
}

fn initialize_server_identity(initialized: &Value) -> String {
    let Some(server_info) = initialized.get("serverInfo") else {
        return "unreported".into();
    };
    let Some(name) = server_info.get("name").and_then(Value::as_str) else {
        return "unreported".into();
    };
    let version = server_info
        .get("version")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if name.is_empty()
        || name.len() > MAX_SERVER_INFO_FIELD_BYTES
        || version.len() > MAX_SERVER_INFO_FIELD_BYTES
    {
        return "unreported".into();
    }
    let mut identity = Vec::with_capacity(name.len() + version.len() + 1);
    identity.extend_from_slice(name.as_bytes());
    identity.push(0);
    identity.extend_from_slice(version.as_bytes());
    format!("reported-{}", hash_bytes(&identity))
}

pub struct LspSnapshotProvider {
    config: LspServerConfig,
    process: Mutex<Option<LspProcess>>,
}

impl LspSnapshotProvider {
    pub(crate) fn new(config: LspServerConfig) -> Self {
        Self {
            config,
            process: Mutex::new(None),
        }
    }

    pub fn from_settings(
        settings: &SourceDocsSettings,
        language: SourceLanguage,
        source_root: &Path,
    ) -> Result<Self, AnalysisError> {
        if !matches!(
            settings.lsp_sandbox,
            super::config::SourceSandboxMode::TrustedLocal
        ) {
            return Err(unavailable(
                &language.to_string(),
                "set source_lsp_sandbox = 'trusted-local' to enable process startup",
            ));
        }
        Ok(Self::new(LspServerConfig::from_settings(
            settings,
            language,
            source_root,
        )?))
    }

    fn configuration_identity(&self, request: &SourceAnalysisRequest) -> String {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(request.cache_identity().as_bytes());
        bytes.extend_from_slice(self.config.language.to_string().as_bytes());
        bytes.extend_from_slice(self.config.workspace_root.to_string_lossy().as_bytes());
        bytes.extend_from_slice(&self.config.request_timeout.as_millis().to_le_bytes());
        bytes.extend_from_slice(&(self.config.max_message_bytes as u64).to_le_bytes());
        for argument in &self.config.command {
            bytes.extend_from_slice(argument.as_bytes());
            bytes.push(0);
        }
        for (key, value) in &self.config.environment {
            bytes.extend_from_slice(key.as_bytes());
            bytes.push(b'=');
            bytes.extend_from_slice(value.as_bytes());
            bytes.push(0);
        }
        hash_bytes(&bytes)
    }

    fn cache_identity_with_server(
        &self,
        request: &SourceAnalysisRequest,
        server_identity: &str,
    ) -> String {
        format!(
            "lsp:{}:server={server_identity}",
            self.configuration_identity(request)
        )
    }

    fn source_paths(&self, request: &SourceAnalysisRequest) -> Result<Vec<PathBuf>, AnalysisError> {
        if !request.source_root.is_absolute() {
            return Err(AnalysisError::InvalidRequest(
                "LSP source root must be absolute".into(),
            ));
        }
        let workspace_root =
            self.config
                .workspace_root
                .canonicalize()
                .map_err(|error| AnalysisError::Io {
                    path: self.config.workspace_root.display().to_string(),
                    message: error.to_string(),
                })?;
        let source_root =
            request
                .source_root
                .canonicalize()
                .map_err(|error| AnalysisError::Io {
                    path: request.source_root.display().to_string(),
                    message: error.to_string(),
                })?;
        if !source_root.starts_with(&workspace_root) {
            return Err(AnalysisError::InvalidRequest(format!(
                "LSP source root {} is outside workspace root {}",
                source_root.display(),
                workspace_root.display()
            )));
        }
        let mut paths = if request.selected.is_empty() {
            if source_root.is_file() {
                vec![source_root.clone()]
            } else {
                collect_source_files(&source_root, self.config.language)?
            }
        } else {
            request
                .selected
                .iter()
                .map(|path| {
                    if path.is_absolute() {
                        path.clone()
                    } else {
                        source_root.join(path)
                    }
                })
                .collect()
        };
        paths.retain(|path| match self.config.language {
            SourceLanguage::Rust => path.extension().is_some_and(|ext| ext == "rs"),
            SourceLanguage::Lean => path.extension().is_some_and(|ext| ext == "lean"),
            SourceLanguage::Python => false,
        });
        if paths.is_empty() && source_root.is_dir() {
            paths = collect_source_files(&source_root, self.config.language)?;
        }
        paths.sort();
        paths.dedup();
        if paths.len() > MAX_SOURCE_FILES {
            return Err(AnalysisError::InvalidRequest(format!(
                "LSP request selects {} files; limit is {MAX_SOURCE_FILES}",
                paths.len()
            )));
        }
        if paths.is_empty() {
            return Err(AnalysisError::InvalidRequest(
                "no supported source files selected for LSP".into(),
            ));
        }
        for path in &paths {
            let canonical = path.canonicalize().map_err(|error| AnalysisError::Io {
                path: path.display().to_string(),
                message: error.to_string(),
            })?;
            if !canonical.starts_with(&workspace_root) {
                return Err(AnalysisError::InvalidRequest(format!(
                    "LSP source file {} is outside workspace root {}",
                    canonical.display(),
                    workspace_root.display()
                )));
            }
            let metadata = canonical.metadata().map_err(|error| AnalysisError::Io {
                path: canonical.display().to_string(),
                message: error.to_string(),
            })?;
            if metadata.len() > MAX_SOURCE_DOCUMENT_BYTES as u64 {
                return Err(AnalysisError::InvalidRequest(format!(
                    "LSP source file {} exceeds the {MAX_SOURCE_DOCUMENT_BYTES}-byte document limit",
                    canonical.display()
                )));
            }
        }
        Ok(paths)
    }

    fn analyze_with_process(
        &self,
        process: &mut LspProcess,
        request: &SourceAnalysisRequest,
    ) -> Result<AnalysisSnapshot, AnalysisError> {
        let paths = self.source_paths(request)?;

        let mut declarations = Vec::new();
        let mut diagnostics = Vec::new();
        let mut truncated = false;
        let mut hover_requests = 0usize;
        let mut hover_limit_reached = false;
        for path in &paths {
            let remaining =
                MAX_DIAGNOSTICS.saturating_sub(declarations.len() + diagnostics.len() + 1);
            if remaining == 0 {
                truncated = true;
                break;
            }
            let hover_budget = MAX_HOVER_REQUESTS.saturating_sub(hover_requests);
            let (mut found, mut reported, file_truncated, file_hover_requests, file_hover_limit) =
                self.analyze_file(process, request, path, remaining, hover_budget)?;
            declarations.append(&mut found);
            diagnostics.append(&mut reported);
            hover_requests = hover_requests.saturating_add(file_hover_requests);
            hover_limit_reached |= file_hover_limit;
            if file_truncated {
                truncated = true;
                break;
            }
        }
        if truncated {
            diagnostics.push(AnalysisDiagnostic {
                severity: DiagnosticSeverity::Warning,
                backend: "lsp".into(),
                message: format!("LSP snapshot truncated at the {MAX_DIAGNOSTICS}-item limit"),
                source: None,
                declaration: None,
            });
        }
        if hover_limit_reached && declarations.len() + diagnostics.len() < MAX_DIAGNOSTICS {
            diagnostics.push(AnalysisDiagnostic {
                severity: DiagnosticSeverity::Info,
                backend: "lsp".into(),
                message: format!(
                    "hover enrichment stopped at the {MAX_HOVER_REQUESTS}-request limit"
                ),
                source: None,
                declaration: None,
            });
        }
        let source_hash = hash_paths(&paths)?;
        let mut snapshot = AnalysisSnapshot::new(
            self.config.language,
            "lsp",
            "json-rpc-2.0",
            &request.source_root,
            source_hash,
            declarations,
            diagnostics,
        );
        snapshot.backend_kind = SourceBackendKind::Lsp;
        snapshot.request_identity = request.cache_identity();
        snapshot.provider_identity =
            self.cache_identity_with_server(request, &process.server_identity);
        Ok(snapshot)
    }

    fn analyze_file(
        &self,
        process: &mut dyn SourceLspClient,
        request: &SourceAnalysisRequest,
        path: &Path,
        item_limit: usize,
        hover_limit: usize,
    ) -> Result<
        (
            Vec<SourceDeclaration>,
            Vec<AnalysisDiagnostic>,
            bool,
            usize,
            bool,
        ),
        AnalysisError,
    > {
        let uri = file_uri(path)?;
        let mut text = String::new();
        std::fs::File::open(path)
            .and_then(|file| {
                file.take((MAX_SOURCE_DOCUMENT_BYTES as u64) + 1)
                    .read_to_string(&mut text)
            })
            .map_err(|error| AnalysisError::Io {
                path: path.display().to_string(),
                message: error.to_string(),
            })?;
        if text.len() > MAX_SOURCE_DOCUMENT_BYTES {
            return Err(AnalysisError::InvalidRequest(format!(
                "LSP source file exceeds the {MAX_SOURCE_DOCUMENT_BYTES}-byte document limit"
            )));
        }
        let language_id = match self.config.language {
            SourceLanguage::Rust => "rust",
            SourceLanguage::Lean => "lean4",
            SourceLanguage::Python => "python",
        };
        process.notify(
            "textDocument/didOpen",
            json!({
                "textDocument": {"uri":uri, "languageId":language_id, "version":1, "text":text}
            }),
        )?;
        let symbols = process.request(
            "textDocument/documentSymbol",
            json!({"textDocument":{"uri":uri}}),
        )?;
        let mut declarations = Vec::new();
        let mut diagnostics = Vec::new();
        process.take_diagnostics(&self.config.workspace_root, item_limit, &mut diagnostics);
        let mut stack = Vec::new();
        let items = symbols
            .as_array()
            .ok_or_else(|| protocol_error("documentSymbol result is not an array"))?;
        for item in items {
            stack.push((item.clone(), None::<String>));
        }
        let mut truncated = false;
        let mut hover_requests = 0usize;
        let mut hover_limit_reached = false;
        let hover_supported = process.supports_hover();
        while let Some((item, parent)) = stack.pop() {
            if declarations.len() + diagnostics.len() >= item_limit {
                truncated = true;
                break;
            }
            let Some(name) = item.get("name").and_then(Value::as_str) else {
                diagnostics.push(unsupported_symbol(path, "symbol has no name"));
                continue;
            };
            let Some(range) = item.get("range") else {
                diagnostics.push(unsupported_symbol(path, "symbol has no range"));
                continue;
            };
            let Some((start, end)) = positions_from_range(range, &text) else {
                diagnostics.push(unsupported_symbol(path, "symbol range is malformed"));
                continue;
            };
            let separator = match self.config.language {
                SourceLanguage::Rust => "::",
                _ => ".",
            };
            let qualified_name = parent.as_ref().map_or_else(
                || name.to_string(),
                |parent| format!("{parent}{separator}{name}"),
            );
            let kind = item.get("kind").and_then(Value::as_u64).and_then(lsp_kind);
            let Some(kind) = kind else {
                diagnostics.push(unsupported_symbol(
                    path,
                    &format!("unsupported symbol kind for {qualified_name}"),
                ));
                continue;
            };
            let detail = item
                .get("detail")
                .and_then(Value::as_str)
                .map(normalize_text);
            let mut documentation = item
                .get("documentation")
                .and_then(|value| {
                    value
                        .as_str()
                        .or_else(|| value.get("value").and_then(Value::as_str))
                })
                .map(normalize_text)
                .unwrap_or_default();
            if documentation.is_empty() && hover_supported {
                if hover_requests >= hover_limit {
                    hover_limit_reached = true;
                } else {
                    let hover_position = item
                        .get("selectionRange")
                        .and_then(|selection_range| selection_range.get("start"))
                        .or_else(|| range.get("start"))
                        .cloned();
                    if let Some(position) = hover_position {
                        hover_requests += 1;
                        let hover = process.request(
                            "textDocument/hover",
                            json!({"textDocument":{"uri":uri}, "position":position}),
                        )?;
                        documentation = hover_documentation(&hover).unwrap_or_default();
                    }
                }
            }
            let (signature, type_text) = match kind {
                DeclarationKind::Function | DeclarationKind::Method => (detail.clone(), None),
                _ => (None, detail.clone()),
            };
            let mut declaration = SourceDeclaration::new(
                self.config.language,
                "lsp",
                &qualified_name,
                separator,
                kind,
                signature,
                type_text,
                documentation,
                Visibility::Unknown,
                SourceSpan::new(path, start, Some(end)),
            );
            declaration.attributes.insert("lsp:uri".into(), uri.clone());
            if let Some(selection_range) = item.get("selectionRange") {
                if let Some((selection_start, selection_end)) =
                    positions_from_range(selection_range, &text)
                {
                    declaration.attributes.insert(
                        "lsp:selection_start".into(),
                        format!("{}:{}", selection_start.line, selection_start.column),
                    );
                    declaration.attributes.insert(
                        "lsp:selection_end".into(),
                        format!("{}:{}", selection_end.line, selection_end.column),
                    );
                }
            }
            declarations.push(declaration.clone());
            let children = item
                .get("children")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            for child in children.into_iter().rev() {
                stack.push((child, Some(declaration.qualified_name.clone())));
            }
        }
        process.notify("textDocument/didClose", json!({"textDocument":{"uri":uri}}))?;
        let remaining = item_limit.saturating_sub(declarations.len() + diagnostics.len());
        process.take_diagnostics(&self.config.workspace_root, remaining, &mut diagnostics);
        truncated |= stack.len() > 0;
        let _ = request;
        Ok((
            declarations,
            diagnostics,
            truncated,
            hover_requests,
            hover_limit_reached,
        ))
    }
}

impl SourceSnapshotProvider for LspSnapshotProvider {
    fn analyze(&self, request: &SourceAnalysisRequest) -> Result<AnalysisSnapshot, AnalysisError> {
        self.source_paths(request)?;
        let mut slot = self
            .process
            .lock()
            .map_err(|_| protocol_error("LSP session lock poisoned"))?;
        let mut process = match slot.take() {
            Some(process) => process,
            None => LspProcess::start(self.config.clone())?,
        };
        match self.analyze_with_process(&mut process, request) {
            Ok(snapshot) => {
                *slot = Some(process);
                Ok(snapshot)
            }
            Err(error) => {
                SourceLspClient::shutdown(&mut process);
                Err(error)
            }
        }
    }

    fn backend_kind(&self) -> SourceBackendKind {
        SourceBackendKind::Lsp
    }

    fn cache_identity(&self, request: &SourceAnalysisRequest) -> String {
        let server_identity = self
            .process
            .lock()
            .ok()
            .and_then(|process| {
                process
                    .as_ref()
                    .map(|process| process.server_identity.clone())
            })
            .unwrap_or_else(|| "unreported".into());
        self.cache_identity_with_server(request, &server_identity)
    }

    fn source_hash(
        &self,
        request: &SourceAnalysisRequest,
        _language: SourceLanguage,
    ) -> Result<String, AnalysisError> {
        let paths = self.source_paths(request)?;
        hash_paths(&paths)
    }
}

fn hash_paths(paths: &[PathBuf]) -> Result<String, AnalysisError> {
    let mut bytes = Vec::new();
    for path in paths {
        bytes.extend(std::fs::read(path).map_err(|error| AnalysisError::Io {
            path: path.display().to_string(),
            message: error.to_string(),
        })?);
    }
    Ok(hash_bytes(&bytes))
}

fn collect_source_files(
    root: &Path,
    language: SourceLanguage,
) -> Result<Vec<PathBuf>, AnalysisError> {
    let mut files = Vec::new();
    let canonical_root = root.canonicalize().map_err(|error| AnalysisError::Io {
        path: root.display().to_string(),
        message: error.to_string(),
    })?;
    let mut pending = vec![canonical_root.clone()];
    let mut visited = HashSet::new();
    let mut entries_seen = 0usize;
    while let Some(directory) = pending.pop() {
        let canonical = directory
            .canonicalize()
            .map_err(|error| AnalysisError::Io {
                path: directory.display().to_string(),
                message: error.to_string(),
            })?;
        if !visited.insert(canonical.clone()) {
            continue;
        }
        let entries = std::fs::read_dir(&canonical).map_err(|error| AnalysisError::Io {
            path: canonical.display().to_string(),
            message: error.to_string(),
        })?;
        for entry in entries {
            entries_seen = entries_seen.saturating_add(1);
            if entries_seen > MAX_DIRECTORY_ENTRIES {
                return Err(AnalysisError::InvalidRequest(format!(
                    "LSP source tree exceeds the {MAX_DIRECTORY_ENTRIES}-entry scan limit"
                )));
            }
            let entry = entry.map_err(|error| AnalysisError::Io {
                path: canonical.display().to_string(),
                message: error.to_string(),
            })?;
            let file_type = entry.file_type().map_err(|error| AnalysisError::Io {
                path: entry.path().display().to_string(),
                message: error.to_string(),
            })?;
            let path = entry.path();
            let (candidate, target_type) = if file_type.is_symlink() {
                let Ok(target) = path.canonicalize() else {
                    continue;
                };
                if !target.starts_with(&canonical_root) {
                    continue;
                }
                let Ok(target_type) = target.metadata() else {
                    continue;
                };
                (target, target_type.file_type())
            } else {
                (path, file_type)
            };
            if target_type.is_dir() {
                pending.push(candidate);
            } else if target_type.is_file()
                && match language {
                    SourceLanguage::Rust => candidate.extension().is_some_and(|ext| ext == "rs"),
                    SourceLanguage::Lean => candidate.extension().is_some_and(|ext| ext == "lean"),
                    SourceLanguage::Python => false,
                }
            {
                files.push(candidate);
                if files.len() > MAX_SOURCE_FILES {
                    return Err(AnalysisError::InvalidRequest(format!(
                        "LSP source tree exceeds the {MAX_SOURCE_FILES}-file limit"
                    )));
                }
            }
        }
    }
    Ok(files)
}

fn hash_bytes(bytes: &[u8]) -> String {
    let hash = bytes.iter().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    });
    format!("{hash:016x}")
}

fn lsp_kind(kind: u64) -> Option<DeclarationKind> {
    Some(match kind {
        2 => DeclarationKind::Module,
        3 => DeclarationKind::Namespace,
        4 => DeclarationKind::Module,
        5 => DeclarationKind::Class,
        6 => DeclarationKind::Method,
        7 => DeclarationKind::Field,
        8 => DeclarationKind::Field,
        9 => DeclarationKind::Method,
        10 => DeclarationKind::Enum,
        11 => DeclarationKind::Trait,
        12 => DeclarationKind::Function,
        13 => DeclarationKind::Other("variable".to_string()),
        14 => DeclarationKind::Constant,
        22 => DeclarationKind::Variant,
        23 => DeclarationKind::Struct,
        26 => DeclarationKind::Other("type_parameter".into()),
        _ => return None,
    })
}

fn positions_from_range(range: &Value, text: &str) -> Option<(SourcePosition, SourcePosition)> {
    let start = position(range.get("start")?, text)?;
    let end = position(range.get("end")?, text)?;
    Some((start, end))
}

fn position(value: &Value, text: &str) -> Option<SourcePosition> {
    let line_index = usize::try_from(value.get("line")?.as_u64()?).ok()?;
    let utf16_character = usize::try_from(value.get("character")?.as_u64()?).ok()?;
    let line = text.lines().nth(line_index)?;
    let byte_column = utf16_column_to_byte(line, utf16_character)?;
    let byte = text
        .lines()
        .take(line_index)
        .map(|line| line.len() + 1)
        .sum::<usize>()
        + byte_column;
    Some(SourcePosition {
        byte,
        line: line_index + 1,
        column: byte_column,
    })
}

fn utf16_column_to_byte(line: &str, column: usize) -> Option<usize> {
    let mut units = 0;
    for (byte_index, character) in line.char_indices() {
        if units == column {
            return Some(byte_index);
        }
        units += character.len_utf16();
        if units > column {
            return None;
        }
    }
    (units == column).then_some(line.len())
}

fn hover_documentation(result: &Value) -> Option<String> {
    let contents = result.get("contents")?;
    let mut values = Vec::new();
    match contents {
        Value::String(value) => values.push(value.as_str()),
        Value::Object(object) => values.push(object.get("value")?.as_str()?),
        Value::Array(items) => {
            for item in items {
                match item {
                    Value::String(value) => values.push(value.as_str()),
                    Value::Object(object) => {
                        if let Some(value) = object.get("value").and_then(Value::as_str) {
                            values.push(value);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => return None,
    }
    if values.iter().map(|value| value.len()).sum::<usize>() > MAX_HOVER_CONTENT_BYTES {
        return None;
    }
    let normalized = normalize_text(&values.join("\n\n"));
    (!normalized.is_empty() && normalized.len() <= MAX_HOVER_CONTENT_BYTES).then_some(normalized)
}

fn unsupported_symbol(path: &Path, message: &str) -> AnalysisDiagnostic {
    AnalysisDiagnostic {
        severity: DiagnosticSeverity::Warning,
        backend: "lsp".into(),
        message: message.to_string(),
        source: Some(SourceSpan::new(
            path,
            SourcePosition {
                byte: 0,
                line: 1,
                column: 0,
            },
            None,
        )),
        declaration: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[cfg(unix)]
    #[test]
    fn fake_stdio_server_produces_normalized_snapshot_and_shuts_down() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() -> i32 { 42 }\n").unwrap();
        let fake_server =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        let config = LspServerConfig {
            command: vec![
                "python3".into(),
                fake_server.to_string_lossy().into_owned(),
                "symbols".into(),
                workspace
                    .path()
                    .join("starts")
                    .to_string_lossy()
                    .into_owned(),
            ],
            workspace_root: workspace.path().to_path_buf(),
            language: SourceLanguage::Rust,
            request_timeout: Duration::from_secs(10),
            allow_fallback: false,
            environment: vec![("PATH".into(), std::env::var("PATH").unwrap_or_default())],
            max_message_bytes: 1024 * 1024,
        };
        let provider = LspSnapshotProvider::new(config);
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source.clone());
        let identity_before_start = provider.cache_identity(&request);
        let snapshot = crate::source_analysis::provider_contract::assert_provider_contract(
            &provider, &request,
        );
        let identity_after_start = provider.cache_identity(&request);
        assert_ne!(identity_before_start, identity_after_start);
        assert_eq!(snapshot.request_identity, request.cache_identity());
        assert_eq!(snapshot.provider_identity, identity_after_start);
        assert!(!identity_after_start.contains("fake-lsp"));
        assert!(!identity_after_start.contains("fixture-1"));
        assert_eq!(snapshot.backend_kind, SourceBackendKind::Lsp);
        assert_eq!(snapshot.declarations.len(), 1);
        assert_eq!(snapshot.declarations[0].qualified_name, "answer");
        assert_eq!(snapshot.declarations[0].kind, DeclarationKind::Function);
        let rustdoc_json = workspace.path().join("rustdoc.json");
        std::fs::write(&rustdoc_json, "{}").unwrap();
        let mut hybrid_request = SourceAnalysisRequest::new(workspace.path());
        hybrid_request.selected.push(rustdoc_json);
        let hybrid_lsp = provider.analyze(&hybrid_request).unwrap();
        assert_eq!(hybrid_lsp.declarations.len(), 1);
        assert_eq!(
            std::fs::read_to_string(workspace.path().join("starts")).unwrap(),
            "1"
        );
    }

    #[cfg(unix)]
    #[test]
    fn fake_server_crash_during_request_returns_error() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() {}\n").unwrap();
        let mut config = fake_server_config(workspace.path(), "crash");
        config.request_timeout = Duration::from_secs(1);
        let provider = LspSnapshotProvider::new(config);
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);
        assert!(provider.analyze(&request).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn fake_server_exit_during_initialize_returns_error_without_timeout() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() {}\n").unwrap();
        let provider =
            LspSnapshotProvider::new(fake_server_config(workspace.path(), "initialize-crash"));
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);

        let error = provider.analyze(&request).unwrap_err().to_string();

        assert!(
            error.contains("process closed stdout")
                || error.contains("server exited during initialize"),
            "{error}"
        );
        assert!(!error.contains("timed out"), "{error}");
    }

    #[cfg(unix)]
    #[test]
    fn fake_server_publish_diagnostics_are_normalized_into_snapshot() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() -> i32 { 42 }\n").unwrap();
        let provider =
            LspSnapshotProvider::new(fake_server_config(workspace.path(), "diagnostics"));
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source.clone());

        let snapshot = provider.analyze(&request).unwrap();

        assert_eq!(snapshot.diagnostics.len(), 1);
        let diagnostic = &snapshot.diagnostics[0];
        assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
        assert_eq!(diagnostic.backend, "lsp");
        assert_eq!(diagnostic.message, "fixture diagnostic");
        let span = diagnostic.source.as_ref().unwrap();
        assert_eq!(span.path, source.canonicalize().unwrap().to_string_lossy());
        assert_eq!(span.start.line, 1);
        assert_eq!(span.start.column, 4);
        assert_eq!(span.end.unwrap().column, 6);
    }

    #[cfg(unix)]
    #[test]
    fn fake_server_hover_fills_missing_symbol_documentation() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() -> i32 { 42 }\n").unwrap();
        let provider = LspSnapshotProvider::new(fake_server_config(workspace.path(), "hover"));
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);

        let snapshot = provider.analyze(&request).unwrap();

        assert_eq!(snapshot.declarations.len(), 1);
        assert_eq!(snapshot.declarations[0].documentation, "Hover **answer**");
    }

    #[cfg(unix)]
    #[test]
    fn fake_server_timeout_terminates_child() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        std::fs::write(&source, "pub fn answer() {}\n").unwrap();
        let mut config = fake_server_config(workspace.path(), "timeout");
        config.request_timeout = Duration::from_millis(75);
        let provider = LspSnapshotProvider::new(config);
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);
        let error = provider.analyze(&request).unwrap_err();
        assert!(error.to_string().contains("timed out"));
    }

    #[cfg(unix)]
    #[test]
    fn timeout_kills_descendants_in_the_server_process_group() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("lib.rs");
        let marker = workspace.path().join("descendant-survived");
        std::fs::write(&source, "pub fn answer() {}\n").unwrap();
        let fake_server =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        let config = LspServerConfig {
            command: vec![
                "python3".into(),
                fake_server.to_string_lossy().into_owned(),
                "descendant-timeout".into(),
                marker.to_string_lossy().into_owned(),
            ],
            workspace_root: workspace.path().to_path_buf(),
            language: SourceLanguage::Rust,
            request_timeout: Duration::from_millis(100),
            allow_fallback: false,
            environment: Vec::new(),
            max_message_bytes: 1024 * 1024,
        };
        let provider = LspSnapshotProvider::new(config);
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);
        assert!(
            provider
                .analyze(&request)
                .unwrap_err()
                .to_string()
                .contains("timed out")
        );
        thread::sleep(Duration::from_millis(700));
        assert!(
            !marker.exists(),
            "server descendant survived timeout cleanup"
        );
    }

    #[cfg(unix)]
    fn fake_server_config(workspace: &Path, behavior: &str) -> LspServerConfig {
        let fake_server =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        LspServerConfig {
            command: vec![
                "python3".into(),
                fake_server.to_string_lossy().into_owned(),
                behavior.into(),
            ],
            workspace_root: workspace.to_path_buf(),
            language: SourceLanguage::Rust,
            request_timeout: Duration::from_secs(10),
            allow_fallback: false,
            environment: vec![("PATH".into(), std::env::var("PATH").unwrap_or_default())],
            max_message_bytes: 1024 * 1024,
        }
    }

    #[test]
    fn frame_reader_rejects_oversized_incomplete_and_duplicate_headers() {
        let mut oversized = &b"Content-Length: 999\r\n\r\nx"[..];
        assert!(read_frame(&mut oversized, 8).is_err());
        let mut missing = &b"X: y\r\n\r\nx"[..];
        assert!(read_frame(&mut missing, 8).is_err());
        let mut bad_length = &b"Content-Length: nope\r\n\r\nx"[..];
        assert!(read_frame(&mut bad_length, 8).is_err());
        let duplicate = &b"Content-Length: 1\r\nContent-Length: 1\r\n\r\nx"[..];
        assert!(read_frame(&mut &duplicate[..], 8).is_err());
        let unterminated = vec![b'a'; MAX_HEADER_BYTES + 1024];
        assert!(read_frame(&mut &unterminated[..], 8).is_err());
        let incomplete = &b"Content-Length: 1\r\n"[..];
        assert!(read_frame(&mut &incomplete[..], 8).is_err());
        let body = vec![b'x'; MAX_MESSAGE_BYTES + 1];
        let mut framed = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
        framed.extend_from_slice(&body);
        assert!(read_frame(&mut &framed[..], usize::MAX).is_err());
    }

    #[test]
    fn lsp_server_settings_reject_unbounded_timeouts() {
        let mut settings = SourceDocsSettings::default();
        settings
            .lsp_servers
            .insert("rust".into(), vec!["rust-analyzer".into()]);
        settings.lsp_timeout_ms = MAX_REQUEST_TIMEOUT_MS + 1;
        let error = LspServerConfig::from_settings(
            &settings,
            SourceLanguage::Rust,
            Path::new("/workspace"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("timeout must be between"));
    }

    #[test]
    fn hover_documentation_normalizes_supported_content_and_rejects_oversized_text() {
        assert_eq!(
            hover_documentation(&json!({
                "contents": {"kind": "markdown", "value": " **docs**\n"}
            }))
            .as_deref(),
            Some("**docs**")
        );
        assert_eq!(
            hover_documentation(&json!({
                "contents": ["summary", {"language": "rust", "value": "fn answer()"}]
            }))
            .as_deref(),
            Some("summary\n\nfn answer()")
        );
        assert!(hover_documentation(&json!({"contents": null})).is_none());
        let oversized = "x".repeat(MAX_HOVER_CONTENT_BYTES + 1);
        assert!(hover_documentation(&json!({"contents": oversized})).is_none());
    }

    #[test]
    fn rpc_reader_rejects_malformed_json_and_non_jsonrpc_messages() {
        assert!(parse_rpc_message(b"not json").is_err());
        assert!(parse_rpc_message(br#"{"method":"textDocument/publishDiagnostics"}"#).is_err());
        assert!(parse_rpc_message(br#"{"jsonrpc":"2.0","id":1,"result":null}"#).is_ok());
    }

    #[test]
    fn utf16_positions_convert_to_utf8_byte_columns() {
        let text = "a\u{1f600}b\nnext";
        let (start, end) = positions_from_range(
            &json!({"start":{"line":0,"character":3},"end":{"line":1,"character":4}}),
            text,
        )
        .unwrap();
        assert_eq!(start.column, 5);
        assert_eq!(end.line, 2);
        assert_eq!(end.column, 4);
    }

    #[test]
    fn document_symbols_map_nested_names_and_kinds() {
        assert_eq!(lsp_kind(5), Some(DeclarationKind::Class));
        assert_eq!(lsp_kind(12), Some(DeclarationKind::Function));
        assert_eq!(lsp_kind(999), None);
    }

    #[test]
    fn executable_resolution_reports_missing_command_without_shell_fallback() {
        let error = resolve_executable("sphinxdocrs-command-that-does-not-exist", Path::new("."))
            .unwrap_err();
        assert!(error.to_string().contains("configured executable"));
    }

    #[test]
    fn file_uri_round_trips_reserved_path_characters() {
        let workspace = TempDir::new().unwrap();
        let path = workspace.path().join("space # percent% unicode-é.rs");
        std::fs::write(&path, "").unwrap();
        let uri = file_uri(&path).unwrap();
        assert!(uri.contains("%20"));
        assert!(uri.contains("%23"));
        assert!(uri.contains("%25"));
        assert_eq!(
            local_path_from_file_uri(&uri).unwrap(),
            path.canonicalize().unwrap()
        );
    }

    #[test]
    fn diagnostic_file_uri_must_resolve_inside_workspace() {
        let workspace = TempDir::new().unwrap();
        let inside = workspace.path().join("inside.rs");
        std::fs::write(&inside, "").unwrap();
        let outside = TempDir::new().unwrap();
        let external = outside.path().join("external.rs");
        std::fs::write(&external, "").unwrap();
        assert!(diagnostic_path_from_uri(&file_uri(&inside).unwrap(), workspace.path()).is_some());
        assert!(
            diagnostic_path_from_uri(&file_uri(&external).unwrap(), workspace.path()).is_none()
        );
        assert!(local_path_from_file_uri("file:///tmp/bad%Q0path").is_none());
    }

    #[test]
    fn provider_identity_hashes_secret_arguments_without_exposing_them() {
        let root = TempDir::new().unwrap();
        let request = SourceAnalysisRequest::new(root.path());
        let mut config = fake_server_config(root.path(), "normal");
        config.command.push("token=do-not-print".into());
        let provider = LspSnapshotProvider::new(config);
        let identity = provider.cache_identity(&request);
        assert!(identity.starts_with("lsp:"));
        assert!(!identity.contains("do-not-print"));
    }

    #[test]
    fn initialize_server_info_is_optional_bounded_and_hashed() {
        let reported = initialize_server_identity(&json!({
            "serverInfo": {"name": "rust-analyzer", "version": "2026-10-04"}
        }));
        let changed_version = initialize_server_identity(&json!({
            "serverInfo": {"name": "rust-analyzer", "version": "next"}
        }));

        assert!(reported.starts_with("reported-"));
        assert!(!reported.contains("rust-analyzer"));
        assert!(!reported.contains("2026-10-04"));
        assert_ne!(reported, changed_version);
        assert_eq!(initialize_server_identity(&json!({})), "unreported");
        assert_eq!(
            initialize_server_identity(&json!({"serverInfo":{"name":""}})),
            "unreported"
        );
        let long_name = "x".repeat(MAX_SERVER_INFO_FIELD_BYTES + 1);
        assert_eq!(
            initialize_server_identity(&json!({"serverInfo":{"name":long_name}})),
            "unreported"
        );
    }

    #[cfg(unix)]
    #[test]
    fn path_outside_workspace_is_rejected_before_server_start() {
        let workspace = TempDir::new().unwrap();
        let outside = TempDir::new().unwrap();
        let source = outside.path().join("outside.rs");
        std::fs::write(&source, "pub fn outside() {}\n").unwrap();
        let counter = workspace.path().join("starts");
        let fake = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        let provider = LspSnapshotProvider::new(LspServerConfig {
            command: vec![
                "python3".into(),
                fake.to_string_lossy().into_owned(),
                "symbols".into(),
                counter.to_string_lossy().into_owned(),
            ],
            workspace_root: workspace.path().to_path_buf(),
            language: SourceLanguage::Rust,
            request_timeout: Duration::from_secs(1),
            allow_fallback: false,
            environment: Vec::new(),
            max_message_bytes: 1024 * 1024,
        });
        let mut request = SourceAnalysisRequest::new(outside.path());
        request.selected.push(source);
        let error = provider.analyze(&request).unwrap_err();
        assert!(error.to_string().contains("outside workspace root"));
        assert!(!counter.exists());
    }

    #[cfg(unix)]
    #[test]
    fn oversized_source_is_rejected_before_server_start() {
        let workspace = TempDir::new().unwrap();
        let source = workspace.path().join("large.rs");
        std::fs::write(&source, vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES + 1]).unwrap();
        let counter = workspace.path().join("starts");
        let fake = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/h14/fake_lsp.py");
        let provider = LspSnapshotProvider::new(LspServerConfig {
            command: vec![
                "python3".into(),
                fake.to_string_lossy().into_owned(),
                "symbols".into(),
                counter.to_string_lossy().into_owned(),
            ],
            workspace_root: workspace.path().to_path_buf(),
            language: SourceLanguage::Rust,
            request_timeout: Duration::from_secs(1),
            allow_fallback: false,
            environment: Vec::new(),
            max_message_bytes: 1024 * 1024,
        });
        let mut request = SourceAnalysisRequest::new(workspace.path());
        request.selected.push(source);
        let error = provider.analyze(&request).unwrap_err();
        assert!(error.to_string().contains("document limit"));
        assert!(!counter.exists());
    }

    #[cfg(unix)]
    #[test]
    fn source_discovery_allows_internal_symlinks_and_skips_external_cycles() {
        use std::os::unix::fs::symlink;

        let workspace = TempDir::new().unwrap();
        let source_dir = workspace.path().join("src");
        let real_dir = source_dir.join("real");
        std::fs::create_dir_all(&real_dir).unwrap();
        let source = real_dir.join("lib.rs");
        std::fs::write(&source, "pub fn answer() {}\n").unwrap();
        symlink(&real_dir, source_dir.join("alias")).unwrap();
        symlink(workspace.path(), source_dir.join("back-to-root")).unwrap();
        let outside = TempDir::new().unwrap();
        let external_dir = outside.path().join("external");
        std::fs::create_dir(&external_dir).unwrap();
        std::fs::write(external_dir.join("outside.rs"), "pub fn outside() {}\n").unwrap();
        symlink(&external_dir, source_dir.join("external")).unwrap();
        let provider = LspSnapshotProvider::new(fake_server_config(workspace.path(), "symbols"));
        let request = SourceAnalysisRequest::new(workspace.path());
        let paths = provider.source_paths(&request).unwrap();
        assert_eq!(paths, [source.canonicalize().unwrap()]);
    }
}

#[test]
fn outgoing_rpc_messages_obey_the_frame_limit() {
    let message = json!({"jsonrpc":"2.0", "method":"test", "params":"x".repeat(100)});
    let mut output = Vec::new();
    assert!(write_frame(&mut output, &message, 32).is_err());
    assert!(output.is_empty());
}
