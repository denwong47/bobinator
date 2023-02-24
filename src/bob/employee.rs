use reqwest::Client;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

use crate::Employee;

map_get_to_struct! (
    get_employee_by_id,
    "Find an [`Employee`] by id.\nMust be used with an API Token.",
    "https://api.hibob.com/v1/people/identifier/{employee_id}",
    (employee_id: String),
    bob_json() -> Employee
);
