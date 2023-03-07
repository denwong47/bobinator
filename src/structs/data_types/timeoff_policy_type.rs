use enum_index::*;
use std::fmt;

use conch::Modifier;

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
impl TimeoffPolicyType {
    pub fn modifier(&self) -> Modifier {
        Modifier::colour("BrightWhite").unwrap()
            + match self {
                Self::AnnualLeave => Modifier::background("R5G0B0").unwrap(),
                Self::FridayOff => Modifier::background("Grayscale13").unwrap(),
                Self::SickLeave => Modifier::background("R2G0B0").unwrap(),
            }
    }

    pub fn short_name(&self) -> &str {
        match self {
            Self::AnnualLeave => "annual leave",
            Self::SickLeave => "sick leave",
            Self::FridayOff => "friday off",
        }
    }
}
