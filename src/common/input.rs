use std::io::{self, Write};
use std::ops::Range;

use rpassword::prompt_password;

use conch;
use conch::StringWrapper;

use super::consts;

/// Prompt the user for an input in text.
fn prompt_text(prompt: impl ToString) -> io::Result<String> {
    print!("{}", prompt.to_string());

    drop(io::stdout().flush());
    let mut input = String::new();

    io::stdin().read_line(&mut input).and(Ok(input
        .trim_end_matches(|chr| {
            "\u{000A}\u{000B}\u{000C}\u{000D}\u{0085}\u{2028}\u{2029}".contains(chr)
        })
        .to_owned()))
}

/// User Input Options.
#[derive(Clone, Debug, PartialEq)]
pub enum UserInput {
    Integer(u64),
    Text(String),
    Password(String),
    RetriesExceeded,
    Exit,
}
impl UserInput {
    /// Prompt for a number input.
    pub fn for_command(
        prompt: impl ToString,
        valid: Range<u64>,
        retries: usize,
        exit_prompt: char,
    ) -> Self {
        for _ in 0..retries {
            let result = prompt_text(prompt.to_string());
            if let Ok(input) = result {
                if let Ok(num) = input.parse::<u64>() {
                    match num {
                        num if valid.contains(&num) => return Self::Integer(num),
                        num => println!(
                            "{}: {} {} {} {} {}",
                            (conch::Modifier::colour("BrightYellow").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps("Wrong input:"),
                            (conch::Modifier::colour("BrightWhite").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps(&num.to_string()),
                            (conch::Modifier::colour("BrightWhite").unwrap())
                                .wraps("is not a valid command; expected"),
                            (conch::Modifier::colour("BrightWhite").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps(&valid.start.to_string()),
                            (conch::Modifier::colour("BrightWhite").unwrap()).wraps("to"),
                            (conch::Modifier::colour("BrightWhite").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps(&valid.end.to_string()),
                        ),
                    }
                } else {
                    if input.len() == 1
                        && input.chars().next().map_or(false, |chr| chr == exit_prompt)
                    {
                        return Self::Exit;
                    }

                    println!(
                        "{}: {}",
                        (conch::Modifier::colour("BrightYellow").unwrap()
                            + conch::Modifier::intensity("Bold").unwrap())
                        .wraps("Wrong input:"),
                        (conch::Modifier::colour("BrightWhite").unwrap()).wraps("number expected.")
                    )
                }
            } else {
                println!(
                    "{}: {}",
                    (conch::Modifier::colour("BrightYellow").unwrap()
                        + conch::Modifier::intensity("Bold").unwrap())
                    .wraps("I/O Error"),
                    (conch::Modifier::colour("BrightWhite").unwrap())
                        .wraps(&result.unwrap_or_else(|err| err.to_string()))
                )
            }
        }

        Self::RetriesExceeded
    }

    /// Prompt for a number input.
    pub fn for_password(
        prompt: impl ToString,
        expects: Option<&str>,
        retries: Option<usize>,
    ) -> Self {
        for _ in 0..retries.unwrap_or(1) {
            let result = prompt_password(prompt.to_string());

            if let Ok(input) = result {
                match input.len() {
                    0 => return Self::Exit,
                    _ => match expects {
                        Some(actual) if (actual == &input) => return Self::Password(input),
                        Some(_) => (),
                        None => return Self::Password(input),
                    },
                }
            } else {
                return Self::Exit;
            }
        }

        Self::RetriesExceeded
    }

    /// Prompt for text.
    pub fn for_text(prompt: impl ToString) -> Self {
        let result = prompt_text(prompt.to_string());

        if let Ok(input) = result {
            match input.len() {
                0 => return Self::Exit,
                _ => return Self::Text(input),
            }
        } else {
            return Self::Exit;
        }
    }

    /// Prompt for email.
    pub fn for_email(prompt: impl ToString, retries: usize) -> Self {
        for _ in 0..retries {
            let result = prompt_text(prompt.to_string());

            if let Ok(input) = result {
                if consts::EMAIL_PATTERN.is_match(&input) {
                    return Self::Text(input);
                } else {
                    if input.len() == 0 {
                        return Self::Exit;
                    } else {
                        println!(
                            "{} is not a valid {}, please try again.",
                            (conch::Modifier::colour("BrightWhite").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps(&input),
                            (conch::Modifier::colour("BrightWhite").unwrap()
                                + conch::Modifier::intensity("Bold").unwrap())
                            .wraps("email"),
                        );
                    }
                }
            } else {
                return Self::Exit;
            }
        }

        Self::RetriesExceeded
    }
}
