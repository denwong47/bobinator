use crate::*;
use reqwest::{Client, StatusCode};

/// Attempt to send a login request using the credentials provided.
pub async fn login(
    conn: &Client,
    email: String,
    password: String,
) -> Result<Employee, BobinatorError> {
    let credentials = Credentials::new(email, password);

    let req = conn
        .post("https://app.hibob.com/api/login")
        .json(&credentials)
        .send()
        .await
        .map_err(|err| BobinatorError::ClientConnectionError(err))?;

    let code = req.status();

    match code {
        StatusCode::UNAUTHORIZED => Err(BobinatorError::BobUnauthorised),
        StatusCode::OK => {
            let employee = req
                .json()
                .await
                .map_err(|de_err| BobinatorError::DataJSONDecodeError(de_err.to_string()))?;

            Ok(employee)
        }
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}

// Attempt to logout from a session.
pub async fn logout(conn: &Client) -> Result<(), BobinatorError> {
    let req = conn
        .post("https://api.hibob.com/api/logout")
        .json("{}")
        .send()
        .await
        .map_err(|err| BobinatorError::ClientConnectionError(err))?;

    let code = req.status();

    match code {
        StatusCode::UNAUTHORIZED => Err(BobinatorError::BobUnauthorised),
        StatusCode::OK => {
            println!("Logout succeeded, bye!");
            Ok(())
        }
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}
