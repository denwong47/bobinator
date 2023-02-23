use enum_index::*;
use serde::{Deserialize, Serialize};

/// Policy Type for mapping the Holiday Balance.
#[derive(Debug, EnumIndex, Serialize, Deserialize)]
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
