use std::io::{self, Write};
use std::ops::Range;

use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rpassword::prompt_password as r_prompt_password;

use conch;
use conch::{Clear, Modifier, StringWrapper};

use super::consts;

/// Prompt the user for an input in text.
fn prompt_text(prompt: &impl ToString) -> io::Result<String> {
    print!("{}", prompt.to_string());

    drop(io::stdout().flush());
    let mut input = String::new();

    io::stdin().read_line(&mut input).and(Ok(input
        .trim_end_matches(|chr| {
            "\u{000A}\u{000B}\u{000C}\u{000D}\u{0085}\u{2028}\u{2029}".contains(chr)
        })
        .to_owned()))
}

/// Prompts on the TTY and then reads a password from TTY
pub fn prompt_password(prompt: &impl ToString) -> std::io::Result<String> {
    r_prompt_password(prompt.to_string())
}

/// Prompt the user for a keypress.
fn prompt_keypress(prompt: &impl ToString) -> io::Result<event::KeyEvent> {
    print!("{}", prompt.to_string());

    drop(io::stdout().flush());

    let result = loop {
        enable_raw_mode().unwrap();
        let result = event::read();
        disable_raw_mode().unwrap();
        match result {
            Ok(event::Event::Key(key)) => break result.and(Ok(key)),
            Err(err) => break Err(err),
            _ => continue,
        }
    };

    result
}

/// User Input Options.
#[derive(Clone, Debug, PartialEq)]
pub enum UserInput {
    Integer(u64),
    Char(char),
    KeyPress(event::KeyEvent),
    Text(String),
    Password(String),
    Choice(bool),
    RetriesExceeded,
    IOError(String),
    Exit,
}
impl UserInput {
    /// Do something immediately after the prompt.
    pub fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }
}
impl UserInput {
    /// Prompt for a number input.
    pub fn for_command(
        prompt: impl ToString,
        valid: Range<u64>,
        attempts: usize,
        exit_prompt: char,
    ) -> Self {
        for _ in 0..attempts {
            let result = prompt_text(&prompt);
            if let Ok(input) = result {
                if let Ok(num) = input.parse::<u64>() {
                    match num {
                        num if valid.contains(&num) => return Self::Integer(num),
                        num => println!(
                            "{}: {} {} {} {} {}",
                            consts::MODIFIER_WARNING.wraps("Wrong input:"),
                            consts::MODIFIER_EMPHASIS.wraps(&num.to_string()),
                            (conch::Modifier::colour("BrightWhite").unwrap())
                                .wraps("is not a valid command; expected"),
                            consts::MODIFIER_EMPHASIS.wraps(&valid.start.to_string()),
                            (conch::Modifier::colour("BrightWhite").unwrap()).wraps("to"),
                            consts::MODIFIER_EMPHASIS.wraps(&valid.end.to_string()),
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
                        consts::MODIFIER_WARNING.wraps("Wrong input:"),
                        (conch::Modifier::colour("BrightWhite").unwrap()).wraps("number expected.")
                    )
                }
            } else {
                println!(
                    "{}: {}",
                    consts::MODIFIER_EMPHASIS.wraps("I/O Error"),
                    (conch::Modifier::colour("BrightWhite").unwrap())
                        .wraps(&result.unwrap_or_else(|err| err.to_string()))
                )
            }
        }

        Self::RetriesExceeded
    }

    /// Prompt for a Key press.
    pub fn for_key_event(
        prompt: impl ToString,
        valid: &[event::KeyEvent],
        attempts: usize,
    ) -> Self {
        let mut prompt: String = prompt.to_string();

        for _ in 0..attempts {
            let result = prompt_keypress(&prompt);

            // We don't need to reprint after the first prompt.
            prompt = String::new();

            match result {
                // Enforce Ctrl+C as exit regardless of whether it's valid or not,
                // to prevent un-exitable scenarios.
                Ok(event::KeyEvent {
                    code: event::KeyCode::Char('c'),
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => return Self::Exit,
                Ok(event) if valid.contains(&event) => return Self::KeyPress(event),
                Ok(_) => print!(
                    "{}",
                    (Modifier::left(1) + Modifier::from(Clear::LineAfterCursor)).wraps(
                        &(consts::MODIFIER_WARNING.wraps(" Wrong input: ")
                            + &consts::MODIFIER_EMPHASIS.wraps("Try again."))
                    )
                ),
                Err(err) => return Self::IOError(err.to_string()),
            }
        }

        Self::RetriesExceeded
    }

    /// Prompt for a character input.
    pub fn for_char(
        prompt: impl ToString,
        valid: &str,
        attempts: usize,
        exit_prompt: char,
    ) -> Self {
        let input = Self::for_key_event(
            prompt,
            &valid
                .chars()
                .chain([exit_prompt].into_iter())
                .map(|chr| {
                    event::KeyEvent::new(event::KeyCode::Char(chr), event::KeyModifiers::NONE)
                })
                .collect::<Vec<event::KeyEvent>>(),
            attempts,
        );

        match input {
            // Valid Input
            Self::KeyPress(event::KeyEvent {
                code: event::KeyCode::Char(chr),
                ..
            }) if valid.contains(chr) => Self::Char(chr),

            // Exit Prompt
            Self::KeyPress(event::KeyEvent {
                code: event::KeyCode::Char(chr),
                ..
            }) if chr == exit_prompt => Self::Exit,

            others => others,
        }
    }

    /// Prompt for a number input.
    pub fn for_password(
        prompt: impl ToString,
        expects: Option<&str>,
        attempts: Option<usize>,
    ) -> Self {
        for _ in 0..attempts.unwrap_or(1) {
            let result = prompt_password(&prompt);

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
        let result = prompt_text(&prompt);

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
    pub fn for_email(prompt: impl ToString, attempts: usize) -> Self {
        for _ in 0..attempts {
            let result = prompt_text(&prompt);

            if let Ok(input) = result {
                if consts::EMAIL_PATTERN.is_match(&input) {
                    return Self::Text(input);
                } else {
                    if input.len() == 0 {
                        return Self::Exit;
                    } else {
                        println!(
                            "{} is not a valid {}, please try again.",
                            consts::MODIFIER_EMPHASIS.wraps(&input),
                            consts::MODIFIER_EMPHASIS.wraps("email"),
                        );
                    }
                }
            } else {
                return Self::Exit;
            }
        }

        Self::RetriesExceeded
    }

    /// Prompt for a bool input.
    pub fn for_choice(
        prompt: impl ToString,
        default: bool,
        attempts: Option<usize>,
        exit_prompt: char,
    ) -> Self {
        for _ in 0..attempts.unwrap_or(1) {
            let result = prompt_text(&prompt);

            if let Ok(input) = result {
                match input.len() {
                    0 => return Self::Choice(default),
                    1 => match input.chars().next().map(|chr| chr.to_ascii_lowercase()) {
                        Some('y') => return Self::Choice(true),
                        Some('n') => return Self::Choice(false),
                        Some(char) if char == exit_prompt => return Self::Exit,
                        _ => {}
                    },
                    _ => {}
                }
            } else {
                return Self::Exit;
            }
        }

        Self::RetriesExceeded
    }
}
