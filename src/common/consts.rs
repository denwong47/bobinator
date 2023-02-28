use lazy_static::lazy_static;

use regex::Regex;

use conch;
use conch::StringWrapper;

use crate::app::config;

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

    pub static ref PROMPT_FOR_PASSWORD_LOGIN: String =
        (
            conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap()
        ).wraps("\u{2503} Why do I need to put in my username and password?")
        + "\n\u{2502}\n\u{2502} In order to make requests on your behalf, "
        + BOBINATOR_NAME.as_str()
        + " simply login to your account\n"
        + "\u{2502} as though you are using a browser, and perform the actions as you would manually.\n"
        + "\u{2502}\n"
        + "\u{2502} While bob can theoretically allow a 'service user', i.e. a robot account\n"
        + "\u{2502} to cover the whole company, this is not currently setup, so we have to just do it \n"
        + "\u{2502} on a personal level.\n\u{2502}\n"
        + "\u{2502} "
        + BOBINATOR_NAME.as_str()
        + " does "
        + &(
            conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap()
        ).wraps("NOT")
        + " store your login credentials; they are immediately discarded\n"
        + "\u{2502} from memory upon sending of the login request.\n"
        + "\u{2502} By the time "
        + BOBINATOR_NAME.as_str()
        + " receives a reply from the server, it would have \n"
        + "\u{2502} forgotten your password already.";

        /// Not currently being used - our personal API tokens do NOT work with Webhooks.
        pub static ref PROMPT_FOR_ONE_TIME_TOKEN_FETCH: String =
        (
            conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap()
        ).wraps("\u{2503} Why do I need to login?")
        + "\n\u{2502}\n\u{2502} In order to make requests on your behalf, "
        + BOBINATOR_NAME.as_str()
        + " needs to have an API Token\n\u{2502} stored on your local machine. "
        + "This token acts as a proof that you have granted permissions\n\u{2502}\n\u{2502} for "
        + BOBINATOR_NAME.as_str()
        + "to make changes.\n\u{2502}\n\u{2502} "
        + BOBINATOR_NAME.as_str()
        + " can obtain your API Token on your behalf, provided you login "
        + &(
            conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap()
        ).wraps("once")
        + " below.\n\u{2502}\n\u{2502} "
        + "Alternatively, you can copy your own API Token to the following file:\n\u{2502}   "
        + &(
            conch::Modifier::colour("BrightWhite").unwrap()
            + conch::Modifier::intensity("Bold").unwrap()
        ).wraps(&config::API_TOKEN_PATH)
        + "\n\u{2502} then restart "
        + BOBINATOR_NAME.as_str()
        + ".";
}
