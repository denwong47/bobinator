use reqwest::Client;

use chrono::{Months, NaiveDate};
use conch::{CalendarMonth, Clear, Modifier, RegionMarker};
use crossterm::event;

use super::command::TimeoffMenuCommand;
use crate::{
    consts, flush_stdout, ApprovalState, BobinatorError, CalendarMonthShiftModifier, HasDate,
    LoginSession, Timeoff, UserInput,
};

const UP_AFTER_INPUT: i32 = 8;

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
            timeoff
                .status
                .as_ref()
                .map(|status| [ApprovalState::Approved, ApprovalState::Pending].contains(status))
                .unwrap_or(false)
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
    let input = UserInput::for_key_event(
        "Enter Command: [f, <, >, q] ",
        &[
            // Friday Off
            event::KeyEvent::new(event::KeyCode::Char('f'), event::KeyModifiers::NONE),
            // Previous Month
            event::KeyEvent::new(event::KeyCode::Char('<'), event::KeyModifiers::NONE),
            event::KeyEvent::new(event::KeyCode::Char(','), event::KeyModifiers::NONE),
            event::KeyEvent::new(event::KeyCode::Left, event::KeyModifiers::NONE),
            // Next Month
            event::KeyEvent::new(event::KeyCode::Char('>'), event::KeyModifiers::NONE),
            event::KeyEvent::new(event::KeyCode::Char('.'), event::KeyModifiers::NONE),
            event::KeyEvent::new(event::KeyCode::Right, event::KeyModifiers::NONE),
            // Previous Year
            event::KeyEvent::new(event::KeyCode::Left, event::KeyModifiers::SHIFT),
            // Next Year
            event::KeyEvent::new(event::KeyCode::Right, event::KeyModifiers::SHIFT),
            // Exit
            event::KeyEvent::new(event::KeyCode::Char('q'), event::KeyModifiers::NONE),
        ],
        1024, // sufficiently large where normal people won't run into it but if you hold a button you are guaranteed to exit.
    );

    Ok(match input {
        // Friday Off
        UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Char('f'),
            modifiers: event::KeyModifiers::NONE,
            ..
        }) => {
            print!(
                "{}",
                Modifier::up(UP_AFTER_INPUT + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::BookFridaysOff(*date, None)
        }

        // Previous Month
        UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Char('<'),
            modifiers: event::KeyModifiers::NONE,
            ..
        })
        | UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Char(','),
            modifiers: event::KeyModifiers::NONE,
            ..
        })
        | UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Left,
            modifiers: event::KeyModifiers::NONE,
            ..
        }) => {
            print!(
                "{}",
                Modifier::up(UP_AFTER_INPUT + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date - Months::new(1))
        }

        // Next Month
        UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Char('>'),
            modifiers: event::KeyModifiers::NONE,
            ..
        })
        | UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Char('.'),
            modifiers: event::KeyModifiers::NONE,
            ..
        })
        | UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Right,
            modifiers: event::KeyModifiers::NONE,
            ..
        }) => {
            print!(
                "{}",
                Modifier::up(UP_AFTER_INPUT + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date + Months::new(1))
        }

        // Previous Year
        UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Left,
            modifiers: event::KeyModifiers::SHIFT,
            ..
        }) => {
            print!(
                "{}",
                Modifier::up(UP_AFTER_INPUT + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date - Months::new(12))
        }

        // Next Year
        UserInput::KeyPress(event::KeyEvent {
            code: event::KeyCode::Right,
            modifiers: event::KeyModifiers::SHIFT,
            ..
        }) => {
            print!(
                "{}",
                Modifier::up(UP_AFTER_INPUT + calendar.weeks_count() as i32 + 4)
                    + Modifier::from(Clear::DisplayBelowCursor)
            );
            TimeoffMenuCommand::Display(*date + Months::new(12))
        }

        // Exit
        _ => {
            println!();
            TimeoffMenuCommand::Exit
        }
    })
}
