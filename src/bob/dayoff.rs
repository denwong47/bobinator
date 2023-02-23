use crate::*;
use chrono::NaiveDate;
use reqwest::{Client, StatusCode};

/// Attempt to send a login request using the credentials provided.
pub async fn query(
    conn: &Client,
    employee: Employee,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<(), BobinatorError> {
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
            let text = req
                .text()
                .await
                .map_err(|err| BobinatorError::ClientJSONDecodeError(err))?;

            println!("{:?} {}", code, text);
            Ok(())
        }
        code => Err(BobinatorError::ServerReturnedUnexpectedStatus(code)),
    }
}
