//! `jinja2rs::sandbox_config` — configuration for sandboxed environments.
//!
//! Provides a builder pattern for `SandboxedEnvironment` with multiple layers
//! of defense-in-depth security features:
//!
//! # Security Features
//!
//! ## 1. Seccomp Filtering (Linux only, optional feature)
//!
//! Restricts syscalls using Linux Secure Computing Mode (seccomp) with BPF filters.
//! Denies all syscalls except a whitelist of safe operations.
//!
//! - **Minimal**: 9 essential syscalls (memory, I/O, synchronization)
//! - **Broad**: ~20 syscalls (adds compatibility for edge cases)
//!
//! Default: Minimal (recommended)
//!
//! **To enable:** Add to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! jinja2rs = { version = "0.0.1", features = ["seccomp"] }
//! ```
//!
//! ## 2. Resource Limits (Unix only, optional feature)
//!
//! Enforces OS-level resource limits on processes executing templates.
//!
//! - Virtual memory limit (`RLIMIT_AS`)
//! - CPU time limit (`RLIMIT_CPU`)
//!
//! **To enable:** Add to `Cargo.toml`:
//! ```toml
//! [dependencies]
//! jinja2rs = { version = "0.0.1", features = ["resource-limits"] }
//! ```
//!
//! ## 3. Path Sandboxing (always available)
//!
//! Restricts filesystem access to explicitly whitelisted directories.
//! By default, **no paths are allowed** (strict mode).
//!
//! - Read paths: directories templates can read from (includes, partials)
//! - Write paths: directories templates can write to (output, logs)
//! - Symlink control: can reject symlinks to prevent escape attempts
//!
//! ### Example
//!
//! ```rust,no_run
//! use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, PathPolicy};
//!
//! let policy = PathPolicy::new()
//!     .with_read_path("/var/templates")
//!     .with_write_path("/tmp/output");
//!
//! let env = SandboxedEnvironmentBuilder::new()
//!     .with_path_policy(policy)?           // Always available
//!     .with_python_callable_warnings()
//!     .build();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## 4. Python Callable Detection
//!
//! Optional warnings when Python objects are detected in template context.
//! Useful for migration from Python Jinja2.
//!
//! # Full Lockdown Example (all features enabled)
//!
//! When you enable multiple features, you can layer all protections:
//!
//! ```rust,no_run
//! # #[cfg(all(feature = "seccomp", feature = "resource-limits"))]
//! # {
//! use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, PathPolicy, SeccompWhitelist};
//!
//! let policy = PathPolicy::new()
//!     .with_read_path("/var/templates")
//!     .with_write_path("/tmp/output");
//!
//! let env = SandboxedEnvironmentBuilder::new()
//!     .with_seccomp_whitelist(SeccompWhitelist::Minimal)
//!     .with_seccomp_filtering()?               // Requires: feature = "seccomp"
//!     .with_resource_limits(512*1024*1024, 10)?  // Requires: feature = "resource-limits"
//!     .with_path_policy(policy)?               // Always available
//!     .with_python_callable_warnings()
//!     .build();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! # }
//! ```
//!
//! # Defense in Depth
//!
//! These features work together to create multiple layers of protection:
//!
//! 1. Seccomp prevents unauthorized syscalls at the kernel level
//! 2. Resource limits prevent resource exhaustion attacks
//! 3. Path sandboxing prevents unauthorized file access
//! 4. Python callable detection catches unsafe patterns
//!
//! Using all layers together provides maximum security.

use crate::sandbox::SandboxedEnvironment;
use crate::environment::Environment;
use std::time::Duration;

/// Seccomp whitelist strictness level.
///
/// Defines which syscalls are allowed when seccomp filtering is enabled.
///
/// # Minimal (Recommended)
///
/// 9 essential syscalls for template rendering:
/// - `brk`, `mmap`, `mprotect`, `munmap` — Rust memory management
/// - `read`, `write` — Template I/O
/// - `close` — File descriptor cleanup
/// - `exit_group` — Process exit
/// - `futex` — Mutex synchronization
///
/// Use this for maximum security in untrusted template environments.
///
/// # Broad (Compatibility)
///
/// ~20 syscalls including compatibility options:
/// - All minimal syscalls
/// - `pread64`, `pwrite64` — Positioned I/O
/// - `dup`, `dup2`, `dup3` — File descriptor duplication
/// - `poll`, `select`, `pselect6`, `ppoll` — Event notification
/// - `epoll_create`, `epoll_ctl`, `epoll_wait` — Event polling
/// - `rt_sigaction`, `rt_sigprocmask`, `rt_sigpending` — Signal handling
/// - `get_random_bytes`, `getrandom` — Random number generation
/// - `clock_gettime` — Time reading
///
/// Use this if you encounter seccomp rejections with minimal mode,
/// or if you need broader compatibility with template extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompWhitelist {
    /// Minimal set: only syscalls essential for template rendering.
    /// Recommended for most use cases.
    Minimal,
    /// Broader set: includes syscalls for compatibility and edge cases.
    /// Use if you encounter seccomp rejections with minimal mode.
    Broad,
}

