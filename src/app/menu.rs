use reqwest::Client;

use crate::common::*;
use crate::Employee;
use bobinator_models::structs::BobinatorError;

/// Show main loop menu for commands.
pub async fn menu(conn: &Client, employee: &Employee) -> Result<(), BobinatorError> {
    loop {
        match UserInput::for_command(consts::PROMPT_FOR_COMMAND.as_str(), 0..5, usize::MAX, 'q') {
            UserInput::Integer(command) => println!("{} requested.", command),
            UserInput::Exit => break,
            _ => {
                println!("{}", "Command aborted.")
            }
        }
    }

    Ok(())
}
