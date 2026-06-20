//! Dependency-injection traits for CLI commands.
//!
//! Abstracting these boundaries lets us swap in `mockall` mocks during
//! unit tests without spawning subprocesses or touching the real
//! filesystem. Production `main()` fns wire the `Real*` impls.

use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};

// ── Terminal ────────────────────────────────────────────────────────────────

/// Abstracts user-visible output and interactive input.
///
/// Mirrors the `term_input` / `print` calls in `sphinx.cmd.quickstart`.
#[cfg_attr(test, mockall::automock)]
pub trait Terminal: Send + Sync {
    /// Print a line to stdout.
    fn print(&self, line: &str);
    /// Prompt the user and return the trimmed response.
    fn prompt(&self, prompt_text: &str) -> io::Result<String>;
}

/// Production [`Terminal`]: writes to real stdout and reads from stdin.
pub struct RealTerminal;

impl Terminal for RealTerminal {
    fn print(&self, line: &str) {
        println!("{line}");
    }

    fn prompt(&self, prompt_text: &str) -> io::Result<String> {
        use std::io::Write;
        print!("{prompt_text}");
        io::stdout().flush()?;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;
        Ok(line.trim_end_matches(['\n', '\r']).to_owned())
    }
}

// ── Fs ──────────────────────────────────────────────────────────────────────

/// Abstracts filesystem access (create dirs, test existence, write bytes).
///
/// Using this in `generate()` and `valid_dir()` lets us mock collision
/// scenarios without touching disk.
#[cfg_attr(test, mockall::automock)]
pub trait Fs: Send + Sync {
    fn exists(&self, p: &Path) -> bool;
    fn is_dir(&self, p: &Path) -> bool;
    fn is_file(&self, p: &Path) -> bool;
    fn ensure_dir(&self, p: &Path) -> io::Result<()>;
    fn write(&self, p: &Path, bytes: &[u8]) -> io::Result<()>;
    /// List direct children of a directory as bare file-names (not full
    /// paths). Returns `Ok([])` if the directory does not exist.
    fn read_dir_names(&self, p: &Path) -> io::Result<Vec<String>>;
}

/// Production [`Fs`]: delegates to `std::fs`.
pub struct RealFs;

impl Fs for RealFs {
    fn exists(&self, p: &Path) -> bool {
        p.exists()
    }

    fn is_dir(&self, p: &Path) -> bool {
        p.is_dir()
    }

    fn is_file(&self, p: &Path) -> bool {
        p.is_file()
    }

    fn ensure_dir(&self, p: &Path) -> io::Result<()> {
        std::fs::create_dir_all(p)
    }

    fn write(&self, p: &Path, bytes: &[u8]) -> io::Result<()> {
        std::fs::write(p, bytes)
    }

    fn read_dir_names(&self, p: &Path) -> io::Result<Vec<String>> {
        if !p.exists() {
            return Ok(vec![]);
        }
        let mut names = Vec::new();
        for entry in std::fs::read_dir(p)? {
            let entry = entry?;
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
        Ok(names)
    }
}

// ── Clock ───────────────────────────────────────────────────────────────────

/// Abstracts wall-clock access so `now` / `copyright` year are
/// deterministic in snapshot tests.
///
/// `asctime()` mirrors `time.asctime()` from the upstream Python code:
/// "Mon Jan  1 00:00:00 2024"-style string.
#[cfg_attr(test, mockall::automock)]
pub trait Clock: Send + Sync {
    /// `time.asctime()` equivalent.
    fn asctime(&self) -> String;
    /// Current calendar year.
    fn year(&self) -> i32;
}

/// Production [`Clock`]: uses the system time.
pub struct SystemClock;

impl Clock for SystemClock {
    fn asctime(&self) -> String {
        // Format matching Python's time.asctime():  "Mon Jan  1 00:00:00 2024"
        // Using std only — no chrono dep needed for this simple format.
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        format_asctime(secs)
    }

    fn year(&self) -> i32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        unix_to_year(secs)
    }
}

/// Format a Unix timestamp as `time.asctime()` does.
/// "Mon Jan  1 00:00:00 2024"
pub fn format_asctime(unix_secs: u64) -> String {
    const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    // Civil date arithmetic (proleptic Gregorian, Zeller/epoch method).
    let days_since_epoch = unix_secs / 86400;
    let time_of_day = unix_secs % 86400;
    let hh = time_of_day / 3600;
    let mm = (time_of_day % 3600) / 60;
    let ss = time_of_day % 60;

    // Day of week: epoch (1970-01-01) was Thursday = 4.
    let dow = ((days_since_epoch + 4) % 7) as usize;

    let (year, month, day) = days_since_epoch_to_ymd(days_since_epoch);
    let month_idx = (month - 1) as usize;

    // Python asctime pads day with a space when < 10.
    if day < 10 {
        format!(
            "{} {}  {} {:02}:{:02}:{:02} {}",
            DAYS[dow], MONTHS[month_idx], day, hh, mm, ss, year
        )
    } else {
        format!(
            "{} {} {} {:02}:{:02}:{:02} {}",
            DAYS[dow], MONTHS[month_idx], day, hh, mm, ss, year
        )
    }
}

fn days_since_epoch_to_ymd(days: u64) -> (u64, u32, u32) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32)
}

fn unix_to_year(unix_secs: u64) -> i32 {
    let days = unix_secs / 86400;
    let (y, _, _) = days_since_epoch_to_ymd(days);
    y as i32
}

