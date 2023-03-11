use serde::{Deserialize, Serialize};

use super::Communication;

/// Personal Object for Employee struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Personal {
    pub honorific: Option<String>,

    #[serde(rename = "shortBirthDate")]
    pub short_birth_date: String,

    #[serde(default)]
    pub nationality: Vec<String>,

    pub communication: Communication,
    pub pronouns: Option<String>,
}
