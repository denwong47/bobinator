use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::EmployeeProxy;

/// Work Object for Employee struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Work {
    #[serde(rename = "reportsTo")]
    pub reports_to: EmployeeProxy,

    pub title: String,
    pub department: String,
    pub site: String,

    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,

    #[serde(rename = "workPhone")]
    pub work_phone: Option<String>,

    #[serde(rename = "workMobile")]
    pub work_mobile: Option<String>,

    #[serde(rename = "siteId")]
    pub site_id: Option<i64>,

    #[serde(rename = "tenureDurationYears")]
    pub tenure_duration_years: Option<f64>,

    #[serde(rename = "tenureYears")]
    pub tenure_years: Option<u32>,

    #[serde(rename = "activeEffectiveDate")]
    pub active_effective_date: Option<NaiveDate>,

    #[serde(rename = "yearsOfService")]
    pub years_of_service: Option<f64>,
}
