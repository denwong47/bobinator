// use std::io::{stdin,stdout,Write};
use tokio;

use conch;
use conch::StringWrapper;

use bobinator::*;

#[tokio::main]
async fn main() {
    // Run the whole app.
    let result = app::run().await;

    drop(result.or_else(|err| {
        println!(
            "\nAn error has occurred: {}",
            (conch::Modifier::colour("BrightYellow").unwrap()
                + conch::Modifier::intensity("Bold").unwrap())
            .wraps(&err.to_string())
        );

        Err(err)
    }));
}
