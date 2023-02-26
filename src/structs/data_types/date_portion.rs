use enum_index::*;

#[derive(Clone, Debug, EnumIndex)]
#[index_type(String)]
pub enum DatePortion {
    #[index("all_day")]
    AllDay,

    #[index("morning")]
    Morning,

    #[index("afternoon")]
    Afternoon,
}
