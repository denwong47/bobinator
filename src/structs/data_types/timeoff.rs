use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{ApprovalState, DatePortion, HasDate};

#[allow(unused_imports)]
use super::{RequestRangeType, TimeoffPolicyType, TimeoffRequest};

/// A struct representing a single timeoff received from bob.
///
/// Sample return from `api/timeoff/employees/{employee_id}/requests/{request_id}`:
///
/// ```ignore
/// {
///     "actions": [
///         "editRequest",
///         "cancelRequest"
///     ],
///     "approvedBy": null,
///     "canEdit": true,
///     "dateRange": {
///         "startDate": "2023-11-24",
///         "startPortion": "all_day",
///         "endDate": "2023-11-24",
///         "endPortion": "all_day",
///         "type": "days"
///     },
///     "description": null,
///     "documents": [],
///     "duration": 1,
///     "employeeId": "0000000000000000000",
///     "endDate": "2023-11-24",
///     "endDatePortion": "all_day",
///     "events": [
///         {
///             "id": 98765432,
///             "companyId": 999999,
///             "requestId": 12345678,
///             "eventType": "submitted",
///             "eventData": {
///                 "request": {
///                     "id": 12345678,
///                     "companyId": 999999,
///                     "employeeId": "0000000000000000000",
///                     "dateRange": {
///                         "startDate": "2023-11-24",
///                         "startPortion": "all_day",
///                         "endDate": "2023-11-24",
///                         "endPortion": "all_day",
///                         "type": "days"
///                     },
///                     "policyTypeReservedName": "type2",
///                     "visibility": null,
///                     "status": "approved",
///                     "userData": {
///                         "attachments": []
///                     },
///                     "sentTo": null
///                 },
///                 "policyApprovalRequired": false,
///                 "submitterApproved": false,
///                 "approvalRequired": false,
///                 "approverSchemeSource": "policy"
///             },
///             "origin": "web",
///             "createdBy": "0000000000000000000",
///             "creationTime": "2023-02-28T00:01:38.695715"
///         }
///     ],
///     "id": 13015100,
///     "latestState": {
///         "tier": null,
///         "status": "approved",
///         "submittedAt": "2023-02-28T00:01:38.695715"
///     },
///     "policyType": "type2",
///     "policyTypeDisplayName": "Friday Off",
///     "policyTypeEmoji": "\u26d4",
///     "policyTypeOrder": 5,
///     "reason": null,
///     "reasonCodeDisplayName": null,
///     "reasonCodeId": null,
///     "requestRangeType": "days",
///     "requestedBy": "0000000000000000000",
///     "startDate": "2023-11-24",
///     "startDatePortion": "all_day",
///     "states": null,
///     "status": "approved",
///     "unit": "days"
/// }
/// ```
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Timeoff {
    pub id: i64,
    // #[serde(rename = "latestState")]
    // pub latest_state: dict,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<String>,

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
    pub end_date: NaiveDate,

    #[serde(rename = "endDatePortion")]
    pub end_date_portion: DatePortion,
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
    pub request_range_type: RequestRangeType,

    #[serde(rename = "requestedBy")]
    pub requested_by: String,

    // #[serde(rename = "requestedPeriod")]
    // pub requested_period: String,
    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,

    #[serde(rename = "startDatePortion")]
    pub start_date_portion: DatePortion,
    pub states: Option<String>,
    pub status: ApprovalState,
    pub unit: String,

    #[serde(default)]
    actions: Vec<String>,
}

impl HasDate for Timeoff {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        &self.start_date
    }
}
