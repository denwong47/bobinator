use reqwest::Client;

use crate::*;

use bobinator_macros::map_post_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

map_post_to_struct! (
    calculate,
    "Enquire the submittability of a [`Timeoff`] request for the current logged in employee by ID.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/employees/{employee_id}/timeoff/requests/calculateTimeOff?ignoreRequiredFields=false",
    (employee_id: String),
    TimeoffRequest,
    bob_json() -> Timeoff
);
