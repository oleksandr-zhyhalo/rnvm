use crate::errors::{NodeError, Result};
use crate::version::NodeVersion;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
pub async fn download_version(version: &NodeVersion, download_dir: &PathBuf) -> Result<PathBuf> {
    let client = Client::new();
    let url = crate::version::remote::get_download_url(version);

    let filename = url
        .split('/')
        .last()
        .ok_or_else(|| NodeError::DownloadError("Invalid URL".to_string()))?;
    let output_path = download_dir.join(filename);

    println!("Downloading Node.js {} from {}", version.version_str(), url);

    let response = client
        .head(&url)
        .send()
        .await
        .map_err(|e| NodeError::DownloadError(e.to_string()))?;
    let total_size = response
        .headers()
        .get("content-length")
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse().ok())
        .unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| NodeError::DownloadError(e.to_string()))?;

    if !response.status().is_success() {
        return Err(NodeError::DownloadError(format!(
            "Failed to download: HTTP {}",
            response.status()
        )));
    }

    let mut file = File::create(&output_path)?;
    let mut downloaded: u64 = 0;
    let mut bytes = response
        .bytes()
        .await
        .map_err(|e| NodeError::DownloadError(e.to_string()))?;
    file.write_all(&bytes)?;
    downloaded += bytes.len() as u64;
    pb.set_position(downloaded);

    pb.finish_with_message("Download completed");

    Ok(output_path)
}
