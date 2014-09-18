use super::{RedditResult, Session};
use time::Timespec;

pub struct User {
    id: String,
    pub name: String,
    pub created_utc: Timespec,
    pub link: u32,
    pub comment: u32,
    pub over_18: bool,
    pub is_gold: bool,
    pub is_mod: bool, 
}

pub struct Message<'a> {
    id: String,
    pub from: &'a User,
    pub to: &'a User,
    pub sent: Timespec,
    title: String,
    text: String,
}

impl<'a> Message<'a> {
    pub fn title(&'a self) -> &'a str {
        self.title.as_slice()
    }

    pub fn text(&'a self) -> &'a str {
        self.text.as_slice()
    }

    pub fn reply<'b>(&self, text: &str) -> ComposeMessage<'b> {
        unimplemented!(); 
    }

    pub fn mark_read(&self, session: &Session) -> bool {
        unimplemented!();
    }

    pub fn mark_unread(&self, session: &Session) -> bool {
        unimplemented!();
    }
}

pub struct ComposeMessage<'a> {
    to: &'a User,
    pub title: String,
    pub text: String,
}

impl<'a> ComposeMessage<'a> {
    pub fn new(to: &'a User, title: &str, text: &str) -> ComposeMessage<'a> {
        ComposeMessage { 
            to: to,
            title: title.into_string(), 
            text: text.into_string() 
        }        
    }

    pub fn send(session: &Session) -> RedditResult<Message<'a>> {
        unimplemented!();
    }
}

