use lazy_static::lazy_static;
use reqwest::header;
use std::iter::Iterator;

macro_rules! expand_str_fields {
    ($($field:ident),+$(,)?) => {
        $(
            #[doc = "Chained method for altering the [`"]
            #[doc = stringify!($field)]
            #[doc = "`] of a [`Headers`] instance."]
            pub fn $field<S>(mut self, value: S) -> Self
            where
                S: ToString
            {
                self.$field = value.to_string();
                self
            }
        )*
    }
}

macro_rules! expand_vec_str_fields {
    ($($field:ident),+$(,)?) => {
        $(
            #[doc = "Chained method for altering the [`Self::"]
            #[doc = stringify!($field)]
            #[doc = "`] of a [`Headers`] instance."]
            pub fn $field<I, S>(mut self, value: I) -> Self
            where
                I: Iterator<Item=S>,
                S: ToString,
            {
                self.$field = value.map(|s| s.to_string()).collect();
                self
            }
        )*
    }
}

lazy_static! {
    pub static ref DEFAULT_USER_AGENT: &'static str =
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 12.3; rv:91.0) Gecko/20100101 Firefox/91.0";
    pub static ref DEFAULT_ACCEPT: Vec<String> = vec![
        String::from("application/json"),
        String::from("text/plain"),
        String::from("*/*")
    ];
    pub static ref DEFAULT_ACCEPT_LANGUAGE: &'static str = "en-US,en;q=0.5";
    pub static ref DEFAULT_ACCEPT_ENCODING: Vec<String> = vec![
        String::from("gzip"),
        String::from("deflate"),
        String::from("br")
    ];
    pub static ref DEFAULT_REFERER: &'static str = "https://app.hibob.com/login/";
    pub static ref DEFAULT_ORIGIN: &'static str = "https://app.hibob.com";
    pub static ref DEFAULT_CONNECTION: &'static str = "keep-alive";
    pub static ref DEFAULT_TE: &'static str = "trailers";
}

/// Constructs headers using the default of:
///
/// ```js
/// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 12.3; rv:91.0) Gecko/20100101 Firefox/91.0
/// Accept: application/json, text/plain, */*
/// Accept-Language: en-US,en;q=0.5
/// Accept-Encoding: gzip, deflate, br
/// Referer: https://app.hibob.com/login/
/// Origin: https://app.hibob.com
/// Connection: keep-alive
/// Sec-Fetch-Dest: empty
/// Sec-Fetch-Mode: cors
/// Sec-Fetch-Site: same-origin
/// TE: trailers
/// ```
#[derive(Debug, PartialEq)]
pub struct Headers {
    user_agent: String,
    accept: Vec<String>,
    accept_language: String,
    accept_encoding: Vec<String>,
    referer: String,
    origin: String,
    connection: String,
    // sec_fetch_dest: String,
    // sec_fetch_mode: String,
    // sec_fetch_site: String,
    te: String,
    authorization: Option<String>,
}
impl Headers {
    /// Create a Headers with default settings, mimicking a Firefox on OS X.
    pub fn new(authorization: Option<String>) -> Self {
        Self {
            user_agent: DEFAULT_USER_AGENT.to_string(),
            accept: DEFAULT_ACCEPT.clone(),
            accept_language: DEFAULT_ACCEPT_LANGUAGE.to_string(),
            accept_encoding: DEFAULT_ACCEPT_ENCODING.clone(),
            referer: DEFAULT_REFERER.to_string(),
            origin: DEFAULT_ORIGIN.to_string(),
            connection: DEFAULT_CONNECTION.to_string(),
            // sec_fetch_dest: String::from("empty"),
            // sec_fetch_mode: String::from("cors"),
            // sec_fetch_site: String::from("same-origin"),
            te: DEFAULT_TE.to_string(),
            authorization,
        }
    }

    expand_str_fields!(
        user_agent,
        accept_language,
        referer,
        origin,
        connection,
        // sec_fetch_dest,
        // sec_fetch_mode,
        // sec_fetch_site,
        te,
    );

    expand_vec_str_fields!(accept, accept_encoding,);

    /// Chained method for altering the [`Self::authorization`]
    /// of a [`Headers`] instance.
    pub fn authorization<S>(mut self, value: String) -> Self
    where
        S: ToString,
    {
        self.authorization = Some(value.to_string());
        self
    }
}
impl Default for Headers {
    /// Create a new [`Headers`] object with default values.
    ///
    /// Identical to [`Headers::new()`].
    fn default() -> Self {
        Self::new(None)
    }
}
impl Into<header::HeaderMap> for Headers {
    fn into(self) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();

        macro_rules! expand_str_fields {
            ($(($field:ident, $key:expr)),+$(,)?) => {
                $(
                    headers.insert($key, self.$field.parse().unwrap());
                )*
            };
        }

        macro_rules! expand_vec_str_fields {
            ($(($field:ident, $key:expr)),+$(,)?) => {
                $(
                    headers.insert($key, self.$field.join(", ").parse().unwrap());
                )*
            };
        }

        expand_str_fields!(
            (user_agent, header::USER_AGENT),
            (accept_language, header::ACCEPT_LANGUAGE),
            (referer, header::REFERER),
            (origin, header::ORIGIN),
            (connection, header::CONNECTION),
            (te, header::TE),
        );

        expand_vec_str_fields!(
            (accept, header::ACCEPT),
            (accept_encoding, header::ACCEPT_ENCODING),
        );

        if let Some(token_str) = self.authorization {
            headers.insert(header::AUTHORIZATION, token_str.parse().unwrap());
        }

        headers
    }
}
