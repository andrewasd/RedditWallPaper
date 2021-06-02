use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    set_schema_version_env_var();

    let rev = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .ok()
        .map(|s| s.stdout)
        .and_then(|s| String::from_utf8(s).ok());
    if let Some(rev) = rev {
        if rev.len() >= 9 {
            println!("cargo:rustc-env=WBG_VERSION={}", &rev[..9]);
        }
    }
}

fn set_schema_version_env_var() {
    let schema_file = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let schema_file = std::fs::read(schema_file).unwrap();

    let mut hasher = DefaultHasher::new();
    hasher.write(&schema_file);

    println!("cargo:rustc-env=SCHEMA_FILE_HASH={}", hasher.finish());
}
