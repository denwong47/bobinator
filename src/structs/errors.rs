use reqwest::{self, StatusCode};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BobinatorError {
    #[error("Cannot build Client because reqwest reported: {0:?}")]
    ClientBuildError(reqwest::Error),

    #[error("Client could not connect to remote host: {0:?}")]
    ClientConnectionError(reqwest::Error),

    #[error("Client could not deserialise returned JSON: {0:?}")]
    ClientJSONDecodeError(reqwest::Error),

    #[error("Data deserialization failed: {0}")]
    DataJSONDecodeError(String),

    #[error("Bob refused your login; check your email and password combination.")]
    BobUnauthorised,

    #[error("Bob returned an error code that we don't expect: {0:?}")]
    ServerReturnedUnexpectedStatus(StatusCode),

    #[error("Bob returned an incomplete record: field '{0}' is missing.")]
    RecordFieldMissing(String),

    #[error("Bob returned a '{0}' that is the wrong format: {1:?}")]
    RecordFieldInvalid(String, Value),
}
