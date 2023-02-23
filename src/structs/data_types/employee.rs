use std::collections::hash_map::HashMap;

use serde::de::Error;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;

use conch;
use conch::StringWrapper;

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

        // Deserialise Value::Number.
        macro_rules! deserialize_num_field {
            ($key:literal, $as_type:ident) => {{
                let value = mapping.get($key).ok_or(D::Error::custom(
                    BobinatorError::RecordFieldMissing($key.to_string()),
                ))?;

                if let serde_json::Value::Number(v) = value {
                    v.$as_type().ok_or(())
                } else {
                    Err(())
                }
                .map_err(|_| {
                    D::Error::custom(BobinatorError::RecordFieldInvalid(
                        $key.to_string(),
                        value.clone(),
                    ))
                })
            }};
        }

        // Deserialise Value::String.
        macro_rules! deserialize_str_field {
            ($key:literal) => {{
                let value = mapping.get($key).ok_or(D::Error::custom(
                    BobinatorError::RecordFieldMissing($key.to_string()),
                ))?;

                if let serde_json::Value::String(v) = value {
                    Ok(v.to_owned())
                } else {
                    Err(D::Error::custom(BobinatorError::RecordFieldInvalid(
                        $key.to_string(),
                        value.clone(),
                    )))
                }
            }};
        }

        Ok(Self {
            id: deserialize_str_field!("id")?,
            first_name: deserialize_str_field!("firstName")?,
            surname: deserialize_str_field!("surname")?,
            last_name: deserialize_str_field!("lastName")?,
            email: deserialize_str_field!("email")?,
            site: deserialize_str_field!("site")?,
            site_id: deserialize_num_field!("siteId", as_i64)?,
            avatar: deserialize_str_field!("avatar")?,
            role: deserialize_num_field!("role", as_i64)?,
            company_id: deserialize_num_field!("companyId", as_i64)?,
            company_name: deserialize_str_field!("companyName")?,
            display_name: deserialize_str_field!("displayName")?,
            session_type: deserialize_str_field!("sessionType")?,
            state: deserialize_str_field!("state")?,
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
