use crate::utils;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub fn uninstall_node_version(version: &str) -> Result<(), Box<dyn Error>> {
    let base_dir: PathBuf = utils::get_base_dir();
    let versions_dir = base_dir.join("versions");
    let version_dir = versions_dir.join(version);

    if !version_dir.exists() {
        return Err(format!("Node.js version {} is not installed.", version).into());
    }

    fs::remove_dir_all(&version_dir)?;
    println!("Uninstalled Node.js version {}.", version);

    let current_symlink = base_dir.join("current");
    if current_symlink.exists() {
        if let Ok(target) = fs::read_link(&current_symlink) {
            if target == version_dir {
                fs::remove_file(&current_symlink)?;
                println!("Removed active symlink since version {} was uninstalled.", version);
            }
        }
    }

    Ok(())
}
