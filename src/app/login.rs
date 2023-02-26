use conch;
use conch::StringWrapper;

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
