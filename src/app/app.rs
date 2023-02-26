use crate::common::consts;
use crate::{Connection, HasToken, Headers};
use bobinator_models::structs::BobinatorError;

use super::*;

use conch;
use conch::StringWrapper;

/// Run the app in command line.
pub async fn run() -> Result<(), BobinatorError> {
    println!("{}\n", consts::BOB_LOGO.as_str());
    println!(
        "{}{}.",
        (conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("Welcome to the "),
        consts::BOBINATOR_NAME.as_str(),
    );

    // Get an existing toekn, or login.
    let token = get_token_or_login().await?;

    // Build headers with authorization built-in.
    let headers = Headers::new(Some(token.key()));
    // Construct the Client.
    let conn = Connection::new(Some(headers))?;

    menu(&conn).await
}