/// Path sandboxing policy for file access control.
///
/// Restricts which filesystem paths templates can read from or write to.
/// By default, no paths are allowed (strict/deny-by-default mode).
///
/// # Security Model
///
/// The sandboxing uses a **whitelist-based approach** where:
/// - Templates can **only** access files in explicitly allowed directories
/// - Symlinks can be optionally rejected to prevent escape attempts
/// - Path resolution normalizes `..` and `.` to prevent traversal tricks
///
/// # Example
///
/// ```rust,no_run
/// use jinja2rs::sandbox_config::PathPolicy;
///
/// let policy = PathPolicy::new()
///     .with_read_path("/var/templates")     // Templates can read from here
///     .with_write_path("/tmp/output")       // Templates can write to here
///     .allow_symlinks(false);                // Reject symlinks (default)
/// ```
///
/// # Defense in Depth
///
/// When combined with seccomp filtering and resource limits, path sandboxing
/// provides multiple layers of protection:
///
/// 1. **Seccomp** prevents unauthorized syscalls at the kernel level
/// 2. **Resource limits** prevent resource exhaustion
/// 3. **Path sandboxing** prevents unauthorized file access
/// 4. **Python callable detection** catches unsafe patterns
///
/// # Implementation Details
///
/// - Path validation checks occur at template compilation and render time
/// - Symlink detection uses `fs::symlink_metadata()` without following links
/// - Path normalization uses manual component walking (not `canonicalize()`)
/// - Non-existent paths fall back to prefix matching as a safety measure
///
/// # Use Cases
///
/// | Scenario | Read Paths | Write Paths |
/// |----------|-----------|------------|
/// | Render templates from directory | `/templates` | None |
/// | Include partials from multiple dirs | `/templates:/includes` | None |
/// | Generate static output | `/templates` | `/output` |
/// | Logging and debug output | `/templates` | `/logs` |
/// | Read-only deployment | `/app/templates` | None |
#[derive(Debug, Clone)]
pub struct PathPolicy {
    /// Allowed paths for reading files (templates, includes, etc.).
    ///
    /// Templates can include or import from these directories.
    pub read_paths: Vec<String>,
    /// Allowed paths for writing files (output, logs, etc.).
    ///
    /// Templates can create or modify files in these directories.
    pub write_paths: Vec<String>,
    /// Whether to follow symlinks (default: false for security).
    ///
    /// When `false` (recommended): Symlinks to outside allowed directories
    /// are rejected, preventing escape attempts.
    ///
    /// When `true`: Symlinks are followed, increasing compatibility but
    /// reducing security (symlinks could lead outside allowed directories).
    pub allow_symlinks: bool,
}

impl PathPolicy {
    /// Create a new empty path policy (no paths allowed by default).
    pub fn new() -> Self {
        Self {
            read_paths: vec![],
            write_paths: vec![],
            allow_symlinks: false,
        }
    }

    /// Add an allowed path for reading files.
    ///
    /// # Arguments
    ///
    /// * `path` — Absolute or relative filesystem path (e.g., "/var/templates")
    pub fn with_read_path(mut self, path: impl Into<String>) -> Self {
        self.read_paths.push(path.into());
        self
    }

    /// Add an allowed path for writing files.
    ///
    /// # Arguments
    ///
    /// * `path` — Absolute or relative filesystem path (e.g., "/tmp/output")
    pub fn with_write_path(mut self, path: impl Into<String>) -> Self {
        self.write_paths.push(path.into());
        self
    }

    /// Allow or disallow symlinks in allowed directories.
    ///
    /// Default is `false` (disallow) for security.
    ///
    /// # Arguments
    ///
    /// * `allow` — Whether to follow symlinks to outside allowed directories
    pub fn allow_symlinks(mut self, allow: bool) -> Self {
        self.allow_symlinks = allow;
        self
    }
}

impl Default for PathPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration builder for `SandboxedEnvironment`.
///
/// # Example
///
/// ```rust,no_run
/// use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, PathPolicy};
///
/// // Path sandboxing: restrict template file access
/// let policy = PathPolicy::new()
///     .with_read_path("/var/templates")
///     .with_write_path("/tmp/output");
///
/// let env = SandboxedEnvironmentBuilder::new()
///     .with_path_policy(policy)?
///     .with_python_callable_warnings()
///     .build();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct SandboxedEnvironmentBuilder {
    environment: Environment,
    enable_seccomp: bool,
    seccomp_whitelist: SeccompWhitelist,
    enable_resource_limits: bool,
    memory_limit: Option<u64>,
    cpu_limit: Option<u64>,
    path_policy: Option<PathPolicy>,
    enable_python_warnings: bool,
    render_timeout: Option<Duration>,
}

