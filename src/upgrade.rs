use crate::installer;
use crate::version;
use crate::utils;
use std::error::Error;
use std::fs;

pub async fn upgrade_node_version(version_spec: Option<&str>) -> Result<String, Box<dyn Error>> {
    // Determine which version branch to upgrade.
    let spec = if let Some(s) = version_spec {
        s.to_string()
    } else {
        let base_dir = utils::get_base_dir();
        let current_symlink = base_dir.join("current");
        if !current_symlink.exists() {
            return Err("No active Node.js version to upgrade.".into());
        }
        fs::read_link(&current_symlink)?
            .file_name()
            .ok_or("Invalid active version folder.")?
            .to_string_lossy()
            .to_string()
    };

    // Resolve the new version from the spec.
    let new_version = installer::resolve_node_version(&spec).await?;
    if let Some(s) = version_spec {
        if s == &new_version {
            return Ok(format!("Already at the latest version for spec {}.", s));
        }
    }

    installer::install_node_version(&new_version).await?;
    let msg = version::use_node_version(&new_version)?;
    Ok(format!("Upgraded to Node.js version {}. {}", new_version, msg))
}