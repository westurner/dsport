//! j2substrs — environment variable substitution using Jinja2 templates.
//!
//! This tool substitutes variables in templates using Jinja2 syntax ({{ VAR }}) with values from:
//! - Environment variables (automatically included)
//! - Custom variables (via -s/--set flags)
//! - Inventory (Ansible mode only)
//! - Multiple compatibility modes (jinja2, minijinja, ansible)
//!
//!
//! USAGE EXAMPLES:
//!
//! Basic substitution from stdin:
//!   echo "Hello {{ USER }}, home is {{ HOME }}" | j2substrs
//!
//! From a template file:
//!   j2substrs --file template.txt
//!   j2substrs template.txt
//!
//! With custom variables:
//!   j2substrs --file config.txt -s APP=myapp -s VERSION=1.0
//!
//! Write output to file:
//!   j2substrs --file template.txt --output result.txt -s NAME=Alice
//!
//! Preview mode (show output and confirm before writing):
//!   j2substrs --file template.txt --output result.txt --preview -s VAR=value
//!
//! Auto-confirm preview (for CI/CD):
//!   j2substrs --file template.txt --output result.txt --preview --yes -s VAR=value
//!
//! Ansible mode with playbook:
//!   j2substrs --mode ansible --file playbook.yml --output out.yml
//!
//! Load inventory from file:
//!   j2substrs --file playbook.j2 --inventory /etc/ansible/hosts \
//!     --inventory-hostname web1 --output playbook.yml
//!
//! Load inventory from stdin:
//!   cat inventory.yml | j2substrs --file config.j2 --inventory-stdin \
//!     --inventory-hostname web1 --output result.yml
//!
//! Load inventory inline:
//!   j2substrs --file template.j2 --inventory-inline 'all: {hosts: {localhost: {}}}' \
//!     --output result.yml
//!
//! Combined: with mode and variables:
//!   j2substrs --mode ansible --file deploy.j2 \
//!     --inventory /etc/ansible/hosts \
//!     --inventory-hostname production-web-01 \
//!     --output deploy.yml \
//!     -s ENV=production -s REGION=us-east-1
//!
//! INVENTORY TEMPLATE VARIABLES (--mode ansible only):
//!
//! When using --inventory, the following variables are available in templates:
//!   - groups: Dictionary mapping group names to lists of hostnames
//!     Example: groups.all, groups.webservers, groups.databases
//!   - hostvars: Dictionary mapping hostnames to their variables
//!     Example: hostvars.web1, hostvars[inventory_hostname]
//!   - inventory_hostname: Current host being deployed to (set via --inventory-hostname)
//!   - [group vars]: Global variables from the "all" group are injected at top level
//!     Example: ansible_user, deploy_env (from all.vars in inventory)
//!
//! INVENTORY TEMPLATE EXAMPLES:
//!
//! Access all hosts in a group:
//!   {% for host in groups.all %}
//!     - {{ host }}
//!   {% endfor %}
//!
//! Access host variables:
//!   hostname: {{ inventory_hostname }}
//!   vars: {{ hostvars[inventory_hostname] }}
//!
//! Use group variables:
//!   ansible_user: {{ ansible_user }}
//!   deploy_env: {{ deploy_env }}
//!
//! Conditional deployment based on group membership:
//!   {% if inventory_hostname in groups.webservers %}
//!     nginx_enabled: true
//!   {% else %}
//!     nginx_enabled: false
//!   {% endif %}
//!
//! COMPATIBILITY MODES:
//!
//! jinja2 (default)
//!   Drop-in compatible with Python Jinja2. Enables Python method syntax:
//!   - {{ dict.items() }}, {{ list.append() }}, {{ str.upper() }}
//!   - Full Jinja2 filter support
//!   - Recommended for general-purpose template rendering
//!
//! minijinja
//!   Uses minijinja's native filter-based approach (lower overhead):
//!   - No method syntax; use filters instead: {{ items|items }}, {{ upper|filter }}
//!   - More explicit and predictable behavior
//!   - Recommended for performance-critical applications
//!
//! ansible
//!   Specialized mode for Ansible playbooks:
//!   - Includes Ansible standard filters (to_nice_json, combine, regex_*, etc.)
//!   - YAML validation for playbooks and inventories
//!   - Inventory support (hosts, groups, hostvars)
//!   - Composable method syntax
//!
//! OPTIONS:
//!
//!   TEMPLATE SOURCES:
//!     TEMPLATE                    Positional: read from file (alternative to -f)
//!     -f, --file <FILE>           Read template from FILE (default: stdin)
//!
//!   OUTPUT:
//!     -o, --output <FILE>         Write to FILE instead of stdout
//!
//!   MODES:
//!     --mode <MODE>               jinja2, minijinja, or ansible (default: jinja2)
//!
//!   PREVIEW & CONFIRMATION:
//!     --preview                   Show rendered output and ask before writing
//!                                 (requires --output to prompt for confirmation)
//!     -y, --yes                   Auto-confirm preview (skips interactive prompt)
//!                                 (use with --preview for non-interactive automation)
//!
//!   VARIABLES:
//!     -s, --set <KEY=VALUE>       Set custom variable (repeatable)
//!                                 Example: -s APP=myapp -s VERSION=2.0
//!     --default-value <VALUE>     Default for undefined vars (with --skip-missing)
//!
//!   ERROR HANDLING:
//!     --strict                    Fail on undefined variables (exit code 1)
//!     --skip-missing              Replace undefined with empty (default)
//!
//!   ANSIBLE INVENTORY (--mode ansible only):
//!     --inventory <FILE>          Load Ansible inventory from file
//!     --inventory-stdin           Load Ansible inventory from stdin
//!     --inventory-inline <YAML>   Load Ansible inventory from inline YAML/JSON
//!     --inventory-hostname <NAME> Set current host for inventory_hostname variable
//!
//!   HELP:
//!     -h, --help                  Show brief help
//!     -h, --help                  Show full help
//!     -V, --version               Show version

