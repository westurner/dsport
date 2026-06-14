//! j2substrs — environment variable substitution using Jinja2 templates.
//!
//! This tool substitutes variables in templates using Jinja2 syntax ({{ VAR }}) with values from:
//! - Environment variables (automatically included)
//! - Custom variables (via -s/--set flags)
//! - Data files: YAML/JSON (via --data-file or --data-var flags)
//! - Inventory (Ansible mode)
//! - Kubernetes manifests (Kubernetes mode)
//! - Docker Compose files (Docker Compose mode)
//! - Multiple compatibility modes (jinja2, minijinja, ansible, kubernetes, docker-compose)
//!
//!
//! USAGE EXAMPLES:
//!
//! ============================================================================
//! BASIC USAGE (all modes)
//! ============================================================================
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
//! ============================================================================
//! HTML GENERATION FROM KUBERNETES MANIFESTS
//! ============================================================================
//!
//! Generate HTML documentation from Kubernetes manifests using jinja2/minijinja2 modes.
//! Load manifest data with --data-file and process with standard filters.
//!
//! Basic example - Load manifest as 'manifest' variable:
//!   j2substrs --file html-template.j2 --data-file deployment.yaml \
//!     --output index.html
//!
//! Load manifest with custom variable name:
//!   j2substrs --file report.j2 \
//!     --data-var kubernetes=manifest.yaml \
//!     --output report.html
//!
//! Load multiple data files:
//!   j2substrs --file config.j2 \
//!     --data-file config.yaml \
//!     --data-var schema=schema.json \
//!     --output output.html
//!
//! Process podman generate kube output as HTML:
//!   podman generate kube mycontainer | j2substrs --file template.j2 \
//!     --data-stdin manifest --output manifest.html
//!
//! ============================================================================
//! DATA FILE VARIABLES (--data-file and --data-var)
//! ============================================================================
//!
//! When using --data-file or --data-var with YAML/JSON files, variables are
//! available in templates via {{ manifest }}, {{ config }}, etc.
//!
//! --data-file <FILE>
//!   Load YAML/JSON file into a variable named 'manifest'
//!   Can be specified MULTIPLE TIMES — files are merged in order (later files override)
//!   Usage: --data-file base.yaml --data-file overrides.yaml
//!   Template access: {{ manifest }}, {{ manifest.metadata.name }}, etc.
//!   Merge behavior:
//!     - JSON objects are merged recursively
//!     - For non-object values, later files override earlier ones
//!     - Non-conflicting keys from all files are preserved
//!   Example (3-level merge):
//!     --data-file base.yaml --data-file prod.yaml --data-file local.yaml
//!     Result: base ← prod ← local (local overrides prod overrides base)
//!
//! --data-var <NAME>=<FILE>
//!   Load YAML/JSON file into a named variable
//!   Can be specified multiple times for different variables
//!   Usage: --data-var config=settings.yaml --data-var schema=schema.json
//!   Template access: {{ config }}, {{ schema }}, etc.
//!   Note: Each variable is independent; use multiple --data-file for merging
//!
//! --data-stdin <NAME>
//!   Read YAML/JSON from stdin into a named variable
//!   Usage: cat manifest.yaml | j2substrs --data-stdin kubernetes ...
//!   Template access: {{ kubernetes }}, {{ kubernetes.items }}, etc.
//!
//! VALIDATION
//! ============================================================================
//! Multiple manifest sources (--manifest, --manifest-stdin, --manifest-inline):
//!   ERROR if more than one is specified
//!   Use only one: --manifest file.yaml OR --manifest-stdin OR --manifest-inline
//!
//! Multiple inventory sources (--inventory, --inventory-stdin, --inventory-inline):
//!   ERROR if more than one is specified
//!   Use only one: --inventory file OR --inventory-stdin OR --inventory-inline
//!
//! Multiple --data-file arguments:
//!   ALLOWED and MERGED (later files override)
//!
//! ============================================================================
//! HTML GENERATION EXAMPLES
//! ============================================================================
//!
//! Simple Pod to HTML (single file):
//!   j2substrs --mode jinja2 --file pod-report.j2 \
//!     --data-file pod.yaml --output report.html
//!
//! Multi-file merge (base config + environment overrides):
//!   j2substrs --mode jinja2 --file report.j2 --output report.html \
//!     --data-file base-config.yaml \
//!     --data-file production-overrides.yaml
//!   Result: base-config merged with production-overrides (prod values on top)
//!
//! Multi-resource manifest to HTML table:
//!   j2substrs --mode minijinja --file resources-table.j2 \
//!     --data-file all-resources.yaml --output resources.html
//!
//! Template example - generate HTML from manifest:
//!   <!DOCTYPE html>
//!   <html>
//!   <head><title>{{ manifest.metadata.name }}</title></head>
//!   <body>
//!     <h1>{{ manifest.kind }}: {{ manifest.metadata.name }}</h1>
//!     <table>
//!       <tr><th>Field</th><th>Value</th></tr>
//!       {% for key, value in manifest.items() %}
//!         <tr><td>{{ key }}</td><td>{{ value }}</td></tr>
//!       {% endfor %}
//!     </table>
//!   </body>
//!   </html>
//!
//! Template with multiple resources (YAML list):
//!   <!DOCTYPE html>
//!   <html>
//!   <body>
//!     <h1>Kubernetes Resources</h1>
//!     {% for item in manifest %}
//!       <div class="resource">
//!         <h2>{{ item.kind }}: {{ item.metadata.name }}</h2>
//!         <p>Namespace: {{ item.metadata.namespace | default('default') }}</p>
//!         <ul>
//!         {% for label_key, label_val in (item.metadata.labels or {}).items() %}
//!           <li>{{ label_key }}: {{ label_val }}</li>
//!         {% endfor %}
//!         </ul>
//!       </div>
//!     {% endfor %}
//!   </body>
//!   </html>
//!
//! Pipeline: podman generate kube → j2substrs → HTML:
//!   podman generate kube my_container | \
//!   j2substrs --mode jinja2 --file container-report.j2 \
//!     --data-stdin container --output report.html
//!
//! Generate HTML from multiple config files:
//!   j2substrs --file dashboard.j2 --output dashboard.html \
//!     --data-var cluster=cluster-config.yaml \
//!     --data-var services=services-config.yaml \
//!     --data-file deployment.yaml
//!
//! ============================================================================
//! ANSIBLE MODE
//! ============================================================================
//!
//! Ansible mode with playbook:
//!   j2substrs --mode ansible --file playbook.yml --output out.yml
//!
//! Load inventory from file:
//!   j2substrs --mode ansible --file playbook.j2 --inventory /etc/ansible/hosts \
//!     --inventory-hostname web1 --output playbook.yml
//!
//! Load inventory from stdin:
//!   cat inventory.yml | j2substrs --mode ansible --file config.j2 --inventory-stdin \
//!     --inventory-hostname web1 --output result.yml
//!
//! Load inventory inline:
//!   j2substrs --mode ansible --file template.j2 \
//!     --inventory-inline 'all: {hosts: {localhost: {}}}' \
//!     --output result.yml
//!
//! Combined: with mode and variables:
//!   j2substrs --mode ansible --file deploy.j2 \
//!     --inventory /etc/ansible/hosts \
//!     --inventory-hostname production-web-01 \
//!     --output deploy.yml \
//!     -s ENV=production -s REGION=us-east-1
//!
//! ============================================================================
//! KUBERNETES MODE
//! ============================================================================
//!
//! Kubernetes mode processes K8s manifests (YAML) from tools like `podman
//! generate kube`, enabling resource selection and metadata filtering.
//!
//! Basic Kubernetes mode with manifest file:
//!   j2substrs --mode kubernetes --file template.j2 --manifest deployment.yaml \
//!     --output result.yaml
//!
//! Process manifest from stdin:
//!   podman generate kube mycontainer | j2substrs --mode kubernetes \
//!     --file template.j2 --manifest-stdin --output manifest.yaml
//!
//! Use inline manifest:
//!   j2substrs --mode kubernetes --file filter.j2 \
//!     --manifest-inline 'apiVersion: v1
//!                        kind: Pod
//!                        metadata: {name: test}' \
//!     --output result.yaml
//!
//! Filter resources by namespace:
//!   j2substrs --mode kubernetes --file template.j2 --manifest config.yaml \
//!     --namespace production --output filtered.yaml
//!
//! Filter by resource kind:
//!   j2substrs --mode kubernetes --file template.j2 --manifest manifest.yaml \
//!     --resource-kind-filter Deployment --output deployments.yaml
//!
//! Combined with variables:
//!   j2substrs --mode kubernetes --file template.j2 --manifest kube.yaml \
//!     --namespace prod --output result.yaml \
//!     -s REPLICAS=3 -s IMAGE_TAG=v1.2.3
//!
//! ============================================================================
//! KUBERNETES TEMPLATE VARIABLES
//! ============================================================================
//!
//! When using --mode kubernetes with --manifest, the following variables are
//! available in templates:
//!
//! kubernetes_resources
//!   Dictionary mapping resource kinds to dictionaries of resources.
//!   Example: kubernetes_resources.Deployment, kubernetes_resources.Pod
//!   Access a specific resource: {{ kubernetes_resources.Deployment['myapp'] }}
//!
//! kubernetes_pods
//!   Shortcut list of all Pod resources
//!   Example: {% for pod in kubernetes_pods %} ... {% endfor %}
//!
//! kubernetes_deployments
//!   Shortcut list of all Deployment resources
//!   Example: {{ kubernetes_deployments[0] | k8s_name }}
//!
//! kubernetes_services
//!   Shortcut list of all Service resources
//!   Example: {% for svc in kubernetes_services %} ... {% endfor %}
//!
//! ============================================================================
//! KUBERNETES FILTERS
//! ============================================================================
//!
//! When in --mode kubernetes, specialized filters are available:
//!
//! k8s_name, k8s_kind, k8s_namespace
//!   Extract metadata from resources
//!   Example: {{ resource | k8s_name }}
//!   Example: {{ resource | k8s_namespace }}
//!
//! replicas
//!   Get replica count from Deployment/StatefulSet (defaults to 1)
//!   Example: {{ deployment | replicas }}
//!
//! container_image
//!   Get first container image from Pod/Deployment spec
//!   Example: {{ pod | container_image }}
//!
//! label, annotation
//!   Access metadata labels and annotations by key
//!   Example: {{ resource | label('app') }}
//!   Example: {{ resource | annotation('description') }}
//!
//! k8s_labels, k8s_annotations
//!   Get all labels or annotations as a dictionary
//!   Example: {{ resource | k8s_labels }}
//!
//! k8s_in_namespace, k8s_has_label
//!   Boolean checks for filtering
//!   Example: {% if resource | k8s_in_namespace('production') %}
//!   Example: {% if resource | k8s_has_label('app', 'myapp') %}
//!
//! ============================================================================
//! KUBERNETES TEMPLATE EXAMPLES
//! ============================================================================
//!
//! List all pods in a namespace:
//!   {% for pod in kubernetes_pods %}
//!     - name: {{ pod | k8s_name }}
//!       namespace: {{ pod | k8s_namespace }}
//!       image: {{ pod | container_image }}
//!   {% endfor %}
//!
//! Filter deployments by label:
//!   {% for deployment in kubernetes_deployments %}
//!     {% if deployment | k8s_has_label('app', 'web') %}
//!     - {{ deployment | k8s_name }}
//!       replicas: {{ deployment | replicas }}
//!     {% endif %}
//!   {% endfor %}
//!
//! Generate resource names with custom prefixes:
//!   {% for resource in kubernetes_resources.Pod %}
//!   prod-{{ resource | k8s_name }}:
//!     image: {{ resource | container_image }}
//!   {% endfor %}
//!
//! Convert K8s resources to other formats:
//!   resources:
//!     {% for pod in kubernetes_pods %}
//!     - kind: {{ pod | k8s_kind }}
//!       name: {{ pod | k8s_name }}
//!       labels: {{ pod | k8s_labels | to_nice_json }}
//!     {% endfor %}
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
//! ============================================================================
//! DOCKER COMPOSE MODE
//! ============================================================================
//!
//! Docker Compose mode processes docker-compose.yml files and makes their contents
//! available to templates through the `docker_compose` variable.
//!
//! Basic Docker Compose mode with file:
//!   j2substrs --mode docker-compose --file template.j2 --compose docker-compose.yml \
//!     --output result.yml
//!
//! Process compose file from stdin:
//!   cat docker-compose.yml | j2substrs --mode docker-compose --file template.j2 \
//!     --compose-stdin --output result.yml
//!
//! Use inline compose definition:
//!   j2substrs --mode docker-compose --file template.j2 \
//!     --compose-inline 'version: "3"
//!                       services:
//!                         web:
//!                           image: nginx:latest
//!                           ports:
//!                             - "80:80"' \
//!     --output result.yml
//!
//! Filter services by name pattern:
//!   j2substrs --mode docker-compose --file template.j2 --compose docker-compose.yml \
//!     --service-filter web --output web-services.yml
//!
//! Merge multiple compose files:
//!   j2substrs --mode docker-compose --file template.j2 \
//!     --compose base.yml --compose production.yml \
//!     --output production.yml
//!
//! ============================================================================
//! DOCKER COMPOSE TEMPLATE VARIABLES
//! ============================================================================
//!
//! When using --mode docker-compose with --compose, the following variables are
//! available in templates:
//!
//! docker_compose.config
//!   Full docker-compose structure (entire parsed YAML)
//!   Example: {{ docker_compose.config.version }}
//!   Example: {{ docker_compose.config.services }}
//!
//! docker_compose.services
//!   Dictionary of all services (shortcut for docker_compose.config.services)
//!   Example: {{ docker_compose.services.web }}
//!   Access a specific service: {{ docker_compose.services['myapp'] }}
//!
//! docker_compose.networks
//!   Dictionary of all networks
//!   Example: {% for net_name, net_cfg in docker_compose.networks.items() %}
//!
//! docker_compose.volumes
//!   Dictionary of all volumes
//!   Example: {% for vol_name in docker_compose.volumes %}
//!
//! ============================================================================
//! DOCKER COMPOSE TEMPLATE EXAMPLES
//! ============================================================================
//!
//! List all services with images:
//!   services:
//!     {% for service_name, service_config in docker_compose.services.items() %}
//!     - name: {{ service_name }}
//!       image: {{ service_config.image | default('unspecified') }}
//!       ports: {{ service_config.ports | default([]) }}
//!     {% endfor %}
//!
//! Generate environment overrides:
//!   {% for name, svc in docker_compose.services.items() %}
//!   {{ name }}_version=1.0
//!   {{ name }}_replicas={{ svc.deploy.replicas | default(1) }}
//!   {% endfor %}
//!
//! Extract port mappings:
//!   ports:
//!     {% for svc_name, svc_cfg in docker_compose.services.items() %}
//!     {% if svc_cfg.ports %}
//!     - service: {{ svc_name }}
//!       mappings:
//!       {% for port_mapping in svc_cfg.ports %}
//!         - {{ port_mapping }}
//!       {% endfor %}
//!     {% endif %}
//!     {% endfor %}
//!
//! Generate monitoring configuration from compose:
//!   monitoring:
//!     {% for name, config in docker_compose.services.items() %}
//!     - service: {{ name }}
//!       image: {{ config.image }}
//!       environment: {{ config.environment | default({}) }}
//!       volumes: {{ config.volumes | default([]) }}
//!     {% endfor %}
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
//! docker-compose (or compose, docker_compose)
//!   Specialized mode for Docker Compose files:
//!   - Loads services, networks, volumes from docker-compose.yml
//!   - Provides docker_compose.config with full compose structure
//!   - Shortcut access: docker_compose.services, docker_compose.networks, docker_compose.volumes
//!   - Optional service filtering with --service-filter
//!   - Supports file, stdin, and inline YAML loading
//!   - YAML validation and merging for multiple compose sources
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
//!     --mode <MODE>               jinja2, minijinja, ansible, kubernetes, docker-compose (default: jinja2)
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
use jinja2rs::kubernetes_inventory::KubernetesManifest;
use jinja2rs::compat::KubernetesInventorySource;
use serde_yaml;

