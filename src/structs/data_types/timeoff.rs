#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

use crate::DatePortion;

#[allow(unused_imports)]
use super::{TimeoffPolicyType, TimeoffRequest};

/// A struct representing a single timeoff received from bob.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Timeoff {
    pub id: i64,
    // #[serde(rename = "latestState")]
    // pub latest_state: dict,
    // pub actions: list,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<u64>,

    #[serde(rename = "canEdit")]
    pub can_edit: bool,

    // #[serde(rename = "dateRange")]
    // pub date_range: dict,
    pub description: Option<String>,
    // pub documents: list,
    pub duration: i64,

    #[serde(rename = "durationDescription")]
    pub duration_description: Option<String>,

    #[serde(rename = "employeeId")]
    pub employee_id: String,

    #[serde(rename = "endDate")]
    pub end_date: String,

    #[serde(rename = "endDatePortion")]
    pub end_date_portion: String,
    // events: list,
    #[serde(rename = "policyType")]
    pub policy_type: String,

    #[serde(rename = "policyTypeDisplayName")]
    pub policy_type_display_name: TimeoffPolicyType,

    #[serde(rename = "policyTypeOrder")]
    pub policy_type_order: i64,
    pub reason: Option<String>,

    #[serde(rename = "reasonCodeDisplayName")]
    pub reason_code_display_name: Option<String>,

    #[serde(rename = "reasonCodeId")]
    pub reason_code_id: Option<String>,

    #[serde(rename = "requestRangeType")]
    pub request_range_type: String,

    #[serde(rename = "requestedBy")]
    pub requested_by: String,

    #[serde(rename = "requestedPeriod")]
    pub requested_period: String,

    #[serde(rename = "startDate")]
    pub start_date: String,

    #[serde(rename = "startDatePortion")]
    pub start_date_portion: DatePortion,
    pub states: Option<String>,
    pub status: String,
    pub unit: String,
}
