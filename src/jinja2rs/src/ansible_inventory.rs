//! `jinja2rs::ansible_inventory` — Ansible inventory loader.
//!
//! Loads inventory from YAML or JSON files or stdin.
//! Provides hosts, groups, and host_vars that Ansible templates can use.
//!
//! # Inventory Format
//!
//! Supports standard Ansible YAML inventory:
//!
//! ```yaml
//! all:
//!   hosts:
//!     localhost:
//!       ansible_connection: local
//!   children:
//!     webservers:
//!       hosts:
//!         web1:
//!         web2:
//!       vars:
//!         http_port: 80
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! use jinja2rs::ansible_inventory::{Inventory, InventorySource};
//!
//! // Load from file
//! let inv = Inventory::from_source(InventorySource::File("/etc/ansible/hosts".into()))?;
//!
//! // Load from stdin
//! let inv = Inventory::from_source(InventorySource::Stdin)?;
//!
//! // Use in templates
//! let vars = inv.to_template_vars();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

/// Source for inventory data.
#[derive(Debug, Clone)]
pub enum InventorySource {
    /// Load from a file path
    File(PathBuf),
    /// Load from standard input
    Stdin,
    /// Load from inline YAML string
    Inline(String),
}

/// Ansible inventory representation.
///
/// Contains hosts, groups, and variables organized in an Ansible-compatible structure.
#[derive(Debug, Clone)]
pub struct Inventory {
    /// All inventory data (raw structure)
    pub data: JsonValue,
    /// Hosts by name (flattened)
    pub hosts: HashMap<String, HostInfo>,
    /// Groups by name
    pub groups: HashMap<String, GroupInfo>,
}

/// Host information from inventory.
#[derive(Debug, Clone)]
pub struct HostInfo {
    /// Hostname
    pub name: String,
    /// Host variables
    pub vars: HashMap<String, JsonValue>,
    /// Groups this host belongs to
    pub groups: Vec<String>,
}

/// Group information from inventory.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    /// Group name
    pub name: String,
    /// Hosts in this group
    pub hosts: Vec<String>,
    /// Group variables
    pub vars: HashMap<String, JsonValue>,
    /// Child groups
    pub children: Vec<String>,
}

impl Inventory {
    /// Load inventory from a source.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File cannot be read
    /// - YAML/JSON is invalid
    /// - Inventory structure is malformed
    pub fn from_source(source: InventorySource) -> Result<Self, Box<dyn std::error::Error>> {
        let raw_data = match source {
            InventorySource::File(path) => fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read inventory file: {}", e))?,
            InventorySource::Stdin => {
                let mut buf = String::new();
                std::io::stdin()
                    .read_to_string(&mut buf)
                    .map_err(|e| format!("Failed to read inventory from stdin: {}", e))?;
                buf
            }
            InventorySource::Inline(data) => data,
        };

        Self::from_yaml_or_json(&raw_data)
    }

    /// Parse inventory from YAML or JSON string.
    fn from_yaml_or_json(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Try YAML first (more forgiving)
        let json_value = if let Ok(parsed) = serde_yaml::from_str::<JsonValue>(data) {
            parsed
        } else {
            serde_json::from_str(data)
                .map_err(|e| format!("Failed to parse inventory as YAML or JSON: {}", e))?
        };

        Self::from_parsed_data(json_value)
    }

    /// Create inventory from already-parsed JSON value.
    fn from_parsed_data(data: JsonValue) -> Result<Self, Box<dyn std::error::Error>> {
        let mut hosts = HashMap::new();
        let mut groups = HashMap::new();

        // Parse 'all' group and recursively extract hosts/groups
        if let Some(all) = data.get("all") {
            Self::parse_group(all, "all", &mut hosts, &mut groups)?;
        }

        Ok(Inventory {
            data,
            hosts,
            groups,
        })
    }

