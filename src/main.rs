use colored::*;
use std::fs;
use std::path::Path;

fn rls_cli(path: &Path) {
    println!("          ┌───────────┐");
    println!("          | Rust List |");
    println!("          └───────────┘");

    let entries = fs::read_dir(path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let name = entry.file_name().to_string_lossy().to_string();

        if file_type.is_dir() {
            println!("{}", format!("            {}", name).green().bold());
        } else {
            println!("{}", format!("            {}", name).white());
        }
    }
    println!("          └───────────┘");
}

fn main() {
    rls_cli(Path::new("."));
}
