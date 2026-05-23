//! Fetch the MathJax bundle referenced from one or more `conf.py`
//! files into a local cache and emit Sub-Resource Integrity (SRI)
//! hashes so the resulting `<script>` tag can use `integrity=…`.
//!
//! Usage:
//!
//! ```text
//! cargo run -p sphinxdocrs --example fetch_mathjax -- \
//!     -o OUTDIR <conf.py> [<conf.py> ...]
//! ```
//!
//! For each `conf.py` the example writes:
//!
//! * `OUTDIR/<parent-dir-name>.mathjax.json` — a small JSON record
//!   with the source URL, cached local path, algorithm, and the
//!   `integrity` attribute value.
//! * `OUTDIR/<parent-dir-name>.mathjax.html` — a ready-to-paste
//!   `<script defer src="…" integrity="…" crossorigin="anonymous">`
//!   snippet (mirrors what `app.add_js_file(…, integrity=…)` produces
//!   in Sphinx 1.8+).
//!
//! The cache lives under `OUTDIR/_cache/` and is content-addressed by
//! the source URL, so repeated runs are no-ops.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use sphinxdocrs::assets::{DEFAULT_SRI_ALGO, fetch_and_cache, sri_hash_file};
use sphinxdocrs::config::Config;

fn usage() -> ! {
    eprintln!("usage: fetch_mathjax -o OUTDIR <conf.py> [<conf.py> ...]");
    std::process::exit(2);
}

fn parse_args() -> (Option<PathBuf>, Vec<PathBuf>) {
    let mut out: Option<PathBuf> = None;
    let mut inputs: Vec<PathBuf> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--output-dir" => {
                let v = args.next().unwrap_or_else(|| usage());
                out = Some(PathBuf::from(v));
            }
            "-h" | "--help" => usage(),
            _ => inputs.push(PathBuf::from(a)),
        }
    }
    if inputs.is_empty() {
        usage();
    }
    (out, inputs)
}

fn label(p: &Path) -> String {
    p.parent()
        .and_then(|d| d.file_name())
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "conf".to_string())
}

fn main() -> ExitCode {
    let (out, inputs) = parse_args();
    let Some(out) = out else {
        eprintln!("error: -o OUTDIR is required");
        return ExitCode::from(2);
    };
    std::fs::create_dir_all(&out).expect("mkdir OUTDIR");
    let cache_dir = out.join("_cache");

    for conf in inputs {
        let cfg = match Config::from_conf_py(&conf) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("error: {}: {e}", conf.display());
                return ExitCode::from(1);
            }
        };
        let url = &cfg.mathjax_path;
        let path = match fetch_and_cache(url, &cache_dir) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("error: fetch {url}: {e}");
                return ExitCode::from(1);
            }
        };
        let integrity = match sri_hash_file(&path, DEFAULT_SRI_ALGO) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error: hash {}: {e}", path.display());
                return ExitCode::from(1);
            }
        };
        let stem = label(&conf);
        let json = format!(
            "{{\n  \"url\": \"{}\",\n  \"path\": \"{}\",\n  \"algo\": \"{}\",\n  \"integrity\": \"{}\"\n}}\n",
            url,
            path.display(),
            DEFAULT_SRI_ALGO.name(),
            integrity,
        );
        std::fs::write(out.join(format!("{stem}.mathjax.json")), &json).expect("write json record");
        let snippet = format!(
            "<script defer src=\"{}\" integrity=\"{}\" crossorigin=\"anonymous\"></script>\n",
            url, integrity,
        );
        std::fs::write(out.join(format!("{stem}.mathjax.html")), &snippet).expect("write snippet");
        println!("{stem}: {} -> {}", url, path.display());
        println!("{stem}: integrity = {integrity}");
    }
    ExitCode::SUCCESS
}
