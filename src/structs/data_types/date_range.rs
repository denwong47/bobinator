use std::ops::RangeInclusive;

use ::serde;
use chrono::NaiveDate;
use enum_index::*;

use super::DatePortion;
use crate::{HasDate, HasDateRange};

#[derive(Clone, Debug, EnumIndex)]
#[index_type(String)]
pub enum DateRangeType {
    #[index("days")]
    Days,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DateRange {
    #[serde(rename = "startDate")]
    pub start_date: NaiveDate,

    #[serde(rename = "startPortion")]
    pub start_portion: DatePortion,

    #[serde(rename = "endDate")]
    pub end_date: NaiveDate,

    #[serde(rename = "endPortion")]
    pub end_portion: DatePortion,

    #[serde(rename = "type")]
    pub kind: DateRangeType,
}

impl HasDate for DateRange {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        &self.start_date
    }
}

impl HasDateRange for DateRange {
    /// Return the date range it represents as a [`RangeInclusive<NaiveDate>`].
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        RangeInclusive::new(self.start_date, self.end_date)
    }
}