use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use jinja2rs::ansible_inventory::{Inventory, InventorySource};

/// Compatibility mode for template rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Drop-in compatible with Python Jinja2 (default)
    Jinja2,
    /// Native minijinja filter-based approach
    Minijinja,
    /// Specialized mode for Ansible playbooks with Ansible filters
    Ansible,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Jinja2
    }
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "jinja2" => Ok(Mode::Jinja2),
            "minijinja" => Ok(Mode::Minijinja),
            "ansible" => Ok(Mode::Ansible),
            _ => Err(format!(
                "Invalid mode '{}'. Valid modes are: jinja2, minijinja, ansible",
                s
            )),
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Jinja2 => write!(f, "jinja2"),
            Mode::Minijinja => write!(f, "minijinja"),
            Mode::Ansible => write!(f, "ansible"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "j2substrs",
    version = "0.1.0",
    about = "Substitute environment variables in templates using Jinja2 syntax",
    long_about = "Reads a j2 template from stdin or file, substitutes variables using {{ VAR }} \
                  syntax, and outputs the result.\n\n\
                  VARIABLES:\n\
                  - All environment variables are automatically available\n\
                  - Use -s/--set to add custom variables\n\
                  - Variables are accessed via {{ VAR_NAME }} in templates\n\n\
                  MODES:\n\
                  - jinja2 (default): Python Jinja2 compatible with method syntax\n\
                  - minijinja: Native filter-based approach (lower overhead)\n\
                  - ansible: Ansible playbooks with Ansible filters and validation\n\n\
                  PREVIEW MODE:\n\
                  - Use --preview to show output before writing to file\n\
                  - Add --yes to auto-confirm (useful for CI/CD pipelines)\n\n\
                  EXAMPLES:\n\
                  echo 'App: {{ APP }}' | j2substrs -s APP=myapp
                  j2substrs template.txt -s VAR=value --output result.txt
                  j2substrs --file config.txt --preview --yes -s NAME=test"
)]
struct Args {
    /// Template file to process (uses stdin if not provided)
    /// Can also be specified via --file flag
    /// Example: j2substrs template.txt
    #[arg(value_name = "TEMPLATE")]
    template: Option<PathBuf>,

