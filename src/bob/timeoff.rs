use crate::*;
use chrono::NaiveDate;
use reqwest::{Client, StatusCode};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TimeoffResponse {
    requests: Vec<Timeoff>,
}

/// Attempt to send a login request using the credentials provided.
pub async fn query(
    conn: &Client,
    employee: Employee,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<Vec<Timeoff>, BobinatorError> {
    let employee_id = employee.id;

    let req = conn
        .get(format!(
            "https://app.hibob.com/api/timeoff/employees/{}/requests/inRange?from={}&to={}",
            employee_id, from, to
        ))
        .send()
        .await
        .map_err(|err| BobinatorError::ClientConnectionError(err))?;

    let code = req.status();

    match code {
        StatusCode::UNAUTHORIZED => Err(BobinatorError::BobUnauthorised),
        StatusCode::OK => {
            let timeoffs: TimeoffResponse = req
                .json()
                .await
                .map_err(|err| BobinatorError::ClientJSONDecodeError(err))?;

            Ok(timeoffs.requests)
        }
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}
