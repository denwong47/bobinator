//! Functions to deal with the users API token.
use std::fs;
use std::path::Path;

use reqwest::Client;

#[allow(unused_imports)]
use conch;

#[cfg(feature = "trace")]
use conch::StringWrapper;

use bobinator_macros::leave_trace;
use serde::Serialize;

use super::*;
use crate::{bob, APIToken, APITokenOnly, APITokenScope, BobinatorError, Connection, HasToken};

/// Read the current path for API Token file.
fn path<'a>() -> &'a Path {
    Path::new(config::API_TOKEN_PATH.as_str())
}

/// Attempt to read a token from the present location.
pub fn read_token() -> Result<APITokenOnly, BobinatorError> {
    leave_trace!("Attempting to retrieve token from" | "{}", path().display());

    fs::read_to_string(path())
        .map(|token_str| {
            leave_trace!("Token retrieved" | "{}", &token_str);
            APITokenOnly { token: token_str }
        })
        .map_err(|io_err| BobinatorError::TokenReadError(config::API_TOKEN_PATH.clone(), io_err))
}

/// Attempt to save a token to the present location.
pub fn save_token(token: &(impl Serialize + HasToken)) -> Result<(), BobinatorError> {
    leave_trace!(
        "Checking for configuration directory" | "{}",
        config::API_CONFIG_PATH.as_str()
    );
    config::ensure_config_path_exists()?;

    leave_trace!("Writing token to" | "{}", path().display());
    fs::write(path(), token.key().as_bytes()).map_err(|io_err| {
        BobinatorError::TokenSaveError(config::API_TOKEN_PATH.clone(), io_err, token.key())
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

/// Check the current machine for Token, if not found, perform a login.
pub async fn get_token_or_login() -> Result<impl HasToken, BobinatorError> {
    // We can't use `.map_or` here because `.or` is eagerly evaluated, whether
    // Result is `Ok` or `Err`, meaning the user will be prompted to login
    // regardless!
    match read_token() {
        Ok(token) => {
            leave_trace!("We have a token, no need to login" | "{:?}", &token);
            Ok(token)
        }
        Err(_) => {
            leave_trace!("No token found" | "We need to prompt user for login.");

            let (email, password) = login::login_prompt()?;

            // Temporary client with full login access
            let conn = Connection::new(None)?;

            let employee = bob::login(&conn, email, password).await?;
            employee.greet();

            let token = fetch_update_and_store_token(
                &conn,
                vec![
                    APITokenScope::FullEmployeeRead,
                    APITokenScope::EmployeeFieldsRead,
                    APITokenScope::Timeoff,
                ],
            )
            .await?;

            Ok(token.into())
        }
    }
}
