extern crate http;

use self::http::client::NetworkStream;
use self::http::client::request::RequestWriter;
use self::http::client::response::ResponseReader;
use self::http::headers::host::Host;
use self::http::headers::request::{ExtensionHeader, HeaderCollection, Header, ContentType};
use self::http::headers::content_type::MediaType;
use self::http::method::{Method, Get, Post};
use super::serialize::json::{Json, from_reader};
use url::Url;
use url::form_urlencoded::serialize as url_encode;

use super::{
    JsonClient, JsonError, JsonResult, 
    set_modhash, get_modhash, has_modhash,
    err_io_to_json_io,
    USER_AGENT    
};

use std::collections::HashMap;
use std::io::{IoResult, IoError}; 

pub struct Client;

impl JsonClient for Client {
    fn get(url: &Url) -> JsonResult<Json> {
        let request = make_request(Get, url,
            |headers| default_headers(headers, url),
            |_| Ok(()),
            // Return type here is Result<Result<Json, ParserError>, IoError>
            |response| from_reader(response)
        );
    
        let result = try!(lift_result_map_err(request));

        // Update the modhash
        result.find(&("modhash".into_string()))
            .and_then(|res| res.as_string())
            .map(|modhash| set_modhash(modhash));
       
        Ok(result)
    }
    
    fn post_session(url: &Url, params: HashMap<String, String>) -> JsonResult<(Json, String)> {
        let content = encode_params(&params);

        let request = make_request(Post, url,
            |headers| {
                default_headers(headers, url);
                post_headers(headers, &content);
            },
            |request| request.write_str(content.as_slice()),
            |response| {
                let cookie = response.headers.extensions.pop(&("Set-Cookie".into_string())).unwrap_or("".into_string());
                from_reader(response).map(|json| (json, cookie.clone()))                
            }
        );        

        lift_result_map_err(request)
    }

    fn post_modhash(url: &Url, params: HashMap<String, String>, cookie: &str) -> JsonResult<Json> {
        let content = encode_params(&params);

        let request = make_request(Post, url,
            |headers| {
                default_headers(headers, url);
                post_headers(headers, &content);
                cookie_header(headers, cookie);
            },
            |request| request.write_str(content.as_slice()),
            |response| from_reader(response)
        );

        lift_result_map_err(request)
    }
}


fn make_request<T>(method: Method, url: &Url, 
    set_headers: |&mut HeaderCollection| -> (),
    write_request_body: |&mut Writer| -> IoResult<()>,
    read_response: |&mut ResponseReader<NetworkStream>| -> T) -> IoResult<T> {
    
    let mut request = try!(RequestWriter::new(method, url.clone()));
    
    set_headers(&mut request.headers);
    try!(request.connect().and_then(|_| request.write_headers()));
    try!(write_request_body(&mut request as &mut Writer));

    match request.read_response() {
        Ok(ref mut response) => Ok(read_response(response)),
        Err((_, err)) => return Err(err),
    }   
}
#[inline]
fn encode_params(params: &HashMap<String, String>) -> String {
   url_encode(params.iter().map(|(key, val)| (key.as_slice(), val.as_slice())), None)    
}

/// Lift a Result and map the outer IoError to a ParserError
#[inline]
fn lift_result_map_err<T>(res: Result<JsonResult<T>, IoError>) -> JsonResult<T> {
   res.map_err(err_io_to_json_io).and_then(|res| res) 
}

#[inline]
fn default_headers(headers: &mut HeaderCollection, url: &Url){
    headers.user_agent = Some(USER_AGENT.into_string());
    headers.host = Some(Host { name: url.host().unwrap().serialize(), port: None });    
}

#[inline]
fn post_headers(headers: &mut HeaderCollection, content: &String) {
    headers.content_type = Some(post_content_type());
    headers.content_length = Some(content.len());
    send_modhash(headers);
}

#[inline]
fn post_content_type() -> MediaType {
    MediaType::new(
        "Application".into_string(),
        "form/x-www-url-encoded".into_string(),
        Vec::new()
    )
}

#[inline]
fn cookie_header(headers: &mut HeaderCollection, cookie: &str) {
    headers.extensions.insert("Cookie".into_string(), cookie.into_string());
}

#[inline]
fn send_modhash(headers: &mut HeaderCollection) {
    get_modhash().map(|modhash| 
        headers.extensions.insert(
            "X-Modhash".into_string(), 
            modhash.clone()
        )
    );
} 