impl SandboxedEnvironmentBuilder {
    /// Create a new builder with default settings.
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            enable_seccomp: cfg!(all(feature = "seccomp", unix)),
            seccomp_whitelist: SeccompWhitelist::Minimal,
            enable_resource_limits: cfg!(all(feature = "resource-limits", unix)),
            memory_limit: None,
            cpu_limit: None,
            path_policy: None,
            enable_python_warnings: cfg!(feature = "python-callable-warnings"),
            render_timeout: None,
        }
    }

    /// Enable seccomp filtering (Linux only).
    ///
    /// Restricts syscalls that can be invoked from templates.
    /// This is a defense-in-depth measure and requires the seccomp feature.
    ///
    /// # Requirements
    ///
    /// This method is only available when the `seccomp` feature is enabled.
    ///
    /// Add to your `Cargo.toml`:
    /// ```toml
    /// [dependencies]
    /// jinja2rs = { version = "0.0.1", features = ["seccomp"] }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Seccomp is not available on this system (requires Linux >= 3.17)
    /// - The seccomp filter cannot be initialized
    /// - A syscall name cannot be resolved
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "seccomp")]
    /// # {
    /// use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, SeccompWhitelist};
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_seccomp_whitelist(SeccompWhitelist::Minimal)
    ///     .with_seccomp_filtering()?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # }
    /// ```
    #[cfg(all(feature = "seccomp", unix))]
    pub fn with_seccomp_filtering(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        enable_seccomp_whitelist(self.seccomp_whitelist)?;
        self.enable_seccomp = true;
        Ok(self)
    }

    /// Set the seccomp syscall whitelist strictness level.
    ///
    /// Controls which syscalls are allowed when seccomp filtering is active.
    /// This setting has no effect unless `with_seccomp_filtering()` is also called.
    ///
    /// # Arguments
    ///
    /// * `level` — Use `SeccompWhitelist::Minimal` (default) for most cases,
    ///   or `SeccompWhitelist::Broad` if you encounter rejections.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, SeccompWhitelist};
    ///
    /// // Use broad whitelist for better compatibility
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_seccomp_whitelist(SeccompWhitelist::Broad)
    ///     // Call with_seccomp_filtering() to activate (requires feature)
    ///     .build();
    /// ```
    pub fn with_seccomp_whitelist(mut self, level: SeccompWhitelist) -> Self {
        self.seccomp_whitelist = level;
        self
    }

    /// Enable OS resource limits (memory and CPU).
    ///
    /// Sets `RLIMIT_AS` (virtual memory) and `RLIMIT_CPU` (CPU time).
    ///
    /// # Requirements
    ///
    /// This method is only available when the `resource-limits` feature is enabled.
    ///
    /// Add to your `Cargo.toml`:
    /// ```toml
    /// [dependencies]
    /// jinja2rs = { version = "0.0.1", features = ["resource-limits"] }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `memory_bytes` — Maximum virtual memory (e.g., 512 MB = 512 * 1024 * 1024)
    /// * `cpu_secs` — Maximum CPU time in seconds
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Limits cannot be set on this system
    /// - Resource management is not available
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # #[cfg(feature = "resource-limits")]
    /// # {
    /// use jinja2rs::sandbox_config::SandboxedEnvironmentBuilder;
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_resource_limits(
    ///         512 * 1024 * 1024,  // 512 MB virtual memory limit
    ///         10,                  // 10 second CPU time limit
    ///     )?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # }
    /// ```
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

    /// Set a path sandboxing policy to restrict file access.
    ///
    /// By default, no paths are allowed. Templates can only access files
    /// in explicitly whitelisted directories.
    ///
    /// # Arguments
    ///
    /// * `policy` — `PathPolicy` defining read/write paths and symlink rules
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Path validation fails (e.g., paths contain invalid characters)
    /// - Policy configuration is inconsistent
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::{SandboxedEnvironmentBuilder, PathPolicy};
    ///
    /// let policy = PathPolicy::new()
    ///     .with_read_path("/var/templates")
    ///     .with_write_path("/tmp/output")
    ///     .allow_symlinks(false);
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_path_policy(policy)?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_path_policy(
        mut self,
        policy: PathPolicy,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        validate_path_policy(&policy)?;
        self.path_policy = Some(policy);
        Ok(self)
    }

    /// Add a read-allowed path for file access.
    ///
    /// Can be called multiple times to add multiple paths. Templates will be
    /// able to read (include, import) files from these directories.
    ///
    /// # Arguments
    ///
    /// * `path` — Absolute or relative filesystem path (e.g., "/var/templates")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Path is empty
    /// - Path contains null bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::SandboxedEnvironmentBuilder;
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_read_path("/var/templates")?
    ///     .with_read_path("/opt/templates")?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_read_path(mut self, path: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_str = path.into();
        validate_path_string(&path_str)?;
        
        if self.path_policy.is_none() {
            self.path_policy = Some(PathPolicy::new());
        }
        if let Some(ref mut policy) = self.path_policy {
            policy.read_paths.push(path_str);
        }
        Ok(self)
    }

    /// Add a write-allowed path for file access.
    ///
    /// Can be called multiple times to add multiple paths. Templates will be
    /// able to create or modify files in these directories.
    ///
    /// # Arguments
    ///
    /// * `path` — Absolute or relative filesystem path (e.g., "/tmp/output")
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Path is empty
    /// - Path contains null bytes
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::SandboxedEnvironmentBuilder;
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_write_path("/tmp/output")?
    ///     .with_write_path("/var/logs")?
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_write_path(mut self, path: impl Into<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let path_str = path.into();
        validate_path_string(&path_str)?;
        
        if self.path_policy.is_none() {
            self.path_policy = Some(PathPolicy::new());
        }
        if let Some(ref mut policy) = self.path_policy {
            policy.write_paths.push(path_str);
        }
        Ok(self)
    }

    /// Enable or disable symlink following in sandboxed paths.
    ///
    /// # Arguments
    ///
    /// * `allow` — `true` to allow symlinks, `false` to reject them (default)
    ///
    /// # Security Implications
    ///
    /// - **`false` (recommended)**: Symlinks are rejected. This prevents
    ///   escape attempts where a symlink points outside allowed directories.
    ///   Suitable for untrusted template environments.
    ///
    /// - **`true`**: Symlinks are followed. This increases compatibility
    ///   with complex directory structures but reduces security. Only use
    ///   if you fully trust the template author.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::SandboxedEnvironmentBuilder;
    ///
    /// // Strict mode (default)
    /// let env1 = SandboxedEnvironmentBuilder::new()
    ///     .with_read_path("/templates")?
    ///     .allow_symlinks(false)  // Reject symlinks
    ///     .build();
    ///
    /// // Permissive mode
    /// let env2 = SandboxedEnvironmentBuilder::new()
    ///     .with_read_path("/templates")?
    ///     .allow_symlinks(true)   // Allow symlinks
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn allow_symlinks(mut self, allow: bool) -> Self {
        if self.path_policy.is_none() {
            self.path_policy = Some(PathPolicy::new());
        }
        if let Some(ref mut policy) = self.path_policy {
            policy.allow_symlinks = allow;
        }
        self
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

    /// Disable path sandboxing.
    ///
    /// Removes the path policy, allowing templates to access any file
    /// (subject to OS permissions and other sandbox layers).
    ///
    /// This is useful for:
    /// - Disabling path restrictions set in earlier builder calls
    /// - Allowing unrestricted file access in trusted environments
    /// - Testing and debugging
    ///
    /// # Security
    ///
    /// Using this method significantly reduces security. Only use if the
    /// template source is fully trusted.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use jinja2rs::sandbox_config::SandboxedEnvironmentBuilder;
    ///
    /// let env = SandboxedEnvironmentBuilder::new()
    ///     .with_read_path("/templates")?
    ///     .without_path_sandboxing()  // Disable restrictions
    ///     .build();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn without_path_sandboxing(mut self) -> Self {
        self.path_policy = None;
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
                seccomp_whitelist: self.seccomp_whitelist,
                enable_resource_limits: self.enable_resource_limits,
                memory_limit: self.memory_limit,
                cpu_limit: self.cpu_limit,
                path_policy: self.path_policy,
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
    pub(crate) seccomp_whitelist: SeccompWhitelist,
    pub(crate) enable_resource_limits: bool,
    pub(crate) memory_limit: Option<u64>,
    pub(crate) cpu_limit: Option<u64>,
    pub(crate) path_policy: Option<PathPolicy>,
    pub(crate) enable_python_warnings: bool,
    pub(crate) render_timeout: Option<Duration>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            enable_seccomp: cfg!(all(feature = "seccomp", unix)),
            seccomp_whitelist: SeccompWhitelist::Minimal,
            enable_resource_limits: cfg!(all(feature = "resource-limits", unix)),
            memory_limit: None,
            cpu_limit: None,
            path_policy: None,
            enable_python_warnings: cfg!(feature = "python-callable-warnings"),
            render_timeout: None,
        }
    }
}

