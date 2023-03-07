use chrono::{Datelike, NaiveDate, Weekday};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestResponse {
    pub id: Option<i64>,
}

use bobinator_macros::map_post_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

map_post_to_struct! (
    _make_request_by_id,
    "Make a [`Timeoff`] request for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/employees/{employee_id}/timeoff/requests",
    (employee_id: String),
    TimeoffRequest,
    bob_json() -> RequestResponse
);

/// Make a [`Timeoff`] request for the current logged in employee.
/// Must be used with cookies session.
pub async fn make_request(
    conn: &Client,
    session: &LoginSession,
    request: TimeoffRequest,
) -> Result<Option<i64>, BobinatorError> {
    let result = _make_request_by_id(conn, session.id.clone(), request);

    result.await.map(|res| res.id)
}

/// Make a [`Timeoff`] request for a Friday off.
/// Must be used with cookies session.
pub async fn make_friday_off_request(
    conn: &Client,
    session: &LoginSession,
    date: NaiveDate,
) -> Result<Option<i64>, BobinatorError> {
    if date.weekday() != Weekday::Fri {
        return Err(BobinatorError::FridayOffOnNonFriday(date));
    }

    let request = TimeoffRequest {
        policy_type: TimeoffPolicyType::FridayOff,
        start_date: date,
        end_date: date,
        start_date_portion: DatePortion::AllDay,
        end_date_portion: DatePortion::AllDay,
        request_range_type: RequestRangeType::Days,
        hours: 1,
        reason_code: None,
    };

    make_request(conn, session, request).await
}
