use crate::errors::{NodeError, Result};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_version_file() -> Option<(PathBuf, String)> {
    let mut current_dir = std::env::current_dir().ok()?;

    loop {
        let nvmrc = current_dir.join(".nvmrc");
        if nvmrc.exists() {
            if let Ok(version) = fs::read_to_string(&nvmrc) {
                return Some((nvmrc, clean_version_string(version)));
            }
        }

        let package_json = current_dir.join("package.json");
        if package_json.exists() {
            if let Ok(content) = fs::read_to_string(&package_json) {
                if let Ok(json) = serde_json::from_str::<Value>(&content) {
                    if let Some(engines) = json.get("engines") {
                        if let Some(node_version) = engines.get("node") {
                            if let Some(version) = node_version.as_str() {
                                return Some((package_json, clean_version_string(version.to_string())));
                            }
                        }
                    }
                    if let Some(volta) = json.get("volta") {
                        if let Some(node_version) = volta.get("node") {
                            if let Some(version) = node_version.as_str() {
                                return Some((package_json, clean_version_string(version.to_string())));
                            }
                        }
                    }
                }
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    None
}

pub fn create_nvmrc(version: &str) -> Result<PathBuf> {
    let nvmrc_path = Path::new(".nvmrc");
    fs::write(nvmrc_path, version)?;
    Ok(nvmrc_path.to_path_buf())
}

fn clean_version_string(version: String) -> String {
    version
        .trim()
        .trim_start_matches('v')
        .trim_matches('"')
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_version_file_detection() {
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path();
        std::env::set_current_dir(temp_path).unwrap();

        fs::write(".nvmrc", "16.0.0").unwrap();
        let (path, version) = find_version_file().unwrap();
        assert_eq!(path.file_name().unwrap(), ".nvmrc");
        assert_eq!(version, "16.0.0");

        // Test package.json detection
        let package_json = r#"{
            "engines": {
                "node": ">=14.0.0"
            }
        }"#;
        fs::write("package.json", package_json).unwrap();
        let (path, version) = find_version_file().unwrap();
        assert_eq!(path.file_name().unwrap(), "package.json");
        assert_eq!(version, ">=14.0.0");
    }

    #[test]
    fn test_clean_version_string() {
        assert_eq!(clean_version_string("v16.0.0".to_string()), "16.0.0");
        assert_eq!(clean_version_string("\"16.0.0\"".to_string()), "16.0.0");
        assert_eq!(clean_version_string(" 16.0.0 ".to_string()), "16.0.0");
    }
}