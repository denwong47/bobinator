use chrono::offset::Local;
use chrono::{Months, NaiveDate};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
struct TimeoffResponse {
    pub requests: Vec<Timeoff>,
}

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

map_get_to_struct! (
    _list_requests,
    "List all [`Timeoff`] of an employee by id.\nMust be used with an cookies.",
    "https://app.hibob.com/api/timeoff/employees/{employee_id}/requests/inRange?from={from}&to={to}",
    // TODO Restructure macro to allow &str?
    (employee_id: String),
    (from: NaiveDate),
    (to: NaiveDate),
    bob_json() -> TimeoffResponse
);

/// List all [`Timeoff`] of an employee.
/// Must be used with an cookies.
pub async fn list_requests(
    conn: &Client,
    employee: &Employee,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Result<Vec<Timeoff>, BobinatorError> {
    let from = from.unwrap_or(Local::now().date_naive());
    let to = to.unwrap_or(from + Months::new(36));

    let response = _list_requests(conn, employee.id.clone(), from, to).await?;

    Ok(response.requests)
}
