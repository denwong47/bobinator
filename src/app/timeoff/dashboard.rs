use std::ops::RangeInclusive;

use chrono::{Datelike, Duration, Month, Months};
use num_traits::FromPrimitive;
use reqwest::Client;

use bobinator_macros::leave_trace;
use bobinator_models::structs::BobinatorError;

use conch::{regions, CalendarMonth, IterRangeByDuration, Lines};

use crate::common::consts;
use crate::{bob, ApprovalState, HasDate, LoginSession, Timeoff};

use super::TimeoffMenuCommand;

#[cfg(feature = "trace")]
use conch::StringWrapper;

/// A UI utility for timeoff booking and display.
pub(crate) async fn timeoff_dashboard(
    conn: &Client,
    session: &LoginSession,
) -> Result<(), BobinatorError> {
    leave_trace!(
        "Entering Timeoff Dashboard" | "for {}.",
        session.display_name
    );

    let mut command = TimeoffMenuCommand::default();

    while command != TimeoffMenuCommand::Exit {
        // This block will print out something like:
        //
        // ┃ March 2023
        // │
        // │  M  T  W  T  F  S  S
        // │        1  2  3  4  5     Friday Off #12505775:  approved
        // │  6  7  8  9 10 11 12     Friday Off #13092905:  canceled
        // │ 13 14 15 16 17 18 19     Friday Off #12886887:  approved
        // │ 20 21 22 23 24 25 26     Forestreet Annual Holiday Policy #12722326:  approved
        // │ 27 28 29 30 31           Forestreet Annual Holiday Policy #12722339:  approved
        let from = *command.date();

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

        // Set up [`CalendarMonth`]  instance and inject the timeoffs into its `decorated_days`.
        // This is setup using [`Iterator::fold()`] starting with a blank [`CalendarMonth`].
        let calendar = timeoffs
            .iter()
            .filter(
                // Remove any timeoffs that had been rejected or canceled
                |timeoff| {
                    timeoff.status == Some(ApprovalState::Approved)
                        || timeoff.status == Some(ApprovalState::Pending)
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

        // Convert the calendar to [`Lines`], then print it first.
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

        // Print menu prompt and do things.
        command = command.execute(conn, session, &calendar, &timeoffs).await?;
    }

    leave_trace!("Exiting Timeoff Dashboard" | "loop ended.",);
    Ok(())
}
