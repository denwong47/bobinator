use std::fmt;
use std::ops::RangeInclusive;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use conch::{ContainsDate, StringWrapper};

use crate::{consts, ApprovalState, DatePortion, HasDate, HasDateRange, HasEmployeeId};

#[allow(unused_imports)]
use super::{DateRange, RequestRangeType, TimeoffPolicyType, TimeoffRequest};

// For docs only
#[allow(unused_imports)]
use crate::CanEnquireEmployee;

/// A struct representing a single timeoff received from bob.
///
/// This struct is used for your own timeoffs as well as the reduced fields when
/// querying other people's timeoffs, hence the number of optional fields.
///
/// Sample return from `api/timeoff/employees/{employee_id}/requests/{request_id}`:
///
/// ```text
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeoff {
    #[serde(alias = "requestId")]
    pub id: i64,
    // #[serde(rename = "latestState")]
    // pub latest_state: dict,
    #[serde(rename = "approvedBy")]
    pub approved_by: Option<String>,

    #[serde(rename = "canEdit")]
    pub can_edit: Option<bool>,

    #[serde(rename = "dateRange")]
    pub date_range: DateRange,

    pub description: Option<String>,
    pub documents: Option<Vec<serde_json::Value>>,
    pub duration: Option<i64>,

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
    pub request_range_type: Option<RequestRangeType>,

    #[serde(rename = "requestedBy")]
    pub requested_by: Option<String>,

    // #[serde(rename = "requestedPeriod")]
    // pub requested_period: String,
    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,

    #[serde(rename = "startDatePortion")]
    pub start_date_portion: DatePortion,
    pub states: Option<String>,
    pub status: Option<ApprovalState>,
    pub unit: Option<String>,

    #[serde(default)]
    actions: Vec<String>,
}

impl HasDate for Timeoff {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        &self.start_date
    }
}

impl HasDateRange for Timeoff {
    /// Return the date range it represents as a [`RangeInclusive<NaiveDate>`].
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        RangeInclusive::new(self.start_date, self.end_date)
    }
}

impl HasEmployeeId for Timeoff {
    /// Return the Employee ID of the [`Timeoff`].
    ///
    /// This `impl` enables auto trait implementation of
    /// [`CanEnquireEmployee`], which allows the use of
    /// [`CanEnquireEmployee::enquire_employee()`] on [`Timeoff`].
    fn employee_id<'a>(&'a self) -> &'a str {
        &self.employee_id
    }
}

impl ContainsDate for &Timeoff {
    /// Check if a date is within the time off.
    fn contains(&self, date: &NaiveDate) -> bool {
        self.date_range().contains(date)
    }
}

impl ContainsDate for Timeoff {
    /// Check if a date is within the time off.
    fn contains(&self, date: &NaiveDate) -> bool {
        self.date_range().contains(date)
    }
}

impl fmt::Display for Timeoff {
    // Generate a time off description like:
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range_description: String = if self.start_date == self.end_date
            && self.start_date_portion == DatePortion::AllDay
            && self.end_date_portion == DatePortion::AllDay
        {
            // One day, full day.
            self.start_date.format("%d/%m").to_string()
        } else {
            let mut s = self.start_date.format("%d/%m").to_string();

            // Add AM/PM if needed.
            if self.start_date_portion != DatePortion::AllDay {
                s.push(' ');
                s.push_str(&self.start_date_portion.to_string());
            }

            s.push('-');

            // If the end date is different, print it.
            if self.start_date != self.end_date {
                s.push_str(&self.end_date.format("%d/%m").to_string());
            }

            // Add AM/PM if needed.
            if self.end_date_portion != DatePortion::AllDay {
                if self.start_date != self.end_date {
                    s.push(' ')
                }

                s.push_str(&self.end_date_portion.to_string());
            }

            s
        };

        write!(
            f,
            "{} {} {}",
            self.policy_type_display_name.modifier().wraps(" "),
            consts::MODIFIER_EMPHASIS.wraps(&range_description),
            self.policy_type_display_name.short_name()
        )
    }
}
