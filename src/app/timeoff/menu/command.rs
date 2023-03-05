use std::ops::Add;

use chrono::offset::Local;
use chrono::{Datelike, Duration, Months, NaiveDate};

use crate::HasDate;

#[cfg(feature = "trace")]
use conch::StringWrapper;

/// Internal enum for the return value of [`timeoff_menu_for_month`].
#[derive(Debug, PartialEq)]
pub(crate) enum TimeoffMenuCommand {
    BookFridaysOff(NaiveDate, i32),
    Display(NaiveDate),
    Exit,
}
impl Default for TimeoffMenuCommand {
    fn default() -> Self {
        let today = Local::now().date_naive();
        let from = today - Duration::days((today.day() - 1) as i64);

        Self::Display(from)
    }
}
impl HasDate for TimeoffMenuCommand {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        match self {
            Self::BookFridaysOff(date, _group) => &date,
            Self::Display(date) => &date,
            // This should not be necessary.
            Self::Exit => Self::default().date(),
        }
    }
}

macro_rules! expand_add_types {
    ($($type:ty),+) => {
        $(
            impl Add<$type> for TimeoffMenuCommand {
                type Output = Self;

                /// Add a [`Duration`] to its internal [`NaiveDate`], then return a new
                /// instance of the same variant.
                fn add(self, rhs: $type) -> Self::Output {
                    let new_date = *self.date() + rhs;

                    match self {
                        Self::BookFridaysOff(date, group) => Self::BookFridaysOff(new_date, group),
                        Self::Display(date) => Self::Display(new_date),
                        Self::Exit => Self::Exit,
                    }
                }
            }
        )*
    };
}

expand_add_types!(Duration, Months);
