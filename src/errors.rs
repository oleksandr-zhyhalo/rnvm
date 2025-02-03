use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Version not found: {0}")]
    VersionNotFound(String),

    #[error("Version {0} is not installed")]
    VersionNotInstalled(String),

    #[error("Download error: {0}")]
    DownloadError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid version format: {0}")]
    InvalidVersion(String),

    #[error("Extraction error: {0}")]
    ExtractionError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Alias error: {0}")]
    AliasError(String),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("ZIP error: {0}")]
    ZipError(#[from] ZipError),
}

pub type Result<T> = std::result::Result<T, NodeError>;