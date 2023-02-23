use enum_index::*;

/// Policy Type for mapping the Holiday Balance.
#[derive(Debug, EnumIndex)]
#[index_type(String)]
pub enum TimeoffPolicyType {
    #[index("Forestreet Annual Holiday Policy")]
    AnnualLeave,

    #[index("Sick")]
    SickLeave,

    #[index("Friday Off")]
    FridayOff,
}
