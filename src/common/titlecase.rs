//! This module is currently obsolete.

/// Trait to allow turning a string Capitalised.
pub trait Capitalise {
    fn to_capitalised(&self) -> String;
}

impl<T> Capitalise for T
where
    T: ToString,
{
    /// Make the first character of the word Capitalised.
    fn to_capitalised(&self) -> String {
        self.to_string()
            .chars()
            .into_iter()
            .enumerate()
            .map(|(id, chr)| match id {
                0 => chr.to_ascii_uppercase(),
                _ => chr,
            })
            .fold(String::new(), |mut s, chr| {
                s.push(chr);
                s
            })
    }
}

/// Trait to allow turning a string Title Case.
pub trait TitleCase {
    fn to_titlecase(&self) -> String;
}

impl<T> TitleCase for T
where
    T: ToString,
{
    /// Split a string at all whitespaces, then [`Capitalise`] each word before piecing them together.
    fn to_titlecase(&self) -> String {
        self.to_string()
            .split_whitespace()
            .map(|word| word.to_capitalised())
            .fold(String::new(), |s, word| s + &word)
    }
}
