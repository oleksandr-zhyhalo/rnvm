use crate::errors::{NodeError, Result};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

pub fn extract_archive(archive_path: &Path, version: &str) -> Result<()> {
    let versions_dir = crate::version::local::get_versions_dir();
    println!("Extracting to: {}", versions_dir.display());

    if archive_path.extension().and_then(|e| e.to_str()) == Some("zip") {
        extract_zip(archive_path, &versions_dir)?;
    } else {
        extract_tar_gz(archive_path, &versions_dir)?;
    }

    let extracted_dir = find_extracted_dir(&versions_dir)?;
    let target_dir = versions_dir.join(version);

    if target_dir.exists() {
        std::fs::remove_dir_all(&target_dir)?;
    }

    std::fs::rename(extracted_dir, target_dir)?;
    Ok(())
}

fn extract_zip(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let file = File::open(archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let pb = create_progress_bar(archive.len() as u64);

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        pb.inc(1);
    }

    pb.finish_with_message("Extraction completed");
    Ok(())
}

fn extract_tar_gz(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    let entries = archive.entries()?;
    let entry_count = entries.count();

    let pb = create_progress_bar(entry_count as u64);

    // Reset the archive after counting entries
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);

    archive.unpack(target_dir)?;
    pb.finish_with_message("Extraction completed");

    Ok(())
}

fn find_extracted_dir(parent_dir: &Path) -> Result<PathBuf> {
    for entry in std::fs::read_dir(parent_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with("node-v"))
            .unwrap_or(false)
        {
            return Ok(path);
        }
    }
    Err(NodeError::ExtractionError("Could not find extracted directory".into()))
}

fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
        .unwrap()
        .progress_chars("#>-"));
    pb
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_extract_tar_gz() {
        let temp_dir = tempfile::tempdir().unwrap();
        let archive_path = temp_dir.path().join("test.tar.gz");
        assert!(extract_tar_gz(&archive_path, temp_dir.path()).is_err());
    }
}