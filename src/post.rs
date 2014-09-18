use super::{Batched, BatchedIter, RedditResult, Session};
use sub::Subreddit;
use user::User;

use time::Timespec;

use url::Url;

pub struct Post<'a> {
    subreddit: &'a Subreddit,
    pub poster: User,
    pub created_utc: Timespec,
    pub title: String,
    name: String,
    pub content: PostContent,
    pub karma: u32,       
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

    fn comments(&'a self) -> BatchedIter<Comment<'a>> { unimplemented!(); }

    fn comment(&'a self, sess: &mut Session, content: &str) -> RedditResult<Comment<'a>> {
        unimplemented!();
    }
}

pub struct Comment<'a> {
    post: &'a Post<'a>,
    name: String,
    pub commenter: User,
    pub created_utc: Timespec,
    content: String,
    pub karma: u32,
}

impl<'a> Comment<'a> {
    pub fn content(&self) -> &str {
        self.content.as_slice()
    }

    pub fn reply(&'a self, sess: &mut Session, content: &str) -> RedditResult<Comment<'a>> {
        unimplemented!();
    }
}

impl<'a> Batched for Comment<'a> {
    fn batch(last: Option<&Comment<'a>>, size: u32) -> Vec<Comment<'a>> { unimplemented!(); }
}

