//! Read one or more `conf.py` files via `sphinxdocrs::config::Config`
//! and print the resolved math configuration.
//!
//! Usage (sphinx-build style):
//!
//! ```text
//! read_conf [-o OUTDIR] <conf.py> [conf.py ...]
//! ```
//!
//! With `-o OUTDIR`, writes one `<parent-dir-name>.txt` file per
//! `conf.py` plus a combined `report.txt` into `OUTDIR` (created if
//! missing). Without `-o`, prints to stdout. Invoked by
//! `make demo-math-sphinx`.

use std::path::{Path, PathBuf};

use sphinxdocrs::config::Config;

fn format_conf(label: &str, cfg: &Config) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(out, "--- {label} ---");
    let _ = writeln!(out, "extensions={:?}", cfg.extensions);
    let _ = writeln!(
        out,
        "math_renderer={}",
        cfg.math_renderer.map(|r| r.name()).unwrap_or("<unset>")
    );
    let _ = writeln!(
        out,
        "effective_math_renderer={}",
        cfg.effective_math_renderer().name()
    );
    let _ = writeln!(out, "mathjax_path={}", cfg.mathjax_path);
    let _ = writeln!(out, "mathjax_options={:?}", cfg.mathjax_options);
    let _ = writeln!(out, "imgmath_image_format={}", cfg.imgmath_image_format);
    let _ = writeln!(out, "imgmath_latex={}", cfg.imgmath_latex);
    out
}

fn usage() -> ! {
    eprintln!("usage: read_conf [-o OUTDIR] <conf.py> [conf.py ...]");
    std::process::exit(2);
}

fn main() {
    let mut outdir: Option<PathBuf> = None;
    let mut paths: Vec<String> = Vec::new();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "-o" | "--outdir" => {
                outdir = Some(args.next().unwrap_or_else(|| usage()).into());
            }
            "-h" | "--help" => usage(),
            _ => paths.push(a),
        }
    }
    if paths.is_empty() {
        usage();
    }

    let mut combined = String::new();
    for p in &paths {
        let cfg = Config::from_conf_py(Path::new(p))
            .unwrap_or_else(|e| panic!("read {p}: {e}"));
        let block = format_conf(p, &cfg);
        combined.push_str(&block);

        if let Some(dir) = &outdir {
            std::fs::create_dir_all(dir)
                .unwrap_or_else(|e| panic!("mkdir {}: {e}", dir.display()));
            // Per-conf filename derived from the parent directory
            // (e.g. `mathjax/conf.py` -> `mathjax.txt`), mirroring how
            // sphinx-build derives output names from source layout.
            let stem = Path::new(p)
                .parent()
                .and_then(Path::file_name)
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "conf".to_string());
            let path = dir.join(format!("{stem}.txt"));
            std::fs::write(&path, &block)
                .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
            println!("wrote {}", path.display());
        } else {
            print!("{block}");
        }
    }

    if let Some(dir) = &outdir {
        let path = dir.join("report.txt");
        std::fs::write(&path, &combined)
            .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        println!("wrote {}", path.display());
    }
}
