use enum_index::*;

#[derive(Debug, EnumIndex)]
#[index_type(String)]
pub enum DatePortion {
    #[index("all_day")]
    AllDay,

    #[index("am")]
    Morning,

    #[index("pm")]
    Afternoon,
}
