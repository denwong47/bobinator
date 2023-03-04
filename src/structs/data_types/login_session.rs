#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

use conch;
use conch::StringWrapper;

use crate::*;
/// Parse data returned from POST /api/login endpoint.
///
/// ```ignore
/// {'id': '9999999999999999999',
///  'firstName': 'Dave',
///  'surname': 'Big',
///  'lastName': 'Big',
///  'email': 'dave@big.com',
///  'site': 'London',
///  'siteId': 1234567,
///  'avatar': 'https://images.hibob.com/default-avatars/BD_19.png',
///  'role': 2,
///  'companyId': 7654321,
///  'companyName': 'Dave Unlimited',
///  'isManager': False,
///  'reportees': [],
///  'allReportees': [],
///  'displayName': 'Big Dave',
///  'mockedRole': False,
///  'sessionType': 'employee',
///  'state': 'active',
///  'isLoggedInIntoSandbox': False}
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginSession {
    pub id: String,

    #[serde(rename = "firstName")]
    pub first_name: String,
    pub surname: String,

    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: String,
    pub site: String,

    #[serde(rename = "siteId")]
    pub site_id: i64,
    pub avatar: String,
    pub role: i64, // Convert to Enum if variants known

    #[serde(rename = "companyId")]
    pub company_id: i64,

    #[serde(rename = "companyName")]
    pub company_name: String,
    // pub is_manager: bool,
    // pub reportees: Vec<i64>,
    // pub all_reportees: Vec<i64>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    // pub mocked_role: bool,
    #[serde(rename = "sessionType")]
    pub session_type: String, // Convert to Enum if variants known
    pub state: String, // Convert to Enum if variants known
    // pub is_logged_in_into_sandbox: bool,

    // Private fields.
    /// The timeoff requests for this employee.
    #[serde(skip)]
    pub timeoff_requests: Vec<Timeoff>,
}

impl LoginSession {
    // Temporary function to test functionality
    pub fn greet(&self) {
        println!(
            "{} of {}, {} is now logged into {} and online.",
            (conch::Modifier::colour("BrightWhite").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&self.display_name),
            (conch::Modifier::colour("BrightWhite").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&self.company_name),
            (conch::Modifier::intensity("Bold").unwrap()).wraps(&self.site),
            consts::BOBINATOR_NAME.as_str(),
        );
    }
}

/// Placeholder for taking a return value from the employee end point.
///
/// ```ignore
/// {
///     "/personal/pronouns": {
///         "value": "He / Him"
///     },
///     "fullName": "John Doe",
///     "/about/avatar": {
///         "value": "https://images.hibob.com/default-avatars/BD_24.png"
///     },
///     "displayName": "Ben Doe",
///     "/work/reportsTo/email": {
///         "value": "john@doe.com"
///     },
///     "/root/firstName": {
///         "value": "John"
///     },
///     "/work/employeeIdInCompany": {
///         "value": 10
///     },
///     "/work/tenureDurationYears": {
///         "value": 1.826
///     },
///     "/about/foodPreferences": {
///         "value": []
///     },
///     "/work/site": {
///         "value": "London"
///     },
///     "personal": {
///         "shortBirthDate": "01-02",
///         "pronouns": "He / Him",
///         "communication": {
///             "skypeUsername": null,
///             "slackUsername": null
///         },
///         "honorific": null,
///         "nationality": [
///             "British"
///         ]
///     },
///     "/about/hobbies": {
///         "value": []
///     },
///     "/work/tenureYears": {
///         "value": 2
///     },
///     "creationDateTime": "2022-05-04T07:16:55.799624",
///     "/root/email": {
///         "value": "john@doe.com"
///     },
///     "/work/title": {
///         "value": "255490048"
///     },
///     "work": {
///         "shortStartDate": "05-04",
///         "startDate": "2021-05-04",
///         "manager": "0000000000000000000",
///         "workPhone": null,
///         "tenureDuration": {
///             "periodISO": "P1Y9M27D",
///             "sortFactor": 657,
///             "humanize": "1 year, 9 months and 27 days"
///         },
///         "durationOfEmployment": {
///             "periodISO": "P1Y9M27D",
///             "sortFactor": 657,
///             "humanize": "1 year, 9 months and 27 days"
///         },
///         "reportsToIdInCompany": 1,
///         "employeeIdInCompany": 10,
///         "reportsTo": {
///             "displayName": "Jane Doe",
///             "email": "john@doe.com",
///             "surname": "Doe",
///             "firstName": "Jane",
///             "id": "0000000000000000000"
///         },
///         "workMobile": "07502177269",
///         "indirectReports": null,
///         "department": "Engineering",
///         "siteId": 2154020,
///         "tenureDurationYears": 1.826,
///         "tenureYears": 2,
///         "isManager": false,
///         "title": "255490048",
///         "site": "London",
///         "activeEffectiveDate": "2023-02-07",
///         "directReports": null,
///         "secondLevelManager": null,
///         "daysOfPreviousService": 0,
///         "yearsOfService": 1.826
///     },
///     "/root/creationDateTime": {
///         "value": "2022-05-04T07:16:55.799624"
///     },
///     "avatarUrl": null,
///     "secondName": null,
///     "/work/startDate": {
///         "value": "2021-05-04"
///     },
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
///         "avatar": "https://images.hibob.com/default-avatars/JD_24.png"
///     },
///     "/root/fullName": {
///         "value": "John Doe"
///     },
///     "/work/workMobile": {
///         "value": "01234567890"
///     },
///     "companyId": 557418,
///     "/personal/nationality": {
///         "value": [
///             "British"
///         ]
///     },
///     "/work/yearsOfService": {
///         "value": 1.826
///     },
///     "/work/daysOfPreviousService": {
///         "value": 0
///     },
///     "/personal/shortBirthDate": {
///         "value": "10-17"
///     },
///     "/root/surname": {
///         "value": "Doe"
///     },
///     "/work/shortStartDate": {
///         "value": "05-04"
///     },
///     "/root/id": {
///         "value": "0000000000000000000"
///     },
///     "/work/activeEffectiveDate": {
///         "value": "2023-02-07"
///     },
///     "email": "john@doe.com",
///     "/work/manager": {
///         "value": "0000000000000000000"
///     },
///     "surname": "Doe",
///     "/work/reportsTo": {
///         "value": {
///             "displayName": "Jane Doe",
///             "email": "jane@doe.com",
///             "surname": "Doe",
///             "firstName": "Jane",
///             "id": "0000000000000000000"
///         }
///     },
///     "/root/displayName": {
///         "value": "Ben Doe"
///     },
///     "/root/companyId": {
///         "value": 557418
///     },
///     "/work/department": {
///         "value": "Human Resource"
///     },
///     "/work/isManager": {
///         "value": false
///     },
///     "/about/superpowers": {
///         "value": []
///     },
///     "/work/tenureDuration": {
///         "value": {
///             "periodISO": "P1Y9M27D",
///             "sortFactor": 657,
///             "humanize": "1 year, 9 months and 27 days"
///         }
///     },
///     "/work/reportsToIdInCompany": {
///         "value": 1
///     },
///     "firstName": "John",
///     "id": "0000000000000000000",
///     "/work/siteId": {
///         "value": 2154020
///     },
///     "/work/durationOfEmployment": {
///         "value": {
///             "periodISO": "P1Y9M27D",
///             "sortFactor": 657,
///             "humanize": "1 year, 9 months and 27 days"
///         }
///     }
/// }
/// ```
#[allow(dead_code)]
struct ReturnedFromEmployeeEndpoint {}
