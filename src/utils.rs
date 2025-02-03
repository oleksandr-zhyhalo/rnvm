use std::path::PathBuf;
use std::error::Error;

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
        std::fs::create_dir_all(&base_dir)?;
    }

    let versions_dir = base_dir.join("versions");
    if !versions_dir.exists() {
        std::fs::create_dir_all(&versions_dir)?;
    }

    Ok(base_dir)
}

pub fn check_permissions(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    use std::fs::OpenOptions;

    // Try to create and remove a test file to check write permissions
    let test_file = path.join(".permissions_test");
    match OpenOptions::new()
        .write(true)
        .create(true)
        .open(&test_file)
    {
        Ok(_) => {
            std::fs::remove_file(&test_file)?;
            Ok(())
        },
        Err(e) => Err(format!(
            "Insufficient permissions for directory {}: {}",
            path.display(),
            e
        ).into())
    }
}
