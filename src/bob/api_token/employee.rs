use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use serde_json;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

use crate::LoginSession;

map_get_to_struct! (
    get_employee_by_id,
    "Find an [`Employee`] by id.\nMust be used with an API Token.",
    "https://api.hibob.com/v1/people/identifier/{employee_id}",
    (employee_id: String),
    bob_json() -> LoginSession
);
