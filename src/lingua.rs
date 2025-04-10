use crate::error::LinguaError;
use once_cell::sync::Lazy;
use serde_json::{Map, Value};
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs;

use std::path::{Path, PathBuf};
use std::sync::RwLock;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Global variables for the library
static TRANSLATIONS: Lazy<RwLock<HashMap<String, Map<String, Value>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));
static CURRENT_LANGUAGE: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new("en".to_string()));
static LANGUAGE_DIR: Lazy<RwLock<PathBuf>> = Lazy::new(|| RwLock::new(PathBuf::from("languages")));

pub struct LinguaBuilder {
    language_dir: String,
}

pub struct Lingua;

impl Lingua {
    pub fn new(language_dir: &str) -> LinguaBuilder {
        LinguaBuilder {
            language_dir: language_dir.to_string(),
        }
    }

    /// Load all available languages from the language directory.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the translated string if successful, otherwise a `LinguaError`.
    #[cfg(not(target_arch = "wasm32"))]
    fn load_available_languages() -> Result<usize, LinguaError> {
        Self::load_languages_fs()
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_available_languages() -> Result<usize, LinguaError> {
        Self::load_languages_wasm().await
    }

    fn load_languages_fs() -> Result<usize, LinguaError> {
        let dir_path = LANGUAGE_DIR.read().unwrap().clone();
        let entries = fs::read_dir(&dir_path).map_err(LinguaError::DirectoryAccess)?;

        let mut count = 0;
        for entry in entries {
            let entry = entry.map_err(LinguaError::DirectoryAccess)?;
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let lang_code = file_name.trim_end_matches(".json");
                    Self::load_language(lang_code)?;
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_languages_wasm() -> Result<usize, LinguaError> {
        let common_languages = ["en", "de", "fr", "es", "it", "ja", "zh"];
        let mut count = 0;

        for lang_code in common_languages.iter() {
            match Self::load_language_wasm(lang_code).await {
                Ok(_) => count += 1,
                Err(_) => {}
            }
        }

        if count == 0 {
            let mut translations = Map::new();
            translations.insert("hello".to_string(), Value::String("Hello".to_string()));
            translations.insert("goodbye".to_string(), Value::String("Goodbye".to_string()));

            TRANSLATIONS
                .write()
                .unwrap()
                .insert("en".to_string(), translations);

            count = 1;
        }

        Ok(count)
    }

    /// Load a language file from the language directory.
    ///
    /// # Arguments
    ///
    /// * `lang_code` - The language code of the language file to load.
    #[cfg(not(target_arch = "wasm32"))]
    fn load_language(lang_code: &str) -> Result<(), LinguaError> {
        Self::load_language_fs(lang_code)
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_language(lang_code: &str) -> Result<(), LinguaError> {
        Self::load_language_wasm(lang_code).await
    }

    fn load_language_fs(lang_code: &str) -> Result<(), LinguaError> {
        let path = LANGUAGE_DIR
            .read()
            .unwrap()
            .join(format!("{}.json", lang_code));

        let content = fs::read_to_string(&path)
            .map_err(|_| LinguaError::LanguageFileNotFound(lang_code.to_string()))?;

        let json = serde_json::from_str::<Map<String, Value>>(&content).map_err(|error| {
            LinguaError::JsonParse {
                file: lang_code.to_string(),
                error,
            }
        })?;

        TRANSLATIONS
            .write()
            .unwrap()
            .insert(lang_code.to_string(), json);
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    async fn load_language_wasm(lang_code: &str) -> Result<(), LinguaError> {
        let dir_path = LANGUAGE_DIR.read().unwrap().clone();
        let url = format!(
            "{}/{}.json",
            dir_path.to_str().unwrap_or("languages"),
            lang_code
        );

        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        opts.mode(web_sys::RequestMode::Cors);

        let request = web_sys::Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| LinguaError::LanguageFileNotFound(lang_code.to_string()))?;

        let window = web_sys::window().ok_or_else(|| {
            LinguaError::LanguageFileNotFound(format!("No window access for {}", lang_code))
        })?;

        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|_| LinguaError::LanguageFileNotFound(lang_code.to_string()))?;

        let response: web_sys::Response = resp_value
            .dyn_into()
            .map_err(|_| LinguaError::LanguageFileNotFound(lang_code.to_string()))?;

        if !response.ok() {
            return Err(LinguaError::LanguageFileNotFound(lang_code.to_string()));
        }

        let json = wasm_bindgen_futures::JsFuture::from(response.json().map_err(|_| {
            LinguaError::JsonParse {
                file: lang_code.to_string(),
                error: serde_json::Error::custom("Failed to parse JSON"),
            }
        })?)
        .await
        .map_err(|_| LinguaError::JsonParse {
            file: lang_code.to_string(),
            error: serde_json::Error::custom("Failed to parse JSON"),
        })?;

        let json_string = js_sys::JSON::stringify(&json)
            .map_err(|_| LinguaError::JsonParse {
                file: lang_code.to_string(),
                error: serde_json::Error::custom("Failed to stringify JSON"),
            })?
            .as_string()
            .ok_or_else(|| LinguaError::JsonParse {
                file: lang_code.to_string(),
                error: serde_json::Error::custom("Failed to get JSON as string"),
            })?;

        let json_map =
            serde_json::from_str::<Map<String, Value>>(&json_string).map_err(|error| {
                LinguaError::JsonParse {
                    file: lang_code.to_string(),
                    error,
                }
            })?;

        TRANSLATIONS
            .write()
            .unwrap()
            .insert(lang_code.to_string(), json_map);

        Ok(())
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
    pub fn set_language(lang_code: &str) -> Result<bool, LinguaError> {
        if Self::has_language(lang_code) {
            *CURRENT_LANGUAGE.write().unwrap() = lang_code.to_string();
            Ok(true)
        } else {
            Err(LinguaError::LanguageNotAvailable(lang_code.to_string()))
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
    pub fn get_languages() -> Result<Vec<String>, LinguaError> {
        Ok(TRANSLATIONS.read().unwrap().keys().cloned().collect())
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
    pub fn get_language() -> Result<String, LinguaError> {
        Ok(CURRENT_LANGUAGE.read().unwrap().clone())
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
    pub fn translate(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError> {
        let lang = CURRENT_LANGUAGE.read().unwrap().clone();
        let translations = TRANSLATIONS.read().unwrap();

        let lang_map = translations
            .get(&lang)
            .ok_or_else(|| LinguaError::LanguageNotAvailable(lang.clone()))?;

        let parts: Vec<&str> = key.split('.').collect();
        let mut current = Some(lang_map);

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

                return Ok(result);
            }
        }

        Err(LinguaError::KeyNotFound(key.to_string()))
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
    pub fn t(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError> {
        let translated = Self::translate(key, params)?;
        Ok(translated)
    }

    /// Detect the system language.
    ///
    /// Load the system language via the `sys-locale` crate for cross-platform compatibility.
    ///
    /// # Returns
    ///
    /// Returns the system language if it was detected, otherwise `None`.
    #[cfg(not(target_arch = "wasm32"))]
    fn detect_system_language() -> Option<String> {
        sys_locale::get_locale()
            .and_then(|locale| locale.split('-').next().map(|lang| lang.to_string()))
    }

    #[cfg(target_arch = "wasm32")]
    fn detect_system_language() -> Option<String> {
        let window = web_sys::window()?;
        let navigator = window.navigator();

        if let Ok(lang) = js_sys::Reflect::get(&navigator, &JsValue::from_str("language")) {
            if let Some(lang_str) = lang.as_string() {
                return lang_str.split('-').next().map(|s| s.to_string());
            }
        }

        if let Ok(langs) = js_sys::Reflect::get(&navigator, &JsValue::from_str("languages")) {
            if js_sys::Array::is_array(&langs) {
                let langs_array = js_sys::Array::from(&langs);
                if langs_array.length() > 0 {
                    if let Some(first_lang) = langs_array.get(0).as_string() {
                        return first_lang.split('-').next().map(|s| s.to_string());
                    }
                }
            }
        }

        Some("en".to_string())
    }

    /// Load a language code from a configuration file.
    ///
    /// The configuration file can be in JSON, TOML, or a simple key-value format.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file.
    /// * `key` - The key to look for in the configuration file.
    ///
    /// # Returns
    ///
    /// Returns the language code if it was found in the configuration file.
    ///
    /// # Example
    ///
    /// ```rust
    /// use lingua_i18n_rs::prelude::*;
    /// use std::path::Path;
    ///
    /// let lang_code = Lingua::load_lang_from_config(Path::new("config.toml"), "language");
    /// ```
    pub fn load_lang_from_config(path: &Path, key: &str) -> Result<String, LinguaError> {
        if !path.exists() {
            return Err(LinguaError::ConfigFileNotFound(path.display().to_string()));
        }

        let content = fs::read_to_string(path)
            .map_err(|e| LinguaError::ConfigFileReadError(e.to_string()))?;

        let seperators = [':', '='];
        let clean_key = key.trim_matches('"').trim();

        for line in content.lines() {
            let line = line.trim();

            if line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            for sep in &seperators {
                if let Some(pos) = line.find(*sep) {
                    let line_key = line[..pos].trim().trim_matches('"');

                    if line_key == clean_key || line_key == format!("\"{}\"", clean_key) {
                        let lang_code = line[pos + 1..].trim().trim_matches('"').trim_matches(',');

                        let lang_code = lang_code.trim_matches('"');

                        if !Self::has_language(lang_code) {
                            return Err(LinguaError::LanguageNotAvailable(lang_code.to_string()));
                        }

                        return Ok(lang_code.to_string());
                    }
                }
            }
        }

        Err(LinguaError::ValueNotFoundInConfig(key.to_string()))
    }
}

impl LinguaBuilder {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn init(self) -> Result<Lingua, LinguaError> {
        *LANGUAGE_DIR.write().unwrap() = PathBuf::from(&self.language_dir);

        let languages_loaded = Lingua::load_available_languages()?;

        if languages_loaded == 0 {
            return Err(LinguaError::DirectoryAccess(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No language files found in '{}'", self.language_dir),
            )));
        }

        if let Some(lang) = Lingua::detect_system_language() {
            let _ = Lingua::set_language(&lang);
        }

        Ok(Lingua)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn init(self) -> Result<Lingua, LinguaError> {
        *LANGUAGE_DIR.write().unwrap() = PathBuf::from(&self.language_dir);

        let languages_loaded = Lingua::load_available_languages().await?;

        if languages_loaded == 0 {
            return Err(LinguaError::DirectoryAccess(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No language files found in '{}'", self.language_dir),
            )));
        }

        if let Some(lang) = Lingua::detect_system_language() {
            let _ = Lingua::set_language(&lang);
        }

        Ok(Lingua)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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

        assert_eq!(Lingua::translate("hello", &[]).unwrap(), "Hallo");
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

        assert_eq!(
            Lingua::translate("menu.file.save", &[]).unwrap(),
            "Speichern"
        );
    }

    #[test]
    fn test_translate_params() {
        setup();
        let mut map = Map::new();
        map.insert(
            "greeting".to_string(),
            Value::String("Hello, {{name}}!".to_string()),
        );
        TRANSLATIONS.write().unwrap().insert("en".to_string(), map);
        *CURRENT_LANGUAGE.write().unwrap() = "en".to_string();

        assert_eq!(
            Lingua::translate("greeting", &[("name", "Alice")]).unwrap(),
            "Hello, Alice!"
        );
    }

    #[test]
    fn test_translate_missing_key() {
        setup();
        let mut map = Map::new();
        map.insert("hello".to_string(), Value::String("Hallo".to_string()));
        TRANSLATIONS.write().unwrap().insert("de".to_string(), map);
        *CURRENT_LANGUAGE.write().unwrap() = "de".to_string();

        assert!(Lingua::translate("world", &[]).is_err());
    }

    #[test]
    fn test_load_lang_from_config() {
        setup();

        let mut map = Map::new();
        map.insert("hello".to_string(), Value::String("Hallo".to_string()));
        TRANSLATIONS.write().unwrap().insert("de".to_string(), map);

        *CURRENT_LANGUAGE.write().unwrap() = "de".to_string();

        let test_dir = std::env::temp_dir().join("lingua_test");
        let _ = fs::create_dir(&test_dir);

        let simple_config_path = test_dir.join("simple_config.txt");
        let _ = fs::write(
            &simple_config_path,
            "# Kommentar\nlanguage=de\nsetting=value",
        );

        let json_config_path = test_dir.join("json_config.txt");
        let _ = fs::write(
            &json_config_path,
            r#"{
            "language": "de",
            "setting": "value"
        }"#,
        );

        let toml_config_path = test_dir.join("toml_config.txt");
        let _ = fs::write(
            &toml_config_path,
            "# Kommentar\nlanguage = \"de\"\nsetting = \"value\"",
        );

        assert_eq!(
            Lingua::load_lang_from_config(&simple_config_path, "language").unwrap(),
            "de"
        );
        assert_eq!(
            Lingua::load_lang_from_config(&json_config_path, "language").unwrap(),
            "de"
        );
        assert_eq!(
            Lingua::load_lang_from_config(&toml_config_path, "language").unwrap(),
            "de"
        );

        let invalid_config_path = test_dir.join("invalid_config.txt");
        let result = Lingua::load_lang_from_config(&invalid_config_path, "language");
        assert!(matches!(result, Err(LinguaError::ConfigFileNotFound(_))));

        let _ = fs::remove_dir_all(&test_dir);
    }
}
