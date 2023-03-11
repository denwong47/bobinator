use reqwest::Client;

use crate::*;
use chrono::NaiveDate;

use bobinator_macros::map_get_to_struct;
use bobinator_models::traits::BobJSONDeserialise;

type VecTimeoff = Vec<Timeoff>;

map_get_to_struct! (
    out_today,
    "Enquire the [`Timeoff`] requests for other employees that are off today.\nMust be used with cookies session.",
    "https://app.hibob.com/api/timeoff/outToday?today={today}&minimumHours=1&includeNonWorkingEvents=true",
    (today: NaiveDate),
    bob_json() -> VecTimeoff
);
