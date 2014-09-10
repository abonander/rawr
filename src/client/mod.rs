#![macro_escape]

use url::Url;
use std::collections::HashMap;

#[cfg(not(teepee))]
pub use self::http::Client as Client;

#[cfg(teepee)]
pub use self::teepee::Client as Client;

mod http;
mod teepee;

#[macro_export]
macro_rules! params {
    {$($key:expr: $val:expr,)+} => (
        {
            use std::collections::HashMap;
            let mut params: HashMap<String, String> = HashMap::new();
            $(
                params.insert($key.into_string(), $val.to_string());
            )+

            params
        }                         
    );
}

pub trait HttpClient {
    /// Make a GET request, returning a string response. The GET parameters should be in the passed URL.
    fn get(url: &Url) -> String;

    /// Make a POST request, returning a string response and the session cookie
    fn post_session(url: &Url, params: HashMap<String, String>) -> (String, String);

    /// Make a POST request, including `modhash` as the `X-Modhash` header
    fn post_modhash(url: &Url, params: HashMap<String, String>, modhash: &str) -> String;
}

#[test]
fn test_params() {
    let params = params!{
        "hello": "goodbye",
        "yes": "no",
    };

    drop(params);
}
