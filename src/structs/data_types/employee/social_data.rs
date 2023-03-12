use serde::{Deserialize, Serialize};

/// Communication Object for Personal struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SocialData {
    linkedin: Option<String>,
    twitter: Option<String>,
    facebook: Option<String>,
}
