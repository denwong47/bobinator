// use std::io::{stdin,stdout,Write};
use chrono::NaiveDate;
use reqwest;
use tokio;

use conch;
use conch::StringWrapper;

use bobinator::*;

mod _credentials;

#[tokio::main]
async fn main() {
    // Say Hi
    println!(
        "{}{}.",
        (conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("Welcome to the "),
        consts::BOBINATOR_NAME.as_str(),
    );

    let mut result = Connection::new(None);

    if let Ok(conn) = result {
        result = user_prompt(&conn).await.and(Ok(conn))
    }

    drop(result.or_else(|err| {
        println!(
            "An error has occurred: {}",
            (conch::Modifier::colour("BrightYellow").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&err.to_string())
        );

        Err(err)
    }));
}

async fn user_prompt(conn: &reqwest::Client) -> Result<(), BobinatorError> {
    let email = String::from(_credentials::USERNAME);
    let password = String::from(_credentials::PASSWORD);

    let employee = bob::login(conn, email, password).await?;

    employee.greet();
    let timeoffs = bob::timeoff::query(
        conn,
        employee,
        NaiveDate::from_ymd_opt(2023, 2, 22).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 2).unwrap(),
    )
    .await?;

    println!("\nYour booked timeoffs:");
    timeoffs.iter().for_each(|timeoff| {
        println!(
            "{}{}{}{}{}{}",
            conch::Modifier::colour("Grayscale13")
                .unwrap()
                .wraps(&timeoff.id.to_string()),
            (conch::Modifier::colour("BrightWhite").unwrap()
                + conch::Modifier::intensity("Bold").unwrap()
                + conch::Modifier::right(12))
            .wraps(&timeoff.start_date),
            (conch::Modifier::colour("Grayscale13").unwrap() + conch::Modifier::right(23))
                .wraps("to"),
            (conch::Modifier::colour("BrightWhite").unwrap()
                + conch::Modifier::intensity("Bold").unwrap()
                + conch::Modifier::right(26))
            .wraps(&timeoff.end_date),
            (conch::Modifier::right(37)).wraps(&timeoff.status.to_string()),
            (conch::Modifier::right(50)).wraps(&timeoff.policy_type_display_name.name())
        )
    });

    println!("\nYour current API Token:");
    let mut token = bob::get_token_scope(conn).await?;
    println!("{:?}", token);

    token.extend_scopes(vec![
        APITokenScope::FullEmployeeRead,
        APITokenScope::Timeoff,
        APITokenScope::EmployeeFieldsRead,
    ]);

    token.sync_scopes(conn).await?;

    bob::logout(conn).await.and({
        println!("Log out succeeded, bye!");
        Ok(())
    })
}