/// Compatibility mode for template rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Drop-in compatible with Python Jinja2 (default)
    Jinja2,
    /// Native minijinja filter-based approach
    Minijinja,
    /// Specialized mode for Ansible playbooks with Ansible filters
    Ansible,
    /// Specialized mode for Kubernetes manifests with K8s filters
    Kubernetes,
    /// Specialized mode for Docker Compose files with container filters
    DockerCompose,
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
            "kubernetes" | "k8s" => Ok(Mode::Kubernetes),
            "docker-compose" | "docker_compose" | "compose" => Ok(Mode::DockerCompose),
            _ => Err(format!(
                "Invalid mode '{}'. Valid modes are: jinja2, minijinja, ansible, kubernetes (k8s), docker-compose (compose)",
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
            Mode::Kubernetes => write!(f, "kubernetes"),
            Mode::DockerCompose => write!(f, "docker-compose"),
        }
    }
}

/// Merge strategy for combining multiple sources
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MergeStrategy {
    /// First source wins, later sources are ignored
    FirstWins,
    /// Last source wins, later sources override earlier ones (default)
    LastWins,
    /// Merge arrays and objects recursively
    MergeLists,
}

impl Default for MergeStrategy {
    fn default() -> Self {
        MergeStrategy::LastWins
    }
}

