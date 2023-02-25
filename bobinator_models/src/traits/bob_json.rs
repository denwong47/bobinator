use async_trait::async_trait;

use reqwest::Response;
use serde::de::DeserializeOwned;

#[allow(unused_imports)]
use serde_json;

use crate::structs::{BobinatorError, UnexpectedError};

/// A trait to be implemented onto [`Response`], to allow for
#[async_trait]
pub trait BobJSONDeserialise {
    async fn bob_json<T>(self) -> Result<T, BobinatorError>
    where
        T: DeserializeOwned;
}

#[async_trait]
impl BobJSONDeserialise for Response {
    /// Deserialise a [`Response`] into a struct; if that fails, try deserialising it
    /// into a [`UnexpectedError`], then wrap it with
    /// [`BobinatorError::BobReturnedUnexpectedError`]; and should that fail,
    /// return a [`BobinatorError::BobJSONDecodeError`] embedding the full JSON text.
    ///
    /// This allow error messages to be a lot more readable, rather than which character
    /// of a (now dropped) text file is violating JSON syntax.
    ///
    /// This method pre-loads the whole JSON text into memory before parsing; which can
    /// be marginally quicker than [`serde_json::from_reader`], but at a cost of
    /// memory efficiency.
    async fn bob_json<T>(self) -> Result<T, BobinatorError>
    where
        T: DeserializeOwned,
    {
        let content = self
            .text()
            .await
            .map_err(|err| BobinatorError::ClientContentError(err))?;

        let result: Result<T, BobinatorError> = serde_json::from_str::<T>(&content).or({
            let err: BobinatorError = serde_json::from_str(&content)
                .map(|uerr: UnexpectedError| {
                    BobinatorError::BobReturnedUnexpectedError(uerr.key, uerr.error)
                })
                .unwrap_or_else(|de_err| {
                    BobinatorError::BobJSONDecodeError(de_err.to_string(), content.clone())
                });

            Err(err)
        });

        result
    }
}
