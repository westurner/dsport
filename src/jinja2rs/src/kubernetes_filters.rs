//! `jinja2rs::kubernetes_filters` — Kubernetes-specific template filters.
//!
//! Filters for Kubernetes manifest processing, workload introspection,
//! and resource selection in templates.

use minijinja::Value;
use std::collections::BTreeMap;

/// Get the replicas count from a Deployment or StatefulSet spec.
///
/// # Examples
///
/// ```rust,ignore
/// {{ deployment | replicas }}  // Returns 3
/// ```
pub fn replicas(value: Value) -> Value {
    match value.get_item(&Value::from("spec")) {
        Ok(spec) => {
            match spec.get_item(&Value::from("replicas")) {
                Ok(repl) => repl,
                Err(_) => Value::from(1), // Default to 1 if not specified
            }
        }
        Err(_) => Value::from(1),
    }
}

/// Get the container image from a Pod or Deployment spec.
///
/// # Examples
///
/// ```rust,ignore
/// {{ pod | container_image }}  // Returns "myapp:1.0"
/// ```
pub fn container_image(value: Value) -> Value {
    match value.get_item(&Value::from("spec")) {
        Ok(spec) => match spec.get_item(&Value::from("containers")) {
            Ok(containers) => {
                if containers.len().unwrap_or(0) > 0 {
                    match containers.get_item(&Value::from(0)) {
                        Ok(container) => match container.get_item(&Value::from("image")) {
                            Ok(image) => image,
                            Err(_) => Value::from(""),
                        },
                        Err(_) => Value::from(""),
                    }
                } else {
                    Value::from("")
                }
            }
            Err(_) => Value::from(""),
        },
        Err(_) => Value::from(""),
    }
}

/// Get a label value from a resource by label key.
///
/// # Examples
///
/// ```rust,ignore
/// {{ pod | label("app") }}  // Returns "myapp"
/// ```
pub fn label(value: Value, key: Value) -> Value {
    let key_str = key.to_string();
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("labels")) {
            Ok(labels) => match labels.get_item(&Value::from(&key_str)) {
                Ok(label_value) => label_value,
                Err(_) => Value::from(""),
            },
            Err(_) => Value::from(""),
        },
        Err(_) => Value::from(""),
    }
}

/// Get an annotation value from a resource by annotation key.
///
/// # Examples
///
/// ```ignore
/// {{ pod | annotation("description") }}
/// ```
pub fn annotation(value: Value, key: Value) -> Value {
    let key_str = key.to_string();
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("annotations")) {
            Ok(annotations) => match annotations.get_item(&Value::from(&key_str)) {
                Ok(annot_value) => annot_value,
                Err(_) => Value::from(""),
            },
            Err(_) => Value::from(""),
        },
        Err(_) => Value::from(""),
    }
}

/// Get the kind of a Kubernetes resource.
///
/// # Examples
///
/// ```ignore
/// {{ resource | k8s_kind }}  // Returns "Deployment"
/// ```
pub fn k8s_kind(value: Value) -> Value {
    match value.get_item(&Value::from("kind")) {
        Ok(kind) => kind,
        Err(_) => Value::from(""),
    }
}

/// Get the name of a Kubernetes resource.
///
/// # Examples
///
/// ```ignore
/// {{ resource | k8s_name }}  // Returns "my-app"
/// ```
pub fn k8s_name(value: Value) -> Value {
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("name")) {
            Ok(name) => name,
            Err(_) => Value::from(""),
        },
        Err(_) => Value::from(""),
    }
}

/// Get the namespace of a Kubernetes resource.
///
/// # Examples
///
/// ```ignore
/// {{ resource | k8s_namespace }}  // Returns "default"
/// ```
pub fn k8s_namespace(value: Value) -> Value {
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => {
            match metadata.get_item(&Value::from("namespace")) {
                Ok(ns) => ns,
                Err(_) => Value::from("default"), // Default to "default" namespace
            }
        }
        Err(_) => Value::from("default"),
    }
}

/// Get all labels from a resource as a dictionary.
///
/// # Examples
///
/// ```ignore
/// {{ resource | k8s_labels }}  // Returns {"app": "myapp", "version": "1.0"}
/// ```
pub fn k8s_labels(value: Value) -> Value {
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("labels")) {
            Ok(labels) => labels,
            Err(_) => Value::from(BTreeMap::<&str, Value>::new()),
        },
        Err(_) => Value::from(BTreeMap::<&str, Value>::new()),
    }
}

