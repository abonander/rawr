#![feature(macro_rules)]

extern crate url;
extern crate time;

use std::iter::{Chain, Peekable};
use std::rc::Rc;
use std::vec::MoveItems;

use self::sub::Subreddit;
use self::user::{User, Message};

use time::Timespec;
use url::Url;


mod client;
mod user;
mod sub;
mod post;

pub enum RedditResult<T>{
    OK(T),
    AuthError(String),
    PermissionDenied,
    NotFound,
    NeedModhash,
}

struct Reddit;

impl Reddit {
    /// Login to reddit. Returns OK(Session) on success, AuthError("reason") on failure.
    pub fn login(user: &str, pass: &str, remain: bool) -> RedditResult<Session> {
        let params = params! {
            "user": user,
            "pass": pass,
            "rem": remain,
        };

        unimplemented!(); 
    }

    fn resume_session(cookie: &str) -> Session {
        Session {
            cookie: cookie.into_string(),
            modhash: None,
        }
    }

    fn r(sub: &str) -> RedditResult<Subreddit> {
       unimplemented!(); 
    }

    fn user(user: &str) -> RedditResult<User> {
        unimplemented!();
    }

}

struct Session {
    cookie: String,
    modhash: Option<String>
}

impl Session {  
    /// Return info about the current user, setting the modhash to self. 
    pub fn me(&mut self) -> User {
        unimplemented!();
    } 

    pub fn cookie(&self) -> &str {
        self.cookie.as_slice()
    } 

    pub fn inbox(&mut self) -> BatchedIter<Message> {
        unimplemented!(); 
    } 
}

/// An iterator that fetches items in batches from an underlying data source.
/// Its item type must implement `Batched`.
pub struct BatchedIter<T> {
    size: u32,
    current: Peekable<Chain<MoveItems<T>>>,
}

impl<T: Batched> Iterator<T> for BatchedIter<T> {
    fn next(&mut self) -> Option<T> {
        let next = self.current.next();

        if self.current.peek().is_none() { 
            let batch = Batched::batch(next, self.size);

            if(!batch.is_empty()) {
                self.current = self.current.chain(batch.move_iter()).peekable();
            } 
        }
 
        next
    }
}

pub trait Batched {
    /// Get the next batch, starting from `last` if provided, the beginning if not.
    /// If no (more) results are available, return an empty vector.
    fn batch(last: Option<Self>, size: u32) -> Vec<Self>;
} 


#[test]
fn it_works() {
}
