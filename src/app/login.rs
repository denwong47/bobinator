use conch;
use conch::StringWrapper;

use reqwest::Client;

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

/// Issue a login request to bob.
/// This is not a necessary step if we are using API Token. Only use this if you intend
/// to use cookies with username and passwords.
pub async fn try_login(conn: &Client) -> Result<Employee, BobinatorError> {
    for _ in 0..3 {
        let (email, password) = login_prompt()?;
        let result = bob::cookies::login(&conn, email, password).await;

        match result {
            Ok(employee) => {
                employee.greet();
                return Ok(employee);
            }
            Err(BobinatorError::BobUnauthorised) => {
                println!("\u{2502} Bob refused your login; please try again.")
            }
            Err(err) => return Err(err),
        }
    }

    Err(BobinatorError::BobUnauthorised)
}
