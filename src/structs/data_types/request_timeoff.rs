use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{DatePortion, HasDate, RequestRangeType, TimeoffPolicyType};

/// Construct a JSON for requesting time off.
///
/// ```ignore
/// {'policyType': 'Friday Off',
///  'startDate': '2023-03-17',
///  'endDate': '2023-03-17',
///  'startDatePortion': 'all_day',
///  'endDatePortion': 'all_day',
///  'requestRangeType': 'days',
///  'hours': 1,
///  'reasonCode': None}
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeoffRequest {
    #[serde(rename = "policyType")]
    pub policy_type: TimeoffPolicyType,

    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,

    #[serde(rename = "endDate")]
    pub end_date: NaiveDate,

    #[serde(rename = "startDatePortion")]
    pub start_date_portion: DatePortion,

    #[serde(rename = "endDatePortion")]
    pub end_date_portion: DatePortion,

    #[serde(rename = "requestRangeType")]
    pub request_range_type: RequestRangeType,

    /// This field is optional if `request_range_type` is set to `days`;
    /// but is set as mandatory because that's how their web UI does it.
    #[serde(rename = "hours")]
    pub hours: usize,

    #[serde(rename = "reasonCode")]
    pub reason_code: Option<i64>,
}

impl HasDate for TimeoffRequest {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        &self.start_date
    }
}
