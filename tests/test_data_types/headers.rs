use reqwest::header;
use std::collections::hash_map::HashMap;

use bobinator::*;

mod test_headers_into {
    use super::*;

    macro_rules! test_factory {
        (
            $name:ident,
            $headers:expr,
            $expected:expr
            $(,)?
        ) => {
            #[test]
            fn $name() {
                assert_eq!($expected, $headers.into())
            }
        };
    }

    test_factory!(default_headers, Headers::new(), {
        let mut headers = HashMap::new();

        headers.insert(header::USER_AGENT, DEFAULT_USER_AGENT.to_string());
        headers.insert(header::ACCEPT, DEFAULT_ACCEPT.clone().join(", "));
        headers.insert(header::ACCEPT_LANGUAGE, DEFAULT_ACCEPT_LANGUAGE.to_string());
        headers.insert(
            header::ACCEPT_ENCODING,
            DEFAULT_ACCEPT_ENCODING.clone().join(", "),
        );
        headers.insert(header::REFERER, DEFAULT_REFERER.to_string());
        headers.insert(header::ORIGIN, DEFAULT_ORIGIN.to_string());
        headers.insert(header::CONNECTION, DEFAULT_CONNECTION.to_string());
        headers.insert(header::TE, DEFAULT_TE.to_string());

        header::HeaderMap::try_from(&headers).unwrap()
    });

    test_factory! (
        headers_replaced_user_agent,
        Headers::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36"),
        {
            let mut headers = HashMap::new();

            headers.insert(header::USER_AGENT, String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36"));
            headers.insert(header::ACCEPT, DEFAULT_ACCEPT.clone().join(", "));
            headers.insert(header::ACCEPT_LANGUAGE, DEFAULT_ACCEPT_LANGUAGE.to_string());
            headers.insert(header::ACCEPT_ENCODING, DEFAULT_ACCEPT_ENCODING.clone().join(", "));
            headers.insert(header::REFERER, DEFAULT_REFERER.to_string());
            headers.insert(header::ORIGIN, DEFAULT_ORIGIN.to_string());
            headers.insert(header::CONNECTION, DEFAULT_CONNECTION.to_string());
            headers.insert(header::TE, DEFAULT_TE.to_string());

            header::HeaderMap::try_from(&headers).unwrap()
        }
    );
}
