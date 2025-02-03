pub mod alias;
pub mod local;

use std::path::PathBuf;

pub fn get_config_dir() -> PathBuf {
    crate::utils::get_base_dir().join("config")
}