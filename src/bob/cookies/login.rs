use reqwest::{Client, StatusCode};

use bobinator_macros::map_post_to_struct;
use bobinator_models;

use crate::*;

/// Attempt to send a login request using the credentials provided.
/// Stores cookie into the active Clients with full user permissions.
pub async fn login(
    conn: &Client,
    email: String,
    password: String,
) -> Result<Employee, BobinatorError> {
    let credentials = Credentials::new(email, password);

    let fut = conn
        .post("https://app.hibob.com/api/login")
        .json(&credentials)
        .send();

    // Immediately discard the credentials to make good our promise.
    drop(credentials);
    let response = fut
        .await
        .map_err(|err| BobinatorError::ClientConnectionError(err))?;

    let code = response.status();

    match code {
        StatusCode::UNAUTHORIZED => Err(BobinatorError::BobUnauthorised),
        StatusCode::OK => {
            let employee = response
                .json()
                .await
                .map_err(|de_err| BobinatorError::DataJSONDecodeError(de_err.to_string()))?;

            Ok(employee)
        }
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}

map_post_to_struct! (
    _logout,
    "Internal function to to logout from a session. Must be used in Clients with active cookie stored.",
    "https://api.hibob.com/api/logout",
    String,
    text() -> String,
);

/// Attempt to logout from a session.
/// Must be used in Clients with active cookie stored.
pub async fn logout(conn: &Client) -> Result<(), BobinatorError> {
    _logout(conn, String::from("{}")).await.and(Ok(()))
}
