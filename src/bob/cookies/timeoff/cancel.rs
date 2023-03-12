use reqwest::Client;

use crate::*;

use bobinator_macros::map_get_to_struct;

map_get_to_struct! (
    _cancel_by_id,
    "Cancel a [`Timeoff`] request for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/employees/{employee_id}/requests/{id}/cancels",
    (employee_id: String),
    (id: i64),
    text() -> String,
);

/// Cancel a [`Timeoff`] request for the current logged in employee.
/// Must be used with cookies session.
#[allow(unused_variables)]
pub async fn cancel(
    conn: &Client,
    session: &LoginSession,
    timeoff: &Timeoff,
) -> Result<(), BobinatorError> {
    let result = _cancel_by_id(conn, timeoff.employee_id.clone(), timeoff.id);

    result.await.and(Ok(()))
}
