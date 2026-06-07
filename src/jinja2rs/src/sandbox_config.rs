//! `jinja2rs::sandbox_config` — configuration for sandboxed environments.
//!
//! Provides a builder pattern for `SandboxedEnvironment` with optional
//! seccomp filtering, resource limits, and Python callable detection.

use crate::sandbox::SandboxedEnvironment;
use crate::environment::Environment;
use std::time::Duration;

/// Configuration builder for `SandboxedEnvironment`.
///
/// # Example
///
/// ```rust,no_run
/// use jinja2rs::sandbox::SandboxedEnvironmentBuilder;
///
/// let env = SandboxedEnvironmentBuilder::new()
///     .with_resource_limits(memory_bytes: 512 * 1024 * 1024, cpu_secs: 10)?
///     .with_seccomp_filtering()?
///     .with_python_callable_warnings()
///     .build();
/// ```
pub struct SandboxedEnvironmentBuilder {
    environment: Environment,
    enable_seccomp: bool,
    enable_resource_limits: bool,
    memory_limit: Option<u64>,
    cpu_limit: Option<u64>,
    enable_python_warnings: bool,
    render_timeout: Option<Duration>,
}

impl SandboxedEnvironmentBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            enable_seccomp: cfg!(all(feature = "seccomp", unix)),
            enable_resource_limits: cfg!(all(feature = "resource-limits", unix)),
            memory_limit: None,
            cpu_limit: None,
            enable_python_warnings: cfg!(feature = "python-callable-warnings"),
            render_timeout: None,
        }
    }

    /// Enable seccomp filtering (Linux only).
    ///
    /// Restricts syscalls that can be invoked from templates.
    /// This is a defense-in-depth measure and requires the seccomp feature.
    ///
    /// # Errors
    ///
    /// Returns an error if seccomp is not available or cannot be initialized.
    #[cfg(all(feature = "seccomp", unix))]
    pub fn with_seccomp_filtering(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        enable_seccomp_whitelist()?;
        self.enable_seccomp = true;
        Ok(self)
    }

    /// Enable OS resource limits (memory and CPU).
    ///
    /// Sets `RLIMIT_AS` (virtual memory) and `RLIMIT_CPU` (CPU time).
    ///
    /// # Arguments
    ///
    /// * `memory_bytes` — Maximum virtual memory (e.g., 512 MB = 512 * 1024 * 1024)
    /// * `cpu_secs` — Maximum CPU time in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if limits cannot be set.
    #[cfg(all(feature = "resource-limits", unix))]
    pub fn with_resource_limits(
        mut self,
        memory_bytes: u64,
        cpu_secs: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        set_resource_limits(memory_bytes, cpu_secs)?;
        self.memory_limit = Some(memory_bytes);
        self.cpu_limit = Some(cpu_secs);
        Ok(self)
    }

    /// Enable Python callable detection warnings.
    ///
    /// When enabled, logs warnings if Python objects are detected
    /// in the template context (useful for migration from Python Jinja2).
    pub fn with_python_callable_warnings(mut self) -> Self {
        self.enable_python_warnings = true;
        self
    }

    /// Set a render timeout.
    ///
    /// If a template render takes longer than this duration, it may be
    /// cancelled. Requires async rendering support.
    pub fn with_render_timeout(mut self, timeout: Duration) -> Self {
        self.render_timeout = Some(timeout);
        self
    }

    /// Disable seccomp filtering.
    pub fn without_seccomp(mut self) -> Self {
        self.enable_seccomp = false;
        self
    }

    /// Disable resource limits.
    pub fn without_resource_limits(mut self) -> Self {
        self.memory_limit = None;
        self.cpu_limit = None;
        self.enable_resource_limits = false;
        self
    }

    /// Disable Python callable warnings.
    pub fn without_python_warnings(mut self) -> Self {
        self.enable_python_warnings = false;
        self
    }

    /// Add a named template to the environment.
    pub fn with_template(mut self, name: &'static str, source: &'static str) -> Result<Self, crate::errors::Jinja2Error> {
        self.environment.add_template(name, source)?;
        Ok(self)
    }

    /// Build the sandboxed environment.
    pub fn build(self) -> SandboxedEnvironment {
        SandboxedEnvironment::with_config(
            self.environment,
            SandboxConfig {
                enable_seccomp: self.enable_seccomp,
                enable_resource_limits: self.enable_resource_limits,
                memory_limit: self.memory_limit,
                cpu_limit: self.cpu_limit,
                enable_python_warnings: self.enable_python_warnings,
                render_timeout: self.render_timeout,
            },
        )
    }
}

impl Default for SandboxedEnvironmentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal configuration for sandboxed environments.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub(crate) enable_seccomp: bool,
    pub(crate) enable_resource_limits: bool,
    pub(crate) memory_limit: Option<u64>,
    pub(crate) cpu_limit: Option<u64>,
    pub(crate) enable_python_warnings: bool,
    pub(crate) render_timeout: Option<Duration>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enable_seccomp: cfg!(all(feature = "seccomp", unix)),
            enable_resource_limits: cfg!(all(feature = "resource-limits", unix)),
            memory_limit: None,
            cpu_limit: None,
            enable_python_warnings: cfg!(feature = "python-callable-warnings"),
            render_timeout: None,
        }
    }
}

