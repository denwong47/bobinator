use reqwest::Client;

use chrono::{Months, NaiveDate};
use conch::{CalendarMonth, Clear, Modifier, RegionMarker};

use super::command::TimeoffMenuCommand;
use crate::{
    consts, flush_stdout, ApprovalState, BobinatorError, CalendarMonthShiftModifier, HasDate,
    LoginSession, Timeoff, UserInput,
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
    (0..calendar.weeks_count()).into_iter().for_each(|n| {});

    timeoffs
        .iter()
        .filter(|timeoff| {
            [ApprovalState::Approved, ApprovalState::Pending].contains(&timeoff.status)
        })
        .fold(
            [vec![], vec![], vec![], vec![], vec![]],
            |mut weeks, timeoff| {
                calendar.week_number_of(&timeoff.date()).and_then(|n| {
                    weeks[n as usize].push(timeoff.to_string());

                    Some(())
                });

                weeks
            },
        )
        .into_iter()
        .enumerate()
        .for_each(|(n, timeoff_descs)| {
            let week_desc = timeoff_descs
                .into_iter()
                .reduce(|lhs, rhs| lhs + " " + &rhs)
                .unwrap_or_default();

            CalendarMonthShiftModifier::<NaiveDate>::shift_print_on_week(
                calendar,
                n as u32,
                (Modifier::from(Clear::LineAfterCursor).to_string() + &week_desc).as_str(),
            );
        });

    let lines = consts::STANDARD_LINES
        .clone()
        .title("What would you like to do?")
        .extend(vec![
            "f: Book friday offs",
            "<: Previous Month",
            ">: Next Month",
            "q: Exit",
        ]);

    flush_stdout();
    println!("\n{}\n", lines);
    let input = UserInput::for_char("Enter Command: [f, <, >, q] ", "f<>,.", Some('>'), 1, 'q');

    Ok(match input {
        UserInput::Char('f') => {
            print!(
                "{}",
                Modifier::up(9 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::BookFridaysOff(*date, None)
        }
        UserInput::Char('<') | UserInput::Char(',') => {
            print!(
                "{}",
                Modifier::up(9 + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date - Months::new(1))
        }
        UserInput::Char('>') | UserInput::Char('.') => {
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
