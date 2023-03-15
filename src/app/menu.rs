use futures::stream::{FuturesOrdered, StreamExt};
use lazy_static::lazy_static;
use reqwest::Client;
use std::sync::Arc;

use conch;

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
pub async fn menu(conn: Arc<Client>, session: &LoginSession) -> Result<(), BobinatorError> {
    loop {
        println!("\n{}", MENU_PROMPT.to_string());

        match UserInput::for_char(PROMPT_FOR_COMMAND.as_str(), "01", usize::MAX, 'q').and_then(
            |input| {
                println!("");
                input
            },
        ) {
            // UserInput::Integer(0) => app::timeoff::legacy_book_fridays_off(conn, session).await?,
            UserInput::Char('0') => {
                app::timeoff::timeoff_dashboard(&Arc::clone(&conn), session).await?
            }
            UserInput::Char('1') => {
                println!("Off This Friday:");

                let off_today = Arc::new(
                    bob::cookies::timeoff::absentees_on(&Arc::clone(&conn), FridayOff::this_week())
                        .await?,
                );

                let mut tasks = FuturesOrdered::new();

                off_today.iter_requests().for_each(|timeoff| {
                    let conn_local = Arc::clone(&conn);
                    let employee_id = timeoff.employee_id().to_string();

                    tasks.push_back(tokio::spawn(async move {
                        employee_id.enquire_employee(&conn_local).await
                    }))
                });

                let mut timeoffs = off_today.iter_requests();
                while let Some(output) = tasks.next().await {
                    if let Some(timeoff) = timeoffs.next() {
                        output
                            .map_err(|err| BobinatorError::AsyncJoinError(err))
                            .and_then(|result| result)
                            .map(|employee| {
                                println!(
                                    "{} {} ({})",
                                    timeoff, employee.display_name, employee.work.department
                                )
                            })?;
                    }
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
