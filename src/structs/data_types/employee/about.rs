use serde::{Deserialize, Serialize};

use super::SocialData;

/// About Object for Employee struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct About {
    avatar: String,
    hobbies: Vec<String>,

    #[serde(default)]
    #[serde(rename = "foodPreferences")]
    food_preferences: Vec<String>,

    #[serde(default)]
    superpowers: Vec<String>,

    about: Option<String>,

    #[serde(rename = "socialData")]
    social_data: SocialData,
}
