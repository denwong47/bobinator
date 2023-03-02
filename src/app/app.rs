use conch::StringWrapper;

use crate::common::consts;
use crate::Connection;
use bobinator_models::structs::BobinatorError;

use super::*;

/// Run the app in command line.
pub async fn run() -> Result<(), BobinatorError> {
    println!("{}\n", consts::BOB_LOGO.as_str());
    println!(
        "{}{}.\n",
        consts::MODIFIER_EMPHASIS.wraps("Welcome to the "),
        consts::BOBINATOR_NAME.as_str(),
    );

    // .. this is what we would have wrote if API tokens actually work.
    // .. No, they don't.
    // .. so back to go old login it is.
    // // Get an existing token, or login.
    // let token = get_token_or_login().await?;
    // // Build headers with authorization built-in.
    // let headers = Headers::new(Some(token.key()));
    // // Construct the Client.
    // let conn = Connection::new(Some(headers))?;

    let conn = Connection::new(None)?;
    println!("{}", consts::PROMPT_FOR_PASSWORD_LOGIN.to_string());
    let employee = try_login(&conn).await?;

    menu(&conn, &employee).await
}
