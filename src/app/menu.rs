use lazy_static::lazy_static;
use reqwest::Client;

use conch;

use chrono::{offset::Local, Duration};

use crate::common::*;
use crate::*;

lazy_static! {
    pub static ref MENU_PROMPT: conch::Lines = consts::STANDARD_LINES
        .clone()
        .title("Main Menu")
        .extend(vec!["0 - Timeoff Management", "", "q - Exit",]);
    pub static ref PROMPT_FOR_COMMAND: String = String::from("\nEnter Command: [0, q] ");
}

/// Show main loop menu for commands.
pub async fn menu(conn: &Client, session: &LoginSession) -> Result<(), BobinatorError> {
    loop {
        println!("\n{}", MENU_PROMPT.to_string());

        match UserInput::for_char(PROMPT_FOR_COMMAND.as_str(), "01", usize::MAX, 'q').and_then(
            |input| {
                println!("");
                input
            },
        ) {
            // UserInput::Integer(0) => app::timeoff::legacy_book_fridays_off(conn, session).await?,
            UserInput::Char('0') => app::timeoff::timeoff_dashboard(conn, session).await?,
            UserInput::Char('1') => {
                let off_today = bob::cookies::timeoff::absentees_on(
                    conn,
                    Local::now().date_naive() + Duration::days(3),
                )
                .await?;

                for timeoff in off_today.iter_requests() {
                    println!("{:?}", timeoff.employee_id.enquire_employee(conn).await)
                }
            }
            UserInput::Exit => break,
            i => {
                println!("{:?}", i);
                println!("{}", "Command aborted.")
            }
        }
    }

    Ok(())
}
