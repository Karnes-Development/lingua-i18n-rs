use std::path::Path;

use lingua_i18n_rs::prelude::*;

fn main() {
    Lingua::new("examples/basic/languages").init().unwrap();

    let config_file_toml = Path::new("examples/load_config/config.toml");
    let config_file_json = Path::new("examples/load_config/config.json");

    let config_toml = Lingua::load_lang_from_config(config_file_toml, "language").unwrap();
    let config_json = Lingua::load_lang_from_config(config_file_json, "lang").unwrap();

    println!("Current language toml: {}", config_toml);
    println!("Current language json: {}", config_json);
}
