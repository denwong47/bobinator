//! API Calls related to the API token.
//!
//!
use reqwest::Client;
use serde::{Deserialize, Serialize};

use bobinator_macros::{map_get_to_struct, map_put_to_struct};
use bobinator_models::traits::BobJSONDeserialise;

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeRequest {
    scopes: Vec<APITokenScope>,
}
impl From<APIToken> for ScopeRequest {
    fn from(value: APIToken) -> Self {
        Self {
            scopes: value.scopes,
        }
    }
}
impl From<Vec<APITokenScope>> for ScopeRequest {
    fn from(value: Vec<APITokenScope>) -> Self {
        Self { scopes: value }
    }
}

map_put_to_struct!(
    _change_token_scope,
    "Internal function to request a change to the scope of API Token. Must be used in Clients with active cookie stored.",
    "https://app.hibob.com/api/user/tokens/user-token/scopes",
    ScopeRequest,
);

/// Request a change to the scope of your API Token.
/// Must be used in Clients with active cookie stored.
pub async fn change_token_scope(
    conn: &Client,
    scopes: Vec<APITokenScope>,
) -> Result<(), BobinatorError> {
    _change_token_scope(conn, scopes.into()).await
}

map_get_to_struct!(
    get_token_scope,
    "Read the current API Token. Must be used in Clients with active cookie stored.",
    "https://app.hibob.com/api/user/tokens/user-token",
    bob_json() -> APIToken,
);

// =====================================================================================
// Bolt-on functionalities to existing structs.

impl APIToken {
    /// Synchronise scope with the server.
    pub async fn sync_scopes(&self, conn: &Client) -> Result<(), BobinatorError> {
        change_token_scope(conn, self.scopes.clone()).await
    }
}
