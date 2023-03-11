use serde::{Deserialize, Serialize};

use crate::*;

/// An API Token, together with scope information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APIToken {
    pub token: String,
    pub name: String,
    pub scopes: Vec<APITokenScope>,
    #[serde(rename = "employeeId")]
    pub employee_id: String,
    pub id: i64,
}
impl HasToken for APIToken {
    fn key(&self) -> String {
        self.token.clone()
    }
}
impl APIToken {
    /// Add any non-existing scope members to itself.
    pub fn extend_scopes(&mut self, scopes: Vec<APITokenScope>) {
        // self.scopes.push needs &mut ref; we have to have self.scopes.contains
        // completely done and dusted before, all &self.scopes discarded before we
        // can self.scopes.push().
        let duplicated: Vec<bool> = scopes
            .iter()
            .map(|scope| self.scopes.contains(scope))
            .collect();

        scopes
            .into_iter()
            .zip(duplicated)
            .filter(|(_, is_duplicate)| !is_duplicate)
            .for_each(|(scope, _)| self.scopes.push(scope))
    }

    /// Drop from its own scopes any members as requested.
    pub fn drop_scopes(&mut self, scopes: Vec<APITokenScope>) {
        self.scopes.retain(|scope| !scopes.contains(&scope));
    }
}

/// An bare API Token with no scopes attached.
///
/// This is typically used when storing the token locally; scope
/// information is not necessarily stored.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct APITokenOnly {
    pub token: String,
}
impl From<APIToken> for APITokenOnly {
    /// Reduce this instance into a [`APITokenOnly`] before serialization.
    fn from(value: APIToken) -> Self {
        Self { token: value.token }
    }
}
impl HasToken for APITokenOnly {
    fn key(&self) -> String {
        self.token.clone()
    }
}
impl APITokenOnly {
    /// Promote this instance into a APIToken.
    pub fn promote(
        self,
        name: String,
        scopes: Vec<APITokenScope>,
        employee_id: String,
        id: i64,
    ) -> APIToken {
        APIToken {
            token: self.key(),
            name,
            scopes,
            employee_id,
            id,
        }
    }
}
