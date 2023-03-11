use serde::{Deserialize, Serialize};
use std::fmt;

use crate::ValidationMessages;
use conch::Lines;

#[allow(unused_imports)]
use super::{RequestRangeType, TimeoffPolicyType, TimeoffRequest};

/// Response from `calculateTimeOff` endpoint.
///
/// Example Returns:
///
/// - Wrong Weekday:
///
///   ```ignore
///   {
///     "validationMessages": {
///       "level": "ERROR",
///       "messages": [
///         {
///           "reason": "You aren’t working on Sunday.",
///           "explanations": []
///         }
///       ]
///     },
///     "amount": 0,
///     "unit": "days",
///     "rejectReason": "You aren’t working on Sunday.",
///     "submittable": false,
///     "notAllowedFields": [],
///     "isHardUpdate": false,
///     "canAutoApprove": false,
///     "additionalRequiredFields": [],
///     "minTimeOffRequestDuration": "halfDay",
///     "isFirstLevelApproval": false
///   }
///   ```
///
/// - Conflicting requests:
///   ```ignore
///   {
///     "validationMessages": {
///       "level": "ERROR",
///       "messages": [
///         {
///           "reason": "Conflicts with request/s which cannot be overridden:",
///           "explanations": [
///             "Approved Friday Off request from 26/05/2023",
///             "Approved Friday Off request from 09/06/2023"
///           ]
///         }
///       ]
///     },
///     "amount": 11,
///     "unit": "days",
///     "rejectReason": "This request conflicts with an existing request: Approved Friday Off request from 26/05/2023",
///     "submittable": false,
///     "notAllowedFields": [],
///     "isHardUpdate": false,
///     "canAutoApprove": false,
///     "additionalRequiredFields": [],
///     "minTimeOffRequestDuration": "halfDay",
///     "isFirstLevelApproval": false
///   }
///   ```
///
/// - Submittable request:
///   ```ignore
///   {
///     "validationMessages": {
///       "level": "INFO",
///       "messages": [
///         {
///           "reason": "You are requesting 1 day",
///           "explanations": []
///         },
///         {
///           "reason": "The forecasted remaining balance will be 24.08 days",
///           "explanations": []
///         }
///       ]
///     },
///     "amount": 1,
///     "unit": "days",
///     "rejectReason": null,
///     "submittable": true,
///     "notAllowedFields": [],
///     "isHardUpdate": false,
///     "canAutoApprove": false,
///     "additionalRequiredFields": [],
///     "minTimeOffRequestDuration": "halfDay",
///     "isFirstLevelApproval": false
///   }
///   ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CalculateTimeoffResponse {
    #[serde(rename = "validationMessages")]
    validation_messages: ValidationMessages,

    amount: i32,
    unit: Option<String>,

    #[serde(rename = "rejectReason")]
    reject_reason: Option<String>,

    submittable: bool,

    #[serde(rename = "notAllowedFields")]
    not_allowed_fields: Vec<String>,

    #[serde(rename = "isHardUpdate")]
    is_hard_update: bool,

    #[serde(rename = "canAutoApprove")]
    can_auto_approve: bool,

    #[serde(rename = "additionalRequiredFields")]
    additional_required_fields: Vec<String>,

    #[serde(rename = "minTimeOffRequestDuration")]
    min_timeoff_request_duration: String,

    #[serde(rename = "isFirstLevelApproval")]
    is_first_level_approval: bool,
}

impl From<&CalculateTimeoffResponse> for Lines {
    fn from(value: &CalculateTimeoffResponse) -> Self {
        Self::from(&value.validation_messages)
    }
}
impl From<CalculateTimeoffResponse> for Lines {
    fn from(value: CalculateTimeoffResponse) -> Self {
        Self::from(&value)
    }
}

impl fmt::Display for CalculateTimeoffResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Lines::from(self))
    }
}
