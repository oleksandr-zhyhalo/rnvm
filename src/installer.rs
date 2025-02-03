use crate::utils;
use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

const NODE_DIST_URL: &str = "https://nodejs.org/dist";

pub async fn resolve_node_version(version_spec: &str) -> Result<String, Box<dyn Error>> {
    println!("Resolving version for spec: {}", version_spec);
    let index_url = format!("{}/index.json", NODE_DIST_URL);
    println!("Fetching version index from: {}", index_url);

    let response = reqwest::get(&index_url).await?;
    let versions: Vec<Value> = response.json().await?;

    // Find the latest version matching the spec
    let resolved_version = if version_spec.contains('.') {
        // Full version specified
        versions
            .iter()
            .find(|v| v["version"].as_str().unwrap_or("").starts_with(&format!("v{}", version_spec)))
            .ok_or("Version not found")?["version"].as_str().unwrap()
            .trim_start_matches('v')
            .to_string()
    } else {
        // Major version only, find latest LTS
        let mut matching_versions: Vec<&Value> = versions
            .iter()
            .filter(|v| {
                v["version"].as_str()
                    .unwrap_or("")
                    .starts_with(&format!("v{}", version_spec))
            })
            .collect();

        // Sort by version to get the latest
        matching_versions.sort_by(|a, b| {
            let a_ver = a["version"].as_str().unwrap_or("");
            let b_ver = b["version"].as_str().unwrap_or("");
            b_ver.cmp(a_ver)
        });

        matching_versions
            .first()
            .ok_or("No version found for this major version")?["version"]
            .as_str()
            .unwrap()
            .trim_start_matches('v')
            .to_string()
    };

    println!("Resolved to version: {}", resolved_version);
    Ok(resolved_version)
}

pub async fn install_node_version(version: &str) -> Result<(), Box<dyn Error>> {
    println!("Starting installation of Node.js {}", version);

    // First, resolve the full version if a partial version was provided
    let full_version = resolve_node_version(version).await?;
    println!("Using full version: {}", full_version);

    let base_dir = utils::ensure_base_dir()?;
    utils::check_permissions(&base_dir)?;

    let versions_dir = base_dir.join("versions");
    let version_dir = versions_dir.join(&full_version);
    let downloads_dir = versions_dir.join("downloads");

    println!("Creating downloads directory at: {}", downloads_dir.display());
    fs::create_dir_all(&downloads_dir)?;

    let (os, arch, ext) = if cfg!(windows) {
        ("win", if cfg!(target_arch = "x86_64") { "x64" } else { "x86" }, "zip")
    } else if cfg!(target_os = "macos") {
        ("darwin", if cfg!(target_arch = "aarch64") { "arm64" } else { "x64" }, "tar.gz")
    } else {
        ("linux", if cfg!(target_arch = "aarch64") { "arm64" } else { "x64" }, "tar.gz")
    };

    let filename = format!("node-v{}-{}-{}", full_version, os, arch);
    let archive_name = format!("{}.{}", filename, ext);
    let download_url = format!("{}/v{}/{}", NODE_DIST_URL, full_version, archive_name);

    println!("Downloading from URL: {}", download_url);

    // Download the archive
    let response = reqwest::get(&download_url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to download: {} ({})", download_url, response.status()).into());
    }

    let archive_path = downloads_dir.join(&archive_name);
    println!("Saving to: {}", archive_path.display());

    let content = response.bytes().await?;
    let mut file = fs::File::create(&archive_path)?;
    file.write_all(&content)?;

    println!("Download complete. Starting extraction...");

    if ext == "zip" {
        println!("Extracting ZIP archive...");
        extract_zip(&archive_path, &versions_dir)?;
    } else {
        println!("Extracting TAR.GZ archive...");
        extract_tar_gz(&archive_path, &versions_dir)?;
    }

    // Move extracted directory to version directory
    let extracted_dir = versions_dir.join(filename);
    if version_dir.exists() {
        println!("Removing existing version directory...");
        fs::remove_dir_all(&version_dir)?;
    }
    println!("Moving {} to {}", extracted_dir.display(), version_dir.display());
    fs::rename(extracted_dir, version_dir)?;

    println!("Cleaning up downloaded archive...");
    fs::remove_file(archive_path)?;

    println!("Installation complete!");
    Ok(())
}

fn extract_zip(archive_path: &PathBuf, target_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = target_dir.join(file.name());

        if file.name().ends_with('/') {
            println!("Creating directory: {}", outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            println!("Extracting file: {}", outpath.display());
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn extract_tar_gz(archive_path: &PathBuf, target_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Opening tar.gz file: {}", archive_path.display());
    let tar_gz = fs::File::open(archive_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);

    println!("Extracting to: {}", target_dir.display());
    match archive.unpack(target_dir) {
        Ok(_) => println!("Extraction successful"),
        Err(e) => {
            println!("Extraction error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}