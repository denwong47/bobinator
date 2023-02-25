use lazy_static::lazy_static;

use regex::Regex;

use conch;
use conch::StringWrapper;

lazy_static! {
    pub static ref BOBINATOR_NAME: String = (conch::Modifier::colour("BrightRed").unwrap()
        + conch::Modifier::intensity("Bold").unwrap())
    .wraps("bob")
        + &(conch::Modifier::colour("Grayscale13").unwrap()
            + conch::Modifier::intensity("Bold").unwrap())
        .wraps("inator");

    pub static ref BOB_LOGO: String = (conch::Modifier::colour("BrightRed").unwrap() + conch::Modifier::intensity("Bold").unwrap()).wraps(
" ..
!55Y.                                                .Y
JP5P~                                                .5
J55P^                                                .5
J55P^                                                .5
J55P^                                                .5
J55P^  .^!7?JJ?7!^.                 .::^^:..         .5         ...
J55P77YPPPPPPPPPPPPY7:          .~J5YJJ??JY5Y?^.     .5   .^~~~^^^^~~!~.
J5555PP5?~:....:~?5PPPJ:      :JPY~.        :!5P7.   .5 .7!:          .~7^
J55555!.           ~55PP~    !P5:              ~P5:  .57?.               ~?.
J5555:              .555P^  ~PY.                :P5. .P?                  ^Y
JP5P!                ~P5PJ  YP~                  ?P? .5.                   J^
JP5P~                ^P55J  5P^                  7P? .5                    ?~
~P55Y                J55P!  7PJ                 .5P^  ?~                  .5.
 ?P5PJ.            .JP5PJ    JP?               .YP7    J~                .J:
  !5PP5?^.      .^?5PP5!      !55!.          .7P5^      !7:            .!7.
   .75PPPP5YJJY5PPPP57.        .~Y5Y?!^^^~!?55J^          ^!~^:.....:~!~.
      :~?Y555555Y?!:              .:~!7??7!^.                .::^^::.
           ....                                                               "
    );

    pub static ref EMAIL_PATTERN: Regex = Regex::new(
        r#"(?i:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).unwrap();
}
