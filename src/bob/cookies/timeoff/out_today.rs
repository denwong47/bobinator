use reqwest::Client;

use serde::{Deserialize, Serialize};
use serde_json;

use chrono::offset::Local;
use chrono::NaiveDate;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbsenteeResponse {
    requests: Vec<Timeoff>,
    // We don't actually know what type this is, so lets leave it as a JSON Value.
    events: Vec<serde_json::Value>,
}
impl AbsenteeResponse {
    /// Iterate over the timeoff requests relating to today.
    pub fn iter_requests(&self) -> impl Iterator<Item = &Timeoff> {
        self.requests.iter()
    }

    // Iterate over the events relating to today.
    pub fn iter_events(&self) -> impl Iterator<Item = &serde_json::Value> {
        self.events.iter()
    }
}

map_get_to_struct! (
    absentees_on,
    "Enquire the [`Timeoff`] requests for other employees that are off on the given day.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/outToday?today={date}&minimumHours=1&includeNonWorkingEvents=true",
    (date: NaiveDate),
    bob_json() -> AbsenteeResponse
);

/// Enquire the [`Timeoff`] requests for other employees that are off today.
/// Must be used with cookies session.
pub async fn absentees_today(conn: &Client) -> Result<AbsenteeResponse, BobinatorError> {
    absentees_on(conn, Local::now().date_naive()).await
}
