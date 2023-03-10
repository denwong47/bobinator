#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

use conch;
use conch::StringWrapper;

use crate::*;
/// Parse data returned from POST /api/login endpoint.
///
/// ```text
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

impl HasEmployeeId for LoginSession {
    /// Return the Employee ID of the [`LoginSession`].
    ///
    /// This `impl` enables auto trait implementation of
    /// [`CanEnquireEmployee`], which allows the use of
    /// [`CanEnquireEmployee::enquire_employee()`] on [`LoginSession`].
    fn employee_id<'a>(&'a self) -> &'a str {
        &self.id
    }
}