impl FromStr for MergeStrategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "first-wins" | "first_wins" | "first" => Ok(MergeStrategy::FirstWins),
            "last-wins" | "last_wins" | "last" => Ok(MergeStrategy::LastWins),
            "merge-lists" | "merge_lists" | "merge" => Ok(MergeStrategy::MergeLists),
            _ => Err(format!(
                "Invalid merge strategy '{}'. Valid strategies are: first-wins, last-wins, merge-lists",
                s
            )),
        }
    }
}

impl std::fmt::Display for MergeStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MergeStrategy::FirstWins => write!(f, "first-wins"),
            MergeStrategy::LastWins => write!(f, "last-wins"),
            MergeStrategy::MergeLists => write!(f, "merge-lists"),
        }
    }
}

/// Helper struct for per-source merge strategy
/// Supports syntax: path.yaml or path.yaml:merge-lists
#[derive(Debug, Clone)]
struct SourceWithStrategy {
    pub path: PathBuf,
    pub strategy: Option<MergeStrategy>,
}

impl SourceWithStrategy {
    /// Parse a source spec in the format "path" or "path:strategy"
    fn parse(spec: &str) -> Result<Self, String> {
        if let Some((path_str, strategy_str)) = spec.rsplit_once(':') {
            // Check if this looks like a strategy or part of a Windows path (e.g., C:\path)
            // Windows paths have : followed by \, strategies have letters
            if strategy_str.len() > 1 && strategy_str.starts_with('\\') {
                // Windows path, no strategy
                Ok(SourceWithStrategy {
                    path: PathBuf::from(spec),
                    strategy: None,
                })
            } else if strategy_str.chars().all(|c| c.is_alphabetic() || c == '-' || c == '_') {
                // Looks like a strategy
                let strategy = MergeStrategy::from_str(strategy_str)?;
                Ok(SourceWithStrategy {
                    path: PathBuf::from(path_str),
                    strategy: Some(strategy),
                })
            } else {
                // Not a strategy, treat whole thing as path
                Ok(SourceWithStrategy {
                    path: PathBuf::from(spec),
                    strategy: None,
                })
            }
        } else {
            Ok(SourceWithStrategy {
                path: PathBuf::from(spec),
                strategy: None,
            })
        }
    }

