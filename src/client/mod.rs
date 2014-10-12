#![macro_escape]

use serialize::json::{Json, ParserError};
use url::Url;

use std::collections::HashMap;
use std::io::IoError;
use std::local_data::Ref;

#[cfg(not(teepee))]
pub use self::http::Client;

#[cfg(teepee)]
pub use self::teepee::Client;

mod http;
mod teepee;

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

pub static USER_AGENT: &'static str = "rawr v0.1 (github.com/cybergeek94/rawr)";

local_data_key!(_modhash: String)

pub type JsonError = ParserError;

pub type JsonResult<T> = Result<T, JsonError>;

pub trait JsonClient {
    /// Make a GET request, returning a Json response. The GET parameters should be in the passed URL.
    /// Implementers should update the local modhash by using `set_modhash()`
    fn get(&self, url: &Url) -> JsonResult<Json>;

    /// Make a POST request, returning the JSON response
    fn post(&self, url: &Url, params: HashMap<String, String>) -> JsonResult<Json>;

    /// Make a POST request, including the value of `set_modhash` as the `X-Modhash` header
    /// and the session cookie
    fn post_modhash(&self, url: &Url, params: HashMap<String, String>, session: &str) -> JsonResult<Json>;
}

pub fn set_modhash(modhash: &str) {
    _modhash.replace(Some(modhash.into_string()));
}

pub fn get_modhash() -> Option<Ref<String>> {
    _modhash.get()
}

pub fn has_modhash() -> bool {
    _modhash.get().is_some()
}

/// Map a std::io::IoError to a serialize::json::IoError (ParserError variant)
pub fn err_io_to_json_io(err: IoError) -> ParserError {
    super::serialize::json::IoError(err.kind, err.desc)
}

#[test]
fn test_params() {
    let params = params!{
        "hello": "goodbye",
        "yes": "no",
    };

    drop(params);
}
