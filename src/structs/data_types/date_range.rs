use super::DatePortion;
use ::serde;
use chrono::NaiveDate;
use enum_index::*;

use crate::HasDate;

#[derive(Debug, EnumIndex)]
#[index_type(String)]
pub enum DateRangeType {
    #[index("days")]
    Days,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DateRange {
    pub start_date: NaiveDate,
    pub start_portion: DatePortion,
    pub end_date: NaiveDate,
    pub end_portion: DatePortion,
    pub kind: DateRangeType,
}

impl HasDate for DateRange {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        &self.start_date
    }
}