    /// Get the effective merge strategy, using per-source strategy if set, else global default
    fn get_strategy(&self, global_strategy: MergeStrategy) -> MergeStrategy {
        self.strategy.unwrap_or(global_strategy)
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
                  - ansible: Ansible playbooks with Ansible filters and validation\n\
                  - kubernetes: Kubernetes manifests with K8s-specific filters\n\
                  - docker-compose: Docker Compose files with container filters\n\n\
                  PREVIEW MODE:\n\
                  - Use --preview to show output before writing to file\n\
                  - Add --yes to auto-confirm (useful for CI/CD pipelines)\n\n\
                  EXAMPLES:\n\
                  echo 'App: {{ APP }}' | j2substrs -s APP=myapp\n\
                  j2substrs template.txt -s VAR=value --output result.txt\n\
                  j2substrs --file config.txt --preview --yes -s NAME=test\n\
                  j2substrs --mode docker-compose --compose docker-compose.yml --file deploy.j2"
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
    /// - kubernetes: Kubernetes manifests with K8s-specific filters
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

    /// Load Ansible inventory from file (--mode ansible only)
    /// Provides access to groups, hostvars, and group_names in templates
    /// Can be specified multiple times when used with --allow-layering
    /// Per-source merge strategy: FILE or FILE:strategy (e.g., hosts:merge-lists)
    /// Example: --inventory base_hosts --inventory prod_hosts:last-wins
    #[arg(long, value_name = "FILE[:STRATEGY]")]
    inventory: Vec<String>,
    /// Can be specified multiple times when used with --allow-layering
    #[arg(long)]
    inventory_stdin: bool,

    /// Load Ansible inventory from inline YAML or JSON string (--mode ansible only)
    /// Can be specified multiple times when used with --allow-layering
    /// Example: --inventory-inline 'all: {hosts: {localhost: {}}}'
    #[arg(long, value_name = "YAML")]
    inventory_inline: Vec<String>,

    /// Set current inventory hostname for inventory_hostname variable (--mode ansible only)
    /// When combined with --inventory, provides inventory_hostname in templates
    /// Example: --inventory-hostname web1.example.com
    #[arg(long, value_name = "HOSTNAME")]
    inventory_hostname: Option<String>,

    /// Load Kubernetes manifest from file (--mode kubernetes only)
    /// Provides access to kubernetes_resources, kubernetes_pods, kubernetes_deployments, etc.
    /// Can be specified multiple times when used with --allow-layering
    /// Per-source merge strategy: FILE or FILE:strategy (e.g., deployment.yaml:merge-lists)
    /// Example: --manifest base.yaml --manifest prod.yaml:last-wins
    #[arg(long, value_name = "FILE[:STRATEGY]")]
    manifest: Vec<String>,

    /// Load Kubernetes manifest from stdin (--mode kubernetes only)
    /// Useful for piping kubectl output or podman generate kube directly to j2substrs
    /// Can be specified multiple times when used with --allow-layering
    /// Example: podman generate kube mycontainer | j2substrs --mode kubernetes --manifest-stdin
    #[arg(long)]
    manifest_stdin: bool,

    /// Load Kubernetes manifest from inline YAML string (--mode kubernetes only)
    /// Useful for inline manifests in scripts
    /// Can be specified multiple times when used with --allow-layering
    /// Example: --manifest-inline 'apiVersion: v1\nkind: Pod\nmetadata: {name: test}'
    #[arg(long, value_name = "YAML")]
    manifest_inline: Vec<String>,

    /// Filter Kubernetes resources by namespace (--mode kubernetes only)
    /// Only processes resources in the specified namespace
    /// Example: --namespace production
    #[arg(long, value_name = "NAMESPACE")]
    namespace: Option<String>,

    /// Filter Kubernetes resources by kind (--mode kubernetes only)
    /// Only processes resources of the specified kind (e.g., Pod, Deployment, Service)
    /// Example: --resource-kind-filter Deployment
    #[arg(long, value_name = "KIND")]
    resource_kind_filter: Option<String>,

    /// Load Docker Compose file (--mode docker-compose only)
    /// Provides access to docker_compose_services, docker_compose_networks, etc.
    /// Can be specified multiple times when used with --allow-layering
    /// Per-source merge strategy: FILE or FILE:strategy (e.g., base.yml:merge-lists)
    /// Example: --compose base.yml --compose prod.yml:last-wins
    #[arg(long, value_name = "FILE[:STRATEGY]")]
    compose: Vec<String>,

    /// Load Docker Compose file from stdin (--mode docker-compose only)
    /// Useful for piping Docker Compose YAML directly to j2substrs
    /// Can be specified multiple times when used with --allow-layering
    /// Example: cat docker-compose.yml | j2substrs --mode docker-compose --compose-stdin
    #[arg(long)]
    compose_stdin: bool,

    /// Load Docker Compose file from inline YAML string (--mode docker-compose only)
    /// Useful for inline compose definitions in scripts
    /// Can be specified multiple times when used with --allow-layering
    /// Example: --compose-inline 'version: "3"\nservices:\n  web:\n    image: nginx'
    #[arg(long, value_name = "YAML")]
    compose_inline: Vec<String>,

    /// Filter Docker Compose services by name (--mode docker-compose only)
    /// Only processes services matching the specified name pattern
    /// Example: --service-filter web
    #[arg(long, value_name = "PATTERN")]
    service_filter: Option<String>,

    /// Load YAML or JSON data file into a variable (for jinja2/minijinja modes)
    /// File is parsed and made available as a template variable named 'manifest'
    /// Can be specified multiple times — files are merged in order according to merge strategy
    /// Per-source merge strategy: FILE or FILE:strategy (e.g., base.yaml:first-wins)
    /// Useful for generating HTML or other formats from data files
    /// Example: --data-file base.yaml --data-file overrides.yaml:merge-lists
    /// Template access: {{ manifest }}, {{ manifest.key }}, etc.
    #[arg(long, value_name = "FILE[:STRATEGY]")]
    data_file: Vec<String>,

    /// Load YAML or JSON data file into a named variable (for jinja2/minijinja modes)
    /// Format: VARNAME=FILE
    /// Useful for loading multiple data sources with different names
    /// Example: --data-var config=settings.yaml --data-var schema=schema.json
    /// Template access: {{ config }}, {{ schema }}, etc.
    #[arg(long, value_name = "NAME=FILE")]
    data_var: Vec<String>,

    /// Load YAML or JSON data from stdin into a named variable (for jinja2/minijinja modes)
    /// Useful for piping data directly to j2substrs
    /// Example: cat manifest.yaml | j2substrs --data-stdin kubernetes ...
    /// Template access: {{ kubernetes }}, {{ kubernetes.items }}, etc.
    #[arg(long, value_name = "NAME")]
    data_stdin: Option<String>,

    /// Allow layering of manifest and inventory sources
    /// When enabled, allows multiple --manifest, --manifest-stdin, --manifest-inline
    /// and multiple --inventory, --inventory-stdin, --inventory-inline to be merged
    /// Later sources override earlier sources (like --data-file behavior)
    /// When disabled (default), multiple sources cause an error
    /// Example: --allow-layering --manifest base.yaml --manifest prod-overrides.yaml
    #[arg(long)]
    allow_layering: bool,

    /// Merge strategy for combining multiple data sources
    /// - first-wins: First source is used, later sources are ignored
    /// - last-wins (default): Last source wins, later sources override earlier ones
    /// - merge-lists: Recursively merge objects and arrays from all sources
    /// Used when multiple --data-file, --manifest, --inventory, or --compose sources are provided
    /// Example: --merge-strategy last-wins
    #[arg(long, value_name = "STRATEGY", default_value = "last-wins")]
    merge_strategy: MergeStrategy,

    /// Enable merge diff logging
    /// When enabled, logs which keys were changed, added, or removed during merge operations
    /// Useful for debugging merge conflicts and understanding data transformations
    /// Output goes to stderr with [merge-diff] prefix
    /// Example: --merge-strategy merge-lists --merge-diff-log
    #[arg(long)]
    merge_diff_log: bool,
}

/// Recursively merge JSON values according to merge strategy
/// - FirstWins: Never modifies base, other is ignored
/// - LastWins: other overwrites base
/// - MergeLists: Merges arrays and objects recursively
/// Returns diff information if merge_diff_log is enabled
fn merge_json_values_with_strategy(
    base: &mut serde_json::Value,
    other: serde_json::Value,
    strategy: MergeStrategy,
    merge_diff_log: bool,
    path: String,
) {
    match strategy {
        MergeStrategy::FirstWins => {
            // Do nothing, keep base as-is
        }
        MergeStrategy::LastWins => {
            // Perform recursive merge with last-wins semantics
            merge_json_values_last_wins(base, other, merge_diff_log, &path);
        }
        MergeStrategy::MergeLists => {
            // Merge with array merging support
            merge_json_values_merge_lists(base, other, merge_diff_log, &path);
        }
    }
}

/// Merge with last-wins semantics (default): later values override earlier ones
/// If both are objects, merges recursively; otherwise other overwrites base
fn merge_json_values_last_wins(
    base: &mut serde_json::Value,
    other: serde_json::Value,
    merge_diff_log: bool,
    path: &str,
) {
    match (&mut *base, &other) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(other_map)) => {
            for (key, other_val) in other_map {
                let full_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                if let Some(base_val) = base_map.get_mut(key) {
                    let old_val = base_val.clone();
                    merge_json_values_last_wins(base_val, other_val.clone(), merge_diff_log, &full_path);
                    if merge_diff_log && *base_val != old_val {
                        eprintln!("[merge-diff] Modified: {} (was: {})", full_path, old_val.to_string().chars().take(50).collect::<String>());
                    }
                } else {
                    if merge_diff_log {
                        eprintln!("[merge-diff] Added: {}", full_path);
                    }
                    base_map.insert(key.clone(), other_val.clone());
                }
            }

            // Log removed keys (keys in base but not in other)
            if merge_diff_log {
                let removed_keys: Vec<_> = base_map
                    .keys()
                    .filter(|k| !other_map.contains_key(*k))
                    .cloned()
                    .collect();
                for key in removed_keys {
                    let full_path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{}.{}", path, key)
                    };
                    eprintln!("[merge-diff] Unchanged (not in source): {}", full_path);
                }
            }
        }
        _ => {
            if merge_diff_log && *base != other {
                let old_val = base.clone();
                eprintln!(
                    "[merge-diff] Modified: {} (was: {})",
                    path,
                    old_val.to_string().chars().take(50).collect::<String>()
                );
            }
            *base = other;
        }
    }
}

