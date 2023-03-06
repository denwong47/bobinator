use reqwest::Client;

use super::command::TimeoffMenuCommand;
use crate::{
    consts, flush_stdout, BobinatorError, CalendarMonthShiftModifier, LoginSession, Timeoff,
    UserInput,
};
use chrono::{Months, NaiveDate};
use conch::{CalendarMonth, Clear, Modifier, RegionMarker, StringWrapper};

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
    timeoffs.iter().for_each(|timeoff| {
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

    let lines = consts::STANDARD_LINES
        .clone()
        .title("What would you like to do?")
        .extend(vec![
            "0: Book all friday offs (Group 0)",
            "1: Book all friday offs (Group 1)",
            "2: Previous Month",
            "3: Next Month",
            "q: Exit",
        ]);

    flush_stdout();
    println!("\n{}\n", lines);
    let input = UserInput::for_command("Enter Command: [0-3, q] ", 0..4, 1, 'q');

    Ok(match input {
        UserInput::Integer(0) => {
            print!(
                "{}",
                Modifier::up(10) + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::BookFridaysOff(*date, 0)
        }
        UserInput::Integer(1) => {
            print!(
                "{}",
                Modifier::up(10) + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::BookFridaysOff(*date, 1)
        }
        UserInput::Integer(2) => {
            print!(
                "{}",
                Modifier::up(10 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date - Months::new(1))
        }
        UserInput::Integer(3) => {
            print!(
                "{}",
                Modifier::up(10 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date + Months::new(1))
        }
        _ => TimeoffMenuCommand::Exit,
    })
}
