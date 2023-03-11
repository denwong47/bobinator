use reqwest::Client;

use super::command::TimeoffMenuCommand;
use crate::{
    bob, common, consts, ApprovalState, BobinatorError, CalendarMonthShiftModifier, FridayOff,
    LoginSession, Timeoff, TimeoffPolicyType, UserInput,
};
use chrono::{Datelike, Months, NaiveDate};
use conch::{
    CalendarMonth, Clear, ContainsDate, ContainsDateMut, Modifier, RegionMarker, StringWrapper,
};

/// Book friday offs for a certain month.
#[allow(unused_variables)]
pub(crate) async fn book_fridays_off<Region>(
    date: &NaiveDate,
    conn: &Client,
    group: Option<usize>,
    session: &LoginSession,
    calendar: &CalendarMonth<Region>,
    timeoffs: &Vec<Timeoff>,
) -> Result<TimeoffMenuCommand, BobinatorError>
where
    Region: RegionMarker,
{
    match group.or_else(|| friday_off_group_prompt()) {
        Some(group) => {
            for (_, date) in (0..3)
                .into_iter()
                .zip(FridayOff::group_iter(group, Some(calendar.date)))
                .filter(|(_, date)| {
                    if !calendar.contains(date) {
                        // If the calendar does not even contain the date, then
                        // nothing to print out.
                        false
                    } else if timeoffs
                        .iter()
                        .filter(
                            // Only check for Timeoffs that are approved or pending.
                            |timeoff| {
                                timeoff
                                    .status
                                    .as_ref()
                                    .map(|status| {
                                        [ApprovalState::Approved, ApprovalState::Pending]
                                            .contains(status)
                                    })
                                    .unwrap_or(false)
                            },
                        )
                        .contains(date)
                    {
                        // We have already booked it.
                        calendar.wipe_line_for(date);
                        calendar.shifted_print_for(
                            date,
                            &format!(
                                "{} {} is already booked as a timeoff.",
                                TimeoffPolicyType::FridayOff.modifier().wraps("\u{2573}"),
                                consts::MODIFIER_EMPHASIS.wraps(&date.format("%d/%m").to_string())
                            ),
                        );
                        common::flush_stdout();

                        false
                    } else if Region::list_holidays(calendar.date.year()).contains(date) {
                        calendar.wipe_line_for(date);
                        calendar.shifted_print_for(
                            date,
                            &format!(
                                "{} {} is a bank holiday.",
                                TimeoffPolicyType::FridayOff.modifier().wraps("\u{2573}"),
                                consts::MODIFIER_EMPHASIS.wraps(&date.format("%d/%m").to_string())
                            ),
                        );
                        common::flush_stdout();

                        false
                    } else {
                        true
                    }
                })
            {
                // Print a nice waiting message...
                calendar.wipe_line_for(&date);
                calendar.shifted_print_for(
                    &date,
                    &format!(
                        "{} Booking Friday Off for {}...",
                        TimeoffPolicyType::FridayOff.modifier().wraps("\u{2593}"),
                        consts::MODIFIER_EMPHASIS.wraps(&date.format("%d/%m").to_string())
                    ),
                );
                common::flush_stdout();

                // Sending the actual request...
                bob::cookies::timeoff::make_friday_off_request(conn, session, date)
                    .await
                    .map(|result| {
                        calendar.wipe_line_for(&date);
                        calendar.shifted_print_for(
                            &date,
                            &format!(
                                "{} Friday Off confirmed: #{}...",
                                TimeoffPolicyType::FridayOff.modifier().wraps("\u{2592}"),
                                consts::MODIFIER_EMPHASIS.wraps(
                                    &result
                                        .map(|i| i.to_string())
                                        .unwrap_or(String::from("(Not provided)"))
                                )
                            ),
                        );
                        common::flush_stdout();
                    })
                    .unwrap_or_else(|err| {
                        calendar.wipe_line_for(&date);
                        calendar.shifted_print_for(
                            &date,
                            &format!(
                                "{} Friday Off booking failed: {}",
                                TimeoffPolicyType::FridayOff.modifier().wraps("\u{2573}"),
                                consts::MODIFIER_WARNING.wraps(&format!("{:?}", err))
                            ),
                        );
                        common::flush_stdout();
                    });

                // TODO Check if approved?

                // Printing Complete message.
            }

            // Ask about what to do next
            let lines = consts::STANDARD_LINES
                .clone()
                .title("Should we continue?")
                .extend(vec!["0: Book Next Month", "q: Back to Timeoff Dashboard"]);

            println!("\n{}\n", lines);
            common::flush_stdout();

            let input = UserInput::for_command("Enter Command: [0,q] ", 0..1, 1, 'q');

            print!(
                "{}",
                Modifier::up(7 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            common::flush_stdout();

            Ok(match input {
                UserInput::Integer(0) => {
                    TimeoffMenuCommand::BookFridaysOff(*date + Months::new(1), Some(group))
                }
                _ => TimeoffMenuCommand::Display(calendar.date),
            })
        }
        None => Ok(TimeoffMenuCommand::Display(calendar.date)),
    }

    // println!(
    //     "{}",
    //     ContainsDateMut::contains(
    //         &mut timeoffs.iter().filter(|timeoff| [
    //             ApprovalState::Approved,
    //             ApprovalState::Pending
    //         ]
    //         .contains(&timeoff.status)),
    //         &NaiveDate::from_ymd_opt(2023, 3, 10).unwrap()
    //     )
    // );
}

/// Ask the user for a Friday Off Group.
///
/// Returns [`None`] if the user input is invalid.
fn friday_off_group_prompt() -> Option<usize> {
    let group = match UserInput::for_command(
        Modifier::from(Clear::DisplayBelowCursor).to_string()
            + "\n"
            + &consts::STANDARD_LINES
                .clone()
                .title("Which Friday Off group do you belong to?")
                .extend(vec![
                    format!(
                        "0 - I am off This Friday {}",
                        FridayOff::this_week().format("%d/%m")
                    ),
                    format!(
                        "1 - I am off Next Friday {}",
                        FridayOff::next_week().format("%d/%m")
                    ),
                ])
                .to_string()
            + "\n\nAnswer: [0-1,q] ",
        0..2,
        1,
        'q',
    ) {
        UserInput::Integer(i) if i <= 1 => Some(i as usize),
        _ => None,
    };

    print!(
        "{}",
        Modifier::up(7) + Modifier::from(Clear::DisplayBelowCursor)
    );

    group
}
