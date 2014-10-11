use super::{BatchedIter, RedditResult, Session, posix_to_utc};

use json::{FromJson, find_string, find_utc, find_u32};

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
        construct_opt!(
            Subreddit{
                id: find_string(json, "name").map(|id| id.clone()),
                name: find_string(json, "display_name"),
                created_utc: find_utc(json, "created_utc"),
                description: find_string(json, "public_descrption"),
                subscribers: find_u32(json, "subscribers"),
	        }
        )
    }    
}

