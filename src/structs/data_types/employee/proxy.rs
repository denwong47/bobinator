use crate::HasEmployeeId;
use serde::{Deserialize, Serialize};

/// Work Object for Employee struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmployeeProxy {
    pub id: String,

    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    pub surname: Option<String>,

    pub email: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
}

impl<T> From<T> for EmployeeProxy
where
    T: ToString,
{
    /// Allow creation of an [`EmployeeProxy`] from just an id.
    fn from(value: T) -> Self {
        Self {
            id: value.to_string(),
            first_name: None,
            surname: None,
            email: None,
            display_name: None,
        }
    }
}

impl HasEmployeeId for EmployeeProxy {
    /// Return a reference to the employee ID.
    fn employee_id<'a>(&'a self) -> &'a str {
        self.id.as_str()
    }
}
