use enum_index::*;
use std::fmt;

/// Policy Type for mapping the Holiday Balance.
#[derive(Clone, Debug, EnumIndex)]
#[index_type(String)]
pub enum TimeoffPolicyType {
    #[index("Forestreet Annual Holiday Policy")]
    AnnualLeave,

    #[index("Sick")]
    SickLeave,

    #[index("Friday Off")]
    FridayOff,
}
impl Default for TimeoffPolicyType {
    /// Defaults to [`Self::AnnualLeave`].
    fn default() -> Self {
        Self::AnnualLeave
    }
}
impl fmt::Display for TimeoffPolicyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index())
    }
}
