use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Serialize, Deserialize)]
struct TimeoffResponse {
    pub requests: Vec<Timeoff>,
}

use bobinator_macros::map_post_to_struct;
use bobinator_models::traits::BobJSONDeserialise;
