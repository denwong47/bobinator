use reqwest::Client;

use bobinator_models::structs::BobinatorError;

use crate::common::{consts, UserInput};
use crate::{bob, Employee, FridayOff};

/// Book friday offs in sequence.
///
/// Placeholder function, just to get it working. Needs tidying up.
pub async fn book_fridays_off(conn: &Client, employee: &Employee) -> Result<(), BobinatorError> {
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
                            conn, &employee, cur_friday,
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