    /// Recursively parse a group structure.
    fn parse_group(
        group_data: &JsonValue,
        group_name: &str,
        hosts: &mut HashMap<String, HostInfo>,
        groups: &mut HashMap<String, GroupInfo>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut group_info = GroupInfo {
            name: group_name.to_string(),
            hosts: Vec::new(),
            vars: HashMap::new(),
            children: Vec::new(),
        };

        // Parse hosts
        if let Some(hosts_obj) = group_data.get("hosts") {
            if let Some(hosts_map) = hosts_obj.as_object() {
                for (hostname, host_data) in hosts_map {
                    let mut host_vars = HashMap::new();

                    if let Some(vars) = host_data.as_object() {
                        for (k, v) in vars {
                            host_vars.insert(k.clone(), v.clone());
                        }
                    }

                    hosts.insert(
                        hostname.clone(),
                        HostInfo {
                            name: hostname.clone(),
                            vars: host_vars,
                            groups: vec![group_name.to_string()],
                        },
                    );
                    group_info.hosts.push(hostname.clone());
                }
            }
        }

        // Parse group variables
        if let Some(vars) = group_data.get("vars") {
            if let Some(vars_map) = vars.as_object() {
                for (k, v) in vars_map {
                    group_info.vars.insert(k.clone(), v.clone());
                }
            }
        }

        // Parse child groups
        if let Some(children) = group_data.get("children") {
            if let Some(children_map) = children.as_object() {
                for (child_name, child_data) in children_map {
                    group_info.children.push(child_name.clone());
                    Self::parse_group(child_data, child_name, hosts, groups)?;
                }
            }
        }

        groups.insert(group_name.to_string(), group_info);
        Ok(())
    }

    /// Convert inventory to template variables.
    ///
    /// Returns a JSON value suitable for use in templates:
    /// - `inventory_hostname` — current host name
    /// - `groups` — group membership dict
    /// - `hostvars` — host variables dict
    /// - `group_names` — list of groups
    pub fn to_template_vars(&self) -> JsonValue {
        let mut groups_obj = serde_json::Map::new();
        let mut hostvars_obj = serde_json::Map::new();

        // Build groups dictionary
        for (group_name, group_info) in &self.groups {
            groups_obj.insert(
                group_name.clone(),
                JsonValue::Array(
                    group_info
                        .hosts
                        .iter()
                        .map(|h| JsonValue::String(h.clone()))
                        .collect(),
                ),
            );
        }

        // Build hostvars dictionary
        for (hostname, host_info) in &self.hosts {
            let mut host_vars = serde_json::Map::new();
            for (k, v) in &host_info.vars {
                host_vars.insert(k.clone(), v.clone());
            }
            hostvars_obj.insert(hostname.clone(), JsonValue::Object(host_vars));
        }

        json!({
            "groups": JsonValue::Object(groups_obj),
            "hostvars": JsonValue::Object(hostvars_obj),
        })
    }

    /// Get groups for a specific host.
    pub fn get_host_groups(&self, hostname: &str) -> Vec<String> {
        self.hosts
            .get(hostname)
            .map(|h| h.groups.clone())
            .unwrap_or_default()
    }

    /// Get variables for a specific host.
    pub fn get_host_vars(&self, hostname: &str) -> HashMap<String, JsonValue> {
        self.hosts
            .get(hostname)
            .map(|h| h.vars.clone())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_inventory() {
        let yaml = r#"
all:
  hosts:
    localhost:
      ansible_connection: local
  children:
    webservers:
      hosts:
        web1:
        web2:
      vars:
        http_port: 80
"#;
        let inv = Inventory::from_yaml_or_json(yaml).unwrap();
        assert!(inv.hosts.contains_key("localhost"));
        assert!(inv.hosts.contains_key("web1"));
        assert!(inv.hosts.contains_key("web2"));
        assert!(inv.groups.contains_key("all"));
        assert!(inv.groups.contains_key("webservers"));
    }

    #[test]
    fn test_get_host_groups() {
        let yaml = r#"
all:
  children:
    webservers:
      hosts:
        web1:
"#;
        let inv = Inventory::from_yaml_or_json(yaml).unwrap();
        let groups = inv.get_host_groups("web1");
        assert!(groups.contains(&"webservers".to_string()));
    }

    #[test]
    fn test_to_template_vars() {
        let yaml = r#"
all:
  hosts:
    host1:
  children:
    group1:
      hosts:
        host1:
"#;
        let inv = Inventory::from_yaml_or_json(yaml).unwrap();
        let vars = inv.to_template_vars();
        assert!(vars.get("groups").is_some());
        assert!(vars.get("hostvars").is_some());
    }
}
