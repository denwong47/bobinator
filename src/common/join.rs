use std::iter::Iterator;

/// Public trait for joining an [`Iterator`] of [`ToString`] together
/// with separator.
pub trait JoinString {
    fn join(&mut self, sep: &str) -> String;
}

impl<I, S> JoinString for I
where
    I: Iterator<Item = S>,
    S: ToString,
{
    /// Join together all the iterated items as a string, separated by `sep`.
    fn join(&mut self, sep: &str) -> String {
        self.map(|s| s.to_string())
            .reduce(|lhs, rhs| lhs + sep + &rhs)
            .unwrap_or(String::new())
    }
}
