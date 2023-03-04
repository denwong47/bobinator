use lazy_static::lazy_static;
use reqwest::Client;

use conch;
use conch::StringWrapper;

use crate::common::*;
use crate::*;

lazy_static! {
    pub static ref MENU_PROMPT: conch::Lines =
        consts::STANDARD_LINES.clone().title("Menu").extend(vec![
            "0 - Book Friday Offs",
            "1 - Timeoff Management",
            "2 - Does nothing",
            "",
            "q - Exit",
        ]);
    pub static ref PROMPT_FOR_COMMAND: String = String::from("\nEnter Command: [0-4, q] ");
}

/// Show main loop menu for commands.
pub async fn menu(conn: &Client, session: &LoginSession) -> Result<(), BobinatorError> {
    loop {
        println!("\n{}", MENU_PROMPT.to_string());

        match UserInput::for_command(PROMPT_FOR_COMMAND.as_str(), 0..2, usize::MAX, 'q').and_then(
            |input| {
                println!("");
                input
            },
        ) {
            UserInput::Integer(0) => app::timeoff::book_fridays_off(conn, session).await?,
            UserInput::Integer(1) => app::timeoff::timeoff_dashboard(conn, session).await?,
            UserInput::Integer(2) => {
                println!(
                    "{}",
                    consts::MODIFIER_WARNING
                        .wraps("I told you, this does nothing.\nWhat do you expect?")
                        + "\n\nTry something else."
                );
            }
            UserInput::Integer(command) => println!("{} requested.", command),
            UserInput::Exit => break,
            _ => {
                println!("{}", "Command aborted.")
            }
        }
    }

    Ok(())
}
