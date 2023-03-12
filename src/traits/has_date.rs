use chrono::NaiveDate;

/// A trait indicating the struct has an associated date.
pub trait HasDate {
    /// Return the primary associated date of the instance.
    fn date<'a>(&'a self) -> &'a NaiveDate;
}

impl HasDate for NaiveDate {
    fn date<'a>(&'a self) -> &'a NaiveDate {
        return &self;
    }
}

/// A trait to allow a [`NaiveDate`] to be looked up.
/// Returns [`Some<T>`] if found, [`None`] otherwise.
pub trait FindDate<'a, T>
where
    T: HasDate,
{
    /// Check if something contains the date requested.
    fn find_date(&'a mut self, date: &'a NaiveDate) -> Option<T>;
}

impl<'a, I, T> FindDate<'a, T> for I
where
    I: Iterator<Item = T>,
    T: HasDate,
{
    /// Check if an iterable of [`HasDate`] contains the date requested.
    fn find_date(&'a mut self, date: &'a NaiveDate) -> Option<T> {
        self.find(|value| value.date() == date)
    }
}
