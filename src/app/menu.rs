use chrono::NaiveDate;
use reqwest::Client;

use crate::bob;
use crate::common::*;
use crate::Employee;
use bobinator_models::structs::BobinatorError;

/// Show main loop menu for commands.
pub async fn menu(conn: &Client, employee: &Employee) -> Result<(), BobinatorError> {
    loop {
        match UserInput::for_command(consts::PROMPT_FOR_COMMAND.as_str(), 0..5, usize::MAX, 'q') {
            UserInput::Integer(0) => loop {
                match UserInput::for_text("Enter date: [YYYY-MM-DD] ") {
                    UserInput::Text(text) => match NaiveDate::parse_from_str(&text, "%Y-%m-%d") {
                        Ok(date) => {
                            let result = bob::cookies::timeoff::make_friday_off_request(
                                conn, &employee, date,
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
                        Err(_) => println!("`{}` is not a valid date", text),
                    },
                    UserInput::Exit => break,
                    _ => println!("Invalid Input."),
                }
            },
            UserInput::Integer(command) => println!("{} requested.", command),
            UserInput::Exit => break,
            _ => {
                println!("{}", "Command aborted.")
            }
        }
    }

    Ok(())
}
