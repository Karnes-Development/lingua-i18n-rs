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
lingua-i18n-rs = "0.1.1"
```

## Quick Start

```rust
use lingua_i18n_rs::prelude::*;

fn main() {
    // Initialize with language files in the "languages" directory
    Lingua::init().unwrap();

    // Get a simple translation
    println!("{}", Lingua::t("welcome", &[]).unwrap());

    // With parameter substitution
    println!("{}", Lingua::t("greeting", &[("name", "World")]).unwrap());

    // Using nested keys
    println!("{}", Lingua::t("menu.file.save", &[]).unwrap());

    // List available languages
    let languages = Lingua::get_languages().unwrap();
    println!("Available languages: {:?}", languages);

    // Change language
    if Lingua::set_language("fr") {
        println!("Language changed to French");
    }
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

### `Lingua::init() -> Result<(), LinguaError>`
Initialize with the default language directory ("language").

### `Lingua::init_with_dir(dir: &str) -> Result<(), LinguaError>`
Initialize with a custom language directory.

### `Lingua::t(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError>`
Translate a key with optional parameters. Short form of `translate`.

### `Lingua::translate(key: &str, params: &[(&str, &str)]) -> Result<String, LinguaError>`
Translate a key with optional parameters.

### `Lingua::set_language(lang_code: &str) -> Result<bool, LinguaError>`
Change the current language. Returns true if successful.

### `Lingua::get_languages() -> Result<Vec<String>, LinguaError>`
Get a list of all available languages.

### `Lingua::get_language() -> Result<String, LinguaError>`
Get the current language code.

## Examples

See the [examples](examples/) directory for more complete examples.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
