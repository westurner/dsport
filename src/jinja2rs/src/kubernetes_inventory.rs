//! `jinja2rs::kubernetes_inventory` — Kubernetes manifest loader.
//!
//! Loads Kubernetes manifests from YAML files or stdin.
//! Extracts workload information (Pods, Deployments, Services, etc.).
//! Provides pod details, deployment replicas, service endpoints, etc.
//!
//! # Manifest Format
//!
//! Supports standard Kubernetes YAML manifests (as produced by `podman generate kube`):
//!
//! ```yaml
//! apiVersion: v1
//! kind: Pod
//! metadata:
//!   name: my-app
//!   namespace: default
//!   labels:
//!     app: myapp
//! spec:
//!   containers:
//!   - name: app
//!     image: myapp:1.0
//! ---
//! apiVersion: apps/v1
//! kind: Deployment
//! metadata:
//!   name: my-deployment
//! spec:
//!   replicas: 3
//! ```
//!
//! # Usage
//!
//! ```rust,no_run
//! use jinja2rs::kubernetes_inventory::KubernetesManifest;
//! use jinja2rs::compat::KubernetesInventorySource;
//!
//! // Load from file
//! let manifest = KubernetesManifest::from_source(KubernetesInventorySource::File("/path/to/deployment.yaml".into()))?;
//!
//! // Load from stdin
//! let manifest = KubernetesManifest::from_source(KubernetesInventorySource::Stdin)?;
//!
//! // Use in templates
//! let vars = manifest.to_template_vars();
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use crate::compat::KubernetesInventorySource;
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;
use std::fs;
use std::io::Read;

/// Kubernetes resource information (Pod, Deployment, Service, etc.).
#[derive(Debug, Clone)]
pub struct KubernetesResource {
    /// Resource kind (e.g., "Pod", "Deployment", "Service")
    pub kind: String,
    /// Resource name
    pub name: String,
    /// Namespace (default: "default")
    pub namespace: String,
    /// Raw resource object
    pub object: JsonValue,
    /// Labels extracted from metadata
    pub labels: HashMap<String, String>,
    /// Annotations extracted from metadata
    pub annotations: HashMap<String, String>,
}

/// Kubernetes manifest representation.
///
/// Contains parsed manifests organized by kind and name.
#[derive(Debug, Clone)]
pub struct KubernetesManifest {
    /// All resources by kind and name
    pub resources: HashMap<String, HashMap<String, KubernetesResource>>,
    /// Pods (convenience accessor)
    pub pods: HashMap<String, KubernetesResource>,
    /// Deployments (convenience accessor)
    pub deployments: HashMap<String, KubernetesResource>,
    /// Services (convenience accessor)
    pub services: HashMap<String, KubernetesResource>,
}

