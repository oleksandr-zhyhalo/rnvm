use crate::utils;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::cmp::Ordering;
use semver::Version;

pub fn use_node_version(version_spec: &str) -> Result<String, Box<dyn Error>> {
    let base_dir: PathBuf = utils::get_base_dir();
    let versions_dir = base_dir.join("versions");

    if !versions_dir.exists() {
        return Err("No Node.js versions are installed.".into());
    }

    // Get all installed versions
    let mut installed_versions: Vec<String> = fs::read_dir(&versions_dir)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            entry.file_name()
                .into_string()
                .ok()
                .filter(|name| !name.starts_with('.') && name != "downloads")
        })
        .collect();

    if installed_versions.is_empty() {
        return Err("No Node.js versions are installed.".into());
    }

    // Find matching version
    let target_version = if version_spec.contains('.') {
        // Partial or full version specified
        installed_versions
            .into_iter()
            .find(|v| v.starts_with(version_spec))
            .ok_or_else(|| format!("No installed version matches the specification: {}", version_spec))?
    } else {
        // Major version only, find latest matching version
        installed_versions.sort_by(|a, b| {
            let ver_a = Version::parse(a).unwrap_or_else(|_| Version::new(0, 0, 0));
            let ver_b = Version::parse(b).unwrap_or_else(|_| Version::new(0, 0, 0));
            ver_b.cmp(&ver_a)
        });

        installed_versions
            .into_iter()
            .find(|v| v.starts_with(&format!("{}.", version_spec)))
            .ok_or_else(|| format!("No installed version matches major version: {}", version_spec))?
    };

    let version_dir = versions_dir.join(&target_version);
    let current_symlink = base_dir.join("current");

    // Check if already using this version
    if current_symlink.exists() {
        if let Ok(target) = fs::read_link(&current_symlink) {
            if target == version_dir {
                return Ok(format!("Node.js version {} is already active.", target_version));
            }
        }
        fs::remove_file(&current_symlink)?;
    }

    // Create symlink
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&version_dir, &current_symlink)?;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::symlink_dir;
        symlink_dir(&version_dir, &current_symlink)?;
    }

    Ok(format!("Switched to Node.js version {}.", target_version))
}

// Helper function to parse and compare versions
fn compare_versions(a: &str, b: &str) -> Ordering {
    let parse_version = |v: &str| {
        let parts: Vec<u32> = v
            .split('.')
            .map(|p| p.parse().unwrap_or(0))
            .collect();
        (
            *parts.get(0).unwrap_or(&0),
            *parts.get(1).unwrap_or(&0),
            *parts.get(2).unwrap_or(&0),
        )
    };

    let ver_a = parse_version(a);
    let ver_b = parse_version(b);
    ver_b.cmp(&ver_a)
}