#![feature(macro_rules)]
#![allow(unused_variable)]
#![allow(dead_code)]

extern crate serialize;
extern crate url;
extern crate time;

use std::iter::Peekable;
use std::vec::MoveItems;

use self::client::{JsonClient, Client, JsonResult};
use self::sub::Subreddit;
use self::user::{User, Message};

use self::time::{Timespec, Tm, at_utc};
use self::url::Url;

mod client;
pub mod user;
pub mod sub;
pub mod post;

pub static BASE_URL: &'static str = "https://www.reddit.com/";

pub type RedditResult<T> = Result<T, RedditError>;

static CLIENT: Client = self::client::Client;

#[deriving(Show)]
pub enum RedditError {
    /// Authentication failed. Possibly because of an invalid username/password combo,
    /// or an expired session cookie. Contains exact error message from the reddit API.
    AuthError(String),
    /// The user is not allowed to access that resource (HTTP 403).
    PermissionDenied,
    /// The requested resource was not found (HTTP 404).
    NotFound,
    /// This API call requires a modhash 
    NeedModhash,
    /// The submission failed because reddit is requiring the user to solve a captcha.
    NeedCaptcha,
}

/// The global reddit instance that holds the client and modhash.
/// All API endpoints require an instance of this struct to read or update the modhash.

// Sucks we can't init this 
local_data_key!(batch_size: u32)

/// Get the batch size or a sensible default
pub fn get_batch_size() -> u32 {
    batch_size.get().map_or(50u32, |val| *val)
}

pub fn set_batch_size(val: u32) {
    batch_size.replace(Some(50));
}
   
/// Login to reddit. Returns `Ok(Session)` on success, `Err(AuthError("reason"))` on failure.
pub fn login(user: &str, pass: &str, remain: bool) -> RedditResult<Session> {
    let params = params! {
        "user": user,
        "pass": pass,
        "rem": remain,
    };

    unimplemented!(); 
}

/// Resume a session with the given cookie string; does not make a request
pub fn resume_session(cookie: &str) -> Session {
    Session {
        cookie: cookie.into_string(),
    }
}

/// Find the subreddit with the given /r/ value
pub fn sub(sub: &str) -> RedditResult<Subreddit> {
    let url = {
        let url = format!("{}r/{}/about.json", BASE_URL, sub);
        Url::parse(url.as_slice()).unwrap()
    };
    
    let data = CLIENT.get(&url);

    println!("{}", data);

    unimplemented!();

        
}

/// Find a user with the given /u/ value
pub fn user(user: &str) -> RedditResult<User> {
    unimplemented!();
}

/// Struct representing an authenticated user session; 
/// required by any API endpoint that submits changes to reddit, such as posting to subreddits, replying to comments, etc.
pub struct Session {
    cookie: String,
}

impl Session {  
    /// Return info about the current user, retaining the modhash on `self`. 
    pub fn me(&self) -> User {
        unimplemented!();
    } 

    /// Get the session cookie to be restored later
    /// This consumes the Session
    pub fn cookie(self) -> String {
        self.cookie
    } 

    pub fn inbox(&self) -> BatchedIter<Message> {
        unimplemented!(); 
    }

    pub fn unread(&self) -> BatchedIter<Message> {
        unimplemented!();
    }

    pub fn sent(&self) -> BatchedIter<Message> {
        unimplemented!();
    }

    pub fn needs_captcha(&self) -> bool {
        unimplemented!();
    }
}

/// An iterator that fetches items in batches from an underlying data source.
/// Its item type must implement `Batched`.
pub struct BatchedIter<T> {
    size: u32,
    current: Peekable<T, MoveItems<T>>,
}

impl<T: Batched> Iterator<T> for BatchedIter<T> {
    fn next(&mut self) -> Option<T> {
        let next = self.current.next();

        if self.current.peek().is_none() { 
            let batch = Batched::batch(next.as_ref(), self.size);

            if !batch.is_empty() {
                self.current = batch.move_iter().peekable();
            } 
        }
 
        next
    }
}

/// A trait for data types that should be fetched in batches, if possible.
pub trait Batched {
    /// Get the next batch of length `size` or smaller, 
    /// starting after `last` if provided, the beginning if not.
    /// If no (more) results are available, return an empty vector.
    fn batch(last: Option<&Self>, size: u32) -> Vec<Self>;
} 


/// Convert POSIX time (seconds since January 1, 1970 12:00 AM)
/// to Tm (UTC)
pub fn posix_to_utc(seconds: u64) -> Tm {
    // This cast is a bit dangerous,
    // as it may result in a negative value due to overflow,
    // indicating a pre-epoch time instead of the intended time.
    // However, the point at which POSIX time will overflow a 64-bit integer
    // is, according to Wolfram|Alpha, about 300 billion years in the future.
    // I think it's safe to say that we can deal with this later.
    let tmspec = Timespec::new(seconds as i64, 0);
    at_utc(tmspec)    
}

/// Lightweight utilities for working with `serialize::json::Json`.
pub mod json {
    use serialize::json::Json;

    /// Get a u64 from the given JSON and key, convert
    /// it to a Tm in UTC, assuming it is POSIX time
    pub fn find_utc(json: &Json, key: &str) -> Option<Tm> {
        find_u64(json, key).map(super::posix_to_utc)
    }

    #[inline]
    pub fn find<'a>(json: &'a Json, key: &str) -> Option<&'a Json> {
        json.find(&(key.into_string()));
    }

    pub fn find_string(json: &Json, key: &str) -> Option<String> {
        find(json, key).and_then(|j| j.as_string()).map(|s| j.into_string()) 
    }

    pub fn find_u64(json: &Json, key: &str) -> Option<u64> {
        json_find(json, key).and_then(|j| j.as_u64());
    }

    pub fn find_u32(json: &Json, key: &str) -> Option<u32> {
        json_find(json, key).and_then(|j| j.as_u32());
    }
    
    /// A lighter-weight implementation alternative of Decodable for 
    /// structs that just want to deserialize from JSON
    /// TODO: Implement this using a streaming parser
    pub trait FromJson {
        fn from_json(json: &Json) -> Option<Self>;  
    }
}
#[test]
fn it_works() {
}