/// Get all annotations from a resource as a dictionary.
///
/// # Examples
///
/// ```ignore
/// {{ resource | k8s_annotations }}
/// ```
pub fn k8s_annotations(value: Value) -> Value {
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("annotations")) {
            Ok(annotations) => annotations,
            Err(_) => Value::from(BTreeMap::<&str, Value>::new()),
        },
        Err(_) => Value::from(BTreeMap::<&str, Value>::new()),
    }
}

/// Check if a resource is in a specific namespace.
///
/// # Examples
///
/// ```ignore
/// {% if resource | k8s_in_namespace("production") %}
/// ```
pub fn k8s_in_namespace(value: Value, namespace: Value) -> Value {
    let ns_str = namespace.to_string();
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("namespace")) {
            Ok(ns) => Value::from(ns.to_string() == ns_str),
            Err(_) => Value::from("default" == ns_str),
        },
        Err(_) => Value::from("default" == ns_str),
    }
}

/// Check if a resource has a specific label with a specific value.
///
/// # Examples
///
/// ```rust,ignore
/// {% if resource | k8s_has_label("app", "myapp") %}
/// ```
pub fn k8s_has_label(value: Value, key: Value, expected_val: Value) -> Value {
    let key_str = key.to_string();
    let expected = expected_val.to_string();
    match value.get_item(&Value::from("metadata")) {
        Ok(metadata) => match metadata.get_item(&Value::from("labels")) {
            Ok(labels) => match labels.get_item(&Value::from(&key_str)) {
                Ok(label_value) => Value::from(label_value.to_string() == expected),
                Err(_) => Value::from(false),
            },
            Err(_) => Value::from(false),
        },
        Err(_) => Value::from(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replicas() {
        let mut spec = BTreeMap::new();
        spec.insert("replicas", Value::from(3));
        let mut deployment = BTreeMap::new();
        deployment.insert("spec", Value::from(spec));
        let result = replicas(Value::from(deployment));
        assert_eq!(result.to_string(), "3");
    }

    #[test]
    fn test_container_image() {
        let mut container = BTreeMap::new();
        container.insert("image", Value::from("myapp:1.0"));
        container.insert("name", Value::from("app"));

        let mut spec = BTreeMap::new();
        spec.insert("containers", Value::from(vec![Value::from(container)]));

        let mut pod = BTreeMap::new();
        pod.insert("spec", Value::from(spec));

        let result = container_image(Value::from(pod));
        assert_eq!(result.to_string(), "myapp:1.0");
    }

    #[test]
    fn test_k8s_kind() {
        let mut metadata = BTreeMap::new();
        metadata.insert("name", Value::from("test"));
        let mut resource = BTreeMap::new();
        resource.insert("kind", Value::from("Deployment"));
        resource.insert("metadata", Value::from(metadata));
        let result = k8s_kind(Value::from(resource));
        assert_eq!(result.to_string(), "Deployment");
    }

    #[test]
    fn test_k8s_name() {
        let mut metadata = BTreeMap::new();
        metadata.insert("name", Value::from("my-app"));
        let mut resource = BTreeMap::new();
        resource.insert("metadata", Value::from(metadata));
        let result = k8s_name(Value::from(resource));
        assert_eq!(result.to_string(), "my-app");
    }

    #[test]
    fn test_k8s_namespace() {
        let mut metadata = BTreeMap::new();
        metadata.insert("namespace", Value::from("production"));
        let mut resource = BTreeMap::new();
        resource.insert("metadata", Value::from(metadata));
        let result = k8s_namespace(Value::from(resource));
        assert_eq!(result.to_string(), "production");
    }

    #[test]
    fn test_label() {
        let mut labels = BTreeMap::new();
        labels.insert("app", Value::from("myapp"));
        labels.insert("version", Value::from("1.0"));
        let mut metadata = BTreeMap::new();
        metadata.insert("labels", Value::from(labels));
        let mut pod = BTreeMap::new();
        pod.insert("metadata", Value::from(metadata));
        let result = label(Value::from(pod), Value::from("app"));
        assert_eq!(result.to_string(), "myapp");
    }

    #[test]
    fn test_k8s_has_label() {
        let mut labels = BTreeMap::new();
        labels.insert("app", Value::from("myapp"));
        let mut metadata = BTreeMap::new();
        metadata.insert("labels", Value::from(labels));
        let mut pod = BTreeMap::new();
        pod.insert("metadata", Value::from(metadata));
        let result = k8s_has_label(Value::from(pod), Value::from("app"), Value::from("myapp"));
        assert_eq!(result.to_string(), "true");
    }
}
