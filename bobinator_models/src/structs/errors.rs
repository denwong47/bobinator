use std::io;

use chrono::NaiveDate;
#[allow(unused_imports)]
use reqwest::{Response, StatusCode};
use thiserror::Error;
use tokio::task::JoinError;

use super::APITokenScope;

/// Enum class representing all the possible failure modes in Bobinator.
#[derive(Error, Debug)]
pub enum BobinatorError {
    #[error("Cannot create local configuration folder at {0}: {1}")]
    ConfigPathError(String, io::Error),

    #[error("Cannot read token file from {0}: {1}")]
    TokenReadError(String, io::Error),

    #[error("Cannot save token {2} to {0}: {1}")]
    TokenSaveError(String, io::Error, String),

    #[error("No token provided.")]
    TokenNotProvided,

    #[error("Login attempt aborted.")]
    LoginAborted,

    #[error("Cannot build Client because reqwest reported: {0}")]
    ClientBuildError(reqwest::Error),

    #[error("Client could not connect to remote host: {0}")]
    ClientConnectionError(reqwest::Error),

    #[error("Client could not retrieve the response text: {0}")]
    ClientContentError(reqwest::Error),

    /// This is only meant to be used with native [`reqwest::Response::json()`] call.
    #[error("Client could not deserialise returned JSON: {0}")]
    ClientJSONDecodeError(reqwest::Error),

    #[error("Data deserialization failed: {0}")]
    DataJSONDecodeError(String),

    #[error("Bob refused your login; check your email and password combination.")]
    BobUnauthorised,

    #[error("BobJSONDeserialise could not deserialise returned JSON: {0};\n\nError occured at path: {1}.\n\nReturned data: `{2}`")]
    BobJSONDecodeError(String, String, String),

    #[error("Bob returned an Unexpected Error `{0}`: {1}.")]
    BobReturnedUnexpectedError(String, String),

    #[error("Bob returned an validation type that we don't expect: {0:?}")]
    BobReturnedUnexpectedValidationType(String),

    #[error("API Token do not have {0:?} permissions; access denied.")]
    TokenPermissionDenied(Vec<APITokenScope>),

    #[error("Bob returned an error code that we don't expect: {0:?}")]
    ServerReturnedUnexpectedStatus(StatusCode),

    #[error("{0} is not a {1}")]
    IncorrectWeekday(NaiveDate, String),

    #[error("Cannot request Friday off on {0} as it is not a Friday.")]
    FridayOffOnNonFriday(NaiveDate),

    #[error("Failed to join handle after asynchronous operation: {0:?}")]
    AsyncJoinError(JoinError),
}
