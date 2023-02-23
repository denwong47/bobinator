use lazy_static::lazy_static;

use conch;
use conch::StringWrapper;

lazy_static! {
    pub static ref BOBINATOR_NAME: String = (conch::Modifier::colour("BrightRed").unwrap()
        + conch::Modifier::intensity("Bold").unwrap())
    .wraps("bob")
        + &(conch::Modifier::colour("Grayscale13").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("inator");
}
