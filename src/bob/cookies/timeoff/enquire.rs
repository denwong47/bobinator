use reqwest::Client;

use crate::*;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

map_get_to_struct! (
    enquire_by_id,
    "Enquire the details of a [`Timeoff`] request for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/employees/{employee_id}/requests/{request_id}",
    (employee_id: String),
    (request_id: i64),
    bob_json() -> Timeoff
);
