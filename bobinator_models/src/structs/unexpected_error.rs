use serde::{Deserialize, Serialize};

/// The default unexpected error format returned by Bob.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnexpectedError {
    pub key: String,
    pub error: String,
}
