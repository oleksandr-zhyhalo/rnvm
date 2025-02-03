pub mod remote;
pub mod local;
pub mod compare;

use crate::errors::Result;
use semver::Version;

#[derive(Debug, Clone)]
pub struct NodeVersion {
    pub version: Version,
    pub lts: bool,
    pub date: String,
}

impl NodeVersion {
    pub fn new(version: Version, lts: bool, date: String) -> Self {
        Self { version, lts, date }
    }

    pub fn version_str(&self) -> String {
        self.version.to_string()
    }
}


pub async fn get_matching_version(version_spec: &str) -> Result<NodeVersion> {
    match version_spec {
        "lts" | "lts/*" => remote::get_latest_lts().await,
        "latest" | "node" => remote::get_latest().await,
        _ => {
            if let Some(version) = crate::config::alias::get_alias(version_spec)? {
                remote::resolve_version(&version).await
            } else {
                remote::resolve_version(version_spec).await
            }
        }
    }
}