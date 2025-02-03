pub mod download;
pub mod extract;

use crate::errors::Result;
use crate::version::NodeVersion;

pub async fn install_version(version: &NodeVersion) -> Result<()> {
    if crate::version::local::is_installed(&version.version_str())? {
        return Ok(());
    }

    let download_dir = crate::utils::get_base_dir().join("downloads");
    std::fs::create_dir_all(&download_dir)?;

    let archive_path = download::download_version(version, &download_dir).await?;

    extract::extract_archive(&archive_path, &version.version_str())?;

    std::fs::remove_file(archive_path)?;

    Ok(())
}