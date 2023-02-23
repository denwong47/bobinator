// use std::io::{stdin,stdout,Write};
use chrono::NaiveDate;
use reqwest;
use tokio;

use conch;
use conch::StringWrapper;

use bobinator::*;

#[tokio::main]
async fn main() {
    let build_conn = Connection::new(None);

    if let Ok(conn) = build_conn {
        user_prompt(&conn).await.unwrap();
    } else {
        panic!("{:?}", build_conn);
    }
}

async fn user_prompt(conn: &reqwest::Client) -> Result<(), BobinatorError> {
    // Say Hi
    println!(
        "{}{}.",
        (conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("Welcome to the "),
        (conch::Modifier::colour("BrightRed").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("bobinator")
    );

    let email = String::from("big@dave.com");
    let password = String::from("***");

    let result = bob::login(conn, email, password).await;

    if let Ok(employee) = result {
        employee.greet();
        bob::dayoff::query(
            conn,
            employee,
            NaiveDate::from_ymd_opt(2023, 2, 22).unwrap(),
            NaiveDate::from_ymd_opt(2026, 2, 2).unwrap(),
        )
        .await?;
        bob::logout(conn).await?;
    } else {
        println!(
            "An error has occurred: {}",
            (conch::Modifier::colour("BrightYellow").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&result.err().unwrap().to_string())
        )
    }

    Ok(())
}
