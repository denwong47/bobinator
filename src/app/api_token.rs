//! Functions to deal with the users API token.
use std::fs;
use std::path::Path;

#[allow(unused_imports)]
use conch;

use super::config;
use crate::{APIToken, APITokenScope, BobinatorError};

/// Read the current path for API Token file.
fn path<'a>() -> &'a Path {
    Path::new(config::API_TOKEN_PATH.as_str())
}

/// Attempt to read a token from the present location.
pub fn read_token() -> Result<String, BobinatorError> {
    fs::read_to_string(Path::new(path()))
        .map_err(|io_err| BobinatorError::TokenReadError(config::API_TOKEN_PATH.clone(), io_err))
}

/// Attempt to save a token to the present location.
pub fn save_token() -> Result<(), BobinatorError> {
    todo!()
}

/// Check if the token has the permissions we need.
pub fn check_token(token: &APIToken, scopes: Vec<APITokenScope>) -> Result<(), BobinatorError> {
    let scopes_needed: Vec<APITokenScope> = scopes
        .into_iter()
        .filter(|scope| !token.scopes.contains(scope))
        .collect();

    match scopes_needed.len() {
        0 => Ok(()),
        _ => Err(BobinatorError::TokenPermissionDenied(scopes_needed)),
    }
}
