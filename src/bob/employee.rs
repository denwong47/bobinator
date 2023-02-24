use reqwest::Client;

use crate::Employee;
use bobinator_macros::map_get_to_struct;

map_get_to_struct! (
    get_employee_by_id,
    "Find an [`Employee`] by id.\nMust be used with an API Token.",
    "https://api.hibob.com/v1/people/identifier/{employee_id}",
    (employee_id: String),
    json() -> Employee
);
