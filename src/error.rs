use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinguaError {
    #[error("Failed to access language directory: {0}")]
    DirectoryAccess(#[from] std::io::Error),
    #[error("Failed to parse language file {file}: {error}")]
    JsonParse {
        file: String,
        #[source]
        error: serde_json::Error,
    },
    #[error("Language '{0}' is not available")]
    LanguageNotAvailable(String),
    #[error("Translation key '{0}' not found")]
    KeyNotFound(String),
    #[error("Language file for '{0}' not found")]
    LanguageFileNotFound(String),
    #[error("Lingua library has not been initialized")]
    NotInitialized,
    #[error("Config file not found: {0}")]
    ConfigFileNotFound(String),
    #[error("Error reading config file: {0}")]
    ConfigFileReadError(String),
    #[error("Could not find value for key '{0}' in config file")]
    ValueNotFoundInConfig(String),
}
