use directories::UserDirs;
use lazy_static::lazy_static;
use std::fs;
use std::path::{Path, PathBuf};

use bobinator_macros::leave_trace;
use bobinator_models::structs::BobinatorError;

/// Default
pub static API_CONFIG_FOLDER: &'static str = ".bobinator";

/// Default file name for the API token file.
pub static API_TOKEN_FILE_NAME: &'static str = "token";

#[cfg(feature = "trace")]
use conch::StringWrapper;

lazy_static! {
    pub static ref API_CONFIG_PATH: String = UserDirs::new()
        .and_then(|user_dir| user_dir.home_dir().canonicalize().ok())
        .unwrap_or(PathBuf::new())
        .join(API_CONFIG_FOLDER)
        .to_str()
        .unwrap()
        .to_string();
    pub static ref API_TOKEN_PATH: String = Path::new(&API_CONFIG_PATH.to_string())
        .join(API_TOKEN_FILE_NAME)
        .to_str()
        .unwrap()
        .to_string();
}

pub fn ensure_config_path_exists() -> Result<(), BobinatorError> {
    let config_path = Path::new(API_CONFIG_PATH.as_str());

    if !config_path.is_dir() {
        leave_trace!("Creating target directory at" | "{}", config_path.display());
        fs::create_dir(config_path)
            .map_err(|err| BobinatorError::ConfigPathError(API_CONFIG_PATH.clone(), err))
    } else {
        leave_trace!(
            "Target directory already exists at" | "{}",
            config_path.display()
        );
        Ok(())
    }
}