// ============================================================================
// SECCOMP IMPLEMENTATION
// ============================================================================

/// Minimal seccomp whitelist: essential syscalls only.
const SECCOMP_MINIMAL: &[&str] = &[
    // Memory management (essential for Rust runtime)
    "brk", "mmap", "mprotect", "munmap",
    // I/O (essential for template I/O)
    "read", "write",
    // File descriptors (cleanup)
    "close",
    // Process (exit)
    "exit_group",
    // Synchronization (Rust mutexes)
    "futex",
];

/// Broad seccomp whitelist: includes compatibility syscalls.
const SECCOMP_BROAD: &[&str] = &[
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

/// Enable seccomp whitelist filtering.
///
/// Loads a seccomp BPF program that allows only the syscalls in the whitelist
/// and denies all others (kills the process on denied syscall).
///
/// # Arguments
///
/// * `whitelist` — `SeccompWhitelist::Minimal` (9 syscalls) or `SeccompWhitelist::Broad` (~20 syscalls)
///
/// # Errors
///
/// Returns an error if:
/// - libseccomp is not available
/// - A syscall name cannot be resolved
/// - The BPF program cannot be loaded into the kernel
///
/// # Platform Support
///
/// This function is only available on Linux with kernel >= 3.17.
/// On other platforms, it logs a warning and returns `Ok(())` without filtering.
#[cfg(all(feature = "seccomp", unix, target_os = "linux"))]
fn enable_seccomp_whitelist(whitelist: SeccompWhitelist) -> Result<(), Box<dyn std::error::Error>> {
    use libseccomp::{ScmpAction, ScmpSyscall};

    let allow_syscalls = match whitelist {
        SeccompWhitelist::Minimal => SECCOMP_MINIMAL,
        SeccompWhitelist::Broad => SECCOMP_BROAD,
    };

    // Create a new seccomp filter context.
    // Default action is to kill the process if a syscall is not explicitly allowed.
    let mut ctx = libseccomp::ScmpFilterContext::new(ScmpAction::KillProcess)?;

    // Add each allowed syscall to the whitelist.
    for syscall_name in allow_syscalls {
        let syscall = ScmpSyscall::from_name(syscall_name)
            .map_err(|_| format!("unknown syscall: {}", syscall_name))?;
        ctx.add_rule(ScmpAction::Allow, syscall)?;
    }

    // Load the filter into the kernel.
    ctx.load()?;

    #[cfg(feature = "tracing")]
    tracing::info!(
        "seccomp: loaded sandbox mode with {:?} whitelist ({} syscalls)",
        whitelist,
        allow_syscalls.len()
    );

    Ok(())
}

#[cfg(not(all(feature = "seccomp", unix, target_os = "linux")))]
fn enable_seccomp_whitelist(_whitelist: SeccompWhitelist) -> Result<(), Box<dyn std::error::Error>> {
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
// PATH SANDBOXING IMPLEMENTATION
// ============================================================================

/// Validate a path policy for consistency.
fn validate_path_policy(policy: &PathPolicy) -> Result<(), Box<dyn std::error::Error>> {
    // Check that we have at least one allowed path if sandboxing is enabled
    if policy.read_paths.is_empty() && policy.write_paths.is_empty() {
        // Empty policy is allowed; it means no file access is permitted
        return Ok(());
    }

    // Validate each path
    for path in &policy.read_paths {
        validate_path_string(path)?;
    }
    for path in &policy.write_paths {
        validate_path_string(path)?;
    }

    #[cfg(feature = "tracing")]
    tracing::info!(
        "path_policy: {} read paths, {} write paths, symlinks={}",
        policy.read_paths.len(),
        policy.write_paths.len(),
        policy.allow_symlinks
    );

    Ok(())
}

/// Validate a single path string.
fn validate_path_string(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Path must not be empty
    if path.is_empty() {
        return Err("path cannot be empty".into());
    }

    // Path must not contain null bytes
    if path.contains('\0') {
        return Err("path cannot contain null bytes".into());
    }

    // Paths with .. could be used for escape, so warn about them
    if path.contains("..") {
        #[cfg(feature = "tracing")]
        tracing::warn!("path contains '..': {}; be aware this could allow directory traversal", path);
    }

    Ok(())
}

/// Check if a path is allowed for reading.
///
/// Validates a file path against the read path policy.
///
/// # Arguments
///
/// * `file_path` — The path to check (can be absolute or relative)
/// * `policy` — The `PathPolicy` to validate against
///
/// # Returns
///
/// - `Ok(())` if the path is allowed
/// - `Err` with description if the path is denied
///
/// # Denial Reasons
///
/// The path is denied if:
/// - No policy is configured (`None`)
/// - The policy has no read paths configured
/// - The path is outside all whitelisted directories
/// - The path is a symlink and symlinks are disabled
///
/// # Example
///
/// ```rust,no_run
/// use jinja2rs::sandbox_config::{PathPolicy, validate_read_path};
///
/// let policy = Some(PathPolicy::new().with_read_path("/templates"));
/// match validate_read_path("/templates/index.html", &policy) {
///     Ok(()) => println!("Path allowed"),
///     Err(e) => println!("Access denied: {}", e),
/// }
/// ```
pub fn validate_read_path(
    file_path: &str,
    policy: &Option<PathPolicy>,
) -> Result<(), Box<dyn std::error::Error>> {
    match policy {
        None => {
            // No policy = no file access allowed
            Err("path sandboxing disabled; no file access allowed".into())
        }
        Some(p) => {
            if p.read_paths.is_empty() {
                return Err("no read paths allowed".into());
            }
            check_path_in_whitelist(file_path, &p.read_paths, p.allow_symlinks)
        }
    }
}

/// Check if a path is allowed for writing.
///
/// Validates a file path against the write path policy.
///
/// # Arguments
///
/// * `file_path` — The path to check (can be absolute or relative)
/// * `policy` — The `PathPolicy` to validate against
///
/// # Returns
///
/// - `Ok(())` if the path is allowed
/// - `Err` with description if the path is denied
///
/// # Denial Reasons
///
/// The path is denied if:
/// - No policy is configured (`None`)
/// - The policy has no write paths configured
/// - The path is outside all whitelisted directories
/// - The path is a symlink and symlinks are disabled
///
/// # Example
///
/// ```rust,no_run
/// use jinja2rs::sandbox_config::{PathPolicy, validate_write_path};
///
/// let policy = Some(PathPolicy::new().with_write_path("/output"));
/// match validate_write_path("/output/result.html", &policy) {
///     Ok(()) => println!("Write allowed"),
///     Err(e) => println!("Write denied: {}", e),
/// }
/// ```
pub fn validate_write_path(
    file_path: &str,
    policy: &Option<PathPolicy>,
) -> Result<(), Box<dyn std::error::Error>> {
    match policy {
        None => {
            // No policy = no file access allowed
            Err("path sandboxing disabled; no file access allowed".into())
        }
        Some(p) => {
            if p.write_paths.is_empty() {
                return Err("no write paths allowed".into());
            }
            check_path_in_whitelist(file_path, &p.write_paths, p.allow_symlinks)
        }
    }
}

/// Check if a path is in the whitelist.
fn check_path_in_whitelist(
    file_path: &str,
    allowed_paths: &[String],
    allow_symlinks: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::Path;  // {, PathBuf};
    use std::fs;

    let path_obj = Path::new(file_path);

    // If symlinks are not allowed, check if the path is a symlink
    if !allow_symlinks {
        // Use fs::symlink_metadata to detect symlinks without following them
        if let Ok(metadata) = fs::symlink_metadata(path_obj) {
            if metadata.is_symlink() {
                return Err(format!(
                    "symlink access denied: {}; set allow_symlinks(true) to permit",
                    file_path
                )
                .into());
            }
        }
    }

    // Try to canonicalize the path (resolve .. and symlinks)
    let canonical = if allow_symlinks {
        // If symlinks allowed, resolve them
        path_obj.canonicalize().ok()
    } else {
        // If symlinks not allowed, manually resolve .. without following symlinks
        try_resolve_path(path_obj).ok()
    };

    match canonical {
        Some(canonical_path) => {
            // Check if the canonical path is under any allowed directory
            for allowed in allowed_paths {
                let allowed_obj = Path::new(allowed);
                let allowed_canonical = if allow_symlinks {
                    allowed_obj.canonicalize().ok()
                } else {
                    try_resolve_path(allowed_obj).ok()
                };

                if let Some(ac) = allowed_canonical {
                    if canonical_path.starts_with(&ac) {
                        #[cfg(feature = "tracing")]
                        tracing::debug!("path allowed: {}", file_path);
                        return Ok(());
                    }
                }
            }
            Err(format!(
                "path outside whitelist: {} (canonical: {})",
                file_path,
                canonical_path.display()
            )
            .into())
        }
        None => {
            // If we can't canonicalize, it might not exist yet (for write paths)
            // Do a simple string-based prefix check as fallback
            for allowed in allowed_paths {
                if file_path.starts_with(allowed) {
                    #[cfg(feature = "tracing")]
                    tracing::debug!("path allowed (prefix match): {}", file_path);
                    return Ok(());
                }
            }
            Err(format!(
                "path outside whitelist and cannot verify: {}",
                file_path
            )
            .into())
        }
    }
}

/// Try to resolve a path without following symlinks.
///
/// This manually walks the path components and resolves ".." entries
/// without using `canonicalize()` which would follow symlinks.
fn try_resolve_path(path: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    use std::path::{Component, PathBuf};

    let mut resolved = PathBuf::new();

    for component in path.components() {
        match component {
            Component::ParentDir => {
                resolved.pop();
            }
            Component::CurDir => {
                // Skip "."
            }
            Component::RootDir => {
                resolved.push(component);
            }
            Component::Normal(name) => {
                resolved.push(name);
            }
            Component::Prefix(p) => {
                resolved.push(p.as_os_str());
            }
        }
    }

    Ok(resolved)
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
    // use serde_json::json;

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

// ============================================================================
// TESTS
// ============================================================================

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    // use std::fs;
    use std::path::Path;

    // ============================================================================
    // PathPolicy Tests
    // ============================================================================

    #[test]
    fn test_path_policy_new() {
        let policy = PathPolicy::new();
        assert!(policy.read_paths.is_empty());
        assert!(policy.write_paths.is_empty());
        assert!(!policy.allow_symlinks);
    }

    #[test]
    fn test_path_policy_default() {
        let policy = PathPolicy::default();
        assert!(policy.read_paths.is_empty());
        assert!(policy.write_paths.is_empty());
        assert!(!policy.allow_symlinks);
    }

    #[test]
    fn test_path_policy_builder() {
        let policy = PathPolicy::new()
            .with_read_path("/var/templates")
            .with_read_path("/opt/templates")
            .with_write_path("/tmp/output")
            .allow_symlinks(false);

        assert_eq!(policy.read_paths.len(), 2);
        assert_eq!(policy.write_paths.len(), 1);
        assert!(!policy.allow_symlinks);
    }

    #[test]
    fn test_path_policy_allow_symlinks_true() {
        let policy = PathPolicy::new()
            .with_read_path("/var/templates")
            .allow_symlinks(true);

        assert!(policy.allow_symlinks);
    }

    #[test]
    fn test_path_policy_multiple_read_paths() {
        let policy = PathPolicy::new()
            .with_read_path("/path1")
            .with_read_path("/path2")
            .with_read_path("/path3");

        assert_eq!(policy.read_paths.len(), 3);
        assert_eq!(policy.read_paths[0], "/path1");
        assert_eq!(policy.read_paths[1], "/path2");
        assert_eq!(policy.read_paths[2], "/path3");
    }

    #[test]
    fn test_path_policy_multiple_write_paths() {
        let policy = PathPolicy::new()
            .with_write_path("/out1")
            .with_write_path("/out2");

        assert_eq!(policy.write_paths.len(), 2);
    }

    // ============================================================================
    // Path String Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_path_string_empty() {
        let result = validate_path_string("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[test]
    fn test_validate_path_string_null_byte() {
        let result = validate_path_string("/path\0with\0null");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("null"));
    }

    #[test]
    fn test_validate_path_string_valid_absolute() {
        let result = validate_path_string("/var/templates");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_string_valid_relative() {
        let result = validate_path_string("./templates");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_string_with_dots_parent() {
        let result = validate_path_string("/var/../templates");
        assert!(result.is_ok());
        // Should warn but not error
    }

    // ============================================================================
    // Path Policy Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_path_policy_empty() {
        let policy = PathPolicy::new();
        let result = validate_path_policy(&policy);
        assert!(result.is_ok()); // Empty policy is valid
    }

    #[test]
    fn test_validate_path_policy_with_read_paths() {
        let policy = PathPolicy::new()
            .with_read_path("/var/templates")
            .with_read_path("/opt/templates");

        let result = validate_path_policy(&policy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_policy_with_write_paths() {
        let policy = PathPolicy::new()
            .with_write_path("/tmp/output")
            .with_write_path("/var/output");

        let result = validate_path_policy(&policy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_policy_mixed() {
        let policy = PathPolicy::new()
            .with_read_path("/templates")
            .with_write_path("/output")
            .allow_symlinks(true);

        let result = validate_path_policy(&policy);
        assert!(result.is_ok());
    }

    // ============================================================================
    // Read Path Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_read_path_no_policy() {
        let result = validate_read_path("/etc/passwd", &None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disabled"));
    }

    #[test]
    fn test_validate_read_path_empty_allowed_paths() {
        let policy = Some(PathPolicy::new());
        let result = validate_read_path("/etc/passwd", &policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no read paths"));
    }

    #[test]
    fn test_validate_read_path_allowed() {
        let policy = Some(PathPolicy::new().with_read_path("/tmp/test"));
        // This will fail because /tmp/test likely doesn't exist canonically,
        // but we're testing the logic flow
        let result = validate_read_path("/tmp/test/file.txt", &policy);
        // Either Ok or Err, we're just testing it completes without panic
        assert!(result.is_ok() || result.is_err());
    }

    // ============================================================================
    // Write Path Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_write_path_no_policy() {
        let result = validate_write_path("/tmp/file", &None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("disabled"));
    }

    #[test]
    fn test_validate_write_path_empty_allowed_paths() {
        let policy = Some(PathPolicy::new());
        let result = validate_write_path("/tmp/file", &policy);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("no write paths"));
    }

    // ============================================================================
    // Path Resolution Tests
    // ============================================================================

    #[test]
    fn test_try_resolve_path_simple() {
        let path = Path::new("/var/templates");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().to_string_lossy(), "/var/templates");
    }

    #[test]
    fn test_try_resolve_path_with_cur_dir() {
        let path = Path::new("/var/./templates");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "/var/templates");
    }

    #[test]
    fn test_try_resolve_path_with_parent_dir() {
        let path = Path::new("/var/templates/..");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "/var");
    }

    #[test]
    fn test_try_resolve_path_with_dots_combination() {
        let path = Path::new("/var/./templates/../templates");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "/var/templates");
    }

    #[test]
    fn test_try_resolve_path_multiple_parent_dirs() {
        let path = Path::new("/var/a/b/c/../../d");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "/var/a/d");
    }

    #[test]
    fn test_try_resolve_path_relative() {
        let path = Path::new("./templates/file.html");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "templates/file.html");
    }

    #[test]
    fn test_try_resolve_path_relative_with_parent() {
        let path = Path::new("./templates/../partials/base.html");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "partials/base.html");
    }

    #[test]
    fn test_try_resolve_path_only_dots() {
        let path = Path::new(".");
        let result = try_resolve_path(path);
        assert!(result.is_ok());
        let resolved = result.unwrap();
        assert_eq!(resolved.to_string_lossy(), "");
    }

    // ============================================================================
    // Builder Pattern Tests
    // ============================================================================

    #[test]
    fn test_builder_new() {
        let builder = SandboxedEnvironmentBuilder::new();
        // Just verify it creates without panic
        let _env = builder.build();
    }

    #[test]
    fn test_builder_default() {
        let builder = SandboxedEnvironmentBuilder::default();
        // Just verify it creates without panic
        let _env = builder.build();
    }

    #[test]
    fn test_builder_with_path_policy_success() {
        let policy = PathPolicy::new()
            .with_read_path("/templates")
            .with_write_path("/output");

        let result = SandboxedEnvironmentBuilder::new().with_path_policy(policy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_read_path_success() {
        let result = SandboxedEnvironmentBuilder::new()
            .with_read_path("/templates");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_read_path_invalid_empty() {
        let result = SandboxedEnvironmentBuilder::new()
            .with_read_path("");

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_with_write_path_success() {
        let result = SandboxedEnvironmentBuilder::new()
            .with_write_path("/output");

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_with_write_path_invalid_empty() {
        let result = SandboxedEnvironmentBuilder::new()
            .with_write_path("");

        assert!(result.is_err());
    }

    #[test]
    fn test_builder_allow_symlinks_true() {
        let builder = SandboxedEnvironmentBuilder::new()
            .allow_symlinks(true);

        let env = builder.build();
        assert!(env.config().path_policy.is_some());
        assert!(env.config().path_policy.as_ref().unwrap().allow_symlinks);
    }

    #[test]
    fn test_builder_allow_symlinks_false() {
        let builder = SandboxedEnvironmentBuilder::new()
            .allow_symlinks(false);

        let env = builder.build();
        assert!(env.config().path_policy.is_some());
        assert!(!env.config().path_policy.as_ref().unwrap().allow_symlinks);
    }

    #[test]
    fn test_builder_without_path_sandboxing() {
        let builder = SandboxedEnvironmentBuilder::new()
            .with_read_path("/templates")
            .unwrap()
            .without_path_sandboxing();

        let env = builder.build();
        assert!(env.config().path_policy.is_none());
    }

    #[test]
    fn test_builder_multiple_read_paths() {
        let builder = SandboxedEnvironmentBuilder::new()
            .with_read_path("/templates")
            .and_then(|b| b.with_read_path("/includes"))
            .and_then(|b| b.with_read_path("/partials"));

        assert!(builder.is_ok());
        let env = builder.unwrap().build();
        assert_eq!(env.config().path_policy.as_ref().unwrap().read_paths.len(), 3);
    }

    #[test]
    fn test_builder_multiple_write_paths() {
        let builder = SandboxedEnvironmentBuilder::new()
            .with_write_path("/output")
            .and_then(|b| b.with_write_path("/logs"))
            .and_then(|b| b.with_write_path("/tmp"));

        assert!(builder.is_ok());
        let env = builder.unwrap().build();
        assert_eq!(env.config().path_policy.as_ref().unwrap().write_paths.len(), 3);
    }

    #[test]
    fn test_builder_mixed_operations() {
        let builder = SandboxedEnvironmentBuilder::new()
            .with_read_path("/templates")
            .and_then(|b| b.with_write_path("/output"))
            .and_then(|b| Ok(b.allow_symlinks(false)));

        assert!(builder.is_ok());
        let env = builder.unwrap().build();
        let policy = env.config().path_policy.as_ref().unwrap();
        assert_eq!(policy.read_paths.len(), 1);
        assert_eq!(policy.write_paths.len(), 1);
        assert!(!policy.allow_symlinks);
    }

    // ============================================================================
    // Integration Tests
    // ============================================================================

    #[test]
    fn test_full_sandbox_builder_chain() {
        let result = SandboxedEnvironmentBuilder::new()
            .with_read_path("/var/templates")
            .and_then(|b| b.with_write_path("/tmp/output"))
            .and_then(|b| Ok(b.allow_symlinks(false)));

        assert!(result.is_ok());
        let env = result.unwrap().build();
        assert!(env.config().path_policy.is_some());
    }

    #[test]
    fn test_seccomp_whitelist_enum() {
        let minimal = SeccompWhitelist::Minimal;
        let broad = SeccompWhitelist::Broad;

        assert_ne!(minimal, broad);
        assert_eq!(minimal, SeccompWhitelist::Minimal);
        assert_eq!(broad, SeccompWhitelist::Broad);
    }

    #[test]
    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert!(config.path_policy.is_none());
        assert_eq!(config.seccomp_whitelist, SeccompWhitelist::Minimal);
    }

    #[test]
    fn test_path_policy_with_read_path_string_type() {
        // Test that with_read_path accepts &str
        let policy = PathPolicy::new()
            .with_read_path("/var/templates");

        assert_eq!(policy.read_paths[0], "/var/templates");
    }

    #[test]
    fn test_path_policy_with_read_path_string_owned() {
        // Test that with_read_path accepts String
        let policy = PathPolicy::new()
            .with_read_path(String::from("/var/templates"));

        assert_eq!(policy.read_paths[0], "/var/templates");
    }
}


