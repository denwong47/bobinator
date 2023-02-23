use std::collections::hash_map::HashMap;

use serde::de::Error;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;

use conch;
use conch::StringWrapper;

use bobinator_macros::{deserialize_num_field, deserialize_str_field};

use crate::*;
/// Parse data returned from POST /api/login endpoint.
///
/// ```js
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
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub surname: String,
    pub last_name: String,
    pub email: String,
    pub site: String,
    pub site_id: i64,
    pub avatar: String,
    pub role: i64, // Convert to Enum if variants known
    pub company_id: i64,
    pub company_name: String,
    // pub is_manager: bool,
    // pub reportees: Vec<i64>,
    // pub all_reportees: Vec<i64>,
    pub display_name: String,
    // pub mocked_role: bool,
    pub session_type: String, // Convert to Enum if variants known
    pub state: String,        // Convert to Enum if variants known
                              // pub is_logged_in_into_sandbox: bool,
}
impl<'de> Deserialize<'de> for Employee {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mapping: HashMap<&str, serde_json::Value> = HashMap::deserialize(deserializer)?;

        Ok(Self {
            id: deserialize_str_field!(mapping, "id")?,
            first_name: deserialize_str_field!(mapping, "firstName")?,
            surname: deserialize_str_field!(mapping, "surname")?,
            last_name: deserialize_str_field!(mapping, "lastName")?,
            email: deserialize_str_field!(mapping, "email")?,
            site: deserialize_str_field!(mapping, "site")?,
            site_id: deserialize_num_field!(mapping, "siteId", as_i64)?,
            avatar: deserialize_str_field!(mapping, "avatar")?,
            role: deserialize_num_field!(mapping, "role", as_i64)?,
            company_id: deserialize_num_field!(mapping, "companyId", as_i64)?,
            company_name: deserialize_str_field!(mapping, "companyName")?,
            display_name: deserialize_str_field!(mapping, "displayName")?,
            session_type: deserialize_str_field!(mapping, "sessionType")?,
            state: deserialize_str_field!(mapping, "state")?,
        })
    }
}

impl Employee {
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
