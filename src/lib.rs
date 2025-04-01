//! # i18n library for Rust
//!
//! This library provides a simple way to add internationalization to your Rust applications by using JSON files.
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

// Global variables for the library
static TRANSLATIONS: Lazy<RwLock<HashMap<String, Map<String, Value>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("en".to_string()));
static LANGUAGE_DIR: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("languages")));

pub struct Lingua;

impl Lingua {
    /// Initialize the library with the default language directory `language`.
    /// This function should be called before any other function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// Lingua::init();
    /// ```
    pub fn init() {
        Self::init_with_dir("language");
    }

    /// Initialize the library with a custom language directory.
    /// This function should be called before any other function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// Lingua::init_with_dir("languages");
    /// ```
    pub fn init_with_dir(dir: &str) {
        *LANGUAGE_DIR.write().unwrap() = PathBuf::from(dir);

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".json") {
                        let lang_code = file_name.trim_end_matches(".json");
                        Self::load_language(lang_code);
                    }
                }
            }
        }

        if let Some(lang) = Self::detect_system_language() {
            if Self::has_language(&lang) {
                Self::set_language(&lang);
            }
        }
    }

    /// Load a language file from the language directory.
    ///
    /// # Arguments
    ///
    /// * `lang_code` - The language code of the language file to load.
    fn load_language(lang_code: &str) {
        let path = LANGUAGE_DIR
            .read()
            .unwrap()
            .join(format!("{}.json", lang_code));

        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(json) = serde_json::from_str::<Map<String, Value>>(&content) {
                TRANSLATIONS
                    .write()
                    .unwrap()
                    .insert(lang_code.to_string(), json);
            }
        }
    }

    /// Check if a language is available.
    ///
    /// # Arguments
    ///
    /// * `lang_code` - The language code to check.
    fn has_language(lang_code: &str) -> bool {
        TRANSLATIONS.read().unwrap().contains_key(lang_code)
    }

    /// Set the current language.
    ///
    /// # Arguments
    ///
    /// * `lang_code` - The language code to set.
    ///
    /// # Returns
    ///
    /// Returns `true` if the language was set successfully, otherwise `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// Lingua::set_language("de");
    /// ```
    pub fn set_language(lang_code: &str) -> bool {
        if Self::has_language(lang_code) {
            *CURRENT_LANGUAGE.write().unwrap() = lang_code.to_string();
            true
        } else {
            false
        }
    }

    /// Get a list of available languages.
    ///
    /// # Returns
    ///
    /// Returns a list of available languages.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// let languages = Lingua::get_languages();
    /// ```
    pub fn get_languages() -> Vec<String> {
        TRANSLATIONS.read().unwrap().keys().cloned().collect()
    }

    /// Get the current language.
    ///
    /// # Returns
    ///
    /// Returns the current language.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// let lang = Lingua::get_language();
    /// ```
    pub fn get_language() -> String {
        CURRENT_LANGUAGE.read().unwrap().clone()
    }

    /// Translate a key with optional parameters.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    /// * `params` - A list of parameters to replace in the translation.
    ///
    /// # Returns
    ///
    /// Returns the translated string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// let translated = Lingua::translate("hello", &[]);
    /// ```
    pub fn translate(key: &str, params: &[(&str, &str)]) -> String {
        let lang = CURRENT_LANGUAGE.read().unwrap().clone();

        if let Some(translations) = TRANSLATIONS.read().unwrap().get(&lang) {
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = Some(translations);

            for (i, part) in parts.iter().enumerate() {
                if i < parts.len() - 1 {
                    current = current
                        .and_then(|v| v.get(*part))
                        .and_then(|v| v.as_object());
                } else if let Some(val) = current.and_then(|v| v.get(*part)) {
                    let mut result = match val {
                        Value::String(s) => s.clone(),
                        _ => val.to_string().trim_matches('"').to_string(),
                    };

                    for (name, value) in params {
                        result = result.replace(&format!("{{{{{}}}}}", name), value);
                    }

                    return result;
                }
            }
        }

        key.to_string()
    }

    /// Translate a key with optional parameters.
    /// This function is a shorthand for `Lingua::translate`.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to translate.
    /// * `params` - A list of parameters to replace in the translation.
    ///
    /// # Returns
    ///
    /// Returns the translated string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    ///
    /// let translated = Lingua::t("hello", &[]);
    /// ```
    pub fn t(key: &str, params: &[(&str, &str)]) -> String {
        Self::translate(key, params)
    }

    /// Detect the system language.
    ///
    /// # Returns
    ///
    /// Returns the system language if it was detected, otherwise `None`.
    fn detect_system_language() -> Option<String> {
        std::env::var("LANG")
            .ok()
            .and_then(|lang| lang.split('.').next().map(String::from))
            .map(|s| s.split('_').next().unwrap_or("en").to_string())
    }
}

pub mod prelude {
    pub use crate::Lingua;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        TRANSLATIONS.write().unwrap().clear();
    }

    #[test]
    fn test_translate_simple() {
        setup();
        let mut map = Map::new();
        map.insert("hello".to_string(), Value::String("Hallo".to_string()));
        TRANSLATIONS.write().unwrap().insert("de".to_string(), map);
        *CURRENT_LANGUAGE.write().unwrap() = "de".to_string();

        assert_eq!(Lingua::translate("hello", &[]), "Hallo");
    }

    #[test]
    fn test_translate_nested() {
        setup();
        let mut map = Map::new();
        let mut submenu = Map::new();
        submenu.insert("save".to_string(), Value::String("Speichern".to_string()));

        let mut menu = Map::new();
        menu.insert("file".to_string(), Value::Object(submenu));

        map.insert("menu".to_string(), Value::Object(menu));
        TRANSLATIONS.write().unwrap().insert("de".to_string(), map);

        *CURRENT_LANGUAGE.write().unwrap() = "de".to_string();

        assert_eq!(Lingua::translate("menu.file.save", &[]), "Speichern");
    }

    #[test]
    fn test_translate_params() {
        setup();
        let mut map = Map::new();
        map.insert(
            "welcome_user".to_string(),
            Value::String("Hallo {{name}}!".to_string()),
        );
        TRANSLATIONS.write().unwrap().insert("de".to_string(), map);

        *CURRENT_LANGUAGE.write().unwrap() = "de".to_string();

        assert_eq!(
            Lingua::translate("welcome_user", &[("name", "Max")]),
            "Hallo Max!"
        );
    }
}
