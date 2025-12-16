# Lingua I18n for Rust

[![Crates.io](https://img.shields.io/crates/v/lingua-i18n-rs.svg)](https://crates.io/crates/lingua-i18n-rs)
[![Downloads](https://img.shields.io/crates/d/lingua-i18n-rs.svg)](https://crates.io/crates/lingua-i18n-rs)
[![Documentation](https://docs.rs/lingua-i18n-rs/badge.svg)](https://docs.rs/lingua-i18n-rs)
[![License](https://img.shields.io/crates/l/lingua-i18n-rs.svg)](https://github.com/Karnes-Development/lingua-i18n-rs/blob/main/LICENSE)

A simple and lightweight internationalization (i18n) library for Rust applications.

## Features

- Easy to set up and use
- JSON-based translations
- Nested keys support with dot notation
- Variable substitution with {{variable}} syntax
- Automatic language detection from system settings
- Minimal dependencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
lingua-i18n-rs = "0.2.0"
```

## Quick Start

```rust
use lingua_i18n_rs::prelude::*;

fn main() -> Result<(), LinguaError> {
    // Initialize with language files in the "languages" directory
    Lingua::new("languages").init()?;

    // Get a simple translation
    println!("{}", Lingua::t("welcome", &[])?);

    // With parameter substitution
    println!("{}", Lingua::t("greeting", &[("name", "World")])?);

    // Using nested keys
    println!("{}", Lingua::t("menu.file.save", &[])?);

    // List available languages
    let languages = Lingua::get_languages()?;
    println!("Available languages: {:?}", languages);

    // Change language
    if Lingua::set_language("fr")? {
        println!("Language changed to French");
    }

    Ok(())
}
```

## Language Files

Place your translation files in a directory (default: "languages"). Each file should be named with its language code and have a `.json` extension:

**languages/en.json**:
```json
{
  "welcome": "Welcome to the application!",
  "greeting": "Hello, {{name}}!",
  "menu": {
    "file": {
      "save": "Save"
    }
  }
}
```

**languages/de.json**:
```json
{
  "welcome": "Willkommen in der Anwendung!",
  "greeting": "Hallo, {{name}}!",
  "menu": {
    "file": {
      "save": "Speichern"
    }
  }
}
```

## API Reference

### `Lingua::new(language_dir: &str) -> LinguaBuilder`
Create a new builder with a custom language directory.

### `LinguaBuilder::init() -> Result<Lingua, LinguaError>`
Initialize the library and load all available languages from the specified directory.

### `Lingua::t(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError>`
Translate a key with optional parameters. Short form of `translate`.

### `Lingua::translate(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError>`
Translate a key with optional parameters.

### `Lingua::set_language(lang_code: &str) -> Result<bool, LinguaError>`
Change the current language. Returns `Ok(true)` if successful, or an error if the language is not available.

### `Lingua::get_languages() -> Result<Vec<String>, LinguaError>`
Get a list of all available languages.

### `Lingua::get_language() -> Result<String, LinguaError>`
Get the current language code.

### `Lingua::load_lang_from_config(path: &Path, key: &str) -> Result<String, LinguaError>`
Load a language code from a configuration file. If you are using a configuration file to store the language code, you can use this function to load it.

## Examples

See the [examples](examples/) directory for more complete examples. To run an example, use the following command:

```sh
cargo run --example <example_name>
```

For an example using Lingua, see the [rusty-weather](https://github.com/KarnesTH/rusty-weather)

## License

This project is licensed under the MIT License - see the LICENSE file for details.
