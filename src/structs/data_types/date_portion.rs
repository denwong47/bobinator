use std::fmt::{self, Display};

use enum_index::*;

#[derive(Clone, Debug, EnumIndex, PartialEq)]
#[index_type(String)]
pub enum DatePortion {
    #[index("all_day")]
    AllDay,

    #[index("morning")]
    Morning,

    #[index("afternoon")]
    Afternoon,
}
impl Display for DatePortion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AllDay => "Full Day",
                Self::Morning => "AM",
                Self::Afternoon => "PM",
            }
        )
    }
}