/// Merge with array merging: concatenate arrays, merge objects recursively
fn merge_json_values_merge_lists(
    base: &mut serde_json::Value,
    other: serde_json::Value,
    merge_diff_log: bool,
    path: &str,
) {
    match (&mut *base, &other) {
        (serde_json::Value::Object(base_map), serde_json::Value::Object(other_map)) => {
            for (key, other_val) in other_map {
                let full_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };

                if let Some(base_val) = base_map.get_mut(key) {
                    // Check if both are arrays to merge them
                    if let serde_json::Value::Array(base_arr) = base_val {
                        if let serde_json::Value::Array(other_arr) = &other_val {
                            if merge_diff_log {
                                eprintln!("[merge-diff] Merged arrays at {}: {} + {} items", full_path, base_arr.len(), other_arr.len());
                            }
                            base_arr.extend(other_arr.clone());
                        } else {
                            let old_val = base_val.clone();
                            merge_json_values_merge_lists(base_val, other_val.clone(), merge_diff_log, &full_path);
                            if merge_diff_log && *base_val != old_val {
                                eprintln!("[merge-diff] Modified: {} (was: {})", full_path, old_val.to_string().chars().take(50).collect::<String>());
                            }
                        }
                    } else {
                        let old_val = base_val.clone();
                        merge_json_values_merge_lists(base_val, other_val.clone(), merge_diff_log, &full_path);
                        if merge_diff_log && *base_val != old_val {
                            eprintln!("[merge-diff] Modified: {} (was: {})", full_path, old_val.to_string().chars().take(50).collect::<String>());
                        }
                    }
                } else {
                    if merge_diff_log {
                        eprintln!("[merge-diff] Added: {}", full_path);
                    }
                    base_map.insert(key.clone(), other_val.clone());
                }
            }
        }
        (serde_json::Value::Array(base_arr), serde_json::Value::Array(other_arr)) => {
            if merge_diff_log {
                eprintln!("[merge-diff] Merged arrays at {}: {} + {} items", path, base_arr.len(), other_arr.len());
            }
            base_arr.extend(other_arr.clone());
        }
        _ => {
            if merge_diff_log && *base != other {
                let old_val = base.clone();
                eprintln!(
                    "[merge-diff] Modified: {} (was: {})",
                    path,
                    old_val.to_string().chars().take(50).collect::<String>()
                );
            }
            *base = other;
        }
    }
}

