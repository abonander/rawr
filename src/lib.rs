#![feature(macro_rules)]

extern crate url;
extern crate time;

use std::rc::Rc;
use time::Timespec;
use url::Url;

mod client;

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
}

struct User {
    id: String,
    name: String,
    created_utc: Timespec,
    link: u32,
    comment: u32,
    over_18: bool,
    is_gold: bool,
    is_mod: bool, 
}

struct Subreddit {
    name: String,
    r_name: String,
    created_utc: Timespec,
    description: String,
}

impl Subreddit {
    fn hot(&self) -> Posts { unimplemented!(); }
    fn new(&self) -> Posts { unimplemented!(); }
}

pub struct Posts<'a> {
    subreddit: &'a Subreddit,
}

impl<'a> Iterator<Post<'a>> for Posts<'a> {
    fn next(&mut self) -> Option<Post<'a>> { unimplemented!(); }
}

pub struct Post<'a> {
    subreddit: &'a Subreddit,
    poster: User,
    created_utc: Timespec,
    title: String,
    content: PostContent,
    karma: u32,       
}

pub enum PostContent {
    Link(Url),
    Text(String),
}

impl<'a> Post<'a> {
   fn text(&self) -> Option<&str> {
        match self.content {
            Text(ref content) => Some(content.as_slice()),
            Link(_) => None,
        }
    }
    
    fn link(&self) -> Option<&Url> {
        match self.content {
            Link(ref url) => Some(url),
            Text(_) => None,
        }
    }

    fn comments(&'a self) -> Comments<'a> { unimplemented!(); }
}

pub struct Comments<'a> {
    post: &'a Post<'a>,
}

impl<'a> Iterator<Comment<'a>> for Comments<'a> {
    fn next(&mut self) -> Option<Comment<'a>> { unimplemented!(); }
}

pub struct Comment<'a> {
    post: &'a Post<'a>,
    commenter: User,
    created_utc: Timespec,
    content: String,
    karma: u32,
}

#[test]
fn it_works() {
}
