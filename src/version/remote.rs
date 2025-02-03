use crate::errors::{NodeError, Result};
use crate::version::NodeVersion;
use reqwest::Client;
use semver::{Version, VersionReq};
use serde_json::Value;
use std::str::FromStr;

const NODE_DIST_URL: &str = "https://nodejs.org/dist";

pub async fn fetch_version_list() -> Result<Vec<NodeVersion>> {
    let client = Client::new();
    let response = client
        .get(&format!("{}/index.json", NODE_DIST_URL))
        .send()
        .await
        .map_err(|e| NodeError::DownloadError(e.to_string()))?;

    let versions: Vec<Value> = response
        .json()
        .await
        .map_err(|e| NodeError::DownloadError(e.to_string()))?;

    let mut node_versions = Vec::new();

    for version_data in versions {
        if let (Some(version_str), Some(date)) = (
            version_data["version"].as_str(),
            version_data["date"].as_str(),
        ) {
            let cleaned_version = version_str.trim_start_matches('v');
            if let Ok(version) = Version::from_str(cleaned_version) {
                let is_lts = version_data["lts"].as_bool().unwrap_or(false) ||
                    version_data["lts"].is_string();
                node_versions.push(NodeVersion::new(
                    version,
                    is_lts,
                    date.to_string(),
                ));
            }
        }
    }

    node_versions.sort_by(|a, b| b.version.cmp(&a.version));
    Ok(node_versions)
}

pub async fn get_latest_lts() -> Result<NodeVersion> {
    let versions = fetch_version_list().await?;
    versions
        .into_iter()
        .find(|v| v.lts)
        .ok_or_else(|| NodeError::VersionNotFound("No LTS version found. Try specifying a version manually.".to_string()))
}

pub async fn get_latest() -> Result<NodeVersion> {
    let versions = fetch_version_list().await?;
    versions
        .first()
        .cloned()
        .ok_or_else(|| NodeError::VersionNotFound("No versions found".to_string()))
}

pub async fn resolve_version(version_spec: &str) -> Result<NodeVersion> {
    if let Ok(exact_version) = Version::from_str(version_spec) {
        let versions = fetch_version_list().await?;
        if let Some(version) = versions.into_iter().find(|v| v.version == exact_version) {
            return Ok(version);
        }
        return Err(NodeError::VersionNotFound(format!(
            "Version {} not found. Use 'nrvm list --remote' to see available versions.",
            version_spec
        )));
    }

    // Handle version requirements (e.g., "12", "12.x", ">=12.0.0")
    let req_str = if version_spec.contains('.') || version_spec.contains('>') {
        version_spec.to_string()
    } else {
        format!("^{}.0.0", version_spec)
    };

    let req = VersionReq::parse(&req_str).map_err(|_| {
        NodeError::InvalidVersion(format!("Invalid version specification: {}. Use format like '14' or '14.17.0'", version_spec))
    })?;

    let versions = fetch_version_list().await?;
    versions
        .into_iter()
        .find(|v| req.matches(&v.version))
        .ok_or_else(|| NodeError::VersionNotFound(format!("No version matching {} found", version_spec)))
}

pub fn get_download_url(version: &NodeVersion) -> String {
    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        "x86"
    };

    let os = if cfg!(target_os = "windows") {
        "win"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };

    let ext = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };

    format!(
        "{}/v{}/node-v{}-{}-{}.{}",
        NODE_DIST_URL,
        version.version_str(),
        version.version_str(),
        os,
        arch,
        ext
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_version_list() {
        let versions = fetch_version_list().await.unwrap();
        assert!(!versions.is_empty());

        // Verify sorting
        let mut prev_version = None;
        for version in &versions {
            if let Some(prev) = prev_version {
                assert!(version.version <= prev);
            }
            prev_version = Some(version.version.clone());
        }
    }

    #[tokio::test]
    async fn test_get_lts_version() {
        let lts = get_latest_lts().await.unwrap();
        assert!(lts.lts);
    }
}