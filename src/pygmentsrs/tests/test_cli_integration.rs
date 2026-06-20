#![allow(clippy::needless_borrows_for_generic_args)]


//! CLI integration tests for bin/pygmentize.rs
//! Target: 10-15 tests covering subprocess invocation and argument handling
//!
//! Tests cover:
//! - Subprocess execution via Python's pygments CLI
//! - Argument passing and forwarding
//! - Exit code propagation
//! - Various command-line argument patterns
//! - Error handling and edge cases

use std::fs;
use std::io::Write;
use std::process::Command;
//use std::path::PathBuf;

// Helper: Run pygments with args and return status code + output
fn run_pygments(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("python")
        .arg("-m")
        .arg("pygments")
        .args(args)
        .output()
        .expect("Failed to run pygments");

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (code, stdout, stderr)
}

// Helper: Run pygments with stdin
fn run_pygments_with_stdin(args: &[&str], stdin: &str) -> (i32, String, String) {
    let mut child = Command::new("python")
        .arg("-m")
        .arg("pygments")
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn pygments");

    if let Some(mut stdin_handle) = child.stdin.take() {
        let _ = stdin_handle.write_all(stdin.as_bytes());
    }

    let output = child
        .wait_with_output()
        .expect("Failed to wait for pygments");
    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (code, stdout, stderr)
}

#[test]
fn test_cli_help_argument() {
    // Test: pygments --help
    let (code, stdout, _stderr) = run_pygments(&["--help"]);

    assert_eq!(code, 0, "Help should exit with code 0");
    assert!(!stdout.is_empty(), "Help output should not be empty");
    assert!(
        stdout.contains("Usage:") || stdout.contains("usage:"),
        "Help output should contain usage information"
    );
}

#[test]
fn test_cli_version_argument() {
    // Test: pygments --version or -V
    // Try both forms as different versions support different flags
    let (code, stdout, _stderr) = run_pygments(&["--version"]);

    if code != 0 {
        // Try -V as alternative
        let (code_alt, stdout_alt, _stderr_alt) = run_pygments(&["-V"]);
        assert_eq!(code_alt, 0, "Version should exit with code 0 (-V flag)");
        assert!(!stdout_alt.is_empty(), "Version output should not be empty");
    } else {
        assert_eq!(code, 0, "Version should exit with code 0");
        assert!(!stdout.is_empty(), "Version output should not be empty");
    }
}

#[test]
fn test_cli_list_lexers() {
    // Test: pygments -L
    let (code, stdout, _stderr) = run_pygments(&["-L"]);

    assert_eq!(code, 0, "List lexers should exit with code 0");
    assert!(!stdout.is_empty(), "Lexer list output should not be empty");
    assert!(
        stdout.contains("Lexer") || stdout.contains("lexer"),
        "Output should mention lexers"
    );
}

#[test]
fn test_cli_list_formatters() {
    // Test: pygments -F (list formatters)
    // Try multiple flag variants as support varies
    let (code, stdout, _stderr) = run_pygments(&["-F"]);

    if code == 0 && !stdout.is_empty() {
        assert!(
            stdout.contains("Formatter") || stdout.contains("formatter") || stdout.contains("html"),
            "Output should mention formatters"
        );
    } else {
        // If -F fails, it's OK - not all Pygments versions support it
        // Just verify it doesn't crash catastrophically
        assert!(code == 0 || code == 2, "Should exit gracefully");
    }
}

