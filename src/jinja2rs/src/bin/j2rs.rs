//! jinja2-rs CLI — wraps the Python `jinja2` command (if present) or renders
//! a template from stdin/file using the Rust engine.
//!
//! Usage:
//!   jinja2-rs TEMPLATE_FILE [--data KEY=VALUE...]
//!   echo "Hello {{ name }}" | jinja2-rs - -D name=World

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: jinja2-rs <template_file|-> [-D key=value ...]");
        std::process::exit(1);
    }

    let template_arg = &args[0];
    let mut ctx: HashMap<String, String> = HashMap::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-D" && i + 1 < args.len() {
            if let Some((k, v)) = args[i + 1].split_once('=') {
                ctx.insert(k.to_owned(), v.to_owned());
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    let source = if template_arg == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("reading stdin");
        buf
    } else {
        std::fs::read_to_string(template_arg).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", template_arg, e);
            std::process::exit(1);
        })
    };

    let env = jinja2rs::Environment::new();
    let json_ctx: serde_json::Value = ctx
        .iter()
        .fold(serde_json::Map::new(), |mut m, (k, v)| {
            m.insert(k.clone(), serde_json::Value::String(v.clone()));
            m
        })
        .into();

    match env.render_str(&source, &json_ctx) {
        Ok(output) => print!("{}", output),
        Err(e) => {
            eprintln!("Render error: {}", e);
            std::process::exit(1);
        }
    }
}