fn main() {
    let args = Args::parse();

    // Validate that conflicting manifest sources are not provided simultaneously
    // (unless --allow-layering is enabled)
    let manifest_sources = [
        !args.manifest.is_empty(),
        args.manifest_stdin,
        !args.manifest_inline.is_empty(),
    ].iter().filter(|&&x| x).count();
    
    if manifest_sources > 1 && !args.allow_layering {
        eprintln!("Error: Cannot specify multiple manifest sources");
        eprintln!("Use only one of: --manifest <FILE>, --manifest-stdin, or --manifest-inline");
        eprintln!("Or use --allow-layering to merge multiple sources");
        std::process::exit(1);
    }

    // Validate that conflicting inventory sources are not provided simultaneously
    // (unless --allow-layering is enabled)
    let inventory_sources = [
        !args.inventory.is_empty(),
        args.inventory_stdin,
        !args.inventory_inline.is_empty(),
    ].iter().filter(|&&x| x).count();
    
    if inventory_sources > 1 && !args.allow_layering {
        eprintln!("Error: Cannot specify multiple inventory sources");
        eprintln!("Use only one of: --inventory <FILE>, --inventory-stdin, or --inventory-inline");
        eprintln!("Or use --allow-layering to merge multiple sources");
        std::process::exit(1);
    }

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

    // Load and merge data files into 'manifest' variable (can be specified multiple times)
    if !args.data_file.is_empty() {
        let mut merged_data = serde_json::Value::Object(serde_json::Map::new());
        let mut first_wins_encountered = false;
        
        for (idx, data_spec) in args.data_file.iter().enumerate() {
            let source = match SourceWithStrategy::parse(data_spec) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing data source '{}': {}", data_spec, e);
                    std::process::exit(1);
                }
            };
            let effective_strategy = source.get_strategy(args.merge_strategy);
            
            // If first-wins has been encountered in a previous source, skip all remaining sources
            if first_wins_encountered {
                eprintln!("[j2substrs] Data file '{}' skipped (first-wins strategy from earlier source)", source.path.display());
                continue;
            }
            
            // Track if this source is using first-wins for next iteration
            if idx == 0 && effective_strategy == MergeStrategy::FirstWins {
                first_wins_encountered = true;
            }
            
            match fs::read_to_string(&source.path) {
                Ok(content) => {
                    match serde_yaml::from_str::<serde_json::Value>(&content) {
                        Ok(data) => {
                            // Merge data into manifest according to effective merge strategy
                            merge_json_values_with_strategy(&mut merged_data, data, effective_strategy, args.merge_diff_log, "manifest".to_string());
                            eprintln!("[j2substrs] Data file '{}' loaded with {} merge strategy", source.path.display(), effective_strategy);
                        }
                        Err(e) => {
                            eprintln!("Error parsing data file '{}': {}", source.path.display(), e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading data file '{}': {}", source.path.display(), e);
                    std::process::exit(1);
                }
            }
        }
        
        context_json.insert("manifest".to_string(), merged_data);
    }

    // Load named data variables from files
    for var_def in &args.data_var {
        if let Some((var_name, file_path)) = var_def.split_once('=') {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    match serde_yaml::from_str::<serde_json::Value>(&content) {
                        Ok(data) => {
                            context_json.insert(var_name.to_string(), data);
                            eprintln!("[j2substrs] Data file '{}' loaded into '{}' variable", file_path, var_name);
                        }
                        Err(e) => {
                            eprintln!("Error parsing data file '{}': {}", file_path, e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading data file '{}': {}", file_path, e);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Invalid data-var format: '{}' (expected NAME=FILE)", var_def);
            std::process::exit(1);
        }
    }

    // Load data from stdin into a named variable
    if let Some(var_name) = &args.data_stdin {
        let mut stdin_content = String::new();
        match io::stdin().read_to_string(&mut stdin_content) {
            Ok(_) => {
                match serde_yaml::from_str::<serde_json::Value>(&stdin_content) {
                    Ok(data) => {
                        context_json.insert(var_name.clone(), data);
                        eprintln!("[j2substrs] Data from stdin loaded into '{}' variable", var_name);
                    }
                    Err(e) => {
                        eprintln!("Error parsing stdin data: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Load inventory if provided and mode is ansible
    if args.mode == Mode::Ansible && (!args.inventory.is_empty() || args.inventory_stdin || !args.inventory_inline.is_empty()) {
        let mut sources: Vec<(InventorySource, MergeStrategy)> = Vec::new();
        let mut first_wins_encountered = false;
        
        // Collect all sources with their per-source strategies
        for inventory_spec in &args.inventory {
            let source_spec = match SourceWithStrategy::parse(inventory_spec) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing inventory source '{}': {}", inventory_spec, e);
                    std::process::exit(1);
                }
            };
            let effective_strategy = source_spec.get_strategy(args.merge_strategy);
            sources.push((InventorySource::File(source_spec.path), effective_strategy));
        }
        if args.inventory_stdin {
            sources.push((InventorySource::Stdin, args.merge_strategy));
        }
        for inline in &args.inventory_inline {
            sources.push((InventorySource::Inline(inline.clone()), args.merge_strategy));
        }

        for (idx, (source, effective_strategy)) in sources.iter().enumerate() {
            // If first-wins has been encountered in a previous source, skip remaining sources
            if first_wins_encountered {
                eprintln!("[j2substrs] Inventory source #{} skipped (first-wins strategy from earlier source)", idx + 1);
                continue;
            }

            // Track if this source is using first-wins for next iteration
            if idx == 0 && *effective_strategy == MergeStrategy::FirstWins {
                first_wins_encountered = true;
            }

            match Inventory::from_source(source.clone()) {
                Ok(inv) => {
                    // Merge inventory variables into context (preserving JSON structure)
                    // For last-wins strategy, insert (overwrites previous)
                    // For merge-lists strategy, we'd need to deep-merge, but Ansible inventory is typically additive
                    let inv_vars = inv.to_template_vars();
                    if let Some(obj) = inv_vars.as_object() {
                        for (k, v) in obj {
                            if *effective_strategy == MergeStrategy::LastWins || idx == 0 {
                                context_json.insert(k.clone(), v.clone());
                            } else if *effective_strategy == MergeStrategy::MergeLists {
                                let mut existing = context_json.get(k).cloned().unwrap_or(serde_json::Value::Null);
                                merge_json_values_with_strategy(&mut existing, v.clone(), *effective_strategy, args.merge_diff_log, k.clone());
                                context_json.insert(k.clone(), existing);
                            }
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
                    
                    if args.allow_layering && sources.len() > 1 {
                        eprintln!("[j2substrs] Inventory source #{} loaded and merged with {} hosts ({})", idx + 1, inv.hosts.len(), effective_strategy);
                    } else {
                        eprintln!("[j2substrs] Inventory loaded with {} hosts", inv.hosts.len());
                    }
                }
                Err(e) => {
                    eprintln!("Error loading inventory: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Load Kubernetes manifest if provided and mode is kubernetes
    // Load Kubernetes manifest in both Kubernetes and Ansible modes
    if (args.mode == Mode::Kubernetes || args.mode == Mode::Ansible) && (!args.manifest.is_empty() || args.manifest_stdin || !args.manifest_inline.is_empty()) {
        let mut sources: Vec<(KubernetesInventorySource, MergeStrategy)> = Vec::new();
        let mut first_wins_encountered = false;
        
        // Collect all sources with their per-source strategies
        for manifest_spec in &args.manifest {
            let source_spec = match SourceWithStrategy::parse(manifest_spec) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing manifest source '{}': {}", manifest_spec, e);
                    std::process::exit(1);
                }
            };
            let effective_strategy = source_spec.get_strategy(args.merge_strategy);
            sources.push((KubernetesInventorySource::File(source_spec.path), effective_strategy));
        }
        if args.manifest_stdin {
            sources.push((KubernetesInventorySource::Stdin, args.merge_strategy));
        }
        for inline in &args.manifest_inline {
            sources.push((KubernetesInventorySource::Inline(inline.clone()), args.merge_strategy));
        }

        for (idx, (source, effective_strategy)) in sources.iter().enumerate() {
            // If first-wins has been encountered in a previous source, skip remaining sources
            if first_wins_encountered {
                eprintln!("[j2substrs] Kubernetes manifest source #{} skipped (first-wins strategy from earlier source)", idx + 1);
                continue;
            }

            // Track if this source is using first-wins for next iteration
            if idx == 0 && *effective_strategy == MergeStrategy::FirstWins {
                first_wins_encountered = true;
            }

            match KubernetesManifest::from_source(source.clone()) {
                Ok(manifest) => {
                    // Merge Kubernetes resources into context according to effective strategy
                    let k8s_vars = manifest.to_template_vars();
                    if let Some(obj) = k8s_vars.as_object() {
                        for (k, v) in obj {
                            let mut existing = context_json.get(k).cloned().unwrap_or(serde_json::Value::Null);
                            merge_json_values_with_strategy(&mut existing, v.clone(), *effective_strategy, args.merge_diff_log, k.clone());
                            context_json.insert(k.clone(), existing);
                        }
                        if args.allow_layering && sources.len() > 1 {
                            eprintln!("[j2substrs] Kubernetes manifest source #{} merged with {} resources ({})", idx + 1, manifest.resources.len(), effective_strategy);
                        } else {
                            eprintln!("[j2substrs] Kubernetes manifest loaded with {} resources", manifest.resources.len());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error loading Kubernetes manifest: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    // Load Docker Compose file if provided and mode is docker-compose
    if args.mode == Mode::DockerCompose && (!args.compose.is_empty() || args.compose_stdin || !args.compose_inline.is_empty()) {
        let mut compose_data = serde_json::json!({});
        let mut first_wins_encountered = false;

        // Load from files with per-source merge strategy
        for (idx, compose_spec) in args.compose.iter().enumerate() {
            // If first-wins has been encountered in a previous source, skip remaining sources
            if first_wins_encountered {
                let source_spec = match SourceWithStrategy::parse(compose_spec) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                eprintln!("[j2substrs] Docker Compose file '{}' skipped (first-wins strategy from earlier source)", source_spec.path.display());
                continue;
            }

            let source_spec = match SourceWithStrategy::parse(compose_spec) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error parsing compose source '{}': {}", compose_spec, e);
                    std::process::exit(1);
                }
            };
            let effective_strategy = source_spec.get_strategy(args.merge_strategy);
            
            // Track if this source is using first-wins for next iteration
            if idx == 0 && effective_strategy == MergeStrategy::FirstWins {
                first_wins_encountered = true;
            }
            
            match fs::read_to_string(&source_spec.path) {
                Ok(content) => {
                    match serde_yaml::from_str::<serde_json::Value>(&content) {
                        Ok(data) => {
                            merge_json_values_with_strategy(&mut compose_data, data, effective_strategy, args.merge_diff_log, "docker_compose".to_string());
                            eprintln!("[j2substrs] Docker Compose file '{}' loaded with {} merge strategy", source_spec.path.display(), effective_strategy);
                        }
                        Err(e) => {
                            eprintln!("Error parsing Docker Compose file '{}': {}", source_spec.path.display(), e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading Docker Compose file '{}': {}", source_spec.path.display(), e);
                    std::process::exit(1);
                }
            }
        }

        // Load from stdin
        if args.compose_stdin && !first_wins_encountered {
            let mut stdin_content = String::new();
            match io::stdin().read_to_string(&mut stdin_content) {
                Ok(_) => {
                    match serde_yaml::from_str::<serde_json::Value>(&stdin_content) {
                        Ok(data) => {
                            merge_json_values_with_strategy(&mut compose_data, data, args.merge_strategy, args.merge_diff_log, "docker_compose".to_string());
                            eprintln!("[j2substrs] Docker Compose file loaded from stdin");
                        }
                        Err(e) => {
                            eprintln!("Error parsing Docker Compose from stdin: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading Docker Compose from stdin: {}", e);
                    std::process::exit(1);
                }
            }
        } else if args.compose_stdin && first_wins_encountered {
            eprintln!("[j2substrs] Docker Compose from stdin skipped (first-wins strategy from earlier source)");
        }

        // Load from inline
        for (idx, inline) in args.compose_inline.iter().enumerate() {
            // If first-wins has been encountered, skip remaining sources
            if first_wins_encountered {
                eprintln!("[j2substrs] Inline Docker Compose skipped (first-wins strategy from earlier source)");
                continue;
            }

            match serde_yaml::from_str::<serde_json::Value>(inline) {
                Ok(data) => {
                    let compose_idx = args.compose.len() + idx;
                    if compose_idx == 0 && args.merge_strategy == MergeStrategy::FirstWins {
                        first_wins_encountered = true;
                    }
                    merge_json_values_with_strategy(&mut compose_data, data, args.merge_strategy, args.merge_diff_log, "docker_compose".to_string());
                    eprintln!("[j2substrs] Docker Compose from inline loaded");
                }
                Err(e) => {
                    eprintln!("Error parsing inline Docker Compose: {}", e);
                    std::process::exit(1);
                }
            }
        }

        // Extract services and apply filters
        if let Some(services_obj) = compose_data.get_mut("services") {
            if let Some(services_map) = services_obj.as_object_mut() {
                // Apply service filter if provided
                if let Some(pattern) = &args.service_filter {
                    let filtered_services: Vec<String> = services_map
                        .keys()
                        .filter(|name| name.contains(pattern.as_str()))
                        .cloned()
                        .collect();

                    let mut filtered_map = serde_json::Map::new();
                    for key in filtered_services {
                        if let Some(value) = services_map.remove(&key) {
                            filtered_map.insert(key, value);
                        }
                    }
                    *services_map = filtered_map;

                    eprintln!("[j2substrs] Docker Compose services filtered by pattern '{}': {} matching services", pattern, services_map.len());
                } else {
                    eprintln!("[j2substrs] Docker Compose loaded with {} services", services_map.len());
                }
            }
        }

        // Add Docker Compose data to context with helper shortcuts
        let mut docker_vars = serde_json::Map::new();
        docker_vars.insert("config".to_string(), compose_data.clone());

        // Extract services list for easier access
        if let Some(services) = compose_data.get("services") {
            docker_vars.insert("services".to_string(), services.clone());
        }

        // Extract networks list for easier access
        if let Some(networks) = compose_data.get("networks") {
            docker_vars.insert("networks".to_string(), networks.clone());
        }

        // Extract volumes list for easier access
        if let Some(volumes) = compose_data.get("volumes") {
            docker_vars.insert("volumes".to_string(), volumes.clone());
        }

        context_json.insert("docker_compose".to_string(), serde_json::Value::Object(docker_vars));
    }

    let json_ctx = serde_json::Value::Object(context_json);

    // Create Jinja2 environment
    let mut env = jinja2rs::Environment::new();

    // Configure compat mode based on --mode flag
    match args.mode {
        Mode::Jinja2 => {
            env.set_compat_mode(jinja2rs::compat::CompatMode::Jinja2);
            eprintln!("[j2substrs] Using jinja2 mode");
            eprintln!("[j2substrs] Jinja2 mode: Python method syntax enabled");
        }
        Mode::Minijinja => {
            env.set_compat_mode(jinja2rs::compat::CompatMode::Minijinja);
            eprintln!("[j2substrs] Using minijinja mode");
            eprintln!("[j2substrs] Minijinja mode: filter-based syntax (no methods)");
        }
        Mode::Ansible => {
            let ansible_mode = jinja2rs::compat::AnsibleMode {
                method_syntax: true,
                enable_validation: true,
                inventory_source: args.inventory.first()
                    .map(|p| jinja2rs::compat::AnsibleInventorySource::File(PathBuf::from(p)))
                    .or_else(|| if args.inventory_stdin {
                        Some(jinja2rs::compat::AnsibleInventorySource::Stdin)
                    } else {
                        None
                    })
                    .or_else(|| args.inventory_inline.first()
                        .map(|c| jinja2rs::compat::AnsibleInventorySource::Inline(c.clone()))),
            };
            env.set_compat_mode(jinja2rs::compat::CompatMode::Ansible(ansible_mode));
            eprintln!("[j2substrs] Using ansible mode");
            if !args.manifest.is_empty() || args.manifest_stdin || !args.manifest_inline.is_empty() {
                eprintln!("[j2substrs] Ansible mode with Kubernetes manifest data loaded");
            }
        }
        Mode::Kubernetes => {
            let k8s_mode = jinja2rs::compat::KubernetesMode {
                method_syntax: true,
                enable_validation: true,
                manifest_source: args.manifest.first()
                    .map(|p| jinja2rs::compat::KubernetesInventorySource::File(PathBuf::from(p)))
                    .or_else(|| if args.manifest_stdin {
                        Some(jinja2rs::compat::KubernetesInventorySource::Stdin)
                    } else {
                        None
                    })
                    .or_else(|| args.manifest_inline.first()
                        .map(|c| jinja2rs::compat::KubernetesInventorySource::Inline(c.clone()))),
                namespace: args.namespace.clone().unwrap_or_else(|| "default".to_string()),
                resource_kind_filter: args.resource_kind_filter.clone(),
            };
            env.set_compat_mode(jinja2rs::compat::CompatMode::Kubernetes(k8s_mode));
            eprintln!("[j2substrs] Using kubernetes mode");
        }
        Mode::DockerCompose => {
            // Docker Compose mode with Jinja2 features
            env.set_compat_mode(jinja2rs::compat::CompatMode::Jinja2);
            eprintln!("[j2substrs] Using docker-compose mode");
            eprintln!("[j2substrs] Docker Compose mode: Container orchestration support");
        }
    }

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
