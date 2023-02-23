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

    let build_conn = Connection::new(None);

    if let Ok(conn) = build_conn {
        user_prompt(&conn).await.unwrap();
    } else {
        println!(
            "An error has occurred: {}",
            (conch::Modifier::colour("BrightYellow").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&build_conn.err().unwrap().to_string())
        )
    }
}

async fn user_prompt(conn: &reqwest::Client) -> Result<(), BobinatorError> {
    let email = String::from(_credentials::USERNAME);
    let password = String::from(_credentials::PASSWORD);

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
