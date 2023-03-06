use std::ops::Add;

use chrono::offset::Local;
use chrono::{Datelike, Duration, Months, NaiveDate};

use reqwest::Client;

use conch::{CalendarMonth, RegionMarker, StringWrapper};

use crate::{
    BobinatorError,
    CalendarMonthShiftModifier,
    HasDate,
    LoginSession,
    Timeoff,
    // common,
};

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
            Self::Exit => {
                panic!("The Exit command does not have a date; unsupported use of feature.")
            }
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

impl TimeoffMenuCommand {
    /// Consumes and execute the command.
    ///
    /// The command would have embedded the necessary information to perform the
    /// execution, along with the parameters from the dashboard.
    ///
    /// Returns a [`Result<Self, BobinatorError>`], indicating to the dashboard
    /// the next command to call.
    pub(crate) async fn execute<Region>(
        self,
        conn: &Client,
        session: &LoginSession,
        calendar: &CalendarMonth<Region>,
        timeoffs: &Vec<Timeoff>,
    ) -> Result<TimeoffMenuCommand, BobinatorError>
    where
        Region: RegionMarker,
    {
        match &self {
            Self::BookFridaysOff(date, group) => {
                todo!()
            }
            Self::Display(date) => {
                // Print out annotations for timeoffs.
                timeoffs.iter().for_each(|timeoff| {
                    let date = &timeoff.start_date;

                    calendar.shifted_print_for(
                        timeoff,
                        &format!(
                            "{} {} #{}: {}",
                            timeoff.policy_type_display_name.modifier().wraps(" "),
                            timeoff.policy_type_display_name.to_string(),
                            timeoff.id,
                            timeoff.status,
                        ),
                    );
                });

                if *date - Local::now().date_naive() >= Duration::days(365) {
                    Ok(Self::Exit)
                } else {
                    Ok(self + Months::new(1))
                }
            }
            Self::Exit => unreachable!(
                "And Exit command was executed directly; this is only supported as a return value."
            ),
        }
    }
}
