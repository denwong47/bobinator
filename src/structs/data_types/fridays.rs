use chrono::offset::Local;
use chrono::{Duration, NaiveDate, Weekday};

use crate::{BobinatorError, HasDate};

/// A struct grouping together staticmethods about Fridays.
pub struct WeekdayInterval<const D: usize, const G: usize> {}

impl<const D: usize, const G: usize> WeekdayInterval<D, G> {
    fn weekday() -> Weekday {
        match D {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            6 => Weekday::Sun,
            i => {
                panic!("WeekdayInterval cannot take {i} as D; only 0-6 permitted, with 0 = Monday.")
            }
        }
    }

    pub fn this_week() -> NaiveDate {
        let current_week = Local::now().date_naive().week(Weekday::Mon);

        current_week.first_day() + Duration::days(D as i64)
    }

    pub fn next_in_group(group: usize) -> NaiveDate {
        Self::this_week() + Duration::weeks(group as i64)
    }

    pub fn next_week() -> NaiveDate {
        Self::next_in_group(1)
    }

    /// Return the group number of a date.
    ///
    /// All Weekday `D` exactly `n` x `G` weeks away from the Weekday `D` of this week
    /// will be considered group `0`.
    ///
    /// For example, for [`WeekdayInterval<4, 2>`] (Friday, fortnightly), then this
    /// Friday (whether past or future) will be group 0. Next Friday will be group 1.
    /// The Friday after will be group 0 etc.
    pub fn get_group<T>(date: T) -> Result<usize, BobinatorError>
    where
        T: HasDate,
    {
        let date = date.date();
        let date_diff = (*date - Self::this_week()).num_days();

        if date_diff % 7 > 0 {
            // Its not the correct weekday
            Err(BobinatorError::IncorrectWeekday(
                date.clone(),
                Self::weekday().to_string(),
            ))
        } else {
            Ok((date_diff / 7) as usize % G)
        }
    }

    /// Placeholder method, to do a poll among all supplied items and guess the most
    /// probable group that the items belong to.
    pub fn guess_group<I, T>(items: I) -> Result<usize, BobinatorError>
    where
        I: Iterator<Item = T>,
        T: HasDate,
    {
        drop(items);
        todo!()
    }

    /// Return an iterator of each Weekday belonging to the specified group.
    pub fn group_iter(group: usize) -> WeekdayIntervalIter<D, G> {
        WeekdayIntervalIter::<D, G> {
            date: Self::next_in_group(group),
        }
    }
}

pub struct WeekdayIntervalIter<const D: usize, const G: usize> {
    date: NaiveDate,
}
impl<const D: usize, const G: usize> Iterator for WeekdayIntervalIter<D, G> {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        let next_date = self.date + Duration::days(G as i64 * 7);

        Some(self.date).or({
            self.date = next_date;
            None
        })
    }
}

/// Type Alias for Bi-weekly Friday offs.
pub type FridayOff = WeekdayInterval<4, 2>;
