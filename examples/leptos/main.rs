use std::process::Command;
use std::path::Path;

fn main() {
    let example_dir = Path::new("examples/leptos");
    let languages_dir = example_dir.join("languages");
    let basic_languages = Path::new("examples/basic/languages");
    
    if basic_languages.exists() && !languages_dir.exists() {
        std::fs::create_dir_all(&languages_dir).unwrap();
        for entry in std::fs::read_dir(basic_languages).unwrap() {
            let entry = entry.unwrap();
            let _ = std::fs::copy(entry.path(), languages_dir.join(entry.file_name()));
        }
    }
    
    if Command::new("trunk").arg("--version").output().is_err() {
        eprintln!("❌ Trunk is not installed. Install with: cargo install trunk");
        std::process::exit(1);
    }
    
    if !languages_dir.exists() {
        eprintln!("❌ Language directory not found: {}", languages_dir.display());
        std::process::exit(1);
    }
    
    let mut cmd = Command::new("trunk");
    cmd.arg("serve").arg("--open").current_dir(example_dir);
    
    let status = cmd.status().expect("Failed to start trunk");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
