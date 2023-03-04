use enum_index::*;
use std::fmt;

/// Units for the timeoff request range.
/// This is supported only for policy types measured in hours.
#[derive(Clone, Debug, EnumIndex)]
#[index_type(String)]
pub enum RequestRangeType {
    #[index("days")]
    Days,

    #[index("hours")]
    Hours,
}
impl Default for RequestRangeType {
    /// Defaults to [`Self::Days`].
    fn default() -> Self {
        Self::Days
    }
}
impl fmt::Display for RequestRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index())
    }
}
