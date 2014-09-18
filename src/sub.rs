use super::{BatchedIter, RedditResult, Session};
use post::{Post, PostContent};

use time::Timespec;

pub struct Subreddit {
    name: String,
    r_name: String,
    pub created_utc: Timespec,
    description: String,
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
