use crate::structs::BobinatorError;
use reqwest::{Response, StatusCode};

/// Error handle a response, and give a [`Result<Response, BobinatorError>`] in return.
pub fn handle_response(response: Response) -> Result<Response, BobinatorError> {
    match response.status() {
        StatusCode::OK => Ok(response),

        StatusCode::UNAUTHORIZED => Err(BobinatorError::BobUnauthorised),
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}
