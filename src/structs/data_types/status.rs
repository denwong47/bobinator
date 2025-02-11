use conch;
use conch::StringWrapper;
use enum_index::*;
use std::fmt::Display;

/// Approval State of any requests, such as time off.
#[derive(Clone, Debug, EnumIndex, PartialEq)]
#[index_type(String)]
pub enum ApprovalState {
    #[index("approved")]
    Approved,

    #[index("pending")]
    Pending,

    #[index("rejected")]
    Rejected,

    #[index("canceled")]
    Canceled,

    #[index("not known")]
    NotKnown,
}
impl Default for ApprovalState {
    fn default() -> Self {
        Self::NotKnown
    }
}
impl Display for ApprovalState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wrapper = match self {
            Self::Approved => {
                conch::Modifier::background("Green").unwrap()
                    + conch::Modifier::colour("BrightWhite").unwrap()
            }
            Self::Pending => {
                conch::Modifier::background("Grayscale13").unwrap()
                    + conch::Modifier::colour("BrightWhite").unwrap()
            }
            Self::Rejected => {
                conch::Modifier::background("BrightRed").unwrap()
                    + conch::Modifier::colour("BrightWhite").unwrap()
            }
            Self::Canceled => {
                conch::Modifier::background("Grayscale13").unwrap()
                    + conch::Modifier::colour("Grayscale02").unwrap()
            }
            Self::NotKnown => return Ok(()),
        };

        write!(f, "{}", wrapper.wraps(&format!("{:^10}", &self.index())))
    }
}
