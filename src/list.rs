use crate::utils;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn list_installed_versions() -> Result<(), Box<dyn Error>> {
    let base_dir: PathBuf = utils::get_base_dir();
    let versions_dir = base_dir.join("versions");

    if !versions_dir.exists() {
        println!("No Node.js versions installed yet.");
        return Ok(());
    }

    let current_symlink = base_dir.join("current");
    let active_version: Option<String> = if current_symlink.exists() {
        fs::read_link(&current_symlink)
            .ok()
            .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
    } else {
        None
    };

    let mut versions: Vec<String> = Vec::new();
    for entry in fs::read_dir(&versions_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name() {
                let name = dir_name.to_string_lossy().to_string();
                if name == "downloads" {
                    continue;
                }
                versions.push(name);
            }
        }
    }

    versions.sort();

    // Print active version on top.
    if let Some(active) = active_version {
        println!("* {}", active);
        versions.retain(|v| v != &active);
    }

    for v in versions {
        println!("  {}", v);
    }

    Ok(())
}
