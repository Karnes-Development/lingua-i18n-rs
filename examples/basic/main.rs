use lingua_i18n_rs::prelude::*;
use std::io::{self, Write};

fn main() -> Result<(), LinguaError> {
    // Initialize with the default language.
    println!("Initializing i18n...");
    Lingua::init_with_dir("examples/basic/languages")?;

    // List all available languages.
    let languages = Lingua::get_languages()?;
    println!("Available languages: {:?}", languages);
    println!("Current language: {}", Lingua::get_language()?);

    // Show translations for the current language.
    show_translations();

    // Language selection loop
    loop {
        println!(
            "\nSelect a language ({}) or 'q' to quit:",
            languages.join(", ")
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "q" {
            break;
        }

        if Lingua::set_language(input)? {
            println!("Language changed to: {}", input);
            show_translations();
        } else {
            println!("Language '{}' not available", input);
        }
    }

    println!("Goodbye!");

    Ok(())
}

fn show_translations() {
    println!(
        "\n--- Translations in {} ---",
        Lingua::get_language().expect("Failed to get current language")
    );
    println!("Welcome message: {}", Lingua::t("welcome", &[]));
    println!("File menu:");
    println!("  Open: {}", Lingua::t("menu.file.open", &[]));
    println!("  Save: {}", Lingua::t("menu.file.save", &[]));
    println!("  Exit: {}", Lingua::t("menu.file.exit", &[]));
    println!("Edit menu:");
    println!("  Copy: {}", Lingua::t("menu.edit.copy", &[]));
    println!("  Paste: {}", Lingua::t("menu.edit.paste", &[]));
    println!("With parameters:");
    println!(
        "  Greeting: {}",
        Lingua::t("greeting", &[("name", "Alice")])
    );
    println!("  Items: {}", Lingua::t("items_count", &[("count", "5")]));
}
