use super::DatePortion;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Construct a JSON for requesting time off.
///
/// ```js
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
    pub policy_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub start_date_portion: DatePortion,
    pub end_date_portion: DatePortion,
    pub request_range_type: String,
    pub hours: usize,
    pub reason_code: Option<i64>,
}
