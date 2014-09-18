#![feature(macro_rules)]
#![allow(unused_variable)]
#![allow(dead_code)]

extern crate url;
extern crate time;

use std::cell::RefCell;
use std::iter::Peekable;
use std::vec::MoveItems;

use self::client::{HttpClient, Client};
use self::sub::Subreddit;
use self::user::{User, Message};

mod client;
pub mod user;
pub mod sub;
pub mod post;

pub type RedditResult<T> = Result<T, RedditError>;

#[deriving(Show)]
pub enum RedditError {
    /// Authentication failed. Possibly because of an invalid username/password combo,
    /// or an expired session cookie. Contains exact error message from the reddit API.
    AuthError(String),
    /// The user is not allowed to access that resource (HTTP 403).
    PermissionDenied,
    /// The requested resource was not found (HTTP 404).
    NotFound,
    /// This API call requires a modhash returned from another call that 
    /// takes `&mut Session` as a parameter. 
    NeedModhash,
    /// The submission failed because reddit is requiring the user to solve a captcha.
    NeedCaptcha,
}

/// The global reddit instance that holds the client and modhash.
/// All API endpoints require an instance of this struct to read or update the modhash.
struct Reddit {
    client: Client,
    modhash: RefCell<String>,
    batch_size: u32,
}

impl Reddit {
    pub fn new() -> Reddit {
        Reddit {
            client: Client::new(),
            modhash: RefCell::new(String::new()),
            batch_size: 50,
        }
    }

    pub fn with_batch_size(batch_size: u32) -> Reddit {
        Reddit {
            client: Client::new(),
            modhash: RefCell::new(String::new()),
            batch_size: batch_size,
        }      
    }

    #[cfg(test)]
    pub fn with_client(client: Client, batch_size: u32) -> Reddit {
        Reddit {
            client: client,
            modhash: RefCell::new(String::new()),
            batch_size: batch_size,
        } 
    }
    
    /// Login to reddit. Returns `Ok(Session)` on success, `Err(AuthError("reason"))` on failure.
    pub fn login(&self, user: &str, pass: &str, remain: bool) -> RedditResult<Session> {
        let params = params! {
            "user": user,
            "pass": pass,
            "rem": remain,
        };

        unimplemented!(); 
    }

    /// Resume a session with the given cookie string; does not make a request
    pub fn resume_session(&self, cookie: &str) -> Session {
        Session {
            cookie: cookie.into_string(),
            modhash: None,
        }
    }

    /// Find the subreddit with the given /r/ value
    pub fn sub(&self, sub: &str) -> RedditResult<Subreddit> {
       unimplemented!(); 
    }

    /// Find a user with the given /u/ value
    pub fn user(&self, user: &str) -> RedditResult<User> {
        unimplemented!();
    }
}


/// Struct representing an authenticated user session; 
/// required by any API endpoint that submits changes to reddit, such as posting to subreddits, replying to comments, etc.
pub struct Session {
    cookie: String,
}

impl Session {  
    /// Return info about the current user, retaining the modhash on `self`. 
    pub fn me(&self, reddit: &mut Reddit) -> User {
        unimplemented!();
    } 

    /// Get the session cookie to be restored later
    /// This consumes the Session
    pub fn cookie(self) -> String {
        self.cookie
    } 

    pub fn inbox(&self, reddit: &mut Reddit) -> BatchedIter<Message> {
        unimplemented!(); 
    }

    pub fn unread(&self, reddit: &mut Reddit) -> BatchedIter<Message> {
        unimplemented!();
    }

    pub fn sent(&self, reddit: &mut Reddit) -> BatchedIter<Message> {
        unimplemented!();
    }

    pub fn needs_captcha(&self, reddit: &mut Reddit) -> bool {
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

#[test]
fn it_works() {
}