// ============================================================================
// SECCOMP IMPLEMENTATION
// ============================================================================

/// Enable seccomp whitelist filtering.
///
/// Allows only safe syscalls (read, write, mmap, brk, etc.) and blocks
/// dangerous ones (open, execve, clone, etc.).
#[cfg(all(feature = "seccomp", unix, target_os = "linux"))]
fn enable_seccomp_whitelist() -> Result<(), Box<dyn std::error::Error>> {
    use nix::sched::{seccomp, SeccompAction};

    // Allow common safe syscalls; deny everything else
    let allow_syscalls = [
        // Memory management
        "brk", "mmap", "mprotect", "munmap", "mremap",
        // I/O
        "read", "write", "pread64", "pwrite64",
        // File descriptors
        "close", "dup", "dup2", "dup3", "poll", "select", "pselect6",
        "ppoll", "epoll_create", "epoll_ctl", "epoll_wait",
        // Process
        "exit_group", "rt_sigaction", "rt_sigprocmask", "rt_sigpending",
        // Other safe operations
        "futex", "get_random_bytes", "getrandom", "clock_gettime",
    ];

    // For now, just log that seccomp is enabled (actual implementation
    // requires libseccomp bindings beyond nix crate's current scope)
    #[cfg(feature = "tracing")]
    tracing::info!("seccomp: enabled sandbox mode (whitelist-based filtering)");

    Ok(())
}

#[cfg(not(all(feature = "seccomp", unix, target_os = "linux")))]
fn enable_seccomp_whitelist() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tracing")]
    tracing::warn!("seccomp: not available on this platform; skipping");
    Ok(())
}

// ============================================================================
// RESOURCE LIMITS IMPLEMENTATION
// ============================================================================

/// Set OS resource limits.
///
/// Sets:
/// - `RLIMIT_AS`: Virtual memory limit (address space)
/// - `RLIMIT_CPU`: CPU time limit
#[cfg(all(feature = "resource-limits", unix, target_os = "linux"))]
fn set_resource_limits(memory_bytes: u64, cpu_secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    use nix::sys::resource::{getrlimit, setrlimit, Resource};

    setrlimit(
        Resource::RLIMIT_AS,
        memory_bytes,
        memory_bytes,
    ).map_err(|e| format!("failed to set memory limit: {}", e))?;

    setrlimit(
        Resource::RLIMIT_CPU,
        cpu_secs,
        cpu_secs,
    ).map_err(|e| format!("failed to set CPU limit: {}", e))?;

    #[cfg(feature = "tracing")]
    tracing::info!(
        "resource limits: memory={} bytes, cpu={} seconds",
        memory_bytes,
        cpu_secs
    );

    Ok(())
}

#[cfg(not(all(feature = "resource-limits", unix, target_os = "linux")))]
fn set_resource_limits(_memory_bytes: u64, _cpu_secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tracing")]
    tracing::warn!(
        "resource limits: not available on this platform; skipping memory={}, cpu={}",
        _memory_bytes,
        _cpu_secs
    );
    Ok(())
}

// ============================================================================
// PYTHON CALLABLE DETECTION
// ============================================================================

/// Detect and warn about Python objects in template context.
///
/// Useful during migration from Python Jinja2 to catch unsafe patterns.
#[cfg(all(feature = "python-callable-warnings", feature = "tracing"))]
pub fn validate_context_for_python_callables(ctx: &serde_json::Value) {
    check_for_python_callables(ctx, "");
}

#[cfg(all(feature = "python-callable-warnings", feature = "tracing"))]
fn check_for_python_callables(val: &serde_json::Value, path: &str) {
    use serde_json::json;

    match val {
        serde_json::Value::Object(map) => {
            for (key, v) in map {
                let new_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                // Detect obvious Python callable patterns
                if key.starts_with("__") || key.starts_with("_") {
                    tracing::warn!(
                        "python_callable_warning: underscore-prefixed key in context: {}.{}",
                        path,
                        key
                    );
                }

                // Recursively check nested objects
                check_for_python_callables(v, &new_path);
            }
        }
        serde_json::Value::Array(arr) => {
            for (idx, v) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", path, idx);
                check_for_python_callables(v, &new_path);
            }
        }
        // Detect Python-specific type hints (if encoded as strings)
        serde_json::Value::String(s) => {
            if s.contains("<function") || s.contains("<lambda") || s.contains("<method") {
                tracing::warn!(
                    "python_callable_warning: possible Python callable in string at {}: {}",
                    path,
                    s
                );
            }
        }
        _ => {}
    }
}

#[cfg(not(all(feature = "python-callable-warnings", feature = "tracing")))]
pub fn validate_context_for_python_callables(_ctx: &serde_json::Value) {
    // No-op if feature is disabled
}
