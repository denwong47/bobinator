use directories::UserDirs;
use lazy_static::lazy_static;
use std::path::Path;

/// Default
pub static API_CONFIG_FOLDER: &'static str = ".bobinator";

/// Default file name for the API token file.
pub static API_TOKEN_FILE_NAME: &'static str = "token";

lazy_static! {
    pub static ref API_CONFIG_PATH: String = UserDirs::new()
        .and_then(|user_dir| user_dir.home_dir().canonicalize().ok())
        .unwrap_or(
            Path::new(".")
                .join(API_CONFIG_FOLDER)
                .canonicalize()
                .unwrap()
        )
        .to_str()
        .unwrap()
        .to_string();
    pub static ref API_TOKEN_PATH: String = Path::new(&API_CONFIG_PATH.to_string())
        .join(API_TOKEN_FILE_NAME)
        .to_str()
        .unwrap()
        .to_string();
}