    /// Read template from file instead of stdin
    /// Shorthand: use TEMPLATE argument instead
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Write rendered output to file instead of stdout
    /// If omitted, output goes to stdout
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Compatibility mode for template rendering
    /// - jinja2 (default): Python Jinja2 compatible, supports method syntax
    /// - minijinja: Filter-based syntax, lower overhead
    /// - ansible: Ansible mode with specialized filters
    #[arg(long, value_name = "MODE", default_value = "jinja2")]
    mode: Mode,

    /// Show rendered output and ask for confirmation before writing
    /// When combined with --output, displays preview and prompts for y/yes/n
    /// Useful for reviewing changes before committing to file
    #[arg(long)]
    preview: bool,

    /// Automatically accept preview confirmation without prompting
    /// When used with --preview, shows output and writes to --output without waiting for user input
    /// Useful for CI/CD pipelines where manual confirmation is not possible
    /// Example: j2substrs --file template.txt --output result.txt --preview --yes
    #[arg(short, long)]
    yes: bool,

    /// Treat undefined variables as errors and exit with status code 1
    /// Default behavior (--skip-missing) replaces undefined variables with empty string
    /// Use this flag to catch template errors early
    #[arg(long)]
    strict: bool,

    /// Replace undefined variables with empty string (default behavior)
    /// This is the default mode; use --strict to fail on undefined variables instead
    #[arg(long)]
    skip_missing: bool,

    /// Set custom template variables (can be used multiple times)
    /// Format: KEY=VALUE
    /// All environment variables are automatically included and can be overridden here
    /// Example: -s APP=myapp -s VERSION=1.0 -s DEBUG=true
    #[arg(short, long, value_name = "KEY=VALUE")]
    set: Vec<String>,

    /// Provide default value for undefined variables (used with --skip-missing)
    /// When a variable is not found in environment or -s flags, use this default value
    /// If not specified, undefined variables become empty strings
    /// Example: --default-value "UNKNOWN"
    #[arg(long, value_name = "VALUE")]
    default_value: Option<String>,

    /// Load Ansible inventory from file
    /// Provides access to groups, hostvars, and group_names in templates
    /// Example: --inventory /etc/ansible/hosts
    #[arg(long, value_name = "FILE")]
    inventory: Option<PathBuf>,

    /// Load Ansible inventory from stdin
    /// Useful for piping inventory data directly to j2substrs
    #[arg(long)]
    inventory_stdin: bool,

    /// Load Ansible inventory from inline YAML or JSON string
    /// Example: --inventory-inline 'all: {hosts: {localhost: {}}}'
    #[arg(long, value_name = "YAML")]
    inventory_inline: Option<String>,

    /// Set current inventory hostname for inventory_hostname variable
    /// When combined with --inventory, provides inventory_hostname in templates
    /// Example: --inventory-hostname web1.example.com
    #[arg(long, value_name = "HOSTNAME")]
    inventory_hostname: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Determine template source
    let template_path = args.file.or(args.template.clone());

