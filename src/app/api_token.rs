//! Functions to deal with the users API token.
use std::fs;
use std::path::Path;

use reqwest::Client;

#[allow(unused_imports)]
use conch;

#[cfg(feature = "trace")]
use conch::StringWrapper;

use bobinator_macros::leave_trace;

use super::config;
use crate::{bob, APIToken, APITokenScope, BobinatorError};

/// Read the current path for API Token file.
fn path<'a>() -> &'a Path {
    Path::new(config::API_TOKEN_PATH.as_str())
}

/// Attempt to read a token from the present location.
pub fn read_token() -> Result<String, BobinatorError> {
    leave_trace!("Attempting to retrieve token from" | "{}", path().display());
    fs::read_to_string(Path::new(path()))
        .map_err(|io_err| BobinatorError::TokenReadError(config::API_TOKEN_PATH.clone(), io_err))
}

/// Attempt to save a token to the present location.
pub fn save_token(token: &APIToken) -> Result<(), BobinatorError> {
    fs::write(path(), token.token.as_bytes()).map_err(|io_err| {
        BobinatorError::TokenSaveError(config::API_TOKEN_PATH.clone(), io_err, token.token.clone())
    })
}

/// Check if the token has the permissions we need.
pub fn check_token(token: &APIToken, scopes: &Vec<APITokenScope>) -> Result<(), BobinatorError> {
    let scopes_needed: Vec<APITokenScope> = scopes
        .iter()
        .filter(|scope| !token.scopes.contains(*scope))
        .map(|scope_ref| scope_ref.clone())
        .collect();

    match scopes_needed.len() {
        0 => Ok(()),
        _ => Err(BobinatorError::TokenPermissionDenied(scopes_needed)),
    }
}

/// Get the existing token, update it to the permissions requested, then store it locally.
pub async fn fetch_update_and_store_token(
    conn: &Client,
    scopes: Vec<APITokenScope>,
) -> Result<APIToken, BobinatorError> {
    let mut token = bob::get_token_scope(conn).await?;

    leave_trace!("Token retrieved" | "{:?}", token);

    if check_token(&token, &scopes).is_err() {
        leave_trace!(
            "Token does not contain all the scopes required" | "{:?}",
            &scopes
        );

        token.extend_scopes(scopes);
        token.sync_scopes(conn).await?;

        leave_trace!("Successfully synced Token" | "{:?}", token);
    }

    leave_trace!("Saving Token to" | "{}", path().display());

    save_token(&token).and(Ok(token))
}
