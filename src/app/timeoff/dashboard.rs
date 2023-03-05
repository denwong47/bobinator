use std::ops::RangeInclusive;

use chrono::offset::Local;
use chrono::{Datelike, Duration, Month, Months};
use num_traits::FromPrimitive;
use reqwest::Client;

use bobinator_macros::leave_trace;
use bobinator_models::structs::BobinatorError;

use conch::{regions, CalendarMonth, IterRangeByDuration, Lines, StringWrapper};

use crate::common::{consts, flush_stdout, UserInput};
use crate::{bob, ApprovalState, CalendarMonthShiftModifier, LoginSession, Timeoff};

#[cfg(feature = "trace")]
use conch::StringWrapper;

// TESTING IMPORTS
use std::thread::sleep;
use std::time;

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

        // Set up [`CalendarMonth`]  instance and inject the timeoffs into its `decorated_days`.
        // This is setup using [`Iterator::fold()`] starting with a blank [`CalendarMonth`].
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

        // Test input
        for x in 0..3 {
            calendar.wipe_line_for(&calendar.date);
            calendar
                // .shifted_print_for(&calendar.date, &format!("Waiting {} seconds", x));
                .shifted_input_for(&calendar.date, || {
                    UserInput::for_choice("Make a choice: [Ynq] ", true, Some(1), 'q')
                });

            sleep(time::Duration::from_secs(1));
        }
    }

    leave_trace!("Exiting Timeoff Dashboard" | "loop ended.",);
    Ok(())
}
