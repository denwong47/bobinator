use conch::StringWrapper;
use std::sync::Arc;

use crate::common::consts;
use crate::{bob, Connection};
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

    let conn = Arc::new(Connection::new(None)?);

    println!("{}", consts::PROMPT_FOR_PASSWORD_LOGIN.to_string());
    let session = try_login(&conn).await?;

    let result = menu(Arc::clone(&conn), &session).await;

    println!(
        "{}",
        bob::cookies::logout(&conn)
            .await
            .and(Ok("Logout successful."))
            .unwrap_or("Logout failed!")
    );

    result
}
