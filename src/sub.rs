use super::{BatchedIter, RedditResult, Session, posix_to_utc};

use json::{FromJson, find_string, find_utc};

use post::{Post, PostContent};

use serialize::json::Json;
use time::Tm;

pub struct Subreddit {
    id: String,
    name: String,
    pub created_utc: Tm,
    description: String,
    pub subscribers: u32,
}

impl Subreddit {
    pub fn hot(&self) -> BatchedIter<Post> { unimplemented!(); }
    pub fn new(&self) -> BatchedIter<Post> { unimplemented!(); }

    pub fn name(&self) -> &str { self.name.as_slice() }
    pub fn description(&self) -> &str { self.description.as_slice() }

    pub fn submit<'a>(&self, session: &Session, title: &str, content: PostContent, resumbit: bool) -> RedditResult<Post<'a>> {
        unimplemented!();
    }

}

impl FromJson for Subreddit {
    fn from_json(json: &Json) -> Option<Subreddit> {      
        let id = find_string(json, "name").map(|id| id.as_string());
        let name = find_string("display_name");
        let created_utc = find_utc(json, "created_utc");
        let description = find_string(json, "public_description");
        let subscribers = find_u32(json, "subscribers");

    }
}


