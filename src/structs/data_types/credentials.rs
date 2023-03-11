use serde::{Deserialize, Serialize};

/// A set of login credentials to send to the server.
#[derive(Serialize, Deserialize)]
pub struct Credentials {
    // Private fields
    email: String,
    password: String,
}
impl Credentials {
    /// Create a new set of [`Credentials`].
    ///
    /// Deliberately takes ownership of the two variables; this is to remove ownership
    /// from the caller, leaving password to as inner a scope as possible.
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}
