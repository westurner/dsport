//! `jinja2rs::i18n` — Internationalization support (Phase 6).
//!
//! Provides translation globals for Jinja2 templates:
//! - `gettext` — translate a message
//! - `ngettext` — translate a plural message
//! - Translation dictionary management
//!
//! Note: This is a simplified implementation that uses in-memory translation
//! dictionaries. Full `.mo`/`.po` file support is planned for future phases.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use minijinja::value::{Object, ObjectRepr, Value};
use minijinja::{Error, ErrorKind, State};

/// Translation provider — holds translation dictionaries for a locale.
///
/// Maps message keys to translated strings. Supports both singular and
/// plural forms via `(singular, plural, n)` tuples in the translation dict.
#[derive(Debug, Clone)]
pub struct I18nProvider {
    translations: Arc<Mutex<HashMap<String, String>>>,
    plural_forms: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl I18nProvider {
    pub fn new() -> Self {
        Self {
            translations: Arc::new(Mutex::new(HashMap::new())),
            plural_forms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Load a translation dictionary.
    ///
    /// Example dictionary:
    /// ```json
    /// {
    ///   "message": "translated message",
    ///   "items": ["singular form", "plural form"]
    /// }
    /// ```
    pub fn load_translations(&self, dict: HashMap<String, String>) {
        let mut trans = self.translations.lock().unwrap();
        trans.extend(dict);
    }

    /// Load plural forms.
    ///
    /// For a key "items", stores `["singular", "dual", "plural"]` forms
    /// indexed by the same key.
    pub fn load_plural_forms(&self, forms: HashMap<String, Vec<String>>) {
        let mut plurals = self.plural_forms.lock().unwrap();
        plurals.extend(forms);
    }

    /// Retrieve a translated string, falling back to the original message.
    pub fn gettext(&self, message: &str) -> String {
        let trans = self.translations.lock().unwrap();
        trans
            .get(message)
            .cloned()
            .unwrap_or_else(|| message.to_string())
    }

    /// Retrieve a plural-form translation.
    ///
    /// Returns the translated form based on `n`, or falls back to
    /// the singular/plural form provided.
    pub fn ngettext(&self, singular: &str, plural: &str, n: usize) -> String {
        // Try to look up plural forms
        let plurals = self.plural_forms.lock().unwrap();

        // First, try to find an entry indexed by the singular form
        if let Some(forms) = plurals.get(singular) {
            // Simple plural rule: use singular (0) for n=1, plural (1) otherwise
            // (This is a simplification; full i18n would use CLDR plural rules)
            return if n == 1 {
                forms
                    .get(0)
                    .cloned()
                    .unwrap_or_else(|| singular.to_string())
            } else {
                forms.get(1).cloned().unwrap_or_else(|| plural.to_string())
            };
        }

        // Fall back to provided singular/plural
        if n == 1 {
            singular.to_string()
        } else {
            plural.to_string()
        }
    }
}

impl Default for I18nProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for I18nProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<I18nProvider>")
    }
}

/// `gettext` global — translates a message.
///
/// Usage in template:
/// ```jinja
/// {{ gettext("Hello, World!") }}
/// ```
#[derive(Debug, Clone)]
pub struct GettextGlobal {
    provider: I18nProvider,
}

impl GettextGlobal {
    pub fn new(provider: I18nProvider) -> Self {
        Self { provider }
    }
}

impl fmt::Display for GettextGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<gettext>")
    }
}

impl Object for GettextGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        if args.is_empty() {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                "gettext() requires a message argument",
            ));
        }
        let message = args[0].to_string();
        Ok(Value::from(self.provider.gettext(&message)))
    }
}

/// `ngettext` global — translates a plural message.
///
/// Usage in template:
/// ```jinja
/// {{ ngettext("item", "items", count) }}
/// ```
#[derive(Debug, Clone)]
pub struct NgettextGlobal {
    provider: I18nProvider,
}

impl NgettextGlobal {
    pub fn new(provider: I18nProvider) -> Self {
        Self { provider }
    }
}

impl fmt::Display for NgettextGlobal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ngettext>")
    }
}

impl Object for NgettextGlobal {
    fn repr(self: &Arc<Self>) -> ObjectRepr {
        ObjectRepr::Plain
    }

    fn call(self: &Arc<Self>, _state: &State<'_, '_>, args: &[Value]) -> Result<Value, Error> {
        if args.len() < 3 {
            return Err(Error::new(
                ErrorKind::MissingArgument,
                "ngettext() requires singular, plural, and n arguments",
            ));
        }
        let singular = args[0].to_string();
        let plural = args[1].to_string();
        let n = args[2].as_i64().ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidOperation,
                "ngettext() n must be an integer",
            )
        })? as usize;

        Ok(Value::from(self.provider.ngettext(&singular, &plural, n)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gettext_passthrough() {
        let provider = I18nProvider::new();
        let result = provider.gettext("Hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_gettext_translation() {
        let provider = I18nProvider::new();
        let mut dict = HashMap::new();
        dict.insert("Hello".to_string(), "Hola".to_string());
        provider.load_translations(dict);

        let result = provider.gettext("Hello");
        assert_eq!(result, "Hola");
    }

    #[test]
    fn test_ngettext_singular() {
        let provider = I18nProvider::new();
        let result = provider.ngettext("item", "items", 1);
        assert_eq!(result, "item");
    }

    #[test]
    fn test_ngettext_plural() {
        let provider = I18nProvider::new();
        let result = provider.ngettext("item", "items", 5);
        assert_eq!(result, "items");
    }

    #[test]
    fn test_ngettext_with_forms() {
        let provider = I18nProvider::new();
        let mut forms = HashMap::new();
        forms.insert(
            "item".to_string(),
            vec!["artículo".to_string(), "artículos".to_string()],
        );
        provider.load_plural_forms(forms);

        assert_eq!(provider.ngettext("item", "items", 1), "artículo");
        assert_eq!(provider.ngettext("item", "items", 5), "artículos");
    }
}
