use conch;
use conch::StringWrapper;
use reqwest::Client;

use super::api_token;
use crate::*;

/// Prompt the user for email/password login.
pub fn login_prompt() -> Result<(String, String), BobinatorError> {
    let title_wrapper = conch::Modifier::colour("BrightWhite").unwrap()
        + conch::Modifier::intensity("Bold").unwrap();

    let email_title = title_wrapper.wraps("Email");
    let password_title = title_wrapper.wraps("Password");

    for _ in 0..3 {
        println!(
            "\nPlease login with your {} and {}.",
            &email_title, &password_title,
        );

        let email_prompt = common::UserInput::for_email(email_title.clone() + ": ", 3);
        if email_prompt == common::UserInput::Exit
            || email_prompt == common::UserInput::RetriesExceeded
        {
            return Err(BobinatorError::LoginAborted);
        }

        let password_prompt =
            common::UserInput::for_password(password_title.clone() + ": ", None, None);
        if password_prompt == common::UserInput::Exit {
            return Err(BobinatorError::LoginAborted);
        }

        match (email_prompt, password_prompt) {
            (UserInput::Text(email), UserInput::Password(password)) => {
                return Ok((email, password))
            }
            _ => (),
        }
    }

    Err(BobinatorError::LoginAborted)
}

/// Check the current machine for Token, if not found, perform a login.
pub async fn get_token_or_login(conn: &Client) -> Result<impl HasToken, BobinatorError> {
    api_token::read_token().or({
        let (email, password) = app::login_prompt()?;

        let employee = bob::login(conn, email, password).await?;
        employee.greet();

        let token = bob::get_token_scope(conn).await?;

        Ok(token.into())
    })
}
