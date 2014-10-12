use super::{BatchedIter, RedditResult, Session, posix_to_utc};

use json::{FromJson, find_string, find_utc, find_u32, find_f64, find};

use post::{Post, PostContent};

use serialize::json::Json;
use time::Tm;

#[deriving(Show)]
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
		use serialize::json::{U64, F64, I64};
		let id = find_string(json, "name").map(|id| id.clone());
        let name = find_string(json, "display_name");
        let created_utc = find_f64(json, "created_utc").map(|val| posix_to_utc(val as u64));
        let description = find(json, "public_description");
        let subscribers = find_u32(json, "subscribers");

		println!("{} {} {} {} {}", id, name, created_utc, description, subscribers);

        construct_opt!(
            Subreddit{
                id: find_string(json, "name").map(|id| id.clone()),
                name: find_string(json, "display_name"),
                created_utc: find_f64(json, "created_utc").map(|val| posix_to_utc(val as u64)),
                description: find_string(json, "public_description"),
                subscribers: find_u32(json, "subscribers"),
	        }
        )
    }    
}

