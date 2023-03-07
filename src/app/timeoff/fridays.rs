use reqwest::Client;

use super::command::TimeoffMenuCommand;
use crate::{
    bob, consts, ApprovalState, BobinatorError, FridayOff, LoginSession, Timeoff, UserInput,
};
use chrono::NaiveDate;
use conch::{CalendarMonth, ContainsDateMut, RegionMarker};

/// Book friday offs in sequence.
///
/// Placeholder function, just to get it working. Needs tidying up.
pub async fn legacy_book_fridays_off(
    conn: &Client,
    session: &LoginSession,
) -> Result<(), BobinatorError> {
    let this_friday = FridayOff::this_week();
    let next_friday = FridayOff::next_week();

    match UserInput::for_command(
        &consts::STANDARD_LINES
            .clone()
            .title("Let's book some Friday offs.")
            .extend(vec![
                "Are you off on:".to_owned(),
                String::new(),
                format!("0 - this friday ({this_friday})"),
                format!("1 - next friday ({next_friday})?"),
                String::new(),
                "Answer: [01 q] ".to_owned(),
            ]),
        0..2,
        usize::MAX,
        'q',
    ) {
        UserInput::Integer(group) => {
            let mut iter_friday = FridayOff::group_iter(group as usize);

            loop {
                let cur_friday = iter_friday.next().unwrap();

                match UserInput::for_choice(
                    format!("Book Friday off on {cur_friday}? [Yn q] "),
                    true,
                    Some(3),
                    'q',
                ) {
                    UserInput::Choice(true) => {
                        let result = bob::cookies::timeoff::make_friday_off_request(
                            conn, &session, cur_friday,
                        )
                        .await;

                        match result {
                            Ok(id) => println!("Request made: {}", id.unwrap_or(0)),
                            Err(BobinatorError::FridayOffOnNonFriday(date)) => {
                                println!("{} is not a Friday.", date)
                            }
                            Err(err) => println!("{}", err),
                        }
                    }
                    UserInput::Choice(false) => println!(),
                    UserInput::RetriesExceeded => {
                        println!("Input not recognised.");
                        break;
                    }
                    UserInput::Exit => {
                        println!("Aborted.");
                        break;
                    }
                    _ => unreachable!(),
                }
            }

            ()
        }
        _ => (),
    }

    Ok(())
}

/// Book friday offs for a certain month.
#[allow(unused_variables)]
pub(crate) async fn book_fridays_off<Region>(
    date: &NaiveDate,
    conn: &Client,
    group: usize,
    session: &LoginSession,
    calendar: &CalendarMonth<Region>,
    timeoffs: &Vec<Timeoff>,
) -> Result<TimeoffMenuCommand, BobinatorError>
where
    Region: RegionMarker,
{
    println!(
        "{}",
        ContainsDateMut::contains(
            &mut timeoffs.iter().filter(|timeoff| [
                ApprovalState::Approved,
                ApprovalState::Pending
            ]
            .contains(&timeoff.status)),
            &NaiveDate::from_ymd_opt(2023, 3, 10).unwrap()
        )
    );

    todo!("Booking Fridays Off through dashboard is not currently supported.")
    // futures::future::join_all(get_player_futures).await;
}