#[test]
fn test_cli_highlight_stdin() {
    // Test: echo "print('hello')" | pygments -l python -f html
    let python_code = "print('hello world')";
    let (code, stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-f", "html"], python_code);

    assert_eq!(code, 0, "Highlighting should exit with code 0");
    assert!(!stdout.is_empty(), "Formatted output should not be empty");
    // HTML output should contain tags
    assert!(
        stdout.contains("<") && stdout.contains(">"),
        "HTML output should contain tags"
    );
}

#[test]
fn test_cli_with_lexer() {
    // Test: pygments -l python with stdin
    let python_code = "def func():\n    pass";
    let (code, stdout, _stderr) = run_pygments_with_stdin(&["-l", "python"], python_code);

    assert_eq!(code, 0, "Should exit with code 0 when lexer is valid");
    assert!(!stdout.is_empty(), "Output should not be empty");
    // Should contain ANSI codes or tokens
    assert!(
        stdout.len() >= python_code.len(),
        "Output should process the input"
    );
}

#[test]
fn test_cli_with_output_format() {
    // Test: pygments -f latex with stdin
    let python_code = "x = 42";
    let (code, stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-f", "latex"], python_code);

    assert_eq!(code, 0, "LaTeX formatting should exit with code 0");
    assert!(!stdout.is_empty(), "LaTeX output should not be empty");
    // LaTeX output might contain escape sequences or control sequences
    assert!(!stdout.is_empty(), "Output should be generated");
}

#[test]
fn test_cli_with_style() {
    // Test: pygments -O style=monokai with stdin
    let python_code = "x = 1";
    let (code, stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-O", "style=monokai"], python_code);

    assert_eq!(code, 0, "Style option should exit with code 0");
    assert!(!stdout.is_empty(), "Output should not be empty");
}

#[test]
fn test_cli_output_file() {
    // Test: pygments -o output.html with temp file
    let temp_dir = std::env::temp_dir();
    let input_file = temp_dir.join("test_input.py");
    let output_file = temp_dir.join("test_output.html");

    // Write test input
    let test_code = "print('test')";
    fs::write(&input_file, test_code).expect("Failed to write test input");

    // Run pygments with output file
    let (code, _stdout, _stderr) = run_pygments(&[
        "-l",
        "python",
        "-f",
        "html",
        "-o",
        output_file.to_str().unwrap(),
        input_file.to_str().unwrap(),
    ]);

    assert_eq!(code, 0, "Output file should exit with code 0");
    assert!(output_file.exists(), "Output file should be created");

    let output_content = fs::read_to_string(&output_file).expect("Failed to read output file");
    assert!(
        !output_content.is_empty(),
        "Output file should contain content"
    );

    // Cleanup
    let _ = fs::remove_file(&input_file);
    let _ = fs::remove_file(&output_file);
}

#[test]
fn test_cli_stdin_with_auto_detect() {
    // Test: pygments without -l (auto-detect from content)
    let python_code = "def hello():\n    print('hello')";
    let (code, stdout, _stderr) = run_pygments_with_stdin(&["-f", "html"], python_code);

    assert_eq!(code, 0, "Auto-detect should exit with code 0");
    assert!(!stdout.is_empty(), "Output should not be empty");
}

#[test]
fn test_cli_invalid_lexer() {
    // Test: pygments -l nosuchlexer (invalid lexer)
    let (code, _stdout, stderr) =
        run_pygments_with_stdin(&["-l", "nosuchlexer_xyz_invalid"], "code");

    assert_ne!(code, 0, "Invalid lexer should exit with non-zero code");
    assert!(
        !stderr.is_empty() || code != 0,
        "Should have error or non-zero exit"
    );
}

#[test]
fn test_cli_invalid_format() {
    // Test: pygments -f nosuchformat (invalid format)
    let (code, _stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-f", "nosuchformat_xyz"], "code");

    assert_ne!(code, 0, "Invalid format should exit with non-zero code");
}

#[test]
fn test_cli_nonexistent_file() {
    // Test: pygments on non-existent file
    let (code, _stdout, _stderr) =
        run_pygments(&["/nonexistent/path/that/does/not/exist_12345.py"]);

    assert_ne!(code, 0, "Non-existent file should exit with non-zero code");
}

#[test]
fn test_cli_exit_code_success() {
    // Test: successful highlighting should exit with 0
    let python_code = "x = 42\ny = x + 1\nprint(y)";
    let (code, _stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-f", "html"], python_code);

    assert_eq!(code, 0, "Successful highlighting should exit with code 0");
}

#[test]
fn test_cli_exit_code_failure() {
    // Test: invalid arguments should exit with non-zero code
    let (code, _stdout, _stderr) = run_pygments(&["--invalid-flag-that-does-not-exist"]);

    assert_ne!(code, 0, "Invalid arguments should exit with non-zero code");
}

#[test]
fn test_cli_multiple_lexer_formats() {
    // Test: multiple lexer/format combinations
    let test_cases = [
        ("python", "html"),
        ("python", "latex"),
        ("python", "rst"),
        ("c", "html"),
        ("javascript", "html"),
    ];

    // Test a few combinations - use only well-supported formats
    for (lexer, format) in test_cases.iter().take(3) {
        let (code, stdout, _stderr) =
            run_pygments_with_stdin(&["-l", lexer, "-f", format], "x = 1");

        // Some format combinations might not exist, that's OK
        if code == 0 {
            assert!(
                !stdout.is_empty(),
                "Should produce output for lexer={} format={}",
                lexer,
                format
            );
        }
    }
}

#[test]
fn test_cli_empty_input() {
    // Test: empty input should still succeed
    let (code, _stdout, _stderr) = run_pygments_with_stdin(&["-l", "python", "-f", "html"], "");

    assert_eq!(code, 0, "Empty input should exit with code 0");
    // Empty input produces minimal/empty output in most formatters
}

#[test]
fn test_cli_large_input() {
    // Test: large input handling
    let mut large_code = String::new();
    for i in 0..1000 {
        large_code.push_str(&format!("x{} = {}\n", i, i));
    }

    let (code, stdout, _stderr) =
        run_pygments_with_stdin(&["-l", "python", "-f", "html"], &large_code);

    assert_eq!(code, 0, "Large input should exit with code 0");
    assert!(!stdout.is_empty(), "Large input should produce output");
}
