//! Validators mirroring `sphinx.cmd.quickstart`'s pure validation
//! functions. Each function accepts a `&str` and returns either a
//! typed value or a [`ValidationError`].
//!
//! Parity notes:
//! - `boolean`: accepts only Y/YES/N/NO (case-insensitive) — any other
//!   value is an error matching Python's message.
//! - `suffix`: must start with `.` and be longer than one char.
//! - `nonempty`: any non-empty string.
//! - `choice`: matches Python's `choice(*l)` factory.

use std::fmt;
use std::path::Path;

/// Validation error mirroring `sphinx.cmd.quickstart.ValidationError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValidationError {}

/// Mirrors `is_path`: expand `~` and assert the result is an existing
/// directory.
pub fn is_path(x: &str) -> Result<String, ValidationError> {
    let expanded = expand_user(x);
    if Path::new(&expanded).is_dir() {
        Ok(expanded)
    } else {
        Err(ValidationError("Please enter a valid path name.".into()))
    }
}

/// Like `is_path` but also accepts paths that do not exist yet (they will be
/// created by the generator).  Rejects non-empty paths that point at an
/// existing *file* (not a directory).
pub fn is_path_or_new(x: &str) -> Result<String, ValidationError> {
    if x.is_empty() {
        return Err(ValidationError("Please enter a valid path name.".into()));
    }
    let expanded = expand_user(x);
    let p = Path::new(&expanded);
    if p.exists() && !p.is_dir() {
        Err(ValidationError(
            "Please enter a path name that is not an existing file.".into(),
        ))
    } else {
        Ok(expanded)
    }
}

/// Simple `~`-expansion mirroring `os.path.expanduser`.
fn expand_user(x: &str) -> String {
    if x == "~" || x.starts_with("~/") || x.starts_with("~\\") {
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            return x.replacen('~', &home, 1);
        }
    }
    x.to_owned()
}

/// Mirrors `is_path_or_empty`: empty string passes through; non-empty
/// strings are validated as paths.
pub fn is_path_or_empty(x: &str) -> Result<String, ValidationError> {
    if x.is_empty() {
        Ok(String::new())
    } else {
        is_path(x)
    }
}

/// Mirrors `allow_empty`: always succeeds, returning the input as-is.
pub fn allow_empty(x: &str) -> Result<String, ValidationError> {
    Ok(x.to_owned())
}

/// Mirrors `nonempty`: fails if the string is empty.
pub fn nonempty(x: &str) -> Result<String, ValidationError> {
    if x.is_empty() {
        Err(ValidationError("Please enter some text.".into()))
    } else {
        Ok(x.to_owned())
    }
}

/// Mirrors `choice(*l)`: accepts only one of the given strings.
///
/// Returns the input string on success (not an index). The error
/// message matches upstream: "Please enter one of %s."
pub fn choice<'a>(options: &'a [&'a str]) -> impl Fn(&str) -> Result<String, ValidationError> + 'a {
    move |x| {
        if options.contains(&x) {
            Ok(x.to_owned())
        } else {
            Err(ValidationError(format!(
                "Please enter one of {}.",
                options.join(", ")
            )))
        }
    }
}

/// Mirrors `boolean`: Y/YES → `true`, N/NO → `false`.
/// Anything else → error ("Please enter either 'y' or 'n'.").
pub fn boolean(x: &str) -> Result<bool, ValidationError> {
    match x.to_ascii_uppercase().as_str() {
        "Y" | "YES" => Ok(true),
        "N" | "NO" => Ok(false),
        _ => Err(ValidationError("Please enter either 'y' or 'n'.".into())),
    }
}

/// Mirrors `suffix`: must start with `.` and be longer than 1 char.
/// Error message: "Please enter a file suffix, e.g. '.rst' or '.txt'."
pub fn suffix(x: &str) -> Result<String, ValidationError> {
    if x.starts_with('.') && x.len() > 1 {
        Ok(x.to_owned())
    } else {
        Err(ValidationError(
            "Please enter a file suffix, e.g. '.rst' or '.txt'.".into(),
        ))
    }
}

/// Mirrors `ok`: identity function, always succeeds.
pub fn ok(x: &str) -> Result<String, ValidationError> {
    Ok(x.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // boolean
    #[test]
    fn boolean_true_variants() {
        assert_eq!(boolean("y"), Ok(true));
        assert_eq!(boolean("Y"), Ok(true));
        assert_eq!(boolean("yes"), Ok(true));
        assert_eq!(boolean("YES"), Ok(true));
    }

    #[test]
    fn boolean_false_variants() {
        assert_eq!(boolean("n"), Ok(false));
        assert_eq!(boolean("N"), Ok(false));
        assert_eq!(boolean("no"), Ok(false));
        assert_eq!(boolean("NO"), Ok(false));
    }

    #[test]
    fn boolean_invalid() {
        assert!(boolean("maybe").is_err());
        assert!(boolean("").is_err());
        assert!(boolean("1").is_err());
        let err = boolean("maybe").unwrap_err();
        assert!(err.0.contains("'y' or 'n'"), "got: {}", err.0);
    }

    // suffix
    #[test]
    fn suffix_valid() {
        assert_eq!(suffix(".rst"), Ok(".rst".into()));
        assert_eq!(suffix(".txt"), Ok(".txt".into()));
        assert_eq!(suffix(".md"), Ok(".md".into()));
    }

    #[test]
    fn suffix_invalid() {
        assert!(suffix("rst").is_err());
        assert!(suffix(".").is_err());
        assert!(suffix("").is_err());
    }

    // nonempty
    #[test]
    fn nonempty_passes_text() {
        assert_eq!(nonempty("hello"), Ok("hello".into()));
    }

    #[test]
    fn nonempty_fails_empty() {
        assert!(nonempty("").is_err());
    }

    // allow_empty
    #[test]
    fn allow_empty_any_value() {
        assert_eq!(allow_empty(""), Ok("".into()));
        assert_eq!(allow_empty("x"), Ok("x".into()));
    }

    // choice
    #[test]
    fn choice_valid() {
        let c = choice(&["a", "b", "c"]);
        assert_eq!(c("a"), Ok("a".into()));
    }

    #[test]
    fn choice_invalid() {
        let c = choice(&["a", "b"]);
        let err = c("z").unwrap_err();
        assert!(err.0.contains("a, b"), "got: {}", err.0);
    }

    // ok
    #[test]
    fn ok_identity() {
        assert_eq!(ok("anything"), Ok("anything".into()));
    }

    // is_path_or_empty
    #[test]
    fn is_path_or_empty_empty() {
        assert_eq!(is_path_or_empty(""), Ok("".into()));
    }
}
