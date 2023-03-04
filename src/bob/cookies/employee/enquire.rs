use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

use crate::LoginSession;

/// Placeholder struct to receive response from Employee endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeSummaryResponse {
    pub metadata: serde_json::Value,
    pub data: LoginSession,
}

map_get_to_struct! (
    enquire_by_id,
    "Enquire the details of an [`Employee`] for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/employees/{employee_id}",
    (employee_id: String),
    bob_json() -> LoginSession
);
