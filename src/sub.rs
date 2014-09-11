use super::{Batched, BatchedIter};
use post::Post;

use time::Timespec;

struct Subreddit {
    name: String,
    r_name: String,
    created_utc: Timespec,
    description: String,
}

impl Subreddit {
    fn hot(&self) -> BatchedIter<Post> { unimplemented!(); }
    fn new(&self) -> BatchedIter<Post> { unimplemented!(); }
}
