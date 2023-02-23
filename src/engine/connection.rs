use reqwest;

use crate::structs::{BobinatorError, Headers};

pub struct Connection;
impl Connection {
    pub fn new(headers: Option<Headers>) -> Result<reqwest::Client, BobinatorError> {
        reqwest::Client::builder()
            .default_headers(headers.unwrap_or_default().into())
            .cookie_store(true) // Enable Cookie store - we need this
            .build()
            .map_err(|err| BobinatorError::ClientBuildError(err))
    }
}
