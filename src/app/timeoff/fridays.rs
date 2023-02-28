use reqwest::Client;

use bobinator_models::structs::BobinatorError;

use conch;
use conch::StringWrapper;

use crate::common::UserInput;
use crate::{bob, Employee, FridayOff};

/// Book friday offs in sequence.
///
/// Placeholder function, just to get it working. Needs tidying up.
pub async fn book_fridays_off(conn: &Client, employee: &Employee) -> Result<(), BobinatorError> {
    let this_friday = FridayOff::this_week();
    let next_friday = FridayOff::next_week();

    println!(
        "{}",
        (conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("\u{2503} Let's book some Friday offs.")
    );

    match UserInput::for_command(
        &format!("\u{2502} Are you off on:\n\u{2502} \n\u{2502} 0 - this friday ({this_friday}), or\n\u{2502} 1 - next friday ({next_friday})?\n\nAnswer: [01 q] "),
        0..2,
        usize::MAX, 'q'
    ) {
        UserInput::Integer(group) => {
            let mut iter_friday = FridayOff::group_iter(group as usize);

            loop {
                let cur_friday = iter_friday.next().unwrap();

                match UserInput::for_choice(format!("Book Friday off on {cur_friday}? [Yn q] "), true, Some(3), 'q') {
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
                    },
                    UserInput::Choice(false) => println!(),
                    UserInput::RetriesExceeded => {
                        println!("Input not recognised.");
                        break
                    }
                    UserInput::Exit => {
                        println!("Aborted.");
                        break
                    },
                    _ => unreachable!(),
                }
            };

            ()
        },
        _ => ()
    }

    Ok(())
}
