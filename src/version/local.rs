use crate::errors::{NodeError, Result};
use semver::Version;
use std::fs;
use std::path::{Path, PathBuf};

pub struct LocalVersion {
    pub version: Version,
    pub path: PathBuf,
    pub is_current: bool,
}

impl LocalVersion {
    pub fn version_str(&self) -> String {
        self.version.to_string()
    }
}

pub fn get_versions_dir() -> PathBuf {
    crate::utils::get_base_dir().join("versions")
}

pub fn is_empty() -> bool {
    let versions_dir = get_versions_dir();
    if !versions_dir.exists() {
        return true;
    }

    match fs::read_dir(&versions_dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name != "downloads" {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        Err(_) => return true,
    }
    true
}

pub fn is_installed(version: &str) -> Result<bool> {
    let version_dir = get_versions_dir().join(version);
    Ok(version_dir.exists() && version_dir.is_dir())
}

pub fn get_current_version() -> Result<Option<String>> {
    let current_link = crate::utils::get_base_dir().join("current");
    if !current_link.exists() {
        return Ok(None);
    }

    let target = fs::read_link(&current_link)?;
    Ok(target
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string()))
}

pub fn use_version(version: &str) -> Result<()> {
    let versions_dir = get_versions_dir();
    let version_dir = versions_dir.join(version);

    if !version_dir.exists() {
        return Err(NodeError::VersionNotInstalled(format!(
            "Version {} is not installed. Use 'rnvm install {}' first.",
            version, version
        )));
    }

    let current_link = crate::utils::get_base_dir().join("current");
    if current_link.exists() {
        fs::remove_file(&current_link)?;
    }

    symlink_version(&version_dir, &current_link)?;
    Ok(())
}

pub fn remove_version(version: &str) -> Result<()> {
    let version_dir = get_versions_dir().join(version);
    if !version_dir.exists() {
        return Err(NodeError::VersionNotInstalled(format!(
            "Version {} is not installed.",
            version
        )));
    }

    // Check if it's the current version
    if let Some(current) = get_current_version()? {
        if current == version {
            return Err(NodeError::VersionNotInstalled(
                "Cannot remove the currently active version. Switch to a different version first.".to_string()
            ));
        }
    }

    fs::remove_dir_all(&version_dir)?;
    Ok(())
}

pub fn get_installed_versions() -> Result<Vec<LocalVersion>> {
    let versions_dir = get_versions_dir();
    if !versions_dir.exists() || is_empty() {
        return Ok(Vec::new());
    }

    let current_version = get_current_version()?;
    let mut versions = Vec::new();

    for entry in fs::read_dir(&versions_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if let Some(version_str) = path.file_name().and_then(|n| n.to_str()) {
                if version_str == "downloads" {
                    continue;
                }

                if let Ok(version) = Version::parse(version_str) {
                    versions.push(LocalVersion {
                        version,
                        path: path.clone(),
                        is_current: Some(version_str.to_string()) == current_version,
                    });
                }
            }
        }
    }

    versions.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(versions)
}

#[cfg(unix)]
fn symlink_version(version_dir: &Path, link: &Path) -> Result<()> {
    std::os::unix::fs::symlink(version_dir, link)?;
    Ok(())
}

#[cfg(windows)]
fn symlink_version(version_dir: &Path, link: &Path) -> Result<()> {
    std::os::windows::fs::symlink_dir(version_dir, link)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile;

    fn setup_test_env() -> tempfile::TempDir {
        let temp_dir = tempfile::tempdir().unwrap();
        std::env::set_var("HOME", temp_dir.path());
        let versions_dir = temp_dir.path().join(".nrvm").join("versions");
        fs::create_dir_all(&versions_dir).unwrap();
        temp_dir
    }

    #[test]
    fn test_version_management() {
        let temp_dir = setup_test_env();
        let versions_dir = get_versions_dir();

        // Test empty state
        assert!(is_empty());

        // Create test versions
        fs::create_dir_all(versions_dir.join("14.0.0")).unwrap();
        fs::create_dir_all(versions_dir.join("16.0.0")).unwrap();

        // Test non-empty state
        assert!(!is_empty());

        // Test installation check
        assert!(is_installed("14.0.0").unwrap());
        assert!(is_installed("16.0.0").unwrap());
        assert!(!is_installed("18.0.0").unwrap());

        // Test version listing
        let versions = get_installed_versions().unwrap();
        assert_eq!(versions.len(), 2);
        assert_eq!(versions[0].version_str(), "16.0.0");
        assert_eq!(versions[1].version_str(), "14.0.0");

        // Test version switching
        use_version("14.0.0").unwrap();
        assert_eq!(get_current_version().unwrap(), Some("14.0.0".to_string()));

        // Test version removal
        use_version("16.0.0").unwrap();
        remove_version("14.0.0").unwrap();
        assert!(!is_installed("14.0.0").unwrap());

        // Cleanup
        temp_dir.close().unwrap();
    }
}