impl KubernetesManifest {
    /// Load Kubernetes manifests from a source.
    pub fn from_source(
        source: KubernetesInventorySource,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let content = match source {
            KubernetesInventorySource::File(path) => fs::read_to_string(path)?,
            KubernetesInventorySource::Stdin => {
                let mut buffer = String::new();
                std::io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
            KubernetesInventorySource::Inline(s) => s,
        };

        Self::from_yaml(&content)
    }

    /// Parse YAML content into Kubernetes manifests.
    fn from_yaml(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut resources: HashMap<String, HashMap<String, KubernetesResource>> = HashMap::new();
        let mut pods = HashMap::new();
        let mut deployments = HashMap::new();
        let mut services = HashMap::new();

        // Split by YAML document separator (---)
        for doc_str in content.split("---\n") {
            let trimmed = doc_str.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Parse YAML using serde_yaml
            match serde_yaml::from_str::<JsonValue>(trimmed) {
                Ok(obj) => {
                    if let Some(kind) = obj.get("kind").and_then(|v| v.as_str()) {
                        if let Some(metadata) = obj.get("metadata") {
                            let name = metadata
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string();
                            let namespace = metadata
                                .get("namespace")
                                .and_then(|v| v.as_str())
                                .unwrap_or("default")
                                .to_string();

                            // Extract labels and annotations
                            let mut labels = HashMap::new();
                            if let Some(meta_labels) =
                                metadata.get("labels").and_then(|v| v.as_object())
                            {
                                for (k, v) in meta_labels {
                                    if let Some(val) = v.as_str() {
                                        labels.insert(k.clone(), val.to_string());
                                    }
                                }
                            }

                            let mut annotations = HashMap::new();
                            if let Some(meta_annotations) =
                                metadata.get("annotations").and_then(|v| v.as_object())
                            {
                                for (k, v) in meta_annotations {
                                    if let Some(val) = v.as_str() {
                                        annotations.insert(k.clone(), val.to_string());
                                    }
                                }
                            }

                            let resource = KubernetesResource {
                                kind: kind.to_string(),
                                name: name.clone(),
                                namespace: namespace.clone(),
                                object: obj.clone(),
                                labels,
                                annotations,
                            };

                            // Index by kind and name
                            resources
                                .entry(kind.to_string())
                                .or_default()
                                .insert(name.clone(), resource.clone());

                            // Also populate convenience accessors
                            match kind {
                                "Pod" => {
                                    pods.insert(name, resource);
                                }
                                "Deployment" => {
                                    deployments.insert(name, resource);
                                }
                                "Service" => {
                                    services.insert(name, resource);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse manifest: {}", e);
                    // Continue parsing other manifests
                }
            }
        }

        Ok(KubernetesManifest {
            resources,
            pods,
            deployments,
            services,
        })
    }

    /// Convert manifest to template variables.
    ///
    /// Returns a JSON object with:
    /// - `kubernetes_resources`: All resources by kind
    /// - `kubernetes_pods`: Pod details
    /// - `kubernetes_deployments`: Deployment details
    /// - `kubernetes_services`: Service details
    pub fn to_template_vars(&self) -> JsonValue {
        // Convert resources to JSON
        let mut resources_json = serde_json::Map::new();
        for (kind, items) in &self.resources {
            let mut kind_items = serde_json::Map::new();
            for (name, resource) in items {
                kind_items.insert(name.clone(), resource.object.clone());
            }
            resources_json.insert(kind.clone(), JsonValue::Object(kind_items));
        }

        // Convert pods
        let mut pods_json = serde_json::Map::new();
        for (name, pod) in &self.pods {
            pods_json.insert(name.clone(), pod.object.clone());
        }

        // Convert deployments
        let mut deployments_json = serde_json::Map::new();
        for (name, deployment) in &self.deployments {
            deployments_json.insert(name.clone(), deployment.object.clone());
        }

        // Convert services
        let mut services_json = serde_json::Map::new();
        for (name, service) in &self.services {
            services_json.insert(name.clone(), service.object.clone());
        }

        json!({
            "kubernetes_resources": JsonValue::Object(resources_json),
            "kubernetes_pods": JsonValue::Object(pods_json),
            "kubernetes_deployments": JsonValue::Object(deployments_json),
            "kubernetes_services": JsonValue::Object(services_json),
        })
    }

    /// Get a specific resource by kind and name.
    pub fn get_resource(&self, kind: &str, name: &str) -> Option<&KubernetesResource> {
        self.resources.get(kind).and_then(|items| items.get(name))
    }

    /// Get all resources of a specific kind.
    pub fn get_resources_by_kind(&self, kind: &str) -> Vec<&KubernetesResource> {
        self.resources
            .get(kind)
            .map(|items| items.values().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_pod() {
        let yaml = r#"
apiVersion: v1
kind: Pod
metadata:
  name: test-pod
  namespace: default
  labels:
    app: myapp
spec:
  containers:
  - name: app
    image: myapp:1.0
"#;
        let manifest = KubernetesManifest::from_yaml(yaml).unwrap();
        assert!(manifest.pods.contains_key("test-pod"));
        assert_eq!(manifest.get_resources_by_kind("Pod").len(), 1);
    }

    #[test]
    fn test_parse_multiple_resources() {
        let yaml = r#"
apiVersion: v1
kind: Pod
metadata:
  name: pod1
  namespace: default
spec:
  containers:
  - name: app
    image: myapp:1.0
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: deploy1
  namespace: default
spec:
  replicas: 3
"#;
        let manifest = KubernetesManifest::from_yaml(yaml).unwrap();
        assert_eq!(manifest.pods.len(), 1);
        assert_eq!(manifest.deployments.len(), 1);
    }

    #[test]
    fn test_extract_labels() {
        let yaml = r#"
apiVersion: v1
kind: Pod
metadata:
  name: test-pod
  namespace: default
  labels:
    app: myapp
    version: "1.0"
spec:
  containers: []
"#;
        let manifest = KubernetesManifest::from_yaml(yaml).unwrap();
        let pod = manifest.pods.get("test-pod").unwrap();
        assert_eq!(pod.labels.get("app"), Some(&"myapp".to_string()));
        assert_eq!(pod.labels.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_template_vars() {
        let yaml = r#"
apiVersion: v1
kind: Pod
metadata:
  name: test-pod
  namespace: default
spec:
  containers: []
"#;
        let manifest = KubernetesManifest::from_yaml(yaml).unwrap();
        let vars = manifest.to_template_vars();
        assert!(vars.get("kubernetes_pods").is_some());
        assert!(vars.get("kubernetes_resources").is_some());
    }
}
