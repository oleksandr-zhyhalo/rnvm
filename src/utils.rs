use std::path::PathBuf;
use std::error::Error;
use std::fs;

pub fn get_base_dir() -> PathBuf {
    let home = if cfg!(windows) {
        std::env::var("USERPROFILE").unwrap_or_else(|_| {
            std::env::var("HOMEDRIVE").unwrap_or_default() +
                &std::env::var("HOMEPATH").unwrap_or_default()
        })
    } else {
        std::env::var("HOME").unwrap_or_else(|_| ".".into())
    };

    PathBuf::from(home).join(".rnvm")
}

pub fn ensure_base_dir() -> Result<PathBuf, Box<dyn Error>> {
    let base_dir = get_base_dir();
    if !base_dir.exists() {
        fs::create_dir_all(&base_dir)?;
    }

    let versions_dir = base_dir.join("versions");
    if !versions_dir.exists() {
        fs::create_dir_all(&versions_dir)?;
    }

    Ok(base_dir)
}

pub fn clean_install() -> Result<(), Box<dyn Error>> {
    let base_dir = get_base_dir();
    if base_dir.exists() {
        fs::remove_dir_all(&base_dir)?;
    }
    ensure_base_dir()?;
    Ok(())
}

pub fn check_permissions(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    use std::fs::OpenOptions;

    let test_file = path.join(".permissions_test");
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&test_file)
    {
        Ok(_) => {
            fs::remove_file(&test_file)?;
            Ok(())
        },
        Err(e) => Err(format!(
            "Insufficient permissions for directory {}: {}",
            path.display(),
            e
        ).into())
    }
}