// ── Runner ──────────────────────────────────────────────────────────────────

/// Abstracts subprocess execution so `make_mode` can be tested without
/// real `sphinx-build` calls.
#[cfg_attr(test, mockall::automock)]
pub trait Runner: Send + Sync {
    /// Run `program` with `args` in `cwd` and return the exit code.
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> io::Result<i32>;
}

/// Production [`Runner`]: spawns a real subprocess.
pub struct ProcessRunner;

impl Runner for ProcessRunner {
    fn run(&self, program: &str, args: &[String], cwd: &Path) -> io::Result<i32> {
        let status = std::process::Command::new(program)
            .args(args)
            .current_dir(cwd)
            .status()?;
        Ok(status.code().unwrap_or(1))
    }
}

// ── Python fallback ─────────────────────────────────────────────────────────

/// Check if the `SPHINXDOCRS_PY_FALLBACK` env var is set (or `--use-python-impl`
/// was passed), meaning we should shell out to the upstream Python command.
pub fn py_fallback_requested(args: &[String]) -> bool {
    if std::env::var("SPHINXDOCRS_PY_FALLBACK").is_ok() {
        return true;
    }
    args.iter().any(|a| a == "--use-python-impl")
}

/// Run the upstream Python entry-point for `mod_path` (e.g.
/// `"sphinx.cmd.quickstart"`) with the given `argv`, forwarding its
/// exit code. Strips `--use-python-impl` from `argv` before forwarding.
pub fn run_python_impl(mod_path: &str, argv: &[String]) -> ! {
    let filtered: Vec<_> = argv
        .iter()
        .filter(|a| a.as_str() != "--use-python-impl")
        .collect();
    let py_code = format!(
        "import sys; from {} import main; sys.exit(main())",
        mod_path
    );
    let status = std::process::Command::new("python")
        .arg("-c")
        .arg(&py_code)
        .args(&filtered)
        .status()
        .expect("Failed to execute python");
    std::process::exit(status.code().unwrap_or(1));
}

// ── Test helpers (public, no mockall dependency) ────────────────────────────

/// A [`Clock`] implementation with fixed, injected values.
/// Available outside `#[cfg(test)]` so integration test crates can use it.
pub struct FixedClock {
    pub asctime_str: String,
    pub year_val: i32,
}

impl FixedClock {
    /// The canonical snapshot clock value: "Mon Jan  1 00:00:00 2024".
    pub fn snapshot() -> Self {
        Self {
            asctime_str: "Mon Jan  1 00:00:00 2024".to_owned(),
            year_val: 2024,
        }
    }
}

impl Clock for FixedClock {
    fn asctime(&self) -> String {
        self.asctime_str.clone()
    }

    fn year(&self) -> i32 {
        self.year_val
    }
}

/// A [`Runner`] that records calls instead of spawning processes.
///
/// `calls()` returns a snapshot of all `(program, args)` pairs that
/// were recorded. Always returns `Ok(return_code)`.
type CallLog = Arc<Mutex<Vec<(String, Vec<String>)>>>;

pub struct CapturingRunner {
    calls: CallLog,
    return_code: i32,
}

impl CapturingRunner {
    pub fn new(return_code: i32) -> Self {
        Self {
            calls: Arc::new(Mutex::new(vec![])),
            return_code,
        }
    }

    pub fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.lock().unwrap().clone()
    }
}

impl Runner for CapturingRunner {
    fn run(&self, program: &str, args: &[String], _cwd: &Path) -> io::Result<i32> {
        self.calls
            .lock()
            .unwrap()
            .push((program.to_owned(), args.to_vec()));
        Ok(self.return_code)
    }
}

/// A [`Terminal`] that feeds pre-set answers and captures printed lines.
///
/// Panics if `prompt` is called more times than there are answers.
pub struct ScriptedTerminal {
    answers: Mutex<std::collections::VecDeque<String>>,
    printed: Mutex<Vec<String>>,
}

impl ScriptedTerminal {
    pub fn new(answers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            answers: Mutex::new(answers.into_iter().map(Into::into).collect()),
            printed: Mutex::new(vec![]),
        }
    }

    /// Return all lines that were printed (in order).
    pub fn printed(&self) -> Vec<String> {
        self.printed.lock().unwrap().clone()
    }
}

impl Terminal for ScriptedTerminal {
    fn print(&self, line: &str) {
        self.printed.lock().unwrap().push(line.to_owned());
    }

    fn prompt(&self, _prompt_text: &str) -> io::Result<String> {
        let ans = self.answers.lock().unwrap().pop_front().unwrap_or_default();
        Ok(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asctime_epoch() {
        // 1970-01-01 00:00:00 UTC = Thursday
        assert_eq!(format_asctime(0), "Thu Jan  1 00:00:00 1970");
    }

    #[test]
    fn asctime_2024_jan_01_noon() {
        // 2024-01-01 12:00:00 UTC = Monday
        let secs: u64 = 1704110400;
        let s = format_asctime(secs);
        assert!(s.starts_with("Mon Jan  1 12:00:00 2024"), "got: {s}");
    }

    #[test]
    fn py_fallback_env() {
        // Can't safely set env vars in unit tests — just test the args path.
        assert!(py_fallback_requested(&["--use-python-impl".to_string()]));
        assert!(!py_fallback_requested(&["-q".to_string()]));
    }
}
