use lazy_static::lazy_static;

use regex::Regex;

use conch;
use conch::{s, Lines, StringWrapper};

use crate::app::config;

lazy_static! {
    pub static ref STANDARD_LINES: Lines =
        Lines::new(vec![])
        .title_prefix("\u{2503} ")
        .title_modifier(conch::Modifier::colour("BrightWhite").unwrap() + conch::Modifier::intensity("Bold").unwrap())
        .prefix("\u{2502} ")
    ;

    pub static ref MODIFIER_EMPHASIS: conch::Modifier =
        conch::Modifier::colour("BrightWhite").unwrap()
        + conch::Modifier::intensity("Bold").unwrap()
    ;

    pub static ref MODIFIER_WARNING: conch::Modifier =
        conch::Modifier::colour("BrightYellow").unwrap()
        + conch::Modifier::intensity("Bold").unwrap()
    ;

    pub static ref BOBINATOR_NAME: String = s!(
        ((conch::Modifier::colour("BrightRed").unwrap()
        + conch::Modifier::intensity("Bold").unwrap()).wraps("bob"))

        ((conch::Modifier::colour("Grayscale13").unwrap()
        + conch::Modifier::intensity("Bold").unwrap()).wraps("inator"))
    );

    pub static ref BOB_LOGO: String =
        (conch::Modifier::colour("BrightRed").unwrap()
        + conch::Modifier::intensity("Bold").unwrap()).wraps(
        &s!(
            " ..\n"
            "!55Y.                                                .Y\n"
            "JP5P~                                                .5\n"
            "J55P^                                                .5\n"
            "J55P^                                                .5\n"
            "J55P^                                                .5\n"
            "J55P^  .^!7?JJ?7!^.                 .::^^:..         .5         ...\n"
            "J55P77YPPPPPPPPPPPPY7:          .~J5YJJ??JY5Y?^.     .5   .^~~~^^^^~~!~.\n"
            "J5555PP5?~:....:~?5PPPJ:      :JPY~.        :!5P7.   .5 .7!:          .~7^\n"
            "J55555!.           ~55PP~    !P5:              ~P5:  .57?.               ~?.\n"
            "J5555:              .555P^  ~PY.                :P5. .P?                  ^Y\n"
            "JP5P!                ~P5PJ  YP~                  ?P? .5.                   J^\n"
            "JP5P~                ^P55J  5P^                  7P? .5                    ?~\n"
            "~P55Y                J55P!  7PJ                 .5P^  ?~                  .5.\n"
            " ?P5PJ.            .JP5PJ    JP?               .YP7    J~                .J:\n"
            "  !5PP5?^.      .^?5PP5!      !55!.          .7P5^      !7:            .!7.\n"
            "   .75PPPP5YJJY5PPPP57.        .~Y5Y?!^^^~!?55J^          ^!~^:.....:~!~.\n"
            "      :~?Y555555Y?!:              .:~!7??7!^.                .::^^::.\n"
            "           ....                                                               \n"
        )
    );

    pub static ref EMAIL_PATTERN: Regex = Regex::new(
        r#"(?i:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
    ).unwrap();

    pub static ref PROMPT_FOR_PASSWORD_LOGIN: Lines =
        STANDARD_LINES
        .clone()
        .title("Why do I need to put in my username and password?")
        .extend(
            vec![
                format!("In order to make requests on your behalf, {} simply login to your account", BOBINATOR_NAME.as_str()),
                "as though you are using a browser, and perform the actions as you would manually.".to_owned(),
                "While bob can theoretically allow a 'service user', i.e. a robot account".to_owned(),
                "to cover the whole company, this is not currently setup, so we have to just do it".to_owned(),
                "on a personal level.".to_owned(),
                String::new(),
                format!("{} does NOT store your login credentials; they are immediately discarded", BOBINATOR_NAME.as_str()),
                "from memory upon sending of the login request.".to_owned(),
                format!("By the time {} receives a reply from the server, it would have forgotten", BOBINATOR_NAME.as_str()),
                "your password already.".to_owned(),
            ]
        );

    /// Not currently being used - our personal API tokens do NOT work with Webhooks.
    pub static ref PROMPT_FOR_ONE_TIME_TOKEN_FETCH: Lines =
        STANDARD_LINES
        .clone()
        .title("Why do I need to login?")
        .extend(
            vec![
                format!("In order to make requests on your behalf, {} needs to have an API Token", BOBINATOR_NAME.as_str()),
                "stored on your local machine. This token acts as a proof that you have granted permissions".to_owned(),
                format!("for {} to make changes.", BOBINATOR_NAME.as_str()),
                String::new(),
                format!("{} can obtain your API Token on your behalf, provided you login {} below.", BOBINATOR_NAME.as_str(),
                    MODIFIER_EMPHASIS.wraps("once")
                ),
                "Alternatively, you can copy your own API Token to the following file:".to_owned(),
                MODIFIER_EMPHASIS.wraps(&config::API_TOKEN_PATH),
                format!("and restart {}.", BOBINATOR_NAME.as_str()),
            ]
        );
}
