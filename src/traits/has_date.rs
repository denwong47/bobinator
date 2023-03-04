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
