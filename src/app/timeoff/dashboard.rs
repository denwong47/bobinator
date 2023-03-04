use std::ops::RangeInclusive;

use chrono::offset::Local;
use chrono::{Datelike, Duration, Month, Months, NaiveDate};
use num_traits::FromPrimitive;
use reqwest::{Client, StatusCode};

use bobinator_macros::leave_trace;
use bobinator_models::structs::BobinatorError;

use conch::{
    regions, CalendarMonth, IterRangeByDuration, Lines, Modifier, MoveCursor, StringWrapper,
};

use crate::common::{consts, UserInput};
use crate::{bob, ApprovalState, LoginSession, Timeoff};

#[cfg(feature = "trace")]
use conch::StringWrapper;

/// A UI utility for timeoff booking and display.
pub async fn timeoff_dashboard(
    conn: &Client,
    session: &LoginSession,
) -> Result<(), BobinatorError> {
    leave_trace!(
        "Entering Timeoff Dashboard" | "for {}.",
        session.display_name
    );

    for month_count in 0..12 {
        let today = Local::now().date_naive();
        let from = today - Duration::days((today.day() - 1) as i64) + Months::new(month_count);

        let timeoffs: Vec<Timeoff> = bob::cookies::timeoff::list_requests(
            conn,
            session,
            Some(from),
            Some(from + Months::new(1)),
        )
        .await?;

        leave_trace!(
            "Received Timeoffs" | "{} items between {} and {}.",
            timeoffs.len(),
            from,
            from + Months::new(1)
        );

        let calendar = timeoffs
            .iter()
            .filter(
                // Remove any timeoffs that had been rejected or canceled
                |timeoff| {
                    timeoff.status == ApprovalState::Approved
                        || timeoff.status == ApprovalState::Pending
                },
            )
            .fold(
                CalendarMonth::<regions::England>::new(from.clone()),
                |calendar, timeoff| {
                    RangeInclusive::new(timeoff.start_date, timeoff.end_date)
                        .into_iter_by_duration(Duration::days(1))
                        .fold(calendar, |calendar, date| {
                            calendar.decorate_day(date, timeoff.policy_type_display_name.modifier())
                        })
                },
            );

        let lines: Lines = consts::STANDARD_LINES
            .clone()
            .title(format!(
                "{} {}",
                Month::from_u32(from.month())
                    .map(|n| n.name())
                    .unwrap_or("<Invalid Month>"),
                from.year()
            ))
            .extend((&calendar).into());

        println!("\n{}", lines);

        timeoffs.iter().for_each(|timeoff| {
            let date = &timeoff.start_date;

            calendar.week_number_of(date).map(|week_num| {
                print!(
                    "{}",
                    (Modifier::MoveCursor(MoveCursor::Up(
                        (calendar.weeks_count() - week_num) as i32
                    )) + Modifier::MoveCursor(MoveCursor::Right(25)))
                    .wraps(&format!(
                        "{} {} #{}: {}",
                        timeoff.policy_type_display_name.modifier().wraps(" "),
                        timeoff.policy_type_display_name.to_string(),
                        timeoff.id,
                        timeoff.status,
                    ))
                )
            });
        })
    }

    leave_trace!("Exiting Timeoff Dashboard" | "loop ended.",);
    Ok(())
}