    let template_source = if let Some(path) = template_path {
        fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Error reading template file: {}", e);
            std::process::exit(1);
        })
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_else(|e| {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        });
        buf
    };

    // Build context as JSON for proper nested structure support
    let mut context_json = serde_json::Map::new();

    // Add all environment variables as strings
    for (key, value) in std::env::vars() {
        context_json.insert(key, serde_json::Value::String(value));
    }

    // Add custom variables from --set flags
    for var_def in &args.set {
        if let Some((key, value)) = var_def.split_once('=') {
            context_json.insert(key.to_string(), serde_json::Value::String(value.to_string()));
        } else {
            eprintln!("Invalid variable format: '{}' (expected KEY=VALUE)", var_def);
            std::process::exit(1);
        }
    }

    // Load inventory if provided and mode is ansible
    if args.mode == Mode::Ansible && (args.inventory.is_some() || args.inventory_stdin || args.inventory_inline.is_some()) {
        let source = if let Some(path) = &args.inventory {
            InventorySource::File(path.clone())
        } else if args.inventory_stdin {
            InventorySource::Stdin
        } else {
            InventorySource::Inline(args.inventory_inline.clone().unwrap_or_default())
        };

        match Inventory::from_source(source) {
            Ok(inv) => {
                // Merge inventory variables into context (preserving JSON structure)
                let inv_vars = inv.to_template_vars();
                if let Some(obj) = inv_vars.as_object() {
                    for (k, v) in obj {
                        context_json.insert(k.clone(), v.clone());
                    }
                }

                // Add group-level variables from all groups (Ansible pattern)
                for (group_name, group_info) in &inv.groups {
                    for (var_key, var_value) in &group_info.vars {
                        // Only add from "all" group at top level to match Ansible behavior
                        if group_name == "all" {
                            context_json.insert(var_key.clone(), var_value.clone());
                        }
                    }
                }

                // Add current inventory hostname if provided
                if let Some(hostname) = &args.inventory_hostname {
                    context_json.insert("inventory_hostname".to_string(), serde_json::Value::String(hostname.clone()));
                }
                eprintln!("[j2substrs] Inventory loaded with {} hosts", inv.hosts.len());
            }
            Err(e) => {
                eprintln!("Error loading inventory: {}", e);
                std::process::exit(1);
            }
        }
    }

    let json_ctx = serde_json::Value::Object(context_json);

    // Apply mode-specific configuration
    eprintln!("[j2substrs] Using {} mode", args.mode);
    match args.mode {
        Mode::Jinja2 => {
            // Default Jinja2 mode — Python method syntax enabled
            eprintln!("[j2substrs] Jinja2 mode: Python method syntax enabled");
        }
        Mode::Minijinja => {
            // Native minijinja mode — filter-based syntax only
            eprintln!("[j2substrs] Minijinja mode: filter-based syntax (no methods)");
        }
        Mode::Ansible => {
            // Ansible mode — include Ansible filters and variables
            eprintln!("[j2substrs] Ansible mode: Ansible filters and validation enabled");
        }
    }

    // Create Jinja2 environment
    let env = jinja2rs::Environment::new();

    // Configure based on --strict flag
    if args.strict || !args.skip_missing {
        // Strict mode: undefined variables cause errors
        // This is the default behavior in minijinja
    }

    // Render template
    let output = match env.render_str(&template_source, &json_ctx) {
        Ok(result) => result,
        Err(e) => {
            if args.strict {
                eprintln!("Template render error: {}", e);
                std::process::exit(1);
            }
            // In non-strict mode with skip_missing, fall back to original template
            template_source.clone()
        }
    };

    // Handle preview mode
    if args.preview {
        eprintln!("\n=== PREVIEW OUTPUT ===");
        eprintln!("{}", "=".repeat(50));
        eprint!("{}", output);
        eprintln!();  // Add newline after output
        eprintln!("{}", "=".repeat(50));

        if let Some(output_path) = args.output {
            // If --yes flag is present, auto-confirm without prompting
            if args.yes {
                fs::write(&output_path, &output).unwrap_or_else(|e| {
                    eprintln!("Error writing output file: {}", e);
                    std::process::exit(1);
                });
                eprintln!("✓ Written to {} (auto-confirmed)", output_path.display());
            } else {
                // Interactive prompt
                eprint!("Write to {}? (y/yes to confirm): ", output_path.display());
                std::io::stderr().flush().expect("Failed to flush stderr");

                let mut response = String::new();
                io::stdin().read_line(&mut response).unwrap_or_else(|e| {
                    eprintln!("Error reading response: {}", e);
                    std::process::exit(1);
                });

                let response = response.trim().to_lowercase();
                if response == "y" || response == "yes" {
                    fs::write(&output_path, &output).unwrap_or_else(|e| {
                        eprintln!("Error writing output file: {}", e);
                        std::process::exit(1);
                    });
                    eprintln!("✓ Written to {}", output_path.display());
                } else {
                    eprintln!("✗ Cancelled. Output not written.");
                }
            }
        } else {
            eprintln!("[No --output file specified. Preview shown above.]");
        }
    } else {
        // Write output without preview
        if let Some(output_path) = args.output {
            fs::write(&output_path, &output).unwrap_or_else(|e| {
                eprintln!("Error writing output file: {}", e);
                std::process::exit(1);
            });
        } else {
            // Print to stdout directly
            print!("{}", output);
            std::io::stdout().flush().expect("Failed to flush stdout");
        }
    }
}
