//! Module containing all the structs and eunms relating to the
//! [`Employee`] struct.
//!
//! Typically you do not need to use this module; [`Employee`] is all
//! you will need.
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::HasEmployeeId;

mod about;
pub use about::*;

mod communication;
pub use communication::*;

mod personal;
pub use personal::*;

mod proxy;
pub use proxy::*;

mod social_data;
pub use social_data::*;

mod work;
pub use work::*;

/// Metadata about an Employee; it could be the session owner or
/// colleagues.
///
/// Sample Data:
/// ```text
/// {
///     "id": "0000000000000000000"
///     "firstName": "John",
///     "surname": "Doe",
///     "email": "jdoe@mycompany.com",
///     "displayName": "John Doe",
///     "companyId": 123456,
///     "fullName": "John Doe",
///     "personal": {
///         "shortBirthDate": "01-01",
///         "pronouns": null,
///         "communication": {
///             "skypeUsername": null,
///             "slackUsername": null
///         },
///         "honorific": null,
///         "nationality": [
///             "British"
///         ]
///     },
///     "creationDateTime": "2022-05-04T07:16:55.799734",
///     "work": {
///         "shortStartDate": "02-01",
///         "startDate": "2022-02-01",
///         "manager": "0000000000000000000",
///         "workPhone": null,
///         "tenureDuration": {
///             "periodISO": "P1Y1M11D",
///             "sortFactor": 396,
///             "humanize": "1 years, 1 month and 11 days"
///         },
///         "durationOfEmployment": {
///             "periodISO": "P1Y1M11D",
///             "sortFactor": 396,
///             "humanize": "1 years, 1 month and 11 days"
///         },
///         "reportsToIdInCompany": null,
///         "employeeIdInCompany": 8,
///         "reportsTo": {
///             "displayName": "Jane Doe",
///             "email": "jadoe@mycompany.com",
///             "surname": "Doe",
///             "firstName": "Jane",
///             "id": "0000000000000000000"
///         },
///         "workMobile": "01234567890",
///         "indirectReports": null,
///         "department": "Engineering",
///         "siteId": 1234567,
///         "tenureDurationYears": 2.103,
///         "tenureYears": 2,
///         "isManager": false,
///         "title": "123456789",
///         "site": "London",
///         "activeEffectiveDate": "2023-01-04",
///         "directReports": null,
///         "secondLevelManager": "0000000000000000000",
///         "daysOfPreviousService": 0,
///         "yearsOfService": 2.103
///     },
///     "avatarUrl": null,
///     "secondName": null,
///     "about": {
///         "foodPreferences": [],
///         "socialData": {
///             "linkedin": null,
///             "twitter": null,
///             "facebook": null
///         },
///         "superpowers": [],
///         "hobbies": [],
///         "about": null,
///         "avatar": "https://images.hibob.com/default-avatars/JD_01.png"
///     },
/// }
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,

    #[serde(rename = "firstName")]
    pub first_name: String,
    pub surname: String,

    pub email: String,

    #[serde(rename = "displayName")]
    pub display_name: String,

    #[serde(rename = "companyId")]
    pub company_id: i64,

    #[serde(rename = "fullName")]
    pub full_name: Option<String>,

    pub personal: Personal,

    #[serde(rename = "creationDateTime")]
    pub creation_date_time: NaiveDateTime,

    pub work: Work,
}

impl HasEmployeeId for Employee {
    /// Return a reference to the employee ID.
    fn employee_id<'a>(&'a self) -> &'a str {
        self.id.as_str()
    }
}

impl From<&Employee> for EmployeeProxy {
    /// Get an [`EmployeeProxy`] instance out of a [`Employee`].
    fn from(value: &Employee) -> Self {
        Self {
            id: value.id.clone(),
            first_name: Some(value.first_name.clone()),
            surname: Some(value.surname.clone()),
            email: Some(value.email.clone()),
            display_name: Some(value.display_name.clone()),
        }
    }
}
impl From<Employee> for EmployeeProxy {
    /// Consume an [`Employee`] and return a [`EmployeeProxy`] instance.
    fn from(value: Employee) -> Self {
        Self {
            id: value.id,
            first_name: Some(value.first_name),
            surname: Some(value.surname),
            email: Some(value.email),
            display_name: Some(value.display_name),
        }
    }
}
