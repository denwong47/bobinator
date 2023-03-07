//! TODO DO WE NEED THIS??

use chrono::{Duration, Months, NaiveDate, NaiveWeek};
use std::ops::{Range, RangeInclusive};

use conch::{CalendarMonth, RegionMarker};

/// A trait indicating the struct has an associated range of dates.
pub trait HasDateRange {
    /// Return the an inclusive range of dates of the instance.
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate>;
}

impl HasDateRange for RangeInclusive<NaiveDate> {
    /// Returns a clone of itself.
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        self.clone()
    }
}

impl HasDateRange for Range<NaiveDate> {
    /// Returns a [`RangeInclusive<NaiveDate>`] with the last day of the original
    /// [`Range<NaiveDate>`] excluded.
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        RangeInclusive::new(self.start, self.end - Duration::days(1))
    }
}

impl HasDateRange for NaiveWeek {
    /// Returns a [`RangeInclusive<NaiveDate>`] for the week.
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        self.days()
    }
}

impl<Region> HasDateRange for CalendarMonth<Region>
where
    Region: RegionMarker,
{
    /// Returns a [`RangeInclusive<NaiveDate>`] for the month.
    fn date_range<'a>(&'a self) -> RangeInclusive<NaiveDate> {
        RangeInclusive::new(self.date, self.date + Months::new(1) - Duration::days(1))
    }
}
