use reqwest::Client;

use chrono::{Months, NaiveDate};
use conch::{CalendarMonth, Clear, Modifier, RegionMarker};

use super::command::TimeoffMenuCommand;
use crate::{
    consts, flush_stdout, ApprovalState, BobinatorError, CalendarMonthShiftModifier, LoginSession,
    Timeoff, UserInput,
};

#[allow(unused_variables)]
pub(crate) async fn display_timeoffs<Region>(
    date: &NaiveDate,
    conn: &Client,
    session: &LoginSession,
    calendar: &CalendarMonth<Region>,
    timeoffs: &Vec<Timeoff>,
) -> Result<TimeoffMenuCommand, BobinatorError>
where
    Region: RegionMarker,
{
    timeoffs
        .iter()
        .filter(
            // Remove any timeoffs that had been rejected or canceled
            |timeoff| {
                timeoff.status == ApprovalState::Approved
                    || timeoff.status == ApprovalState::Pending
            },
        )
        .for_each(|timeoff| {
            calendar.shifted_print_for(timeoff, &timeoff.to_string());
        });

    let lines = consts::STANDARD_LINES
        .clone()
        .title("What would you like to do?")
        .extend(vec![
            "0: Book friday offs",
            "1: Previous Month",
            "2: Next Month",
            "q: Exit",
        ]);

    flush_stdout();
    println!("\n{}\n", lines);
    let input = UserInput::for_command("Enter Command: [0-2, q] ", 0..3, 1, 'q');

    Ok(match input {
        UserInput::Integer(0) => {
            print!(
                "{}",
                Modifier::up(9 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::BookFridaysOff(*date, None)
        }
        UserInput::Integer(1) => {
            print!(
                "{}",
                Modifier::up(9 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date - Months::new(1))
        }
        UserInput::Integer(2) => {
            print!(
                "{}",
                Modifier::up(9 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date + Months::new(1))
        }
        _ => TimeoffMenuCommand::Exit,
    })
}
