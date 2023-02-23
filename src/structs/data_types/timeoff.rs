#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[allow(unused_imports)]
use super::{TimeoffPolicyType, TimeoffRequest};

/// A struct representing a single timeoff received from bob.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Timeoff {
    id: i64,
    // latest_state: dict,
    // actions: list,
    approved_by: Option<u64>,
    can_edit: bool,
    // date_range: dict,
    description: Option<String>,
    // documents: list,
    duration: i64,
    duration_description: Option<String>,
    employee_id: String,
    end_date: String,
    end_date_portion: String,
    // events: list,
    policy_type: String,
    policy_type_display_name: String,
    policy_type_order: i64,
    reason: Option<String>,
    reason_code_display_name: Option<String>,
    reason_code_id: Option<String>,
    request_range_type: String,
    requested_by: String,
    requested_period: String,
    start_date: String,
    start_date_portion: String,
    states: Option<String>,
    status: String,
    unit: String,
}
impl<'de> Deserialize<'de> for Timeoff {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mapping: HashMap<&str, serde_json::Value> = HashMap::deserialize(deserializer)?;

        // Ok(Self {
        // id: deserialize_str_field!(mapping, "id")?,
        // first_name: deserialize_str_field!(mapping, "firstName")?,
        // surname: deserialize_str_field!(mapping, "surname")?,
        // last_name: deserialize_str_field!(mapping, "lastName")?,
        // email: deserialize_str_field!(mapping, "email")?,
        // site: deserialize_str_field!(mapping, "site")?,
        // site_id: deserialize_num_field!(mapping, "siteId", as_i64)?,
        // avatar: deserialize_str_field!(mapping, "avatar")?,
        // role: deserialize_num_field!(mapping, "role", as_i64)?,
        // company_id: deserialize_num_field!(mapping, "companyId", as_i64)?,
        // company_name: deserialize_str_field!(mapping, "companyName")?,
        // display_name: deserialize_str_field!(mapping, "displayName")?,
        // session_type: deserialize_str_field!(mapping, "sessionType")?,
        // state: deserialize_str_field!(mapping, "state")?,
        // })
        todo!("Not yet implemented, got hashmap of {:?}", mapping)
    }
}
