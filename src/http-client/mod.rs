use url::Url;
use std::collections::HashMap;

#[cfg(not(teepee))]
pub use self::rust-http::Client as Client;

#[cfg(teepee)]
pub use self::teepee::Client as Client;

mod rust-http;
mod teepee;

#[macro_escape]
macro_rules! params {
    {$key:expr => $val:expr,+} => (
        {
            let params = HashMap::new<String, String>();
            $(
                params.insert($key.into_string(), $val.to_string())
            )+

            params
        }                         
    );
}

pub trait HttpClient {
    /// Make a GET request, returning a string response. The GET parameters should be in the passed URL.
    fn get(&self, url: &Url) -> String;

    /// Make a POST request, returning a string response and storing any cookies
    fn post(&mut self, url: &Url, params: HashMap<String, String>) -> String;

    /// Make a POST request, including `modhash` as the `X-Modhash` header
    fn post_modhash(&mut self, url: &Url, params: HashMap<String, String>, modhash: &str) -> String;
}
