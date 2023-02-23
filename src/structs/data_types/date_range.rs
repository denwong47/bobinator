use super::DatePortion;
use chrono::NaiveDate;
use enum_index::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Debug, EnumIndex)]
#[index_type(String)]
pub enum DateRangeType {
    #[index("days")]
    Days,
}

#[derive(Debug)]
pub struct DateRange {
    pub start_date: NaiveDate,
    pub start_portion: DatePortion,
    pub end_date: NaiveDate,
    pub end_portion: DatePortion,
    pub kind: DateRangeType,
}
