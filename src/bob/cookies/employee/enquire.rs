use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;

use bobinator_macros::map_get_to_struct;
use bobinator_models::{structs::BobinatorError, traits::BobJSONDeserialise};

use crate::{Employee, HasEmployeeId};

// /// Only used for Employee Summary endpoint, not used.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct EmployeeSummaryResponse {
//     pub metadata: serde_json::Value,
//     pub data: Employee,
// }

map_get_to_struct! (
    _enquire_by_id,
    "Enquire the details of an [`Employee`] for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/employees/{employee_id}",
    (employee_id: String),
    bob_json() -> Employee
);

pub async fn enquire(
    conn: &Client,
    employee: &dyn HasEmployeeId,
) -> Result<Employee, BobinatorError> {
    let result = _enquire_by_id(conn, employee.employee_id().to_owned());

    result.await
}
