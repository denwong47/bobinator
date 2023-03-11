use serde::{Deserialize, Serialize};

/// Communication Object for Personal struct.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication {
    #[serde(rename = "skypeUsername")]
    skype_username: Option<String>,

    #[serde(rename = "slackUsername")]
    slack_username: Option<String>,
}
